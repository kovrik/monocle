use field_info::*;
use access_flags::*;
use constant::Constant;

use std::collections::HashMap;

pub fn write_field(field: FieldInfo, constant_pool: &HashMap<u8, Constant>) -> String {

    let mut result = String::from("");

    let ref name = constant_pool.get(&(field.name_index as u8)).unwrap().value;
    result.push_str(&format!("Name: {}, ", name));

    let ref descriptor = constant_pool.get(&(field.descriptor_index as u8)).unwrap().value;
    result.push_str(&format!("Descriptor: {}, ", descriptor));
    result.push_str(&format!("Attributes count: {}, ", field.attributes_count));
    // TODO: attribute_info
    
    let access_flags = read_access_flags(field.access_flags_mask, &FIELD_ACC_FLAGS);
    result.push_str(&format!("Access flags: [{}], ", &access_flags.join(", ")));
    result.push_str(&format!("Size: {}", field.size));
    return result
}