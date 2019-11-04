use super::layout::*;

pub const CALENDAR_DATA: CalendarData<&'static str> = CalendarData {
    months: MonthNames {
        format: Some(MonthNamesTypes {
            abbreviated: Some([
                "sty", "lut", "mar", "kwi", "maj", "cze", "lip", "sie", "wrz", "paź", "lis", "gru",
            ]),
            narrow: Some(["s", "l", "m", "k", "m", "c", "l", "s", "w", "p", "l", "g"]),
            short: None,
            wide: Some([
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
            ]),
        }),
        stand_alone: Some(MonthNamesTypes {
            abbreviated: Some([
                "sty", "lut", "mar", "kwi", "maj", "cze", "lip", "sie", "wrz", "paź", "lis", "gru",
            ]),
            narrow: Some(["S", "L", "M", "K", "M", "C", "L", "S", "W", "P", "L", "G"]),
            short: None,
            wide: Some([
                "styczeń",
                "luty",
                "marzec",
                "kwiecień",
                "maj",
                "czerwiec",
                "lipiec",
                "sierpień",
                "wrzesień",
                "październik",
                "listopad",
                "grudzień",
            ]),
        }),
    },
    date_formats: ["EEEE, d MMMM y", "d MMMM y", "d MMM y", "dd.MM.y"],
    time_formats: ["HH:mm:ss zzzz", "HH:mm:ss z", "HH:mm:ss", "HH:mm"],
    date_time_formats: ["{1} {0}", "{1} {0}", "{1}, {0}", "{1}, {0}"],
};
