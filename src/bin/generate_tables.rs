use std::fmt::Write;
use unic_datetime::data::layout::{
    CalendarData, DayList, DayNamesTypes, MonthList, MonthNamesTypes, Pattern, PatternElement,
};
use unic_datetime::data::load::get_calendar_data;

fn serialize_day_list(list: &Option<DayList>) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    if let Some(list) = list {
        writeln!(
            result,
            "Some([{}])",
            list.iter()
                .map(|s| format!(r#"Cow::Borrowed("{}")"#, s))
                .collect::<Vec<_>>()
                .join(", ")
        )?;
    } else {
        write!(result, "None")?;
    }
    Ok(result)
}

fn serialize_month_list(list: &Option<MonthList>) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    if let Some(list) = list {
        writeln!(
            result,
            "Some([{}])",
            list.iter()
                .map(|s| format!(r#"Cow::Borrowed("{}")"#, s))
                .collect::<Vec<_>>()
                .join(", ")
        )?;
    } else {
        write!(result, "None")?;
    }
    Ok(result)
}

fn serialize_day_names_types(days: &Option<DayNamesTypes>) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    if let Some(days) = days {
        writeln!(result, "Some(DayNamesTypes {{")?;
        writeln!(
            result,
            "            abbreviated: {},",
            serialize_day_list(&days.abbreviated)?
        )?;
        writeln!(
            result,
            "            narrow: {},",
            serialize_day_list(&days.narrow)?
        )?;
        writeln!(
            result,
            "            short: {},",
            serialize_day_list(&days.short)?
        )?;
        writeln!(
            result,
            "            wide: {},",
            serialize_day_list(&days.wide)?
        )?;
        write!(result, "        }})")?;
    } else {
        write!(result, "None")?;
    }
    Ok(result)
}

fn serialize_month_names_types(
    months: &Option<MonthNamesTypes>,
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

fn serialize_dt_formats(list: &[Pattern]) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    writeln!(result, "[")?;
    for pattern in list {
        writeln!(result, "        Cow::Borrowed(&[")?;
        for elem in pattern.as_ref().iter() {
            match elem {
                PatternElement::Literal(s) => {
                    writeln!(
                        result,
                        r#"            PatternElement::Literal(Cow::Borrowed("{}")),"#,
                        s
                    )?;
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

fn serialize_calendar_data(data: &CalendarData) -> Result<String, std::fmt::Error> {
    let mut result = String::new();

    writeln!(result, "use super::layout::*;")?;
    writeln!(result, "use std::borrow::Cow;\n")?;

    writeln!(
        result,
        "pub const CALENDAR_DATA: CalendarData = CalendarData {{"
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
    writeln!(result, "    days: DayNames {{")?;
    writeln!(
        result,
        "        format: {},",
        serialize_day_names_types(&data.days.format)?
    )?;
    writeln!(
        result,
        "        stand_alone: {},",
        serialize_day_names_types(&data.days.stand_alone)?
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
    let data = get_calendar_data("pl");

    let result = serialize_calendar_data(&data).unwrap();
    println!("{}", result);
}
