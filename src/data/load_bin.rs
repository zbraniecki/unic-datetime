use super::layout2;
use bincode;
use std::fs::File;
use std::io::prelude::*;

pub fn get_calendar_data(locale: &str) -> layout2::Resource {
    let mut fh = File::open(format!("./res/{}.dat", locale)).expect("Opening file failed");

    let mut buffer: Vec<u8> = vec![];
    fh.read_to_end(&mut buffer).expect("Failed to read");

    bincode::deserialize(&buffer).unwrap()
}
