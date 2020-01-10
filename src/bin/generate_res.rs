use bincode;
use std::fs::File;
use std::io::prelude::*;
use unic_datetime::data::load3;

fn main() {
    let list = load3::get_calendar_data("pl");

    let encoded: Vec<u8> = bincode::serialize(&list).unwrap();

    let mut buffer = File::create("./res/pl.dat").expect("Opening file failed");
    buffer.write_all(&encoded).expect("Writing failed");
    // println!("{:?}", encoded);

    // let decoded: layout2::DayList = bincode::deserialize(&encoded[..]).unwrap();
    //
    // assert_eq!(list, decoded);
}
