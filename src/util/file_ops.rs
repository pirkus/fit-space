use std::io::Read;

pub fn load_part<T: Read>(size: usize, reader: &mut T) -> Vec<u8> {
    let mut buf = Vec::with_capacity(size);
    let mut part_reader = reader.take(size as u64);
    part_reader.read_to_end(&mut buf).unwrap();

    buf
}
