use bitvec::vec::BitVec;
use bitvec::order::Lsb0;
use crate::util::file_ops::load_part;
use byteorder::{ByteOrder, LittleEndian};
use std::io::Read;
use crate::field_definition::FieldDefinition;

#[derive(Debug)]
pub struct Record {
    record_header: BitVec<Lsb0, u8>,
    reserved: u8,
    architecture: u8,
    global_message_number: u16,
    number_of_fields: u8,
    field_definitions: Vec<FieldDefinition>
}

impl Record {
    pub fn load<T: Read>(reader: &mut T) -> Record {
        let record_header = LittleEndian::read_uint(&load_part(1, reader), 1) as u8;
        let header = BitVec::from_element(record_header);
        let is_definition_msg = header[6];
        let is_data_msg = !header[6];

        if is_definition_msg {
            let reserved = LittleEndian::read_uint(&load_part(1, reader), 1) as u8;
            ;
            let architecture = LittleEndian::read_uint(&load_part(1, reader), 1) as u8;
            let global_message_number = LittleEndian::read_u16(&load_part(2, reader));
            let number_of_fields = LittleEndian::read_uint(&load_part(1, reader), 1) as u8;
            let mut field_definitions: Vec<FieldDefinition> = Vec::with_capacity(number_of_fields as usize);

            for i in 0..(number_of_fields) {
                field_definitions.push(FieldDefinition::load(reader));
            }

            if architecture == 1 { panic!("No support for BigEndianes yet") }
            if header[7] == true { panic!("You need to implement the compressed timestamp header support!") }
            if header[5] == true { panic!("You need to implement support for reading developer data.") }

            Record {
                record_header: header,
                reserved,
                architecture,
                global_message_number,
                number_of_fields,
                field_definitions
            }
        } else {
            panic!("Whoah")
        }
    }
}