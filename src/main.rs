extern crate crypto;
extern crate chrono;

use chrono::*;

use crypto::digest::Digest;
use crypto::md5::Md5;

use std::f32;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::time;
use std::env;

mod hexdump;
use hexdump::print_hexdump;

mod constant;
use constant::*;

mod access_flags;
use access_flags::*;

mod field_info;
use field_info::*;

mod attribute_info;
use attribute_info::*;

mod constant_value;
use constant_value::*;

mod writer;
use writer::*;
 
const MAGIC: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];
 
// ----------------------------------------------------------------------------------
// FIXME depends on endiaess
// TODO check different Java versions and platforms
// TODO different CONSTANT types print their values a bit differently
// ----------------------------------------------------------------------------------
fn main() {
 
    println!("___________________________________________");
    let filename = env::args().nth(1).unwrap_or("test/Test.class".to_string());
    println!("{}", filename);

    let mut f = File::open(&filename).expect("Unable to open file");
    let mut data = Vec::new();
    f.read_to_end(&mut data).expect("Unable to read data");
   
    // assert magic
    assert_eq!(&MAGIC[..], &data[0..4], "Error: class not found: {}", filename);
    // seems to be a valid .class file
    println!("Classfile {}", filename);
 
    let modified_timestamp = read_last_modified(&filename);
    print!("  Last modified {}; ", modified_timestamp);
 
    // Size in bytes
    let size = data.len();
    println!("size: {} bytes", size);
 
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
    let (current, constant_pool) = read_constant_pool(&data, constant_pool_count);
 
    let access_flags_mask = (data[current] << 2 | data[current + 1]) as u16;
    let access_flags = read_access_flags(access_flags_mask, &CLASS_ACC_FLAGS);
    println!("\n\tflags: {}\n", access_flags.join(", "));

    let this_class_ref = data[current + 2] << 2 | data[current + 3];
    let this_class_ref = constant_pool.get(&this_class_ref).unwrap().references[0];
    let ref this_class = constant_pool.get(&this_class_ref).unwrap().value;
    println!("This class: {}", this_class);
    
    let super_class_ref = data[current + 4] << 2 | data[current + 5];
    let super_class_ref = constant_pool.get(&super_class_ref).unwrap().references[0];
    let ref super_class = constant_pool.get(&super_class_ref).unwrap().value;
    println!("Super class: {}", super_class);
    println!("");

    // read interfaces_count
    let interfaces_count= data[current + 6] << 2 | data[current + 7];
    println!("Interfaces count: {}", interfaces_count);

    // read interfaces
    // TODO cleanup and move into a separate method and struct
    let mut current = current + 8;
    for i in 0..interfaces_count {
      let interface_ref = data[current + (i as usize)] << 2 | data[current + 1 + (i as usize)];
      let ref interface_ref = constant_pool.get(&interface_ref).unwrap().references[0];
      let ref interface = constant_pool.get(interface_ref).unwrap().value;
      println!("\tInterface: {}", interface);
      current = current + 1;
    }

    // read fields count
    let mut current = current + 2;
    let fields_count = data[current] << 2 | data[current + 1];
    println!("Fields count: {}", fields_count);

    // read fields
    let mut current = current + 1;
    for n in 1..fields_count {
        print!("\tField {}: ", n);
        let field = read_field(&data, current, &constant_pool);
        println!("{}: ", writer::write_field(field, &constant_pool));
    }
    println!("Bytes:");
    print_hexdump(&data);
}

fn read_version(data: &[u8]) -> (u16, u16) {
    let minor = (data[4] << 2 | data[5]) as u16;
    let major = (data[6] << 2 | data[7]) as u16;
    return (minor, major);
}

fn read_last_modified(filename: &str) -> String {
    let metadata = fs::metadata(filename);
    let modified_timestamp = match metadata {
        Ok(meta) => meta.modified().unwrap().duration_since(time::UNIX_EPOCH).unwrap().as_secs(),
        Err(e)   => panic!("Unable to get last_modified for {}", filename),
    };
    return UTC.timestamp((modified_timestamp as i64), 0).format("%d/%m/%Y").to_string();
}