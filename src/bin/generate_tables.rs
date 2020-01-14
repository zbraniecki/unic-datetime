use std::fmt::Write;
use std::fs;
use std::path::Path;
use unic_datetime::data::layout::*;
use unic_datetime::data::load_json::get_calendar_data;

fn serialize_day_list(list: &Option<DayList>) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    if let Some(list) = list {
        let mut i = 0;
        write!(result, "Some(DayList {{\n")?;
        for key in &["sun", "mon", "tue", "wed", "thu", "fri", "sat"] {
            writeln!(
                result,
                r#"                                   {}: Cow::Borrowed("{}"),"#,
                key,
                list.get(i),
            )?;
            i += 1;
        }
        write!(result, "                                }}),")?;
    } else {
        write!(result, "None,")?;
    }
    Ok(result)
}

fn serialize_month_list(list: &Option<MonthList>) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    if let Some(list) = list {
        write!(result, "Some(MonthList {{\n")?;
        for i in 1..=12 {
            writeln!(
                result,
                r#"                                    m{}: Cow::Borrowed("{}"),"#,
                i,
                list.get(i - 1)
            )?;
        }
        write!(result, "                                }}),")?;
    } else {
        write!(result, "None,")?;
    }
    Ok(result)
}

fn serialize_day_names(days: &DayTypes) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    writeln!(result, "DayTypes {{")?;
    writeln!(
        result,
        "                                abbreviated: {}",
        serialize_day_list(&days.abbreviated)?
    )?;
    writeln!(
        result,
        "                                narrow: {}",
        serialize_day_list(&days.narrow)?
    )?;
    writeln!(
        result,
        "                                short: {}",
        serialize_day_list(&days.short)?
    )?;
    writeln!(
        result,
        "                                wide: {}",
        serialize_day_list(&days.wide)?
    )?;
    write!(result, "                           }}")?;
    Ok(result)
}

fn serialize_month_names(months: &MonthTypes) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    writeln!(result, "MonthTypes {{")?;
    writeln!(
        result,
        "                                abbreviated: {}",
        serialize_month_list(&months.abbreviated)?
    )?;
    writeln!(
        result,
        "                                narrow: {}",
        serialize_month_list(&months.narrow)?
    )?;
    writeln!(
        result,
        "                                short: {}",
        serialize_month_list(&months.short)?
    )?;
    writeln!(
        result,
        "                                wide: {}",
        serialize_month_list(&months.wide)?
    )?;
    write!(result, "                           }}")?;
    Ok(result)
}

fn serialize_dt_format(pattern: &Option<DateTimePattern>) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    if let Some(pattern) = pattern {
        let pattern = pattern.to_parsed();
        writeln!(result, r#"Some(DateTimePattern::Parsed(Cow::Borrowed(&["#)?;
        for elem in pattern {
            match elem {
                PatternElement::Literal(s) => {
                    writeln!(
                        result,
                        r#"                                PatternElement::Literal(Cow::Borrowed("{}")),"#,
                        s
                    )?;
                }
                PatternElement::Token(t) => {
                    writeln!(
                        result,
                        r#"                                PatternElement::Token(DateTimeToken::{}),"#,
                        t.get_name()
                    )?;
                }
            }
        }
        write!(result, r#"                            ])))"#)?;
    } else {
        writeln!(result, "None")?;
    }
    Ok(result)
}
fn serialize_dt_formats(formats: &Formats) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    writeln!(result, "Formats {{")?;
    writeln!(
        result,
        "                            full: {},",
        serialize_dt_format(&formats.full)?
    )?;
    writeln!(
        result,
        "                            long: {},",
        serialize_dt_format(&formats.long)?
    )?;
    writeln!(
        result,
        "                            medium: {},",
        serialize_dt_format(&formats.medium)?
    )?;
    writeln!(
        result,
        "                            short: {},",
        serialize_dt_format(&formats.short)?
    )?;
    write!(result, "                        }},")?;
    Ok(result)
}

