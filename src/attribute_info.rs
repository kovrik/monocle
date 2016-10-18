use constant::*;
use constant_value::*;

use std::fmt;
use std::collections::HashMap;

pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    // TODO pub info[attribute_length];
}

impl fmt::Display for AttributeInfo {
    fn fmt(&self, c: &mut fmt::Formatter) -> fmt::Result {
        write!(c, "attribute_name_index: {}, attribute_length: {}, info: TODO", self.attribute_name_index, self.attribute_length)
    }
}

// TODO
pub fn read_attributes(data: &[u8], current: usize, count: u16, constant_pool: &HashMap<u8, Constant>) -> Vec<AttributeInfo> {
    let attributes = Vec::new();
    for a in 0..count {
        let attribute_name_index = (&data[current] << 2 | &data[current + 1]) as u16;
        println!("Attribute_name_index: {}", attribute_name_index);

        // read attribute length (in bytes)
        let current = current + 2;
        let slice = &data[(current)..(current + 4)];
        let attribute_length: u32 = (slice[0] as u32) << 24 | (slice[1] as u32) << 16 |
                                    (slice[2] as u32) << 8  | (slice[3] as u32) << 0;
        println!("Attribute length: {}", attribute_length);

        // let this_class_ref = constant_pool.get(&this_class_ref).unwrap().references[0];
        // let ref this_class = constant_pool.get(&this_class_ref).unwrap().value;
        let ref attribute_name = constant_pool.get(&(attribute_name_index as u8)).unwrap().value;
        // read attribute info
        let attribute = match attribute_name.as_ref() {
            "ConstantValue" => read_constant_value(data, current),
            _ => panic!("Unknown attribute: {}", attribute_name_index),
        };
        
        // attributes.push(attribute);
    }
    return attributes;
}

// FIXME
fn read_constant_value(data: &[u8], current: usize) {

    let attribute_name_index = (&data[current] << 2 | &data[current + 1]) as u16;
    let slice = &data[(current)..(current + 4)];
    let attribute_length: u32 = (slice[0] as u32) << 24 | (slice[1] as u32) << 16 |
                                (slice[2] as u32) << 8  | (slice[3] as u32) << 0;
    let constantvalue_index = (&data[current + 5] << 2 | &data[current + 6]) as u16;

    println!("{}", attribute_name_index);
    println!("{}", attribute_length);
    println!("{}", constantvalue_index);
}