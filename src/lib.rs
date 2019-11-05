// XXX: This should be private, since we want to be able to modify the internal
// data layout without breaking version.
// Unfortunately, we use this for data generation binary.
pub mod data;

use data::pl::CALENDAR_DATA as PL_CALENDAR_DATA;
use std::borrow::Cow;

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
    pattern: Cow<'static, data::layout::Pattern<&'static str>>,
    calendar_data: &'static data::layout::CalendarData<&'static str>,
}

impl DateTimeFormat {
    pub fn new(
        _locale: &str,
        date_style: Option<DateStyle>,
        time_style: Option<TimeStyle>,
    ) -> Self {
        let pattern = match (date_style, time_style) {
            (Some(date_style), Some(time_style)) => {
                let mut pattern = PL_CALENDAR_DATA.date_time_formats[date_style.idx()].to_vec();

                if let Some(idx) = pattern.iter().position(|s| {
                    s == &data::layout::PatternElement::Token(data::layout::DateTimeToken::Sub1)
                }) {
                    pattern.splice(
                        idx..(idx + 1),
                        PL_CALENDAR_DATA.date_formats[date_style.idx()]
                            .iter()
                            .cloned(),
                    );
                }
                if let Some(idx) = pattern.iter().position(|s| {
                    s == &data::layout::PatternElement::Token(data::layout::DateTimeToken::Sub0)
                }) {
                    pattern.splice(
                        idx..(idx + 1),
                        PL_CALENDAR_DATA.time_formats[time_style.idx()]
                            .iter()
                            .cloned(),
                    );
                }
                Cow::from(pattern)
            }
            (Some(date_style), None) => Cow::from(PL_CALENDAR_DATA.date_formats[date_style.idx()]),
            (None, Some(time_style)) => Cow::from(PL_CALENDAR_DATA.time_formats[time_style.idx()]),
            (None, None) => panic!(),
        };
        Self {
            pattern,
            calendar_data: &PL_CALENDAR_DATA,
        }
    }

    pub fn format(&self, value: &DateTime) -> String {
        let mut result = String::new();
        self.calendar_data
            .format_pattern(&mut result, &self.pattern, value);
        result
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
