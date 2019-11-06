use serde_json::Value;
use std::fmt::Write;
use std::fs;
use unic_datetime::data::layout::{
    CalendarData, DateTimeToken, MonthList, MonthNames, MonthNamesTypes, Pattern, PatternElement,
};
use unic_datetime::data::patterns::parse_pattern;

fn get_month_list(v: &Value) -> Option<MonthList<String>> {
    if let Some(values) = v.as_object() {
        let mut list = Vec::new();
        for i in 1..13 {
            let name = values
                .get(&i.to_string())
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            list.push(name);
        }
        // XXX: I'm so sorry, Mom.
        let array: [String; 12] = [
            list[0].clone(),
            list[1].clone(),
            list[2].clone(),
            list[3].clone(),
            list[4].clone(),
            list[5].clone(),
            list[6].clone(),
            list[7].clone(),
            list[8].clone(),
            list[9].clone(),
            list[10].clone(),
            list[11].clone(),
        ];
        Some(array)
    } else {
        None
    }
}

fn get_months_data(v: &Value) -> Option<MonthNamesTypes<String>> {
    Some(MonthNamesTypes {
        abbreviated: get_month_list(&v["abbreviated"]),
        narrow: get_month_list(&v["narrow"]),
        short: get_month_list(&v["short"]),
        wide: get_month_list(&v["wide"]),
    })
}

fn get_format_patterns(v: &Value) -> [Pattern<String>; 4] {
    let values = v.as_object().unwrap();
    [
        parse_pattern(values.get("full").unwrap().as_str().unwrap()).unwrap(),
        parse_pattern(values.get("long").unwrap().as_str().unwrap()).unwrap(),
        parse_pattern(values.get("medium").unwrap().as_str().unwrap()).unwrap(),
        parse_pattern(values.get("short").unwrap().as_str().unwrap()).unwrap(),
    ]
}

fn get_calendar_data() -> CalendarData<String> {
    let path = "./data/cldr-dates-modern/main/pl/ca-gregorian.json";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let v: Value = serde_json::from_str(&contents).unwrap();
    let values = &v["main"]["pl"]["dates"]["calendars"]["gregorian"];

    CalendarData {
        months: MonthNames {
            stand_alone: get_months_data(&values["months"]["stand-alone"]),
            format: get_months_data(&values["months"]["format"]),
        },
        date_formats: get_format_patterns(&values["dateFormats"]),
        time_formats: get_format_patterns(&values["timeFormats"]),
        date_time_formats: get_format_patterns(&values["dateTimeFormats"]),
    }
}

fn serialize_month_list(list: &Option<MonthList<String>>) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    if let Some(list) = list {
        writeln!(
            result,
            "Some([{}])",
            list.iter()
                .map(|s| format!(r#""{}""#, s))
                .collect::<Vec<_>>()
                .join(", ")
        )?;
    } else {
        write!(result, "None")?;
    }
    Ok(result)
}

fn serialize_month_names_types(
    months: &Option<MonthNamesTypes<String>>,
) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    if let Some(months) = months {
        writeln!(result, "Some(MonthNamesTypes {{")?;
        writeln!(
            result,
            "            abbreviated: {},",
            serialize_month_list(&months.abbreviated)?
        )?;
        writeln!(
            result,
            "            narrow: {},",
            serialize_month_list(&months.narrow)?
        )?;
        writeln!(
            result,
            "            short: {},",
            serialize_month_list(&months.short)?
        )?;
        writeln!(
            result,
            "            wide: {},",
            serialize_month_list(&months.wide)?
        )?;
        write!(result, "        }})")?;
    } else {
        write!(result, "None")?;
    }
    Ok(result)
}

fn serialize_dt_formats(list: &[Pattern<String>]) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    writeln!(result, "[")?;
    for pattern in list {
        writeln!(result, "        Cow::Borrowed(&[")?;
        for elem in pattern.as_ref().iter() {
            match elem {
                PatternElement::Literal(s) => {
                    writeln!(result, r#"            PatternElement::Literal("{}"),"#, s)?;
                }
                PatternElement::Token(t) => {
                    writeln!(
                        result,
                        r#"            PatternElement::Token(DateTimeToken::{}),"#,
                        t.get_name()
                    )?;
                }
            }
        }
        writeln!(result, "        ]),")?;
    }
    write!(result, "    ]")?;
    Ok(result)
}

fn serialize_calendar_data(data: &CalendarData<String>) -> Result<String, std::fmt::Error> {
    let mut result = String::new();

    writeln!(result, "use super::layout::*;")?;
    writeln!(result, "use std::borrow::Cow;\n")?;

    writeln!(
        result,
        "pub const CALENDAR_DATA: CalendarData<&'static str> = CalendarData {{"
    )?;
    writeln!(result, "    months: MonthNames {{")?;
    writeln!(
        result,
        "        format: {},",
        serialize_month_names_types(&data.months.format)?
    )?;
    writeln!(
        result,
        "        stand_alone: {},",
        serialize_month_names_types(&data.months.stand_alone)?
    )?;
    writeln!(result, "    }},")?;
    writeln!(
        result,
        "    date_formats: {},",
        serialize_dt_formats(&data.date_formats)?
    )?;
    writeln!(
        result,
        "    time_formats: {},",
        serialize_dt_formats(&data.time_formats)?
    )?;
    writeln!(
        result,
        "    date_time_formats: {},",
        serialize_dt_formats(&data.date_time_formats)?
    )?;
    write!(result, "}};")?;
    Ok(result)
}

fn main() {
    let data = get_calendar_data();

    let result = serialize_calendar_data(&data).unwrap();
    println!("{}", result);
}
