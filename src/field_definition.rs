use std::io::Read;
use crate::util::file_ops::load_part;
use byteorder::{ByteOrder, LittleEndian};
use bitvec::vec::BitVec;
use bitvec::order::Lsb0;

#[derive(Debug)]
pub struct FieldDefinition {
    field_definition_number: u8,
    size: u8,
    base_type: BitVec<Lsb0, u8>
}

impl FieldDefinition {
    pub fn load<T: Read>(reader: &mut T) -> FieldDefinition {
        FieldDefinition {
            field_definition_number: LittleEndian::read_uint(&load_part(1, reader), 1) as u8,
            size: LittleEndian::read_uint(&load_part(1, reader), 1) as u8,
            base_type: BitVec::from_element(LittleEndian::read_uint(&load_part(1, reader), 1) as u8)
        }
    }
}