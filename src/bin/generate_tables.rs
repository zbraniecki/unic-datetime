use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

fn get_langid_to_direction_map(path: &str) -> HashMap<LanguageIdentifier, CharacterDirection> {
    let mut result = HashMap::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let mut path = entry.path();
        path.push("layout.json");
        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
        let v: Value = serde_json::from_str(&contents).unwrap();

        let langid_key = v["main"].as_object().unwrap().keys().nth(0).unwrap();

        if langid_key == "root" {
            continue;
        }
        let langid: LanguageIdentifier = langid_key.parse().unwrap();

        let character_order = match v["main"][langid_key]["layout"]["orientation"]["characterOrder"]
            .as_str()
            .unwrap()
        {
            "right-to-left" => CharacterDirection::RTL,
            "left-to-right" => CharacterDirection::LTR,
            _ => unimplemented!("Encountered unknown directionality!"),
        };
        result.insert(langid, character_order);
    }
    result
}

fn main() {
    let path = "./data/cldr-misc-modern/main/";
    let map = get_langid_to_direction_map(path);
    //let contents = fs::read_to_string("./data/likelySubtags.json")
    //.expect("Something went wrong reading the file");
    //let v: Value = serde_json::from_str(&contents).unwrap();
    //let values = v["supplemental"]["likelySubtags"].as_object().unwrap();
}
