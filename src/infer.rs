use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use std::fmt;

mod formats;

const CHECKS: [&fn(&str, usize) -> Option<SQLType>; 15] = [
    &(check_bit as fn(&str, usize) -> Option<SQLType>),
    &(check_tinyint as fn(&str, usize) -> Option<SQLType>),
    &(check_smallint as fn(&str, usize) -> Option<SQLType>),
    &(check_int as fn(&str, usize) -> Option<SQLType>),
    &(check_bigint as fn(&str, usize) -> Option<SQLType>),
    &(check_decimal as fn(&str, usize) -> Option<SQLType>),
    &(check_real as fn(&str, usize) -> Option<SQLType>),
    &(check_float as fn(&str, usize) -> Option<SQLType>),
    &(check_date as fn(&str, usize) -> Option<SQLType>),
    &(check_time as fn(&str, usize) -> Option<SQLType>),
    &(check_datetime as fn(&str, usize) -> Option<SQLType>),
    &(check_datetimeoffset as fn(&str, usize) -> Option<SQLType>),
    &(check_char as fn(&str, usize) -> Option<SQLType>),
    &(check_varchar as fn(&str, usize) -> Option<SQLType>),
    &(check_varcharmax as fn(&str, usize) -> Option<SQLType>),
];

#[derive(Clone, Debug, Default)]
pub struct SQLType {
    pub name: String,
    pub size: usize,
    pub index: usize,
    pub subindex: usize,
    pub scale: usize,
    pub fixed: bool,
}

impl SQLType {
    pub fn merge(&mut self, other: &Self) {
        if other.fixed && self.fixed && other.size != self.size {
            self.name = "varchar".to_string();
            self.size = other.size.max(self.size);
            self.index += 1;
            self.fixed = false;
        } else if self.index == other.index {
            self.subindex = other.subindex.max(self.subindex);
            self.size = other.size.max(self.size);
            self.scale = other.scale.max(self.scale);
        } else if self.index < other.index {
            self.name = other.name.clone();
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
        let mut name = self.name.clone();
        if self.size > 0 || self.name.contains("time") {
            name.push_str(&format!("({}", self.size));
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
            name: "bit".to_string(),
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
            name: "bit".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_tinyint(value: &str, _subindex: usize) -> Option<SQLType> {
    if value.parse::<u8>().is_ok() {
        Some(SQLType {
            name: "tinyint".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_smallint(value: &str, _subindex: usize) -> Option<SQLType> {
    if value.parse::<i16>().is_ok() {
        Some(SQLType {
            name: "smallint".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_int(value: &str, _subindex: usize) -> Option<SQLType> {
    if value.parse::<i32>().is_ok() {
        Some(SQLType {
            name: "int".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_bigint(value: &str, _subindex: usize) -> Option<SQLType> {
    if value.parse::<i64>().is_ok() {
        Some(SQLType {
            name: "bigint".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_decimal(value: &str, _subindex: usize) -> Option<SQLType> {
    let numeric = value.trim().replace('.', "");
    let length = numeric.len();
    if value.parse::<f64>().is_ok() && numeric.chars().all(char::is_numeric) && length <= 38 {
        if let Some(point) = value.find('.') {
            Some(SQLType {
                name: "numeric".to_string(),
                size: length,
                scale: value.trim()[point + 1..].len(),
                ..Default::default()
            })
        } else {
            Some(SQLType {
                name: "numeric".to_string(),
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
                name: "float(24)".to_string(),
                ..Default::default()
            });
        }
    }
    None
}

fn check_float(value: &str, _subindex: usize) -> Option<SQLType> {
    if value.parse::<f64>().is_ok() {
        Some(SQLType {
            name: "float".to_string(),
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
                name: "date".to_string(),
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
                name: "time".to_string(),
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
        if let Ok(parsed) = DateTime::parse_from_str(value, form).or(DateTime::parse_from_str(
            &format!("{}+00:00", &value),
            form,
        )) {
            return Some(SQLType {
                name: "datetimeoffset".to_string(),
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
                name: "datetime2".to_string(),
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
            name: "char".to_string(),
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
            name: "varchar".to_string(),
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
            name: "varchar(max)".to_string(),
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
