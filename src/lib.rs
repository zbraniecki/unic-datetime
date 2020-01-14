// XXX: This should be private, since we want to be able to modify the internal
// data layout without breaking version.
// Unfortunately, we use this for data generation binary.
pub mod data;

use data::layout;
use data::layout::Resource;

use std::borrow::Borrow;
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
    pub fn idx(self) -> usize {
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
    pub fn idx(self) -> usize {
        match self {
            Self::FULL => 0,
            Self::LONG => 1,
            Self::MEDIUM => 2,
            Self::SHORT => 3,
        }
    }
}

pub struct DateTimeFormat<R> {
    pattern: data::layout::DateTimePattern,
    calendar_data: R,
}

#[cfg(not(feature = "no-static"))]
impl DateTimeFormat<&'static Resource<'_>> {
    pub fn new_from_static(
        locale: &str,
        date_style: Option<DateStyle>,
        time_style: Option<TimeStyle>,
    ) -> Self {
        let data = crate::data::generated::get(locale);
        Self::new(locale, date_style, time_style, &data)
    }
}

fn create_date_time_pattern<'l, R: Borrow<Resource<'l>>>(
    pattern: &layout::DateTimePattern,
    date_style: DateStyle,
    time_style: TimeStyle,
    resource: R,
) -> Cow<'static, [layout::PatternElement]> {
    let calendar_data = &resource
        .borrow()
        .get("pl")
        .unwrap()
        .dates
        .calendars
        .gregorian;
    let mut pattern: Vec<data::layout::PatternElement> = pattern.to_parsed().to_vec();

    if let Some(idx) = pattern
        .iter()
        .position(|s| s == &data::layout::PatternElement::Token(data::layout::DateTimeToken::Sub1))
    {
        pattern.splice(
            idx..=idx,
            calendar_data
                .date_formats
                .get(date_style.idx())
                .unwrap()
                .to_parsed(),
        );
    }
    if let Some(idx) = pattern
        .iter()
        .position(|s| s == &data::layout::PatternElement::Token(data::layout::DateTimeToken::Sub0))
    {
        pattern.splice(
            idx..=idx,
            calendar_data
                .time_formats
                .get(time_style.idx())
                .unwrap()
                .to_parsed(),
        );
    }
    Cow::Owned(pattern)
}

impl<'l, R> DateTimeFormat<R> {
    pub fn new(
        _locale: &str,
        date_style: Option<DateStyle>,
        time_style: Option<TimeStyle>,
        data: R,
    ) -> Self
    where
        R: Borrow<Resource<'l>>,
    {
        let calendar_data = &data.borrow().get("pl").unwrap().dates.calendars.gregorian;
        let pattern = match (date_style, time_style) {
            (Some(date_style), Some(time_style)) => {
                let pattern = &calendar_data
                    .date_time_formats
                    .get(date_style.idx())
                    .unwrap();
                let pattern =
                    create_date_time_pattern(pattern, date_style, time_style, data.borrow());
                layout::DateTimePattern::Parsed(pattern)
            }
            (Some(date_style), None) => calendar_data
                .date_formats
                .get(date_style.idx())
                .unwrap()
                .clone(),
            (None, Some(time_style)) => calendar_data
                .time_formats
                .get(time_style.idx())
                .unwrap()
                .clone(),
            (None, None) => panic!(),
        };
        Self {
            pattern,
            calendar_data: data,
        }
    }

    pub fn format(&self, value: &DateTime) -> String
    where
        R: Borrow<Resource<'l>>,
    {
        let mut result = String::new();
        self.calendar_data
            .borrow()
            .format_pattern(&mut result, self.pattern.borrow(), value)
            .unwrap();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let dt = DateTime::new(2019, 10, 29, 10, 23, 5);
        let dtf = DateTimeFormat::new_from_static("pl", Some(DateStyle::LONG), None);
        assert_eq!(dtf.format(&dt), "29 października 2019");

        let dtf = DateTimeFormat::new_from_static("pl", Some(DateStyle::SHORT), None);
        assert_eq!(dtf.format(&dt), "29.10.2019");

        let dtf =
            DateTimeFormat::new_from_static("pl", Some(DateStyle::MEDIUM), Some(TimeStyle::MEDIUM));
        assert_eq!(dtf.format(&dt), "29 paź 2019, 10:23:05");
    }
}
