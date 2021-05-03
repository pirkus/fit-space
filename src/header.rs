use std::io::Read;

use byteorder::{ByteOrder, LittleEndian};
use crate::util::file_ops::{load_part};

#[derive(Debug)]
pub struct Header {
    header_size: u8,
    protocol_version: u8,
    profile_version: u16,
    data_size: u32,
    data_type: [u8; 4],
    crc: u16
}

impl Header {
    pub fn load<T: Read>(reader: &mut T) -> Header {
        let header_size = LittleEndian::read_uint(&load_part(1, reader), 1) as u8;
        if header_size > 14 || header_size < 14 {
            panic!("Unknown header size: {}", header_size);
        }

        Header {
            header_size,
            protocol_version: LittleEndian::read_uint(&load_part(1, reader), 1) as u8,
            profile_version: LittleEndian::read_u16(&load_part(2, reader)),
            data_size: LittleEndian::read_u32(&load_part(4, reader)), // I guess these are bytes? Does not include CRC or header size
            data_type: [
                LittleEndian::read_uint(&load_part(1, reader), 1) as u8,
                LittleEndian::read_uint(&load_part(1, reader), 1) as u8,
                LittleEndian::read_uint(&load_part(1, reader), 1) as u8,
                LittleEndian::read_uint(&load_part(1, reader), 1) as u8
            ],
            crc: LittleEndian::read_u16(&load_part(2, reader))
        }
    }
}