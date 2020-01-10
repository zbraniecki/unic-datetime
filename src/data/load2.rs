use super::layout::{
    CalendarData, DayList, DayNames, DayNamesTypes, MonthList, MonthNames, MonthNamesTypes, Pattern,
};
use super::patterns::parse_pattern;
use serde::de::{IgnoredAny, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::fs;

#[derive(Deserialize)]
pub struct Resource<'s> {
    #[serde(borrow)]
    pub main: HashMap<&'s str, LocaleResource>,
}

#[derive(Deserialize)]
pub struct LocaleResource {
    pub dates: CalendarDates,
}

#[derive(Deserialize)]
pub struct CalendarDates {
    pub calendars: Calendar,
}

fn deserialize_calendar_data<'de, D>(de: D) -> Result<Option<CalendarData>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Helper(#[serde(with = "GregorianCalendar")] CalendarData);
    let helper = Helper::deserialize(de)?;
    Ok(Some(helper.0))
}

#[derive(Deserialize)]
pub struct Calendar {
    #[serde(deserialize_with = "deserialize_calendar_data")]
    pub gregorian: Option<CalendarData>,
}

struct FormatsMapVisitor {}

impl FormatsMapVisitor {
    fn new() -> Self {
        FormatsMapVisitor {}
    }
}

impl<'de> Visitor<'de> for FormatsMapVisitor {
    // The type that our Visitor is going to produce.
    type Value = [Pattern; 4];

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a very special map")
    }

    // Deserialize MyMap from an abstract "map" provided by the
    // Deserializer. The MapAccess input is a callback provided by
    // the Deserializer to let us see each entry in the map.
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut full = None;
        let mut long = None;
        let mut medium = None;
        let mut short = None;
        while let Some(key) = access.next_key::<&[u8]>()? {
            match key {
                b"full" => full = Some(parse_pattern(access.next_value::<&[u8]>()?).unwrap()),
                b"long" => long = Some(parse_pattern(access.next_value::<&[u8]>()?).unwrap()),
                b"medium" => medium = Some(parse_pattern(access.next_value::<&[u8]>()?).unwrap()),
                b"short" => short = Some(parse_pattern(access.next_value::<&[u8]>()?).unwrap()),
                _ => {
                    access.next_value::<IgnoredAny>()?;
                }
            }
        }

        match (full, long, medium, short) {
            (Some(full), Some(long), Some(medium), Some(short)) => Ok([full, long, medium, short]),
            _ => panic!(),
        }
    }
}

fn deserialize_format_map_of_values<'de, D>(de: D) -> Result<[Pattern; 4], D::Error>
where
    D: Deserializer<'de>,
{
    de.deserialize_map(FormatsMapVisitor::new())
}

#[derive(Deserialize)]
#[serde(remote = "CalendarData")]
pub struct GregorianCalendar {
    #[serde(with = "Months")]
    pub months: MonthNames,
    #[serde(with = "Days")]
    pub days: DayNames,
    #[serde(
        rename = "dateFormats",
        deserialize_with = "deserialize_format_map_of_values"
    )]
    pub date_formats: [Pattern; 4],
    #[serde(
        rename = "timeFormats",
        deserialize_with = "deserialize_format_map_of_values"
    )]
    pub time_formats: [Pattern; 4],
    #[serde(
        rename = "dateTimeFormats",
        deserialize_with = "deserialize_format_map_of_values"
    )]
    pub date_time_formats: [Pattern; 4],
}

fn deserialize_month_types<'de, D>(de: D) -> Result<Option<MonthNamesTypes>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Helper(#[serde(with = "MonthTypes")] MonthNamesTypes);
    let helper = Option::deserialize(de)?;
    Ok(helper.map(|Helper(external)| external))
}

fn deserialize_day_types<'de, D>(de: D) -> Result<Option<DayNamesTypes>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Helper(#[serde(with = "DayTypes")] DayNamesTypes);
    let helper = Option::deserialize(de)?;
    Ok(helper.map(|Helper(external)| external))
}

#[derive(Deserialize)]
#[serde(remote = "MonthNames")]
pub struct Months {
    #[serde(rename = "stand-alone", deserialize_with = "deserialize_month_types")]
    pub stand_alone: Option<MonthNamesTypes>,
    #[serde(deserialize_with = "deserialize_month_types")]
    pub format: Option<MonthNamesTypes>,
}

#[derive(Deserialize)]
#[serde(remote = "DayNames")]
pub struct Days {
    #[serde(rename = "stand-alone", deserialize_with = "deserialize_day_types")]
    pub stand_alone: Option<DayNamesTypes>,
    #[serde(deserialize_with = "deserialize_day_types")]
    pub format: Option<DayNamesTypes>,
}

struct MonthsMapVisitor {}

impl MonthsMapVisitor {
    fn new() -> Self {
        MonthsMapVisitor {}
    }
}

impl<'de> Visitor<'de> for MonthsMapVisitor {
    // The type that our Visitor is going to produce.
    type Value = Option<[Cow<'static, str>; 12]>;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a very special map")
    }

