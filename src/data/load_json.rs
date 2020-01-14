use super::layout;
use std::fs;

pub fn get_calendar_data<'l, 'a>(path: &'l str, locale: &'l str) -> layout::Resource<'a> {
    unsafe { layout::IN_JSON = true };
    let path = format!("{}/main/{}/ca-gregorian.json", path, locale);
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let list: layout::Resource = serde_json::from_str(&contents).unwrap();
    unsafe { layout::IN_JSON = false };
    list
}
