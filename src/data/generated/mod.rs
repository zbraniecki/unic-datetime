use crate::data::layout::Resource;
pub mod pl;

pub fn get(locale: &str) -> &'static Resource<'static> {
    match locale {
        "pl" => &pl::RESOURCE,
        _ => panic!(),
    }
}