    // Deserialize MyMap from an abstract "map" provided by the
    // Deserializer. The MapAccess input is a callback provided by
    // the Deserializer to let us see each entry in the map.
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut values: [Cow<'static, str>; 12] = [
            Cow::Borrowed(""),
            Cow::Borrowed(""),
            Cow::Borrowed(""),
            Cow::Borrowed(""),
            Cow::Borrowed(""),
            Cow::Borrowed(""),
            Cow::Borrowed(""),
            Cow::Borrowed(""),
            Cow::Borrowed(""),
            Cow::Borrowed(""),
            Cow::Borrowed(""),
            Cow::Borrowed(""),
        ];
        while let Some((key, value)) = access.next_entry::<&[u8], Cow<'static, str>>()? {
            match key {
                b"1" => values[0] = value,
                b"2" => values[1] = value,
                b"3" => values[2] = value,
                b"4" => values[3] = value,
                b"5" => values[4] = value,
                b"6" => values[5] = value,
                b"7" => values[6] = value,
                b"8" => values[7] = value,
                b"9" => values[8] = value,
                b"10" => values[9] = value,
                b"11" => values[10] = value,
                b"12" => values[11] = value,
                _ => panic!(),
            }
        }
        Ok(Some(values))
    }
}

fn deserialize_months_map_of_values<'de, D>(
    de: D,
) -> Result<Option<[Cow<'static, str>; 12]>, D::Error>
where
    D: Deserializer<'de>,
{
    de.deserialize_map(MonthsMapVisitor::new())
}

struct DaysMapVisitor {}

impl DaysMapVisitor {
    fn new() -> Self {
        DaysMapVisitor {}
    }
}

impl<'de> Visitor<'de> for DaysMapVisitor {
    // The type that our Visitor is going to produce.
    type Value = Option<[Cow<'static, str>; 7]>;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a very special map")
    }

    // Deserialize MyMap from an abstract "map" provided by the
    // Deserializer. The MapAccess input is a callback provided by
    // the Deserializer to let us see each entry in the map.
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut values: [Cow<'static, str>; 7] = [
            Cow::Borrowed(""),
            Cow::Borrowed(""),
            Cow::Borrowed(""),
            Cow::Borrowed(""),
            Cow::Borrowed(""),
            Cow::Borrowed(""),
            Cow::Borrowed(""),
        ];
        while let Some((key, value)) = access.next_entry::<&[u8], Cow<'static, str>>()? {
            match key {
                b"mon" => values[0] = value,
                b"tue" => values[1] = value,
                b"wed" => values[2] = value,
                b"thu" => values[3] = value,
                b"fri" => values[4] = value,
                b"sat" => values[5] = value,
                b"sun" => values[6] = value,
                _ => panic!(),
            }
        }
        Ok(Some(values))
    }
}

fn deserialize_days_map_of_values<'de, D>(de: D) -> Result<Option<[Cow<'static, str>; 7]>, D::Error>
where
    D: Deserializer<'de>,
{
    de.deserialize_map(DaysMapVisitor::new())
}

#[derive(Deserialize)]
#[serde(remote = "MonthNamesTypes")]
pub struct MonthTypes {
    #[serde(default, deserialize_with = "deserialize_months_map_of_values")]
    pub abbreviated: Option<MonthList>,
    #[serde(default, deserialize_with = "deserialize_months_map_of_values")]
    pub narrow: Option<MonthList>,
    #[serde(default, deserialize_with = "deserialize_months_map_of_values")]
    pub short: Option<MonthList>,
    #[serde(default, deserialize_with = "deserialize_months_map_of_values")]
    pub wide: Option<MonthList>,
}

#[derive(Deserialize)]
#[serde(remote = "DayNamesTypes")]
pub struct DayTypes {
    #[serde(default, deserialize_with = "deserialize_days_map_of_values")]
    pub abbreviated: Option<DayList>,
    #[serde(default, deserialize_with = "deserialize_days_map_of_values")]
    pub narrow: Option<DayList>,
    #[serde(default, deserialize_with = "deserialize_days_map_of_values")]
    pub short: Option<DayList>,
    #[serde(default, deserialize_with = "deserialize_days_map_of_values")]
    pub wide: Option<DayList>,
}

fn deserialize_pattern<'de, D>(de: D) -> Result<Pattern, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(de).map_err(serde::de::Error::custom)?;
    Ok(parse_pattern(s).unwrap())
}

#[derive(Deserialize)]
pub struct Formats {
    #[serde(deserialize_with = "deserialize_pattern")]
    pub full: Pattern,
    #[serde(deserialize_with = "deserialize_pattern")]
    pub long: Pattern,
    #[serde(deserialize_with = "deserialize_pattern")]
    pub medium: Pattern,
    #[serde(deserialize_with = "deserialize_pattern")]
    pub short: Pattern,
}

pub fn get_calendar_data(locale: &str) -> CalendarData {
    let path = format!("./data/cldr-dates-modern/main/{}/ca-gregorian.json", locale);
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let mut res: Resource = serde_json::from_str(&contents).unwrap();

    res.main
        .get_mut(locale)
        .unwrap()
        .dates
        .calendars
        .gregorian
        .take()
        .unwrap()
}
