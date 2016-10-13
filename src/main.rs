// use std::fs;
use std::str;
use std::fs::File;
use std::io::prelude::*;
// use std::env;

// use rust_crypto::md5::Md5;
// use rust_crypto::digest::Digest;
 
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
 
    // TODO read metadata
    // let metadata = try!(fs::metadata(filename));
    // if let Ok(time) = metadata.modified() {
    //     println!("Last modified {:?}", time);
    // } else {
    //     println!("Not supported on this platform");
    // }
 
    // Size in bytes
    println!("Size: {} bytes", data.len());
 
    // TODO calculate MD5 checksum
    // let mut sh = Md5::new();
    // sh.input_str("The quick brown fox jumps over the lazy dog");
    // let out_str = sh.result_str();
    // println!("{}",out_str);
    println!("");
 
    // read minor and major versions
    let (minor, major) = read_version(&data);
    println!("\tminor version: {}", minor);
    println!("\tmajor version: {}", major);
 
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
    while read < count {
        let tag = data[current];
        read = read + 1;
        current = match tag as u8 {
            CONSTANT_METHODREF => read_constant_method_ref(&data, current, read),
            CONSTANT_INTERFACEMETHODREF => read_constant_interface_method_ref(&data, current, read),
            CONSTANT_FIELDREF  => read_constant_field_ref(&data, current, read),
            CONSTANT_STRING    => read_constant_string(&data, current, read),
            CONSTANT_CLASS     => read_constant_class(&data, current, read),
            CONSTANT_UTF8      => read_constant_utf8(&data, current, read),
            CONSTANT_INTEGER   => read_constant_integer(&data, current, read),
            // CONSTANT_LONG      => read_constant_long(&data, current, read),
            CONSTANT_NAMEANDTYPE => read_constant_name_and_type(&data, current, read),
            _  => read_constant_unknown(&data, current),
        };
    }
}

fn read_constant_method_ref(data: &[u8], current: usize, read: u8) -> usize {
    let class_index = &data[current + 1] << 2 | &data[current + 2];
    let name_and_type_index = &data[current + 3] << 2 | &data[current + 4];
    println!("\t#{} = Methodref\t\t#{}.#{}", read, class_index, name_and_type_index);
    return current + 5;
}

fn read_constant_string(data: &[u8], current: usize, read: u8) -> usize {
    let string_index = &data[current + 1] << 2 | &data[current + 2];
    println!("\t#{} = String\t\t#{}", read, string_index);
    return current + 3;
}

fn read_constant_field_ref(data: &[u8], current: usize, read: u8) -> usize {
    let class_index = &data[current + 1] << 2 | &data[current + 2];
    let name_and_type_index = &data[current + 3] << 2 | &data[current + 4];
    println!("\t#{} = Fieldref\t\t#{}.#{}", read, class_index, name_and_type_index);
    return current + 5;
}

fn read_constant_interface_method_ref(data: &[u8], current: usize, read: u8) -> usize {
    let class_index = &data[current + 1] << 2 | &data[current + 2];
    let name_and_type_index = &data[current + 3] << 2 | &data[current + 4];
    println!("\t#{} = InterfaceMethodref\t\t#{}.#{}", read, class_index, name_and_type_index);
    return current + 5;
}

fn read_constant_class(data: &[u8], current: usize, read: u8) -> usize {
    let name_index = &data[current + 1] << 2 | &data[current + 2];
    println!("\t#{} = Class\t\t#{}", read, name_index);
    return current + 3;
}

fn read_constant_utf8(data: &[u8], current: usize, read: u8) -> usize {
    let length = &data[current + 1] << 2 | &data[current + 2];
    let start = current + 3;
    let end = start + (length as usize);
    let utf8_string = match str::from_utf8(&data[start..end]) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("\t#{} = Utf8\t\t{}", read, utf8_string);
    return current + 3 + (length as usize);
}

fn read_constant_integer(data: &[u8], current: usize, read: u8) -> usize {
    let bytes = &data[current + 1] << 6 | &data[current + 2] << 4 |
                &data[current + 3] << 2 | &data[current + 4] << 0;
    println!("\t#{} = Integer\t\t{}", read, bytes);
    return current + 5;
}

fn read_constant_name_and_type(data: &[u8], current: usize, read: u8) -> usize {
    let name_index = &data[current + 1] << 2 | &data[current + 2];
    let descriptor_index = &data[current + 3] << 2 | &data[current + 4];
    println!("\t#{} = NameAndType\t#{}.#{}", read, name_index, descriptor_index);
    return current + 5;
}

fn read_constant_unknown(data: &[u8], current: usize) -> usize {
    panic!("Unknown CONSTANT tag type: {}!", &data[current]);
}