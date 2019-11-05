use std::fmt::Write;

#[derive(Default)]
pub struct CalendarData<S: 'static + AsRef<str>> {
    pub months: MonthNames<S>,
    pub date_formats: [&'static Pattern<S>; 4],
    pub time_formats: [&'static Pattern<S>; 4],
    pub date_time_formats: [&'static Pattern<S>; 4],
}

pub type Pattern<S> = [PatternElement<S>];

#[derive(Clone, Debug, PartialEq)]
pub enum PatternElement<S> {
    Literal(S),
    Token(DateTimeToken),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DateTimeToken {
    WeekDayWide,          // EEEE
    DayNumeric,           // d
    Day2digit,            // dd
    MonthNameLong,        // MMMM
    MonthNameAbbreviated, // MMM
    Month2digit,          // MM
    YearNumeric,          // y

    Hour2digit,
    Minute2digit,
    Second2digit,

    ZoneLong,
    ZoneShort,

    Sub0, // {0}
    Sub1, // {1}
}

#[derive(Default)]
pub struct MonthNames<S> {
    pub stand_alone: Option<MonthNamesTypes<S>>,
    pub format: Option<MonthNamesTypes<S>>,
}

#[derive(Default)]
pub struct MonthNamesTypes<S> {
    pub abbreviated: Option<MonthList<S>>,
    pub narrow: Option<MonthList<S>>,
    pub short: Option<MonthList<S>>,
    pub wide: Option<MonthList<S>>,
}

pub type MonthList<S> = [S; 12];

pub enum MonthNamesLength {
    ABBREVIATED,
    NARROW,
    SHORT,
    WIDE,
}

fn format_number(
    result: &mut impl Write,
    num: usize,
    two_digit: bool,
) -> Result<(), std::fmt::Error> {
    if two_digit {
        write!(result, "{:0>2}", num)
    } else {
        write!(result, "{}", num)
    }
}

impl<S> CalendarData<S>
where
    S: std::convert::AsRef<str>,
{
    pub fn format_pattern(
        &self,
        mut result: &mut impl Write,
        pattern: &Pattern<S>,
        input: &crate::DateTime,
    ) -> Result<(), std::fmt::Error> {
        for elem in pattern {
            match elem {
                PatternElement::Literal(s) => result.write_str(s.as_ref())?,
                PatternElement::Token(t) => match t {
                    DateTimeToken::WeekDayWide => result.write_str("Wtorek")?,
                    DateTimeToken::DayNumeric => format_number(&mut result, input.day, false)?,
                    DateTimeToken::Day2digit => format_number(&mut result, input.day, true)?,
                    DateTimeToken::Month2digit => format_number(&mut result, input.month, true)?,
                    DateTimeToken::MonthNameLong => {
                        let month_name =
                            &self.months.get_list(false, MonthNamesLength::WIDE).unwrap()
                                [input.month - 1];
                        result.write_str(month_name.as_ref())?
                    }
                    DateTimeToken::MonthNameAbbreviated => {
                        let month_name = &self
                            .months
                            .get_list(false, MonthNamesLength::ABBREVIATED)
                            .unwrap()[input.month - 1];
                        result.write_str(month_name.as_ref())?
                    }
                    DateTimeToken::YearNumeric => format_number(&mut result, input.year, false)?,
                    DateTimeToken::Hour2digit => format_number(&mut result, input.hour, true)?,
                    DateTimeToken::Minute2digit => format_number(&mut result, input.minute, true)?,
                    DateTimeToken::Second2digit => format_number(&mut result, input.second, true)?,

                    DateTimeToken::ZoneLong => result.write_str("Pacific Daylight Time")?,
                    DateTimeToken::ZoneShort => result.write_str("PDT")?,
                    _ => unimplemented!(),
                },
            }
        }
        Ok(())
    }
}

impl<S> MonthNames<S>
where
    S: std::convert::AsRef<str>,
{
    pub fn get_list(&self, stand_alone: bool, length: MonthNamesLength) -> Option<&MonthList<S>> {
        let list = if stand_alone {
            &self.stand_alone
        } else {
            &self.format
        }
        .as_ref()
        .unwrap();

        match length {
            MonthNamesLength::ABBREVIATED => list.abbreviated.as_ref(),
            MonthNamesLength::NARROW => list.narrow.as_ref(),
            MonthNamesLength::SHORT => list.short.as_ref(),
            MonthNamesLength::WIDE => list.wide.as_ref(),
        }
    }
}
