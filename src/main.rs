extern crate crypto;
extern crate chrono;

use chrono::*;

use crypto::digest::Digest;
use crypto::md5::Md5;

use std::fmt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::time;
// use std::env;

 
// CAFE BABE
const MAGIC: [u8; 4]  = [202, 254, 186, 190];
 
// Constant Pool Tags
const CONSTANT_CLASS              : u8 = 7;
const CONSTANT_FIELDREF           : u8 = 9;
const CONSTANT_METHODREF          : u8 = 10;
const CONSTANT_INTERFACEMETHODREF : u8 = 11;
const CONSTANT_STRING             : u8 = 8;
const CONSTANT_INTEGER            : u8 = 3;
const CONSTANT_FLOAT              : u8 = 4;
const CONSTANT_LONG               : u8 = 5;
const CONSTANT_DOUBLE             : u8 = 6;
const CONSTANT_NAMEANDTYPE        : u8 = 12;
const CONSTANT_UTF8               : u8 = 1;
const CONSTANT_METHODHANDLE       : u8 = 15;
const CONSTANT_METHODTYPE         : u8 = 16;
const CONSTANT_INVOKEDYNAMIC      : u8 = 18;
 
// ------------------------------------------------------
// FIXME depends on endiaess
// TODO put results into structs
// TODO check different Java versions and platforms
// ------------------------------------------------------

struct Constant {
    number: u8,
    constant_type: u8,
    type_name: String,
    value: String,
    comment: String,
    size: u8,
}

impl fmt::Display for Constant {
    fn fmt(&self, c: &mut fmt::Formatter) -> fmt::Result {
        if self.comment.is_empty() {
            write!(c, "\t#{} = {}\t\t{}", self.number, self.type_name, self.value)
        } else {
            if self.constant_type == CONSTANT_NAMEANDTYPE {
                write!(c, "\t#{} = {}\t{}\t\t\t// {})", self.number, self.type_name, self.value, self.comment)
            } else {
                write!(c, "\t#{} = {}\t\t{}\t\t\t// {})", self.number, self.type_name, self.value, self.comment)
            }
        }
    }
}

fn main() {
 
    println!("___________________________________________");
    // let filename = get_filename();
    let filename = "test/Test.class";
    println!("{}", filename);

    let mut f = File::open(filename).expect("Unable to open file");
    let mut data = Vec::new();
    f.read_to_end(&mut data).expect("Unable to read data");
   
    // assert magic
    assert_eq!(&MAGIC[..], &data[0..4], "Error: class not found: {}", filename);
    // seems to be a valid .class file
    println!("Classfile {}", filename);
 
    let modified_timestamp = read_last_modified(filename);
    print!("  Last modified {}; ", modified_timestamp);
 
    // Size in bytes
    println!("size: {} bytes", data.len());
 
    // calculate md5 checksum
    let mut md5 = Md5::new();
    md5.input(&data);
    println!("  MD5 checksum {}", md5.result_str());

    // compiled from
    let compiled_from = "NOT IMPLEMENTED";
    println!("  Compiled from \"{}\"", compiled_from);
 
    // read minor and major versions
    let (minor, major) = read_version(&data);
    println!("  minor version: {}", minor);
    println!("  major version: {}", major);
 
    // read constant pool count
    let constant_pool_count = read_constant_pool_count(&data);
    println!("Constant pool count: {}", constant_pool_count);
    println!("Constant pool:");
 
    // read constant pool
    // TODO
    read_constant_pool(&data, constant_pool_count);
 
    // read access flags
    // TODO
 
    println!("");
    println!("Bytes:\n{:?}\n", data);
 
    // let hex = data.iter().map(|&x| format!("{:X}", x)).collect::<Vec<_>>();
    // println!("Hex:\n{:?}\n", hex);
}

// fn get_filename() -> &'static String {
//     let args: Vec<String> = env::args().collect();
//     if args.len() < 2 {
//         panic!("Please specify a .class file!");
//     } else {
//         return &args[1];
//     }
// }

