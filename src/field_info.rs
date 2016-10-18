use attribute_info::*;
use access_flags::read_access_flags;
use access_flags::FIELD_ACC_FLAGS;
use constant::*;

use std::fmt;
use std::collections::HashMap;

pub struct FieldInfo {
    pub access_flags_mask: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attribute_info: Vec<AttributeInfo>,
    pub size: u8,
}

impl fmt::Display for FieldInfo {
    fn fmt(&self, c: &mut fmt::Formatter) -> fmt::Result {
        write!(c, "access_flags_mask: {}, access_flags: {:?}, name_index: {}, descriptor_index: {}, attributes_count: {}, attributes: {}, size: {}",
               self.access_flags_mask, read_access_flags(self.access_flags_mask, &FIELD_ACC_FLAGS),
               self.name_index, self.descriptor_index, self.attributes_count, 
               "TODO",
               self.size )
    }
}

pub fn read_field(data: &[u8], current: usize, constant_pool: &HashMap<u8, Constant>) -> FieldInfo {

    let access_flags_mask = (&data[current + 1] << 2 | &data[current + 2]) as u16;
    let access_flags = read_access_flags(access_flags_mask, &FIELD_ACC_FLAGS);
    let name_index = (&data[current + 3] << 2 | &data[current + 4]) as u16;
    let descriptor_index = (&data[current + 5] << 2 | &data[current + 6]) as u16;
    let attributes_count = (&data[current + 7] << 2 | &data[current + 8]) as u16;
    let attribute_info = read_attributes(&data, current + 9, attributes_count, &constant_pool);
    return FieldInfo { access_flags_mask: access_flags_mask, name_index: name_index,
                       descriptor_index: descriptor_index, attributes_count: attributes_count,
                       attribute_info: attribute_info, size: 0, };
}
