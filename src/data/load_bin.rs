use super::layout;
use bincode;
use std::fs::File;
use std::io::prelude::*;

pub fn get_calendar_data<'l, 'a>(path: &'l str, locale: &'l str) -> layout::Resource<'a> {
    let mut fh = File::open(format!("{}/{}.dat", path, locale)).expect("Opening file failed");

    let mut buffer: Vec<u8> = vec![];
    fh.read_to_end(&mut buffer).expect("Failed to read");

    bincode::deserialize(&buffer).unwrap()
}