fn read_version(data: &[u8]) -> (u8, u8) {
    let minor = data[4] << 2 | data[5];
    let major = data[6] << 2 | data[7];
    return (minor, major);
}

fn read_constant_pool_count(data: &[u8]) -> u8 {
    // The value of the constant_pool_count item is equal
    // to the number of entries in the constant_pool table plus one
    return (data[8] << 2 | data[9]) - 1;
}

fn read_constant_pool(data: &[u8], count: u8) {
    // constants read so far
    let mut read = 0;
    // current tag index
    let mut current = 10;
    while read < count - 1 {
        let tag = data[current];
        read = read + 1;
        let constant = match tag as u8 {
            CONSTANT_METHODREF => read_constant_method_ref(&data, current, read),
            CONSTANT_NAMEANDTYPE => read_constant_name_and_type(&data, current, read),
            CONSTANT_INTERFACEMETHODREF => read_constant_interface_method_ref(&data, current, read),
            CONSTANT_FIELDREF  => read_constant_field_ref(&data, current, read),
            CONSTANT_STRING    => read_constant_string(&data, current, read),
            CONSTANT_CLASS     => read_constant_class(&data, current, read),
            CONSTANT_UTF8      => read_constant_utf8(&data, current, read),
            CONSTANT_INTEGER   => read_constant_integer(&data, current, read),
            // TODO
            CONSTANT_LONG      => read_constant_long(&data, current, read),
            CONSTANT_FLOAT     => read_constant_float(&data, current, read),
            CONSTANT_DOUBLE    => read_constant_double(&data, current, read),
            CONSTANT_METHODHANDLE => read_constant_method_handle(&data, current, read),
            CONSTANT_METHODTYPE  => read_constant_method_type(&data, current, read),
            CONSTANT_INVOKEDYNAMIC => read_constant_invoke_dynamic(&data, current, read),
            _  => read_constant_unknown(&data, current),
        };
        println!("{}", constant);
        current = current + (constant.size as usize);
    }
}

fn read_constant_method_ref(data: &[u8], current: usize, read: u8) -> Constant {
    let class_index = &data[current + 1] << 2 | &data[current + 2];
    let name_and_type_index = &data[current + 3] << 2 | &data[current + 4];
    let result = Constant {number: read,
                           constant_type: CONSTANT_METHODREF,
                           type_name: "Methodref".to_string(),
                           value: format!("#{}.#{}", class_index, name_and_type_index),
                           comment: "NOT IMPLEMENTED".to_string(),
                           size: 5};
    return result;
}

fn read_constant_string(data: &[u8], current: usize, read: u8) -> Constant {
    let string_index = &data[current + 1] << 2 | &data[current + 2];

    let result = Constant {number: read,
                           constant_type: CONSTANT_STRING,
                           type_name: "String".to_string(),
                           value: format!("#{}", string_index),
                           comment: "NOT IMPLEMENTED".to_string(),
                           size: 3};
    return result;
}

fn read_constant_field_ref(data: &[u8], current: usize, read: u8) -> Constant {
    let class_index = &data[current + 1] << 2 | &data[current + 2];
    let name_and_type_index = &data[current + 3] << 2 | &data[current + 4];

    let result = Constant {number: read,
                           constant_type: CONSTANT_FIELDREF,
                           type_name: "Fieldref".to_string(),
                           value: format!("#{}.#{}", class_index, name_and_type_index),
                           comment: "NOT IMPLEMENTED".to_string(),
                           size: 5};
    return result;
}

fn read_constant_interface_method_ref(data: &[u8], current: usize, read: u8) -> Constant {
    let class_index = &data[current + 1] << 2 | &data[current + 2];
    let name_and_type_index = &data[current + 3] << 2 | &data[current + 4];

    let result = Constant {number: read,
                           constant_type: CONSTANT_FIELDREF,
                           type_name: "InterfaceMethodref".to_string(),
                           value: format!("#{}.#{}", class_index, name_and_type_index),
                           comment: "NOT IMPLEMENTED".to_string(),
                           size: 5};
    return result;
}

