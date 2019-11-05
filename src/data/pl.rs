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
    date_formats: [
        &[
            PatternElement::Token(DateTimeToken::WeekDayWide),
            PatternElement::Literal(", "),
            PatternElement::Token(DateTimeToken::DayNumeric),
            PatternElement::Literal(" "),
            PatternElement::Token(DateTimeToken::MonthNameLong),
            PatternElement::Literal(" "),
            PatternElement::Token(DateTimeToken::YearNumeric),
        ],
        &[
            PatternElement::Token(DateTimeToken::DayNumeric),
            PatternElement::Literal(" "),
            PatternElement::Token(DateTimeToken::MonthNameLong),
            PatternElement::Literal(" "),
            PatternElement::Token(DateTimeToken::YearNumeric),
        ],
        &[
            PatternElement::Token(DateTimeToken::DayNumeric),
            PatternElement::Literal(" "),
            PatternElement::Token(DateTimeToken::MonthNameAbbreviated),
            PatternElement::Literal(" "),
            PatternElement::Token(DateTimeToken::YearNumeric),
        ],
        &[
            PatternElement::Token(DateTimeToken::Day2digit),
            PatternElement::Literal("."),
            PatternElement::Token(DateTimeToken::Month2digit),
            PatternElement::Literal("."),
            PatternElement::Token(DateTimeToken::YearNumeric),
        ],
    ],
    time_formats: [
        &[
            PatternElement::Token(DateTimeToken::Hour2digit),
            PatternElement::Literal(":"),
            PatternElement::Token(DateTimeToken::Minute2digit),
            PatternElement::Literal(":"),
            PatternElement::Token(DateTimeToken::Second2digit),
            PatternElement::Literal(" "),
            PatternElement::Token(DateTimeToken::ZoneLong),
        ],
        &[
            PatternElement::Token(DateTimeToken::Hour2digit),
            PatternElement::Literal(":"),
            PatternElement::Token(DateTimeToken::Minute2digit),
            PatternElement::Literal(":"),
            PatternElement::Token(DateTimeToken::Second2digit),
            PatternElement::Literal(" "),
            PatternElement::Token(DateTimeToken::ZoneShort),
        ],
        &[
            PatternElement::Token(DateTimeToken::Hour2digit),
            PatternElement::Literal(":"),
            PatternElement::Token(DateTimeToken::Minute2digit),
            PatternElement::Literal(":"),
            PatternElement::Token(DateTimeToken::Second2digit),
        ],
        &[
            PatternElement::Token(DateTimeToken::Hour2digit),
            PatternElement::Literal(":"),
            PatternElement::Token(DateTimeToken::Minute2digit),
        ],
    ],
    date_time_formats: [
        &[
            PatternElement::Token(DateTimeToken::Sub1),
            PatternElement::Literal(" "),
            PatternElement::Token(DateTimeToken::Sub0),
        ],
        &[
            PatternElement::Token(DateTimeToken::Sub1),
            PatternElement::Literal(" "),
            PatternElement::Token(DateTimeToken::Sub0),
        ],
        &[
            PatternElement::Token(DateTimeToken::Sub1),
            PatternElement::Literal(", "),
            PatternElement::Token(DateTimeToken::Sub0),
        ],
        &[
            PatternElement::Token(DateTimeToken::Sub1),
            PatternElement::Literal(", "),
            PatternElement::Token(DateTimeToken::Sub0),
        ],
    ],
};
