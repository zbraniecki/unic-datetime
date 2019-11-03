/* DateTime */
struct DateTime {
    pub year: usize,
    pub month: usize,
    pub day: usize,
    pub hour: usize,
    pub minute: usize,
    pub second: usize,
}

impl DateTime {
    pub fn new(instant: usize) -> Self {
        Self {
            year: 2019,
            month: 10,
            day: 29,
            hour: 10,
            minute: 23,
            second: 05,
        }
    }
}

/* Data */
const PL_DATE_PATTERNS: [&str; 4] = ["EEEE, d MMMM y", "d MMMM y", "d MMM y", "dd.MM.y"];
const PL_TIME_PATTERNS: [&str; 4] = ["HH:mm:ss zzzz", "HH:mm:ss z", "HH:mm:ss", "HH:mm"];
const PL_DATE_TIME_PATTERNS: [&str; 4] = ["{1} {0}", "{1} {0}", "{1}, {0}", "{1}, {0}"];

const PL_MONTH_NAMES_WIDE: [&str; 12] = [
    "stycznia",
    "lutego",
    "marca",
    "kwietnia",
    "maja",
    "czerwca",
    "lipca",
    "sierpnia",
    "września",
    "października",
    "listopada",
    "grudnia",
];
const PL_MONTH_NAMES_ABBREVIATED: [&str; 12] = [
    "sty", "lut", "mar", "kwi", "maj", "cze", "lip", "sie", "wrz", "paź", "lis", "gru",
];

/* DateTimeFormat */

enum DateStyle {
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

enum TimeStyle {
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

struct DateTimeFormat {
    pattern: String,
    month_names_wide: &'static [&'static str],
    month_names_abbreviated: &'static [&'static str],
}

impl DateTimeFormat {
    pub fn new(locale: &str, date_style: Option<DateStyle>, time_style: Option<TimeStyle>) -> Self {
        let pattern = match (date_style, time_style) {
            (Some(date_style), Some(time_style)) => {
                let connector = PL_DATE_TIME_PATTERNS[date_style.idx()];
                let date_pattern = PL_DATE_PATTERNS[date_style.idx()];
                let time_pattern = PL_TIME_PATTERNS[time_style.idx()];
                connector
                    .replace("{1}", date_pattern)
                    .replace("{0}", time_pattern)
            }
            (Some(date_style), None) => PL_DATE_PATTERNS[date_style.idx()].to_string(),
            (None, Some(time_style)) => PL_TIME_PATTERNS[time_style.idx()].to_string(),
            (None, None) => panic!(),
        };
        Self {
            pattern,
            month_names_wide: &PL_MONTH_NAMES_WIDE,
            month_names_abbreviated: &PL_MONTH_NAMES_ABBREVIATED,
        }
    }

    pub fn format(&self, value: &DateTime) -> String {
        let month_name_wide = self.month_names_wide[value.month - 1];
        let month_name_abbreviated = self.month_names_abbreviated[value.month - 1];
        self.pattern
            .clone()
            .replace("dd", &format_number(value.day, true))
            .replace("d", &format_number(value.day, false))
            .replace("MMMM", month_name_wide)
            .replace("MMM", month_name_abbreviated)
            .replace("MM", &format_number(value.month, true))
            .replace("y", &format_number(value.year, false))
            .replace("HH", &format_number(value.hour, true))
            .replace("mm", &format_number(value.minute, true))
            .replace("ss", &format_number(value.second, true))
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
        let dt = DateTime::new(0);
        let dtf = DateTimeFormat::new("pl", Some(DateStyle::LONG), None);
        assert_eq!(dtf.format(&dt), "29 października 2019");

        let dtf = DateTimeFormat::new("pl", Some(DateStyle::SHORT), None);
        assert_eq!(dtf.format(&dt), "29.10.2019");

        let dtf = DateTimeFormat::new("pl", Some(DateStyle::MEDIUM), Some(TimeStyle::MEDIUM));
        assert_eq!(dtf.format(&dt), "29 paź 2019, 10:23:05");
    }
}
