pub struct GregorianMonths {
    pub stand_alone: Option<MonthNames>,
    pub format: Option<MonthNames>,
}

pub struct MonthNames {
    pub abbreviated: Option<MonthList>,
    pub narrow: Option<MonthList>,
    pub short: Option<MonthList>,
    pub wide: Option<MonthList>,
}

pub type MonthList = [&'static str; 12];

pub enum MonthNamesLength {
    ABBREVIATED,
    NARROW,
    SHORT,
    WIDE,
}

impl GregorianMonths {
    pub fn get_list(&self, stand_alone: bool, length: MonthNamesLength) -> Option<MonthList> {
        let list = if stand_alone {
            &self.stand_alone
        } else {
            &self.format
        }
        .as_ref()
        .unwrap();

        match length {
            MonthNamesLength::ABBREVIATED => list.abbreviated,
            MonthNamesLength::NARROW => list.narrow,
            MonthNamesLength::SHORT => list.short,
            MonthNamesLength::WIDE => list.wide,
        }
    }
}

pub struct DateFormats(pub [&'static str; 4]);
pub struct TimeFormats(pub [&'static str; 4]);
pub struct DateTimeFormats(pub [&'static str; 4]);
