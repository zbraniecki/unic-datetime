#[cfg(feature = "serde")]
use serde::{de, Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Write;

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Resource<'l> {
    pub main: MainResource<'l>,
}

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

fn get_day_of_week(year: usize, month: usize, day: usize) -> usize {
    let t = &[0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let year = if month < 3 { year - 1 } else { year };
    (year + year / 4 - year / 100 + year / 400 + t[month - 1] + day) % 7
}

impl<'l> Resource<'l> {
    pub fn format_pattern(
        &self,
        mut result: &mut impl Write,
        pattern: &DateTimePattern,
        input: &crate::DateTime,
    ) -> Result<(), std::fmt::Error> {
        let calendar_data = &self.main.pl.dates.calendars.gregorian;
        for elem in pattern.to_parsed().iter() {
            match elem {
                PatternElement::Literal(s) => result.write_str(s.as_ref())?,
                PatternElement::Token(t) => match t {
                    DateTimeToken::WeekDayWide => {
                        let day_name = &calendar_data
                            .days
                            .get_list(false, NamesLength::WIDE)
                            .get(get_day_of_week(input.year, input.month, input.day));
                        result.write_str(day_name.as_ref())?
                    }
                    DateTimeToken::DayNumeric => format_number(&mut result, input.day, false)?,
                    DateTimeToken::Day2digit => format_number(&mut result, input.day, true)?,
                    DateTimeToken::Month2digit => format_number(&mut result, input.month, true)?,
                    DateTimeToken::MonthNameLong => {
                        let month_name = &calendar_data
                            .months
                            .get_list(false, NamesLength::WIDE)
                            .get(input.month - 1);
                        result.write_str(month_name.as_ref())?
                    }
                    DateTimeToken::MonthNameAbbreviated => {
                        let month_name = &calendar_data
                            .months
                            .get_list(false, NamesLength::ABBREVIATED)
                            .get(input.month - 1);
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

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MainResource<'l> {
    pub pl: LocaleResource<'l>,
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LocaleResource<'l> {
    pub dates: CalendarDates<'l>,
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CalendarDates<'l> {
    pub calendars: Calendar<'l>,
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Calendar<'l> {
    pub gregorian: GregorianCalendar<'l>,
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GregorianCalendar<'l> {
    pub months: Months<'l>,
    pub days: Days<'l>,
    #[cfg_attr(feature = "serde", serde(rename = "dateFormats"))]
    pub date_formats: Formats,
    #[cfg_attr(feature = "serde", serde(rename = "timeFormats"))]
    pub time_formats: Formats,
    #[cfg_attr(feature = "serde", serde(rename = "dateTimeFormats"))]
    pub date_time_formats: Formats,
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Days<'l> {
    #[cfg_attr(feature = "serde", serde(rename = "stand-alone"))]
    pub stand_alone: DayTypes<'l>,
    pub format: DayTypes<'l>,
}

impl<'l> Days<'l> {
    pub fn get_list(&self, stand_alone: bool, length: NamesLength) -> &DayList<'l> {
        let list = if stand_alone {
            &self.stand_alone
        } else {
            &self.format
        };

        match length {
            NamesLength::ABBREVIATED => &list.abbreviated,
            NamesLength::NARROW => &list.narrow,
            NamesLength::SHORT => &list.short,
            NamesLength::WIDE => &list.wide,
        }
    }
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Months<'l> {
    #[cfg_attr(feature = "serde", serde(rename = "stand-alone"))]
    pub stand_alone: MonthTypes<'l>,
    pub format: MonthTypes<'l>,
}

impl<'l> Months<'l> {
    pub fn get_list(&self, stand_alone: bool, length: NamesLength) -> &MonthList<'l> {
        let list = if stand_alone {
            &self.stand_alone
        } else {
            &self.format
        };

        match length {
            NamesLength::ABBREVIATED => &list.abbreviated,
            NamesLength::NARROW => &list.narrow,
            // NamesLength::SHORT => &list.short,
            NamesLength::WIDE => &list.wide,
            _ => unimplemented!(),
        }
    }
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DayTypes<'l> {
    pub abbreviated: DayList<'l>,
    pub narrow: DayList<'l>,
    pub short: DayList<'l>,
    pub wide: DayList<'l>,
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MonthTypes<'l> {
    pub abbreviated: MonthList<'l>,
    pub narrow: MonthList<'l>,
    // pub short: MonthList<'l>,
    pub wide: MonthList<'l>,
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DayList<'l> {
    pub sun: Cow<'l, str>,
    pub mon: Cow<'l, str>,
    pub tue: Cow<'l, str>,
    pub wed: Cow<'l, str>,
    pub thu: Cow<'l, str>,
    pub fri: Cow<'l, str>,
    pub sat: Cow<'l, str>,
}

impl<'l> DayList<'l> {
    pub fn get(&self, idx: usize) -> &Cow<'l, str> {
        match idx {
            0 => &self.sun,
            1 => &self.mon,
            2 => &self.tue,
            3 => &self.wed,
            4 => &self.thu,
            5 => &self.fri,
            6 => &self.sat,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MonthList<'l> {
    #[cfg_attr(feature = "serde", serde(rename = "1"))]
    pub m1: Cow<'l, str>,
    #[cfg_attr(feature = "serde", serde(rename = "2"))]
    pub m2: Cow<'l, str>,
    #[cfg_attr(feature = "serde", serde(rename = "3"))]
    pub m3: Cow<'l, str>,
    #[cfg_attr(feature = "serde", serde(rename = "4"))]
    pub m4: Cow<'l, str>,
    #[cfg_attr(feature = "serde", serde(rename = "5"))]
    pub m5: Cow<'l, str>,
    #[cfg_attr(feature = "serde", serde(rename = "6"))]
    pub m6: Cow<'l, str>,
    #[cfg_attr(feature = "serde", serde(rename = "7"))]
    pub m7: Cow<'l, str>,
    #[cfg_attr(feature = "serde", serde(rename = "8"))]
    pub m8: Cow<'l, str>,
    #[cfg_attr(feature = "serde", serde(rename = "9"))]
    pub m9: Cow<'l, str>,
    #[cfg_attr(feature = "serde", serde(rename = "10"))]
    pub m10: Cow<'l, str>,
    #[cfg_attr(feature = "serde", serde(rename = "11"))]
    pub m11: Cow<'l, str>,
    #[cfg_attr(feature = "serde", serde(rename = "12"))]
    pub m12: Cow<'l, str>,
}

impl<'l> MonthList<'l> {
    pub fn get(&self, idx: usize) -> &Cow<'l, str> {
        match idx {
            0 => &self.m1,
            1 => &self.m2,
            2 => &self.m3,
            3 => &self.m4,
            4 => &self.m5,
            5 => &self.m6,
            6 => &self.m7,
            7 => &self.m8,
            8 => &self.m9,
            9 => &self.m10,
            10 => &self.m11,
            11 => &self.m12,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Formats {
    pub full: DateTimePattern,
    pub long: DateTimePattern,
    pub medium: DateTimePattern,
    pub short: DateTimePattern,
}

impl Formats {
    pub fn get(&self, idx: usize) -> &DateTimePattern {
        match idx {
            0 => &self.full,
            1 => &self.long,
            2 => &self.medium,
            3 => &self.short,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DateTimePattern {
    Raw(Cow<'static, str>),
    Parsed(Cow<'static, [PatternElement]>),
}

impl DateTimePattern {
    pub fn to_parsed(&self) -> Vec<PatternElement> {
        match *self {
            DateTimePattern::Raw(ref s) => super::patterns2::parse_pattern(&s.as_ref()).unwrap(),
            DateTimePattern::Parsed(ref elements) => elements.to_vec(),
        }
    }
}

pub static mut in_json: bool = false;

#[cfg(feature = "serde")]
impl<'de> de::Deserialize<'de> for DateTimePattern {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MyVisitor;
        impl<'de> de::Visitor<'de> for MyVisitor {
            type Value = DateTimePattern;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a potential or array of potentials")
            }

            fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut vec = vec![];
                while let Some(pot) = seq.next_element()? {
                    vec.push(pot);
                }
                Ok(DateTimePattern::Parsed(Cow::Owned(vec)))
            }

            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                Ok(DateTimePattern::Raw(Cow::Owned(s.to_string())))
            }
        }

        if unsafe { in_json } {
            deserializer.deserialize_any(MyVisitor)
        } else {
            deserializer.deserialize_seq(MyVisitor)
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PatternElement {
    Literal(Cow<'static, str>),
    Token(DateTimeToken),
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
