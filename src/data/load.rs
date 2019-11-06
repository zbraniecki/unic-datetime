use crate::data::layout::{
    CalendarData, DayList, DayNames, DayNamesTypes, MonthList, MonthNames, MonthNamesTypes, Pattern,
};
use crate::data::patterns::parse_pattern;
use serde_json::Value;
use std::borrow::Cow;
use std::fs;

fn get_day_list(v: &Value) -> Option<DayList> {
    if let Some(values) = v.as_object() {
        let array: [Cow<'static, str>; 7] = [
            Cow::Owned(values.get("sun").unwrap().as_str().unwrap().to_string()),
            Cow::Owned(values.get("mon").unwrap().as_str().unwrap().to_string()),
            Cow::Owned(values.get("tue").unwrap().as_str().unwrap().to_string()),
            Cow::Owned(values.get("wed").unwrap().as_str().unwrap().to_string()),
            Cow::Owned(values.get("thu").unwrap().as_str().unwrap().to_string()),
            Cow::Owned(values.get("fri").unwrap().as_str().unwrap().to_string()),
            Cow::Owned(values.get("sat").unwrap().as_str().unwrap().to_string()),
        ];
        Some(array)
    } else {
        None
    }
}

fn get_days_data(v: &Value) -> Option<DayNamesTypes> {
    Some(DayNamesTypes {
        abbreviated: get_day_list(&v["abbreviated"]),
        narrow: get_day_list(&v["narrow"]),
        short: get_day_list(&v["short"]),
        wide: get_day_list(&v["wide"]),
    })
}

fn get_month_list(v: &Value) -> Option<MonthList> {
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
        let array: [Cow<'static, str>; 12] = [
            Cow::Owned(list[0].clone()),
            Cow::Owned(list[1].clone()),
            Cow::Owned(list[2].clone()),
            Cow::Owned(list[3].clone()),
            Cow::Owned(list[4].clone()),
            Cow::Owned(list[5].clone()),
            Cow::Owned(list[6].clone()),
            Cow::Owned(list[7].clone()),
            Cow::Owned(list[8].clone()),
            Cow::Owned(list[9].clone()),
            Cow::Owned(list[10].clone()),
            Cow::Owned(list[11].clone()),
        ];
        Some(array)
    } else {
        None
    }
}

fn get_months_data(v: &Value) -> Option<MonthNamesTypes> {
    Some(MonthNamesTypes {
        abbreviated: get_month_list(&v["abbreviated"]),
        narrow: get_month_list(&v["narrow"]),
        short: get_month_list(&v["short"]),
        wide: get_month_list(&v["wide"]),
    })
}

fn get_format_patterns(v: &Value) -> [Pattern; 4] {
    let values = v.as_object().unwrap();
    [
        parse_pattern(values.get("full").unwrap().as_str().unwrap()).unwrap(),
        parse_pattern(values.get("long").unwrap().as_str().unwrap()).unwrap(),
        parse_pattern(values.get("medium").unwrap().as_str().unwrap()).unwrap(),
        parse_pattern(values.get("short").unwrap().as_str().unwrap()).unwrap(),
    ]
}
pub fn get_calendar_data() -> CalendarData {
    let path = "./data/cldr-dates-modern/main/pl/ca-gregorian.json";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let v: Value = serde_json::from_str(&contents).unwrap();
    let values = &v["main"]["pl"]["dates"]["calendars"]["gregorian"];

    CalendarData {
        months: MonthNames {
            stand_alone: get_months_data(&values["months"]["stand-alone"]),
            format: get_months_data(&values["months"]["format"]),
        },
        days: DayNames {
            stand_alone: get_days_data(&values["days"]["stand-alone"]),
            format: get_days_data(&values["days"]["format"]),
        },
        date_formats: get_format_patterns(&values["dateFormats"]),
        time_formats: get_format_patterns(&values["timeFormats"]),
        date_time_formats: get_format_patterns(&values["dateTimeFormats"]),
    }
}
