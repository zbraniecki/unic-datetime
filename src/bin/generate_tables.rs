use std::fmt::Write;
use unic_datetime::data::layout2::*;
use unic_datetime::data::load3::get_calendar_data;

fn serialize_day_list(list: &DayList) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    let mut i = 0;
    write!(result, "DayList {{\n");
    for key in &["sun", "mon", "tue", "wed", "thu", "fri", "sat"] {
        writeln!(
            result,
            r#"                                   {}: Cow::Borrowed("{}"),"#,
            key,
            list.get(i),
        );
        i += 1;
    }
    write!(result, "                                }},");
    Ok(result)
}

fn serialize_month_list(list: &MonthList) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    write!(result, "MonthList {{\n");
    for i in 1..=12 {
        writeln!(
            result,
            r#"                                    m{}: Cow::Borrowed("{}"),"#,
            i,
            list.get(i - 1)
        )?;
    }
    write!(result, "                                }},");
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
    // writeln!(
    //     result,
    //     "                                short: {}",
    //     serialize_month_list(&months.short)?
    // )?;
    writeln!(
        result,
        "                                wide: {}",
        serialize_month_list(&months.wide)?
    )?;
    write!(result, "                           }}")?;
    Ok(result)
}

fn serialize_dt_format(pattern: &[PatternElement]) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    writeln!(result, r#"DateTimePattern::Parsed(Cow::Borrowed(&["#)?;
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
    write!(result, r#"                            ]))"#)?;
    Ok(result)
}
fn serialize_dt_formats(formats: &Formats) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    writeln!(result, "Formats {{")?;
    writeln!(
        result,
        "                            full: {},",
        serialize_dt_format(&formats.full.to_parsed())?
    )?;
    writeln!(
        result,
        "                            long: {},",
        serialize_dt_format(&formats.long.to_parsed())?
    )?;
    writeln!(
        result,
        "                            medium: {},",
        serialize_dt_format(&formats.medium.to_parsed())?
    )?;
    writeln!(
        result,
        "                            short: {},",
        serialize_dt_format(&formats.short.to_parsed())?
    )?;
    write!(result, "                        }},")?;
    Ok(result)
}

fn serialize_calendar_data(data: &Resource) -> Result<String, std::fmt::Error> {
    let mut result = String::new();

    writeln!(result, "use super::layout2::*;")?;
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
        serialize_month_names(&data.main.pl.dates.calendars.gregorian.months.format)?
    )?;
    writeln!(
        result,
        "                           stand_alone: {},",
        serialize_month_names(&data.main.pl.dates.calendars.gregorian.months.stand_alone)?
    )?;
    writeln!(result, "                        }},")?;
    writeln!(result, "                        days: Days {{")?;
    writeln!(
        result,
        "                           format: {},",
        serialize_day_names(&data.main.pl.dates.calendars.gregorian.days.format)?
    )?;
    writeln!(
        result,
        "                           stand_alone: {},",
        serialize_day_names(&data.main.pl.dates.calendars.gregorian.days.stand_alone)?
    )?;
    writeln!(result, "                        }},")?;
    writeln!(
        result,
        "                        date_formats: {}",
        serialize_dt_formats(&data.main.pl.dates.calendars.gregorian.date_formats)?
    )?;
    writeln!(
        result,
        "                        time_formats: {}",
        serialize_dt_formats(&data.main.pl.dates.calendars.gregorian.time_formats)?
    )?;
    writeln!(
        result,
        "                        date_time_formats: {}",
        serialize_dt_formats(&data.main.pl.dates.calendars.gregorian.date_time_formats)?
    )?;
    writeln!(result, "                    }},")?;
    writeln!(result, "                }},")?;
    writeln!(result, "            }},")?;
    writeln!(result, "        }},")?;
    writeln!(result, "    }},")?;
    write!(result, "}};")?;
    Ok(result)
}

fn main() {
    let data = get_calendar_data("./data/cldr-dates-modern", "pl");

    let result = serialize_calendar_data(&data).unwrap();
    println!("{}", result);
}
