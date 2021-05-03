#![allow(unused)]

mod record;
mod header;
mod util{pub mod file_ops;}

use std::error::Error;

use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::prelude::*;
use crate::header::{Header};
use crate::util::file_ops;
use byteorder::{LittleEndian, ByteOrder};
use bitvec::slice::BitSlice;
use bitvec::vec::BitVec;
use bitvec::order::Lsb0;
use crate::util::file_ops::load_part;
use crate::record::Record;

fn main() -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open("/Users/filken/Downloads/6590669034_ACTIVITY.fit")?;
    let mut buf_reader = BufReader::new(file);

    dbg!(Header::load(&mut buf_reader));

    dbg!(Record::load(&mut buf_reader));

    Ok(())
}
