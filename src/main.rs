extern crate crypto;
extern crate chrono;

use chrono::*;

use crypto::digest::Digest;
use crypto::md5::Md5;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::f32;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::time;
// use std::env;

 
// CAFE BABE
const MAGIC: [u8; 4]  = [202, 254, 186, 190];

const POSITIVE_INFINITY_32: u32 = 0x7f800000;
const NEGATIVE_INFINITY_32: u32 = 0xff800000;
const POSITIVE_INFINITY_64: u64 = 0x7ff0000000000000;
const NEGATIVE_INFINITY_64: u64 = 0xfff0000000000000;
 
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

// All 8-byte constants take up two entries in the constant_pool table of the class file. 
static EIGHT_BYTE_CONSTANTS: &'static [u8] = &[CONSTANT_LONG, CONSTANT_DOUBLE];
 
// ----------------------------------------------------------------------------------
// FIXME depends on endiaess
// TODO check different Java versions and platforms
// TODO different CONSTANT types print their values a bit differently
// ----------------------------------------------------------------------------------

struct Constant {
    constant_type: u8,
    type_name: String,
    references:  Vec<u8>,
    value: String,
    size: u8,
}

impl fmt::Display for Constant {
    fn fmt(&self, c: &mut fmt::Formatter) -> fmt::Result {
        if self.constant_type == CONSTANT_NAMEANDTYPE {
            write!(c, "{}\t{}\t\t\t", self.type_name, self.value)
        } else {
            write!(c, "{}\t\t{}\t\t\t", self.type_name, self.value)
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
    let mut constant_pool = HashMap::new();
    let mut used_indexes = Vec::new();
    while read < count {
        let tag = data[current];
        read = read + 1;
        let constant = match tag as u8 {
            CONSTANT_METHODREF => read_constant_method_ref(&data, current),
            CONSTANT_NAMEANDTYPE => read_constant_name_and_type(&data, current),
            CONSTANT_INTERFACEMETHODREF => read_constant_interface_method_ref(&data, current),
            CONSTANT_FIELDREF  => read_constant_field_ref(&data, current),
            CONSTANT_STRING    => read_constant_string(&data, current),
            CONSTANT_CLASS     => read_constant_class(&data, current),
            CONSTANT_UTF8      => read_constant_utf8(&data, current),
            CONSTANT_INTEGER   => read_constant_integer(&data, current),
            // TODO
            CONSTANT_LONG      => read_constant_long(&data, current),
            CONSTANT_FLOAT     => read_constant_float(&data, current),
            CONSTANT_DOUBLE    => read_constant_double(&data, current),
            CONSTANT_METHODHANDLE => read_constant_method_handle(&data, current),
            CONSTANT_METHODTYPE  => read_constant_method_type(&data, current),
            CONSTANT_INVOKEDYNAMIC => read_constant_invoke_dynamic(&data, current),
            _  => read_constant_unknown(&data, current),
        };
        current = current + (constant.size as usize);
        constant_pool.insert(read, constant);
        used_indexes.push(read);
        // reserve additional unused place in a constant pool for 8-byte constants
        if EIGHT_BYTE_CONSTANTS.contains(&tag) {
            read = read + 1;
        };
    }
    // iterate through all constants and print results
    for i in used_indexes {
        let constant = match constant_pool.get(&i) {
            Some(v) => v,
            None    => panic!("Unknown index: {}", i),
        };
        
        if constant.references.is_empty() {
            println!("\t#{} = {}", i, constant);
        } else {
            print!("\t#{} = {} // ", i, constant);
            let leaves = traverse(&constant.references, &constant_pool);
            let mut first = true;
            for r in &leaves {
                let value = &constant_pool.get(r).unwrap().value;
                if first {
                    first = false;
                    print!("{}", value);
                } else {
                    print!(".{}", value);
                }
            }
            println!("");
        }
    }
}

// TODO ugly; reimplement and optimize
// recursively traverse references and return a path
fn traverse(references: &Vec<u8>, constant_pool: &HashMap<u8, Constant>) -> Vec<u8> {
    let mut path = Vec::new();
    for r in references {
        let c = &constant_pool.get(r);
        if c.unwrap().references.is_empty() {
            path.push(r.clone());
        } else {
            let inner = traverse(&c.unwrap().references, &constant_pool);
            for i in inner {
                path.push(i.clone());
            }
        }
    }
    return path;
}

fn read_constant_method_ref(data: &[u8], current: usize) -> Constant {
    let class_index = &data[current + 1] << 2 | &data[current + 2];
    let name_and_type_index = &data[current + 3] << 2 | &data[current + 4];

    let refs = vec![class_index, name_and_type_index];
    return Constant {constant_type: CONSTANT_METHODREF,
                     type_name: "Methodref".to_string(),
                     references: refs,
                     value: format!("#{}.#{}", class_index, name_and_type_index),
                     size: 5};
}

fn read_constant_string(data: &[u8], current: usize) -> Constant {
    let string_index = &data[current + 1] << 2 | &data[current + 2];

    let refs = vec![string_index];
    return Constant {constant_type: CONSTANT_STRING,
                     type_name: "String".to_string(),
                     references: refs,
                     value: format!("#{}", string_index),
                     size: 3};
}

fn read_constant_field_ref(data: &[u8], current: usize) -> Constant {
    let class_index = &data[current + 1] << 2 | &data[current + 2];
    let name_and_type_index = &data[current + 3] << 2 | &data[current + 4];

    let refs = vec![class_index, name_and_type_index];
    return Constant {constant_type: CONSTANT_FIELDREF,
                     type_name: "Fieldref".to_string(),
                     references: refs,
                     value: format!("#{}.#{}", class_index, name_and_type_index),
                     size: 5};
}

fn read_constant_interface_method_ref(data: &[u8], current: usize) -> Constant {
    let class_index = &data[current + 1] << 2 | &data[current + 2];
    let name_and_type_index = &data[current + 3] << 2 | &data[current + 4];

    let refs = vec![class_index, name_and_type_index];
    return Constant {constant_type: CONSTANT_FIELDREF,
                     type_name: "InterfaceMethodref".to_string(),
                     references: refs,
                     value: format!("#{}.#{}", class_index, name_and_type_index),
                     size: 5};
}

fn read_constant_class(data: &[u8], current: usize) -> Constant {
    let name_index = &data[current + 1] << 2 | &data[current + 2];

    let refs = vec![name_index];
    return Constant {constant_type: CONSTANT_CLASS,
                     type_name: "Class".to_string(),
                     references: refs,
                     value: format!("#{}", name_index),
                     size: 3};
}

fn read_constant_utf8(data: &[u8], current: usize) -> Constant {
    let length = &data[current + 1] << 2 | &data[current + 2];
    let start = current + 3;
    let end = start + (length as usize);
    let utf8_string = match str::from_utf8(&data[start..end]) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    return Constant {constant_type: CONSTANT_UTF8,
                     type_name: "Utf8".to_string(),
                     references: Vec::new(),
                     value: format!("{}", utf8_string),
                     size: (3 as u8) + length};
}

fn read_constant_integer(data: &[u8], current: usize) -> Constant {

    let slice = &data[(current + 1)..(current + 5)];
    let bytes = (slice[0] as i32) << 24 | (slice[1] as i32) << 16 |
                (slice[2] as i32) << 8  | (slice[3] as i32) << 0;

    return Constant {constant_type: CONSTANT_INTEGER,
                     type_name: "Integer".to_string(),
                     references: Vec::new(),
                     value: format!("{}", bytes),
                     size: 5};
}

fn read_constant_name_and_type(data: &[u8], current: usize) -> Constant {
    let name_index = &data[current + 1] << 2 | &data[current + 2];
    let descriptor_index = &data[current + 3] << 2 | &data[current + 4];

    let refs = vec![name_index, descriptor_index];
    return Constant {constant_type: CONSTANT_NAMEANDTYPE,
                     type_name: "NameAndType".to_string(),
                     references: refs,
                     value: format!("#{}.#{}", name_index, descriptor_index),
                     size: 5};
}

fn read_constant_unknown(data: &[u8], current: usize) -> Constant {
    panic!("Unknown CONSTANT tag type: {}!", &data[current]);
}

fn read_constant_long(data: &[u8], current: usize) -> Constant {

    let slice = &data[(current + 1)..(current + 9)];
    let low:  i64 = ((slice[4] as i64) << 24) | ((slice[5] as i64) << 16) | ((slice[6] as i64) << 8) | (slice[7] as i64);
    let high: i64 = ((slice[0] as i64) << 24) | ((slice[1] as i64) << 16) | ((slice[2] as i64) << 8) | (slice[3] as i64);
    let long = (high << 32) + low;
    
    return Constant {constant_type: CONSTANT_LONG,
                     type_name: "Long".to_string(),
                     references: Vec::new(),
                     value: format!("{}", long),
                     size: 9};
}

// TODO
fn read_constant_float(data: &[u8], current: usize) -> Constant {

    let slice = &data[(current + 1)..(current + 5)];
    let bytes = (slice[0] as u32) << 24 | (slice[1] as u32) << 16 |
                (slice[2] as u32) << 8  | (slice[3] as u32) << 0;
    
    let mut value = match bytes as u32 {
        POSITIVE_INFINITY_32 => "Infinityf",
        NEGATIVE_INFINITY_32 => "-Infinityf",
        0x7f800001...0x7fffffff => "NaNf",
        0xff800001...0xffffffff => "NaNf",
        _ =>  ""
    };
    if value.is_empty() {
        let s = if (bytes >> 31) == 0 { 1 } else { -1 };
        let e = (bytes >> 23) & 0xff;
        let m = if e == 0 {
                    (bytes & 0x7fffff) << 1 
                } else {
                    (bytes & 0x7fffff) | 0x800000 
                };
                
        // FIXME loss of precision
        let float = (s as f32) * (m as f32) * 2.0_f32.powi((e as i32) - 150);
        // TODO format float
        return Constant {constant_type: CONSTANT_FLOAT,
                        type_name: "Float".to_string(),
                        references: Vec::new(),
                        value: float.to_string(),
                        size: 5};
    }
    return Constant {constant_type: CONSTANT_FLOAT,
                     type_name: "Float".to_string(),
                     references: Vec::new(),
                     value: value.to_string(),
                     size: 5};
}

fn read_constant_double(data: &[u8], current: usize) -> Constant {

    return Constant {constant_type: CONSTANT_DOUBLE,
                     type_name: "Double".to_string(),
                     references: Vec::new(),
                     value: format!("{}", "NOT IMPLEMENTED"),
                     size: 9};
}

fn read_constant_invoke_dynamic(data: &[u8], current: usize) -> Constant {
    panic!("NOT IMPLEMENTED");
}

fn read_constant_method_type(data: &[u8], current: usize) -> Constant {
    panic!("NOT IMPLEMENTED");
}

fn read_constant_method_handle(data: &[u8], current: usize) -> Constant {
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