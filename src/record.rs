use bitvec::vec::BitVec;
use bitvec::order::Lsb0;
use crate::util::file_ops::load_part;
use byteorder::{ByteOrder, LittleEndian};
use std::io::Read;

#[derive(Debug)]
pub struct Record {
    record_header: BitVec<Lsb0, u8>
}

impl Record {
    pub fn load<T: Read>(reader: &mut T) -> Record {
        let record_header = LittleEndian::read_uint(&load_part(1, reader), 1) as u8;

        Record {
            record_header: BitVec::from_element(record_header)
        }
    }
}