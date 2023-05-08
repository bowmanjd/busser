use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use std::fmt;

mod formats;

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

type Check = fn(&str, usize) -> Option<SQLType>;

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
    pub fixed: bool,
}

impl SQLType {
    pub fn merge(&mut self, other: &Self) {
        if other.fixed && self.fixed && other.size != self.size {
            self.name = SQLTypeName::Varchar;
            self.size = other.size.max(self.size);
            self.index += 1;
            self.fixed = false;
        } else if self.index == other.index {
            self.subindex = other.subindex.max(self.subindex);
            self.size = other.size.max(self.size);
            self.scale = other.scale.max(self.scale);
        } else if self.index < other.index {
            self.name = other.name;
            self.index = other.index;
            self.subindex = other.subindex;
            self.fixed = other.fixed;
            self.size = other.size;
            self.scale = other.scale;
        }
    }
}

impl fmt::Display for SQLType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut name = if self.name == SQLTypeName::Varcharmax {
            "varchar(max)".to_string()
        } else {
            format!("{:?}", self.name).to_ascii_lowercase()
        };
        let size = self.size + self.scale;
        if self.size > 0 || name.contains("time") {
            name.push_str(&format!("({}", size));
            if self.scale > 0 {
                name.push_str(&format!(", {}", self.scale));
            }
            name.push(')');
        }
        write!(f, "{}", name)
    }
}

pub fn infer(value: &str, index: usize, subindex: usize) -> Option<SQLType> {
    if value.is_empty() {
        return Some(SQLType {
            ..Default::default()
        });
    }
    let mut index = index;
    while index < CHECKS.len() {
        if let Some(mut typesize) = CHECKS[index](value, subindex) {
            typesize.index = index;
            return Some(typesize);
        } else {
            index += 1;
        }
    }
    None
}
// multiple compare function should check if return value is greater or less than previous, advance
// if fixed is true and values differ, if none then advance
//
// infer function walks through functions in order of priority until Some, return Some

fn check_bit(value: &str, _subindex: usize) -> Option<SQLType> {
    let bit = value.parse::<i8>().unwrap_or(-1);
    if bit == 0 || bit == 1 {
        Some(SQLType {
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_tinyint(value: &str, _subindex: usize) -> Option<SQLType> {
    if value.parse::<u8>().is_ok() {
        Some(SQLType {
            name: SQLTypeName::Tinyint,
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_smallint(value: &str, _subindex: usize) -> Option<SQLType> {
    if value.parse::<i16>().is_ok() {
        Some(SQLType {
            name: SQLTypeName::Smallint,
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_int(value: &str, _subindex: usize) -> Option<SQLType> {
    if value.parse::<i32>().is_ok() {
        Some(SQLType {
            name: SQLTypeName::Int,
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_bigint(value: &str, _subindex: usize) -> Option<SQLType> {
    if value.parse::<i64>().is_ok() {
        Some(SQLType {
            name: SQLTypeName::Bigint,
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_decimal(value: &str, _subindex: usize) -> Option<SQLType> {
    let length = value.chars().filter(|c| c.is_numeric()).count();
    if value.parse::<f64>().is_ok()
        && value.trim().chars().all(|c| match c {
            '-' | '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
            _ => false,
        })
        && length <= 38
    {
        if let Some(point) = value.find('.') {
            let scale = value[point + 1..]
                .chars()
                .filter(|c| c.is_numeric())
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
                size: value.trim().len(),
                ..Default::default()
            })
        }
    } else {
        None
    }
}

fn check_real(value: &str, _subindex: usize) -> Option<SQLType> {
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

fn check_float(value: &str, _subindex: usize) -> Option<SQLType> {
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

fn check_date(value: &str, subindex: usize) -> Option<SQLType> {
    for i in (subindex..formats::DATE_FORMATS.len()).chain(0..subindex) {
        let form = formats::DATE_FORMATS[i];
        if NaiveDate::parse_from_str(value, form).is_ok() {
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

fn check_time(value: &str, subindex: usize) -> Option<SQLType> {
    // Fail if straight integer
    if value.parse::<u8>().is_ok() {
        return None;
    }
    for i in (subindex..formats::TIME_FORMATS.len()).chain(0..subindex) {
        let form = formats::TIME_FORMATS[i];
        if let Ok(parsed) = NaiveTime::parse_from_str(value, form) {
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

fn check_datetimeoffset(value: &str, subindex: usize) -> Option<SQLType> {
    for i in (subindex..formats::DATETIMEOFFSET_FORMATS.len()).chain(0..subindex) {
        let form = formats::DATETIMEOFFSET_FORMATS[i];
        if let Ok(parsed) = DateTime::parse_from_str(value, form)
            .or(DateTime::parse_from_str(&format!("{}+00:00", &value), form))
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

fn check_datetime(value: &str, subindex: usize) -> Option<SQLType> {
    for i in (subindex..formats::DATETIME_FORMATS.len()).chain(0..subindex) {
        let form = formats::DATETIME_FORMATS[i];
        if let Ok(parsed) = NaiveDateTime::parse_from_str(value, form) {
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

fn check_char(value: &str, _subindex: usize) -> Option<SQLType> {
    if value.len() <= 8000 {
        Some(SQLType {
            name: SQLTypeName::Char,
            size: value.len(),
            fixed: true,
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_varchar(value: &str, _subindex: usize) -> Option<SQLType> {
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

fn check_varcharmax(value: &str, _subindex: usize) -> Option<SQLType> {
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
        assert!(check_datetimeoffset("2002-11-09T07:18:21", 0).is_some());
    }
}
