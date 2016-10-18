use std::fmt;

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
fn read_attributes() {
}