fn read_constant_class(data: &[u8], current: usize, read: u8) -> Constant {
    let name_index = &data[current + 1] << 2 | &data[current + 2];

    let result = Constant {number: read,
                           constant_type: CONSTANT_CLASS,
                           type_name: "Class".to_string(),
                           value: format!("#{}", name_index),
                           comment: "NOT IMPLEMENTED".to_string(),
                           size: 3};
    return result;
}

fn read_constant_utf8(data: &[u8], current: usize, read: u8) -> Constant {
    let length = &data[current + 1] << 2 | &data[current + 2];
    let start = current + 3;
    let end = start + (length as usize);
    let utf8_string = match str::from_utf8(&data[start..end]) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let result = Constant {number: read,
                           constant_type: CONSTANT_UTF8,
                           type_name: "Utf8".to_string(),
                           value: format!("{}", utf8_string),
                           comment: "".to_string(),
                           size: (3 as u8) + length};
    return result;
}

fn read_constant_integer(data: &[u8], current: usize, read: u8) -> Constant {
    let bytes = &data[current + 1] << 6 | &data[current + 2] << 4 |
                &data[current + 3] << 2 | &data[current + 4] << 0;

    let result = Constant {number: read,
                           constant_type: CONSTANT_INTEGER,
                           type_name: "Integer".to_string(),
                           value: format!("{}", bytes),
                           comment: "".to_string(),
                           size: 5};
    return result;
}

fn read_constant_name_and_type(data: &[u8], current: usize, read: u8) -> Constant {
    let name_index = &data[current + 1] << 2 | &data[current + 2];
    let descriptor_index = &data[current + 3] << 2 | &data[current + 4];

    let result = Constant {number: read,
                           constant_type: CONSTANT_NAMEANDTYPE,
                           type_name: "NameAndType".to_string(),
                           value: format!("#{}.#{}", name_index, descriptor_index),
                           comment: "NOT IMPLEMENTED".to_string(),
                           size: 5};
    return result;
}

fn read_constant_unknown(data: &[u8], current: usize) -> Constant {
    panic!("Unknown CONSTANT tag type: {}!", &data[current]);
}

fn read_constant_long(data: &[u8], current: usize, read: u8) -> Constant {
    let result = Constant {number: read,
                           constant_type: CONSTANT_LONG,
                           type_name: "Long".to_string(),
                           value: format!("{}", "NOT IMPLEMENTED"),
                           comment: "".to_string(),
                           size: 9};
    return result;
}

fn read_constant_float(data: &[u8], current: usize, read: u8) -> Constant {
    let result = Constant {number: read,
                           constant_type: CONSTANT_FLOAT,
                           type_name: "Float".to_string(),
                           value: format!("{}", "NOT IMPLEMENTED"),
                           comment: "".to_string(),
                           size: 9};
    return result;
}

fn read_constant_double(data: &[u8], current: usize, read: u8) -> Constant {
    let result = Constant {number: read,
                           constant_type: CONSTANT_DOUBLE,
                           type_name: "Double".to_string(),
                           value: format!("{}", "NOT IMPLEMENTED"),
                           comment: "".to_string(),
                           size: 9};
    return result;
}

fn read_constant_invoke_dynamic(data: &[u8], current: usize, read: u8) -> Constant {
    panic!("NOT IMPLEMENTED");
}

fn read_constant_method_type(data: &[u8], current: usize, read: u8) -> Constant {
    panic!("NOT IMPLEMENTED");
}

fn read_constant_method_handle(data: &[u8], current: usize, read: u8) -> Constant {
    panic!("NOT IMPLEMENTED");
}

fn read_last_modified(filename: &str) -> String {
    let metadata = fs::metadata(filename);
    let modified_timestamp = match metadata {
        Ok(time) => time.modified().unwrap().duration_since(time::UNIX_EPOCH).unwrap().as_secs(),
        Err(e)   => panic!("Unable to get last_modified for {}", filename),
    };
    return UTC.timestamp((modified_timestamp as i64), 0).format("%d/%m/%Y").to_string();
}