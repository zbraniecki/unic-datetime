use bincode;
use std::fs::File;
use std::io::prelude::*;
use unic_datetime::data::layout;
use unic_datetime::data::load_json;
use unic_datetime::data::patterns;

fn parse_formats(formats: &mut layout::Formats) {
    if let Some(layout::DateTimePattern::Raw(ref r)) = formats.full {
        let parsed = patterns::parse_pattern(r.as_ref()).expect("Failed to parse");
        formats.full = Some(layout::DateTimePattern::Parsed(parsed.into()));
    }

    if let Some(layout::DateTimePattern::Raw(ref r)) = formats.long {
        let parsed = patterns::parse_pattern(r.as_ref()).expect("Failed to parse");
        formats.long = Some(layout::DateTimePattern::Parsed(parsed.into()));
    }

    if let Some(layout::DateTimePattern::Raw(ref r)) = formats.medium {
        let parsed = patterns::parse_pattern(r.as_ref()).expect("Failed to parse");
        formats.medium = Some(layout::DateTimePattern::Parsed(parsed.into()));
    }

    if let Some(layout::DateTimePattern::Raw(ref r)) = formats.short {
        let parsed = patterns::parse_pattern(r.as_ref()).expect("Failed to parse");
        formats.short = Some(layout::DateTimePattern::Parsed(parsed.into()));
    }
}

fn parse_all_formats(res: &mut layout::Resource) {
    parse_formats(&mut res.main.pl.dates.calendars.gregorian.date_formats);
    parse_formats(&mut res.main.pl.dates.calendars.gregorian.time_formats);
    parse_formats(&mut res.main.pl.dates.calendars.gregorian.date_time_formats);
}

fn main() {
    let mut res = load_json::get_calendar_data("./data/cldr-dates-modern", "pl");
    parse_all_formats(&mut res);

    let encoded: Vec<u8> = bincode::serialize(&res).unwrap();

    let mut buffer = File::create("./res/pl.dat").expect("Opening file failed");
    buffer.write_all(&encoded).expect("Writing failed");
    // println!("{:?}", encoded);

    // let decoded: layout::DayList = bincode::deserialize(&encoded[..]).unwrap();
    //
    // assert_eq!(list, decoded);
}
