#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Resource<'l> {
    pub main: MainResource<'l>,
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
    pub date_formats: Formats<'l>,
    #[cfg_attr(feature = "serde", serde(rename = "timeFormats"))]
    pub time_formats: Formats<'l>,
    #[cfg_attr(feature = "serde", serde(rename = "dateTimeFormats"))]
    pub date_time_formats: Formats<'l>,
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Days<'l> {
    #[cfg_attr(feature = "serde", serde(rename = "stand-alone"))]
    pub stand_alone: DayTypes<'l>,
    pub format: DayTypes<'l>,
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Months<'l> {
    #[cfg_attr(feature = "serde", serde(rename = "stand-alone"))]
    pub stand_alone: MonthTypes<'l>,
    pub format: MonthTypes<'l>,
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

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Formats<'l> {
    pub full: DateTimePattern<'l>,
    pub long: DateTimePattern<'l>,
    pub medium: DateTimePattern<'l>,
    pub short: DateTimePattern<'l>,
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DateTimePattern<'l> {
    Raw(Cow<'l, str>),
    Parsed(Cow<'l, [PatternElement<'l>]>),
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PatternElement<'l> {
    Literal(Cow<'l, str>),
    Token(DateTimeToken),
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DateTimeToken {
    Sub0, // {0}
    Sub1, // {1}
}
