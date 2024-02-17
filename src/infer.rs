// Copyright 2023 Jonathan Bowman
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use atoi::atoi;
use simdutf8::basic::from_utf8;
use std::fmt;
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time};

mod timeformats;

#[derive(Clone, Copy)]
struct ByteText<'a> {
    bytes: &'a [u8],
    text: Option<&'a str>,
}

impl<'a> ByteText<'a> {
    fn new(bytes: &'a [u8]) -> ByteText<'a> {
        let text = None;
        ByteText { bytes, text }
    }

    fn text(&mut self) -> &'a str {
        if let Some(text) = self.text {
            text
        } else {
            let text = from_utf8(self.bytes).unwrap();
            self.text = Some(text);
            text
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SQLTypeName {
    #[default]
    Bit,
    Tinyint,
    Smallint,
    Int,
    Bigint,
    Numeric,
    Float,
    Date,
    Time,
    Datetime2,
    Datetimeoffset,
    Char,
    Varchar,
    Varcharmax,
}

type Check = fn(ByteText, usize) -> Option<SQLType>;

const CHECKS: [&Check; 15] = [
    &(check_bit as Check),
    &(check_tinyint as Check),
    &(check_smallint as Check),
    &(check_int as Check),
    &(check_bigint as Check),
    &(check_decimal as Check),
    &(check_real as Check),
    &(check_float as Check),
    &(check_date as Check),
    &(check_time as Check),
    &(check_datetime as Check),
    &(check_datetimeoffset as Check),
    &(check_char as Check),
    &(check_varchar as Check),
    &(check_varcharmax as Check),
];

#[derive(Clone, Debug, Default)]
pub struct SQLType {
    pub name: SQLTypeName,
    pub size: usize,
    pub index: usize,
    pub subindex: usize,
    pub scale: usize,
    pub byte_length: usize,
}

impl SQLType {
    pub fn merge(&mut self, other: &Self) {
        self.byte_length = other.byte_length.max(self.byte_length);
        if self.name == other.name && other.name == SQLTypeName::Char && other.size != self.size {
            self.name = SQLTypeName::Varchar;
            self.size = other.size.max(self.size);
            self.index += 1;
        } else if self.name == other.name {
            self.subindex = other.subindex.max(self.subindex);
            self.size = other.size.max(self.size);
            self.scale = other.scale.max(self.scale);
        } else if self.index < other.index {
            if other.name == SQLTypeName::Char {
                self.size = other.size.max(self.byte_length);
            } else {
                self.size = other.size;
            }
            self.name = other.name;
            self.index = other.index;
            self.subindex = other.subindex;
            self.scale = other.scale;
        }
    }

    pub fn varchar(&self) -> String {
        if self.name == SQLTypeName::Varcharmax {
            "VARCHAR(MAX)".to_string()
        } else {
            format!(
                "VARCHAR({})",
                if self.byte_length == 0 {
                    1
                } else {
                    self.byte_length
                }
            )
        }
    }
}

impl fmt::Display for SQLType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut name = if self.name == SQLTypeName::Varcharmax {
            "VARCHAR(MAX)".to_string()
        } else {
            format!("{:?}", self.name).to_ascii_uppercase()
        };
        let size = self.size + self.scale;
        if self.size > 0 || name.contains("TIME") {
            name.push_str(&format!("({}", size));
            if self.scale > 0 {
                name.push_str(&format!(", {}", self.scale));
            }
            name.push(')');
        }
        write!(f, "{}", name)
    }
}

pub fn infer(value: &[u8], mut index: usize, subindex: usize) -> Option<SQLType> {
    if value.is_empty() {
        return Some(SQLType {
            ..Default::default()
        });
    }
    let value = ByteText::new(value);
    let byte_length = value.bytes.len();
    while index < CHECKS.len() {
        let fun = CHECKS[index];

        if let Some(mut typesize) = fun(value, subindex) {
            typesize.index = index;
            typesize.byte_length = byte_length;
            return Some(typesize);
        } else {
            index += 1;
        }
    }
    None
}

fn check_bit(value: ByteText, _subindex: usize) -> Option<SQLType> {
    let value = trim(value.bytes);
    if !value.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let bit = atoi::<i8>(value).unwrap_or_else(|| -1);
    if bit == 1 || bit == 0 {
        Some(SQLType {
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_tinyint(value: ByteText, _subindex: usize) -> Option<SQLType> {
    let value = trim(value.bytes);
    if value.len() > 0 && value[0] == b'0' && !value.iter().all(|&x| x == b'0') {
        return None;
    }
    if value.iter().all(u8::is_ascii_digit) && atoi::<u8>(value).is_some() {
        Some(SQLType {
            name: SQLTypeName::Tinyint,
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_smallint(value: ByteText, _subindex: usize) -> Option<SQLType> {
    let value = trim(value.bytes);
    if value.len() > 0 && value[0] == b'0' && !value.iter().all(|&x| x == b'0') {
        return None;
    }
    let value = signed(value);
    if value.iter().all(u8::is_ascii_digit) && atoi::<i16>(value).is_some() {
        Some(SQLType {
            name: SQLTypeName::Smallint,
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_int(value: ByteText, _subindex: usize) -> Option<SQLType> {
    let value = trim(value.bytes);
    if value.len() > 0 && value[0] == b'0' && !value.iter().all(|&x| x == b'0') {
        return None;
    }
    let value = signed(value);
    if value.iter().all(u8::is_ascii_digit) && atoi::<i32>(value).is_some() {
        Some(SQLType {
            name: SQLTypeName::Int,
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_bigint(value: ByteText, _subindex: usize) -> Option<SQLType> {
    let value = trim(value.bytes);
    if value.len() > 0 && value[0] == b'0' && !value.iter().all(|&x| x == b'0') {
        return None;
    }
    let value = signed(value);
    if value.iter().all(u8::is_ascii_digit) && atoi::<i64>(value).is_some() {
        Some(SQLType {
            name: SQLTypeName::Bigint,
            ..Default::default()
        })
    } else {
        None
    }
}

fn signed(value: &[u8]) -> &[u8] {
    if value.len() > 0 && value[0] == b'-' {
        &value[1..]
    } else {
        value
    }
}

fn trim(value: &[u8]) -> &[u8] {
    let from = match value.iter().position(|x| !x.is_ascii_whitespace()) {
        Some(i) => i,
        None => return &value[0..0],
    };
    let to = value
        .iter()
        .rposition(|x| !x.is_ascii_whitespace())
        .unwrap();
    &value[from..=to]
}

fn check_decimal(value: ByteText, _subindex: usize) -> Option<SQLType> {
    let value = trim(value.bytes);
    if value.len() > 0 && value[0] == b'0' && !value.iter().all(|&x| x == b'0' || x == b'.') {
        return None;
    }
    let length = value.iter().filter(|c| c.is_ascii_digit()).count();
    let value = signed(value);
    if value.iter().filter(|c| **c == b'.').count() <= 1
        && value != b"."
        && value.iter().all(|c| match c {
            b'.' | b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => true,
            _ => false,
        })
        && length <= 38
    {
        if let Some(point) = value.iter().position(|&x| x == b'.') {
            let scale = value[point + 1..]
                .iter()
                .filter(|c| c.is_ascii_digit())
                .count();
            Some(SQLType {
                name: SQLTypeName::Numeric,
                size: length - scale,
                scale: scale,
                ..Default::default()
            })
        } else {
            Some(SQLType {
                name: SQLTypeName::Numeric,
                size: length,
                ..Default::default()
            })
        }
    } else {
        None
    }
}

fn check_real(mut value: ByteText, _subindex: usize) -> Option<SQLType> {
    if value.bytes.len() > 0
        && value.bytes[0] == b'0'
        && !value.bytes.iter().all(|&x| x == b'0' || x == b'.')
    {
        return None;
    }
    let value = value.text();
    if let Ok(real) = value.parse::<f32>() {
        if real.is_normal() {
            return Some(SQLType {
                name: SQLTypeName::Float,
                size: 24,
                ..Default::default()
            });
        }
    }
    None
}

fn check_float(mut value: ByteText, _subindex: usize) -> Option<SQLType> {
    if value.bytes.len() > 0
        && value.bytes[0] == b'0'
        && !value.bytes.iter().all(|&x| x == b'0' || x == b'.')
    {
        return None;
    }
    let value = value.text();
    if value.parse::<f64>().is_ok() {
        Some(SQLType {
            name: SQLTypeName::Float,
            size: 53,
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_date(mut value: ByteText, subindex: usize) -> Option<SQLType> {
    for i in (subindex..timeformats::DATE_FORMATS.len()).chain(0..subindex) {
        let form = timeformats::DATE_FORMATS[i];
        if Date::parse(value.text(), form).is_ok() {
            return Some(SQLType {
                name: SQLTypeName::Date,
                subindex,
                ..Default::default()
            });
        }
    }
    None
}

fn time_precision(nanoseconds: u32) -> usize {
    if nanoseconds == 0 {
        0
    } else {
        let nanos = nanoseconds.to_string();
        (nanos.trim_end_matches('0').len() + 9 - nanos.len()).min(7)
    }
}

fn check_time(mut value: ByteText, subindex: usize) -> Option<SQLType> {
    let value = value.text();
    // Fail if straight integer
    if value.parse::<u8>().is_ok() {
        return None;
    }
    for i in (subindex..timeformats::TIME_FORMATS.len()).chain(0..subindex) {
        let form = timeformats::TIME_FORMATS[i];
        if let Ok(parsed) = Time::parse(value, form) {
            return Some(SQLType {
                name: SQLTypeName::Time,
                subindex,
                size: time_precision(parsed.nanosecond()),
                ..Default::default()
            });
        }
    }
    None
}

fn check_datetimeoffset(mut value: ByteText, subindex: usize) -> Option<SQLType> {
    let value = value.text();
    for i in (subindex..timeformats::DATETIMEOFFSET_FORMATS.len()).chain(0..subindex) {
        let form = timeformats::DATETIMEOFFSET_FORMATS[i];
        if let Ok(parsed) = OffsetDateTime::parse(value, form)
            .or_else(|_| OffsetDateTime::parse(&format!("{}+00:00", &value), form))
        {
            return Some(SQLType {
                name: SQLTypeName::Datetimeoffset,
                subindex,
                size: time_precision(parsed.nanosecond()),
                ..Default::default()
            });
        }
    }
    None
}

fn check_datetime(mut value: ByteText, subindex: usize) -> Option<SQLType> {
    for i in (subindex..timeformats::DATETIME_FORMATS.len()).chain(0..subindex) {
        let form = timeformats::DATETIME_FORMATS[i];
        if let Ok(parsed) = PrimitiveDateTime::parse(value.text(), form) {
            return Some(SQLType {
                name: SQLTypeName::Datetime2,
                subindex,
                size: time_precision(parsed.nanosecond()),
                ..Default::default()
            });
        }
    }
    None
}

fn check_char(value: ByteText, _subindex: usize) -> Option<SQLType> {
    let value = value.bytes;
    if value.len() <= 8000 {
        Some(SQLType {
            name: SQLTypeName::Char,
            size: value.len(),
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_varchar(value: ByteText, _subindex: usize) -> Option<SQLType> {
    let value = value.bytes;
    if value.len() <= 8000 {
        Some(SQLType {
            name: SQLTypeName::Varchar,
            size: value.len(),
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_varcharmax(value: ByteText, _subindex: usize) -> Option<SQLType> {
    let value = value.bytes;
    if value.len() > 8000 {
        Some(SQLType {
            name: SQLTypeName::Varcharmax,
            ..Default::default()
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datetimeoffset_if_no_tz() {
        assert!(check_datetimeoffset(ByteText::new(b"2002-11-09T07:18:21"), 0).is_some());
    }
}
