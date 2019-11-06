use unic_datetime::data::layout::{DateTimeToken, PatternElement};
use unic_datetime::data::patterns::parse_pattern;

#[test]
fn test_literal_patterns() {
    assert_eq!(parse_pattern("").unwrap().as_ref(), []);

    assert_eq!(
        parse_pattern("''").unwrap().as_ref(),
        [PatternElement::Literal("'".to_string())]
    );

    assert_eq!(
        parse_pattern("'John'").unwrap().as_ref(),
        [PatternElement::Literal("John".to_string())]
    );

    assert_eq!(
        parse_pattern("'John '' Smith'").unwrap().as_ref(),
        [PatternElement::Literal("John ' Smith".to_string())]
    );

    assert_eq!(
        parse_pattern("'John' 'Smith'").unwrap().as_ref(),
        [
            PatternElement::Literal("John".to_string()),
            PatternElement::Literal(" ".to_string()),
            PatternElement::Literal("Smith".to_string()),
        ]
    );
}

#[test]
fn test_tokens() {
    assert_eq!(
        parse_pattern("MMMM").unwrap().as_ref(),
        [PatternElement::Token(DateTimeToken::MonthNameLong)]
    );

    assert_eq!(
        parse_pattern("MMMM.MMMM").unwrap().as_ref(),
        [
            PatternElement::Token(DateTimeToken::MonthNameLong),
            PatternElement::Literal(".".to_string()),
            PatternElement::Token(DateTimeToken::MonthNameLong),
        ]
    );

    assert_eq!(
        parse_pattern("'Hello 'MMMM' Token'").unwrap().as_ref(),
        [
            PatternElement::Literal("Hello ".to_string()),
            PatternElement::Token(DateTimeToken::MonthNameLong),
            PatternElement::Literal(" Token".to_string()),
        ]
    );

    assert_eq!(
        parse_pattern("EEEE, d MMMM y").unwrap().as_ref(),
        [
            PatternElement::Token(DateTimeToken::WeekDayWide),
            PatternElement::Literal(", ".to_string()),
            PatternElement::Token(DateTimeToken::DayNumeric),
            PatternElement::Literal(" ".to_string()),
            PatternElement::Token(DateTimeToken::MonthNameLong),
            PatternElement::Literal(" ".to_string()),
            PatternElement::Token(DateTimeToken::YearNumeric),
        ]
    );
}
#[test]
fn test_replace() {
    assert_eq!(
        parse_pattern("{0} {1}").unwrap().as_ref(),
        [
            PatternElement::Token(DateTimeToken::Sub0),
            PatternElement::Literal(" ".to_string()),
            PatternElement::Token(DateTimeToken::Sub1),
        ]
    );
}
