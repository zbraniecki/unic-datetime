use std::time::Instant;
use unic_datetime::data::load_bin::get_calendar_data;
use unic_datetime::*;

fn main() {
    let now = Instant::now();

    let data = get_calendar_data("pl");
    // println!("{:#?}", data);
    //
    // for value in values {
    //     let dtf = DateTimeFormat::new(value.0, value.1, value.2, &data);
    //     for date in dates {
    //         let _ = dtf.format(date);
    //         // println!("{}", s);
    //     }
    // }
    let elapsed = now.elapsed();
    println!("us: {:#?}", elapsed.as_micros());
}
