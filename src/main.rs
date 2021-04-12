#![allow(unused)]

use std::error::Error;

use byteorder::{ByteOrder, LittleEndian};

use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open("/Users/filken/Downloads/6590669034_ACTIVITY.fit")?;

    let mut buf_reader = BufReader::new(file);

    #[derive(Debug)]
    struct Header {
        size: u8
    }

    impl Header {
        fn load<T: Read>(reader: &mut T) -> Header {
            let mut load_part = |size| {
                let mut buf = Vec::with_capacity(size);

                let mut part_reader = reader.take(size as u64);

                part_reader.read_to_end(&mut buf).unwrap();

                buf
            };

            Header {
                size: LittleEndian::read_uint(&load_part(1), 1) as u8
            }
        }
    }

    dbg!(Header::load(&mut buf_reader));
    Ok(())
}
