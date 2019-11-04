#[derive(Default)]
pub struct CalendarData<S> {
    pub months: MonthNames<S>,
    pub date_formats: [S; 4],
    pub time_formats: [S; 4],
    pub date_time_formats: [S; 4],
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

impl<S> MonthNames<S> {
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
