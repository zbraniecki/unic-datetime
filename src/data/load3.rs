use super::layout2;
use std::fs;

pub fn get_calendar_data(locale: &str) -> layout2::Resource {
    unsafe { layout2::in_json = true };
    let path = format!("./data/cldr-dates-modern/main/{}/ca-gregorian.json", locale);
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let list: layout2::Resource = serde_json::from_str(&contents).unwrap();
    unsafe { layout2::in_json = false };
    list
}
