use super::layout::{DateTimeToken, PatternElement};
use std::borrow::Cow;

#[derive(Debug)]
pub enum ParserError {
    UnterminatedLiteral,
}

fn collect_literal(
    literal_start: &mut usize,
    idx: usize,
    input: &[u8],
    result: &mut Vec<PatternElement>,
) {
    if literal_start == &idx {
        return;
    }
    let slice = std::str::from_utf8(&input[*literal_start..idx]).unwrap();
    result.push(PatternElement::Literal(Cow::Owned(slice.to_owned())));
    *literal_start = idx;
}

pub fn parse_pattern<S: AsRef<[u8]>>(input: S) -> Result<Vec<PatternElement>, ParserError> {
    let mut result = Vec::with_capacity(input.as_ref().len());

    let mut iter = input.as_ref().iter().enumerate().peekable();

    let mut literal_start = 0;
    // println!("{:#?}", String::from_utf8_lossy(input.as_ref()));

    while let Some((i, ch)) = iter.next() {
        match ch {
            b'\'' => {
                collect_literal(&mut literal_start, i, input.as_ref(), &mut result);
                let next = iter.next();
                if let Some((_, b'\'')) = next {
                    result.push(PatternElement::Literal(Cow::Owned("'".to_string())));
                } else if let Some((start, _)) = next {
                    while let Some((i, ch)) = iter.next() {
                        if ch == &b'\'' {
                            if let Some((_, b'\'')) = iter.peek() {
                                iter.next();
                            } else {
                                let slice = std::str::from_utf8(&input.as_ref()[start..i]).unwrap();
                                result.push(PatternElement::Literal(Cow::Owned(
                                    slice.replace("''", "'"),
                                )));
                                literal_start = i + 1;
                                break;
                            }
                        }
                    }
                } else {
                    return Err(ParserError::UnterminatedLiteral);
                }
            }
            b'M' => {
                collect_literal(&mut literal_start, i, input.as_ref(), &mut result);
                let mut length = 1;
                while let Some((_, b'M')) = iter.peek() {
                    length += 1;
                    iter.next();
                }
                let token = match length {
                    4 => DateTimeToken::MonthNameLong,
                    3 => DateTimeToken::MonthNameAbbreviated,
                    2 => DateTimeToken::Month2digit,
                    1 => DateTimeToken::MonthNumeric,
                    _ => unimplemented!(),
                };
                literal_start += length;
                result.push(PatternElement::Token(token));
            }
            b'y' => {
                collect_literal(&mut literal_start, i, input.as_ref(), &mut result);
                let mut length = 1;
                while let Some((_, b'y')) = iter.peek() {
                    length += 1;
                    iter.next();
                }
                let token = match length {
                    1 => DateTimeToken::YearNumeric,
                    2 => DateTimeToken::Year2digit,
                    _ => unimplemented!(),
                };
                literal_start += length;
                result.push(PatternElement::Token(token));
            }
            b'd' => {
                collect_literal(&mut literal_start, i, input.as_ref(), &mut result);
                let mut length = 1;
                while let Some((_, b'd')) = iter.peek() {
                    length += 1;
                    iter.next();
                }
                let token = match length {
                    1 => DateTimeToken::DayNumeric,
                    2 => DateTimeToken::Day2digit,
                    _ => unimplemented!(),
                };
                literal_start += length;
                result.push(PatternElement::Token(token));
            }
            b'E' => {
                collect_literal(&mut literal_start, i, input.as_ref(), &mut result);
                let mut length = 1;
                while let Some((_, b'E')) = iter.peek() {
                    length += 1;
                    iter.next();
                }
                let token = match length {
                    4 => DateTimeToken::WeekDayWide,
                    _ => unimplemented!(),
                };
                literal_start += length;
                result.push(PatternElement::Token(token));
            }
            b'H' => {
                collect_literal(&mut literal_start, i, input.as_ref(), &mut result);
                let mut length = 1;
                while let Some((_, b'H')) = iter.peek() {
                    length += 1;
                    iter.next();
                }
                let token = match length {
                    2 => DateTimeToken::Hour2digit,
                    1 => DateTimeToken::HourNumeric,
                    _ => unimplemented!(),
                };
                literal_start += length;
                result.push(PatternElement::Token(token));
            }
            b'm' => {
                collect_literal(&mut literal_start, i, input.as_ref(), &mut result);
                let mut length = 1;
                while let Some((_, b'm')) = iter.peek() {
                    length += 1;
                    iter.next();
                }
                let token = match length {
                    2 => DateTimeToken::Minute2digit,
                    1 => DateTimeToken::MinuteNumeric,
                    _ => unimplemented!(),
                };
                literal_start += length;
                result.push(PatternElement::Token(token));
            }
            b's' => {
                collect_literal(&mut literal_start, i, input.as_ref(), &mut result);
                let mut length = 1;
                while let Some((_, b's')) = iter.peek() {
                    length += 1;
                    iter.next();
                }
                let token = match length {
                    2 => DateTimeToken::Second2digit,
                    1 => DateTimeToken::SecondNumeric,
                    _ => unimplemented!(),
                };
                literal_start += length;
                result.push(PatternElement::Token(token));
            }
            b'z' => {
                collect_literal(&mut literal_start, i, input.as_ref(), &mut result);
                let mut length = 1;
                while let Some((_, b'z')) = iter.peek() {
                    length += 1;
                    iter.next();
                }
                let token = match length {
                    1 => DateTimeToken::ZoneShort,
                    2 => DateTimeToken::ZoneShort,
                    3 => DateTimeToken::ZoneShort,
                    4 => DateTimeToken::ZoneLong,
                    _ => unimplemented!(),
                };
                literal_start += length;
                result.push(PatternElement::Token(token));
            }
            b'{' => {
                collect_literal(&mut literal_start, i, input.as_ref(), &mut result);
                let num = iter.next().unwrap();
                match num.1 {
                    b'0' => result.push(PatternElement::Token(DateTimeToken::Sub0)),
                    b'1' => result.push(PatternElement::Token(DateTimeToken::Sub1)),
                    _ => unimplemented!(),
                }
                iter.next(); // }
                literal_start += 3;
            }
            _ => {}
        }
    }
    Ok(result)
}
