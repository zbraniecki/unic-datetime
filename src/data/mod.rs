pub mod layout;

use layout::*;

pub const PL_GREGORIAN_MONTHS: GregorianMonths = GregorianMonths {
    format: Some(MonthNames {
        abbreviated: Some([
            "sty", "lut", "mar", "kwi", "maj", "cze", "lip", "sie", "wrz", "paź", "lis", "gru",
        ]),
        narrow: None,
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
    stand_alone: None,
};

pub const PL_DATE_FORMATS: DateFormats =
    DateFormats(["EEEE, d MMMM y", "d MMMM y", "d MMM y", "dd.MM.y"]);
pub const PL_TIME_FORMATS: TimeFormats =
    TimeFormats(["HH:mm:ss zzzz", "HH:mm:ss z", "HH:mm:ss", "HH:mm"]);
pub const PL_DATE_TIME_FORMATS: DateTimeFormats =
    DateTimeFormats(["{1} {0}", "{1} {0}", "{1}, {0}", "{1}, {0}"]);
