use byteorder::{LittleEndian, ReadBytesExt}; // 1.2.7
use std::{
    fs::File,
    io::{self, Read},
};

struct Configuration {
    header_size: u8,
}

impl Configuration {
    fn from_reader(mut rdr: impl Read) -> io::Result<Self> {
        let header_size = rdr.read_u8()?;

        Ok(Configuration {
           header_size,
        })
    }
}

fn main() {
    let file = File::open("/Users/filken/Downloads/6590669034_ACTIVITY.fit").unwrap();

    let config = Configuration::from_reader(file);
    println!("{}", config.unwrap().header_size);
}
