use std::borrow::{Cow, ToOwned};
use std::fmt::Write;

#[derive(Clone)]
pub struct CalendarData
where
    [PatternElement]: ToOwned,
{
    pub months: MonthNames,
    pub days: DayNames,
    pub date_formats: [Pattern; 4],
    pub time_formats: [Pattern; 4],
    pub date_time_formats: [Pattern; 4],
}

pub type Pattern = Cow<'static, [PatternElement]>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PatternElement {
    Literal(Cow<'static, str>),
    Token(DateTimeToken),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DateTimeToken {
    WeekDayWide,          // EEEE
    DayNumeric,           // d
    Day2digit,            // dd
    MonthNameLong,        // MMMM
    MonthNameAbbreviated, // MMM
    Month2digit,          // MM
    YearNumeric,          // y
    Year2digit,           // yy

    Hour2digit,
    Minute2digit,
    Second2digit,

    ZoneLong,
    ZoneShort,

    Sub0, // {0}
    Sub1, // {1}
}

impl DateTimeToken {
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::WeekDayWide => "WeekDayWide",
            Self::DayNumeric => "DayNumeric",
            Self::Day2digit => "Day2digit",
            Self::MonthNameLong => "MonthNameLong",
            Self::MonthNameAbbreviated => "MonthNameAbbreviated",
            Self::Month2digit => "Month2digit",
            Self::YearNumeric => "YearNumeric",
            Self::Year2digit => "Year2digit",
            Self::Hour2digit => "Hour2digit",
            Self::Minute2digit => "Minute2digit",
            Self::Second2digit => "Second2digit",
            Self::ZoneLong => "ZoneLong",
            Self::ZoneShort => "ZoneShort",
            Self::Sub0 => "Sub0",
            Self::Sub1 => "Sub1",
        }
    }
}

#[derive(Clone)]
pub struct MonthNames {
    pub stand_alone: Option<MonthNamesTypes>,
    pub format: Option<MonthNamesTypes>,
}

#[derive(Clone)]
pub struct MonthNamesTypes {
    pub abbreviated: Option<MonthList>,
    pub narrow: Option<MonthList>,
    pub short: Option<MonthList>,
    pub wide: Option<MonthList>,
}

pub type MonthList = [Cow<'static, str>; 12];

#[derive(Clone)]
pub struct DayNames {
    pub stand_alone: Option<DayNamesTypes>,
    pub format: Option<DayNamesTypes>,
}

#[derive(Clone)]
pub struct DayNamesTypes {
    pub abbreviated: Option<DayList>,
    pub narrow: Option<DayList>,
    pub short: Option<DayList>,
    pub wide: Option<DayList>,
}

pub type DayList = [Cow<'static, str>; 7];

pub enum NamesLength {
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

//def day_of_week(year, month, day):

fn get_day_of_week(year: usize, month: usize, day: usize) -> usize {
    let t = &[0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let year = if month < 3 { year - 1 } else { year };
    return (year + year / 4 - year / 100 + year / 400 + t[month - 1] + day) % 7;
}

impl CalendarData
where
    [PatternElement]: ToOwned,
{
    pub fn format_pattern(
        &self,
        mut result: &mut impl Write,
        pattern: &[PatternElement],
        input: &crate::DateTime,
    ) -> Result<(), std::fmt::Error> {
        for elem in pattern {
            match elem {
                PatternElement::Literal(s) => result.write_str(s.as_ref())?,
                PatternElement::Token(t) => match t {
                    DateTimeToken::WeekDayWide => {
                        let day_name = &self.days.get_list(false, NamesLength::WIDE).unwrap()
                            [get_day_of_week(input.year, input.month, input.day)];
                        result.write_str(day_name.as_ref())?
                    }
                    DateTimeToken::DayNumeric => format_number(&mut result, input.day, false)?,
                    DateTimeToken::Day2digit => format_number(&mut result, input.day, true)?,
                    DateTimeToken::Month2digit => format_number(&mut result, input.month, true)?,
                    DateTimeToken::MonthNameLong => {
                        let month_name = &self.months.get_list(false, NamesLength::WIDE).unwrap()
                            [input.month - 1];
                        result.write_str(month_name.as_ref())?
                    }
                    DateTimeToken::MonthNameAbbreviated => {
                        let month_name = &self
                            .months
                            .get_list(false, NamesLength::ABBREVIATED)
                            .unwrap()[input.month - 1];
                        result.write_str(month_name.as_ref())?
                    }
                    DateTimeToken::YearNumeric => format_number(&mut result, input.year, false)?,
                    DateTimeToken::Year2digit => format_number(&mut result, input.year, true)?,
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

impl MonthNames {
    pub fn get_list(&self, stand_alone: bool, length: NamesLength) -> Option<&MonthList> {
        let list = if stand_alone {
            &self.stand_alone
        } else {
            &self.format
        }
        .as_ref()
        .unwrap();

        match length {
            NamesLength::ABBREVIATED => list.abbreviated.as_ref(),
            NamesLength::NARROW => list.narrow.as_ref(),
            NamesLength::SHORT => list.short.as_ref(),
            NamesLength::WIDE => list.wide.as_ref(),
        }
    }
}

impl DayNames {
    pub fn get_list(&self, stand_alone: bool, length: NamesLength) -> Option<&DayList> {
        let list = if stand_alone {
            &self.stand_alone
        } else {
            &self.format
        }
        .as_ref()
        .unwrap();

        match length {
            NamesLength::ABBREVIATED => list.abbreviated.as_ref(),
            NamesLength::NARROW => list.narrow.as_ref(),
            NamesLength::SHORT => list.short.as_ref(),
            NamesLength::WIDE => list.wide.as_ref(),
        }
    }
}