fn serialize_calendar_data(data: &Resource, locale: &str) -> Result<String, std::fmt::Error> {
    let calendar = &data.get(locale).unwrap().dates.calendars.gregorian;
    let mut result = String::new();

    writeln!(result, "use crate::data::layout::*;")?;
    writeln!(result, "use std::borrow::Cow;\n")?;

    writeln!(
        result,
        "pub const RESOURCE: Resource<'static> = Resource {{"
    )?;
    writeln!(result, "    main: MainResource {{")?;
    writeln!(result, "        pl: LocaleResource {{")?;
    writeln!(result, "            dates: CalendarDates {{")?;
    writeln!(result, "                calendars: Calendar {{")?;
    writeln!(
        result,
        "                    gregorian: GregorianCalendar {{"
    )?;

    writeln!(result, "                        months: Months {{")?;
    writeln!(
        result,
        "                           format: {},",
        serialize_month_names(&calendar.months.format)?
    )?;
    writeln!(
        result,
        "                           stand_alone: {},",
        serialize_month_names(&calendar.months.stand_alone)?
    )?;
    writeln!(result, "                        }},")?;
    writeln!(result, "                        days: Days {{")?;
    writeln!(
        result,
        "                           format: {},",
        serialize_day_names(&calendar.days.format)?
    )?;
    writeln!(
        result,
        "                           stand_alone: {},",
        serialize_day_names(&calendar.days.stand_alone)?
    )?;
    writeln!(result, "                        }},")?;
    writeln!(
        result,
        "                        date_formats: {}",
        serialize_dt_formats(&calendar.date_formats)?
    )?;
    writeln!(
        result,
        "                        time_formats: {}",
        serialize_dt_formats(&calendar.time_formats)?
    )?;
    writeln!(
        result,
        "                        date_time_formats: {}",
        serialize_dt_formats(&calendar.date_time_formats)?
    )?;
    writeln!(result, "                    }},")?;
    writeln!(result, "                }},")?;
    writeln!(result, "            }},")?;
    writeln!(result, "        }},")?;
    writeln!(result, "    }},")?;
    write!(result, "}};")?;
    Ok(result)
}

fn clean_dir(path: &std::path::Path) -> std::io::Result<()> {
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        fs::remove_file(path.unwrap().path())?;
    }
    Ok(())
}

fn should_skip(locale: &str, include_locales: Option<&'static [&str]>) -> bool {
    if let Some(includes) = include_locales {
        !includes.contains(&locale)
    } else {
        true
    }
}

fn main() {
    // Future config!
    let cldr_dates_modern_path = "./data/cldr-dates-modern";
    let include_locales: Option<&'static [&str]> = Some(&["pl"]);
    let exclude_locales: &'static [&str] = &[];
    let dest_path = "./src/data/generated/";

    let path = Path::new(cldr_dates_modern_path).join("main");
    let dest_dir = Path::new(dest_path);

    clean_dir(dest_dir);

    let paths = fs::read_dir(path).unwrap();

    let mut generated_locales = vec![];
    for path in paths {
        let locale = path
            .unwrap()
            .file_name()
            .to_os_string()
            .into_string()
            .unwrap();
        if should_skip(&locale, include_locales) {
            continue;
        }

        // println!("Name: {:#?}", locale);
        let data = get_calendar_data("./data/cldr-dates-modern", &locale);
        let result = serialize_calendar_data(&data, &locale).unwrap();

        let rust_file = locale.replace("-", "_");
        fs::write(dest_dir.join(format!("{}.rs", rust_file)), result)
            .expect("Unable to write file");
        generated_locales.push(rust_file);
    }

    let mut res = String::from("use crate::data::layout::Resource;\n");

    res.push_str(
        &generated_locales
            .iter()
            .map(|l| format!("pub mod {};", l))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    res.push_str(
        r#"

pub fn get(locale: &str) -> &'static Resource<'static> {
    match locale {
        "pl" => &pl::RESOURCE,
        _ => panic!()
    }
}
    "#,
    );
    fs::write(dest_dir.join("mod.rs"), res).expect("Unable to write file");
}
