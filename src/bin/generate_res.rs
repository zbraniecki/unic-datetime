use bincode;
use std::fs::File;
use std::io::prelude::*;
use unic_datetime::data::layout2;
use unic_datetime::data::load3;
use unic_datetime::data::patterns2;

fn parse_formats(formats: &mut layout2::Formats) {
    if let layout2::DateTimePattern::Raw(ref r) = formats.full {
        let parsed = patterns2::parse_pattern(r.as_ref()).expect("Failed to parse");
        formats.full = layout2::DateTimePattern::Parsed(parsed);
    }

    if let layout2::DateTimePattern::Raw(ref r) = formats.long {
        let parsed = patterns2::parse_pattern(r.as_ref()).expect("Failed to parse");
        formats.long = layout2::DateTimePattern::Parsed(parsed);
    }

    if let layout2::DateTimePattern::Raw(ref r) = formats.medium {
        let parsed = patterns2::parse_pattern(r.as_ref()).expect("Failed to parse");
        formats.medium = layout2::DateTimePattern::Parsed(parsed);
    }

    if let layout2::DateTimePattern::Raw(ref r) = formats.short {
        let parsed = patterns2::parse_pattern(r.as_ref()).expect("Failed to parse");
        formats.short = layout2::DateTimePattern::Parsed(parsed);
    }
}

fn parse_all_formats(res: &mut layout2::Resource) {
    parse_formats(&mut res.main.pl.dates.calendars.gregorian.date_formats);
    parse_formats(&mut res.main.pl.dates.calendars.gregorian.time_formats);
    parse_formats(&mut res.main.pl.dates.calendars.gregorian.date_time_formats);
}

fn main() {
    let mut res = load3::get_calendar_data("pl");
    parse_all_formats(&mut res);

    let encoded: Vec<u8> = bincode::serialize(&res).unwrap();

    let mut buffer = File::create("./res/pl.dat").expect("Opening file failed");
    buffer.write_all(&encoded).expect("Writing failed");
    // println!("{:?}", encoded);

    // let decoded: layout2::DayList = bincode::deserialize(&encoded[..]).unwrap();
    //
    // assert_eq!(list, decoded);
}
