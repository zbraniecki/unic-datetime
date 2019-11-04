mod data;

use data::{PL_DATE_FORMATS, PL_DATE_TIME_FORMATS, PL_GREGORIAN_MONTHS, PL_TIME_FORMATS};

/* DateTime */
pub struct DateTime {
    pub year: usize,
    pub month: usize,
    pub day: usize,
    pub hour: usize,
    pub minute: usize,
    pub second: usize,
}

impl DateTime {
    pub fn new(
        year: usize,
        month: usize,
        day: usize,
        hour: usize,
        minute: usize,
        second: usize,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }
}

/* DateTimeFormat */

#[derive(Clone, Copy)]
pub enum DateStyle {
    FULL,
    LONG,
    MEDIUM,
    SHORT,
}

impl DateStyle {
    pub fn idx(&self) -> usize {
        match self {
            Self::FULL => 0,
            Self::LONG => 1,
            Self::MEDIUM => 2,
            Self::SHORT => 3,
        }
    }
}

#[derive(Clone, Copy)]
pub enum TimeStyle {
    FULL,
    LONG,
    MEDIUM,
    SHORT,
}

impl TimeStyle {
    pub fn idx(&self) -> usize {
        match self {
            Self::FULL => 0,
            Self::LONG => 1,
            Self::MEDIUM => 2,
            Self::SHORT => 3,
        }
    }
}

pub struct DateTimeFormat {
    pattern: String,
    month_names: &'static data::layout::GregorianMonths,
}

impl DateTimeFormat {
    pub fn new(
        _locale: &str,
        date_style: Option<DateStyle>,
        time_style: Option<TimeStyle>,
    ) -> Self {
        let pattern = match (date_style, time_style) {
            (Some(date_style), Some(time_style)) => {
                let connector = &PL_DATE_TIME_FORMATS.0[date_style.idx()];
                let date_pattern = PL_DATE_FORMATS.0[date_style.idx()];
                let time_pattern = PL_TIME_FORMATS.0[time_style.idx()];
                connector
                    .replace("{1}", date_pattern)
                    .replace("{0}", time_pattern)
            }
            (Some(date_style), None) => PL_DATE_FORMATS.0[date_style.idx()].to_string(),
            (None, Some(time_style)) => PL_TIME_FORMATS.0[time_style.idx()].to_string(),
            (None, None) => panic!(),
        };
        Self {
            pattern,
            month_names: &PL_GREGORIAN_MONTHS,
        }
    }

    pub fn format(&self, value: &DateTime) -> String {
        let month_names_wide = self
            .month_names
            .get_list(false, data::layout::MonthNamesLength::WIDE)
            .unwrap();
        let month_names_abbreviated = self
            .month_names
            .get_list(false, data::layout::MonthNamesLength::ABBREVIATED)
            .unwrap();
        let month_name_wide = month_names_wide[value.month - 1];
        let month_name_abbreviated = month_names_abbreviated[value.month - 1];
        self.pattern
            .clone()
            .replace("zzzz", "Pacific Dailight Time")
            .replace("z", "PDT")
            .replace("dd", &format_number(value.day, true))
            .replace("d", &format_number(value.day, false))
            .replace("y", &format_number(value.year, false))
            .replace("HH", &format_number(value.hour, true))
            .replace("mm", &format_number(value.minute, true))
            .replace("ss", &format_number(value.second, true))
            .replace("EEEE", "Wtorek")
            .replace("MMMM", month_name_wide)
            .replace("MMM", month_name_abbreviated)
            .replace("MM", &format_number(value.month, true))
    }
}

fn format_number(num: usize, two_digit: bool) -> String {
    if two_digit && num < 10 {
        let mut s = String::from("0");
        s.push_str(&num.to_string());
        s
    } else {
        num.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let dt = DateTime::new(2019, 10, 29, 10, 23, 5);
        let dtf = DateTimeFormat::new("pl", Some(DateStyle::LONG), None);
        assert_eq!(dtf.format(&dt), "29 października 2019");

        let dtf = DateTimeFormat::new("pl", Some(DateStyle::SHORT), None);
        assert_eq!(dtf.format(&dt), "29.10.2019");

        let dtf = DateTimeFormat::new("pl", Some(DateStyle::MEDIUM), Some(TimeStyle::MEDIUM));
        assert_eq!(dtf.format(&dt), "29 paź 2019, 10:23:05");
    }
}
