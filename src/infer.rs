use chrono::{DateTime, NaiveDate, NaiveTime, Timelike};
use std::fmt;

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
    &(check_datetimeoffset as fn(&str, usize) -> Option<SQLType>),
    &(check_datetime as fn(&str, usize) -> Option<SQLType>),
    &(check_char as fn(&str, usize) -> Option<SQLType>),
    &(check_varchar as fn(&str, usize) -> Option<SQLType>),
    &(check_varcharmax as fn(&str, usize) -> Option<SQLType>),
];

const DATE_FORMATS: [&str; 14] = [
    "%Y-%m-%d",
    "%Y%m%d",
    "%m/%d/%Y",
    "%m-%d-%Y",
    "%Y.%m.%d",
    "%Y/%m/%d",
    "%m.%d.%Y",
    "%B %d %Y",
    "%B %d, %Y",
    "%d %B %Y",
    "%d %B, %Y",
    "%d %Y %B",
    "%Y %B %d",
    "%Y %d %B",
];

const TIME_FORMATS: [&str; 6] = ["%T%.f", "%I:%M:%S%.f %p", "%T", "%H:%M", "%r", "%I:%M %p"];

const DATETIME_FORMATS: [&str; 85] = [
    "%Y-%m-%dT%H:%M:%S%.f",
    "%Y-%m-%d %T%.f",
    "%Y-%m-%d %I:%M:%S%.f %p",
    "%Y-%m-%d %T",
    "%Y-%m-%d %H:%M",
    "%Y-%m-%d %r",
    "%Y-%m-%d %I:%M %p",
    "%Y%m%d %T%.f",
    "%Y%m%d %I:%M:%S%.f %p",
    "%Y%m%d %T",
    "%Y%m%d %H:%M",
    "%Y%m%d %r",
    "%Y%m%d %I:%M %p",
    "%m/%d/%Y %T%.f",
    "%m/%d/%Y %I:%M:%S%.f %p",
    "%m/%d/%Y %T",
    "%m/%d/%Y %H:%M",
    "%m/%d/%Y %r",
    "%m/%d/%Y %I:%M %p",
    "%m-%d-%Y %T%.f",
    "%m-%d-%Y %I:%M:%S%.f %p",
    "%m-%d-%Y %T",
    "%m-%d-%Y %H:%M",
    "%m-%d-%Y %r",
    "%m-%d-%Y %I:%M %p",
    "%Y.%m.%d %T%.f",
    "%Y.%m.%d %I:%M:%S%.f %p",
    "%Y.%m.%d %T",
    "%Y.%m.%d %H:%M",
    "%Y.%m.%d %r",
    "%Y.%m.%d %I:%M %p",
    "%Y/%m/%d %T%.f",
    "%Y/%m/%d %I:%M:%S%.f %p",
    "%Y/%m/%d %T",
    "%Y/%m/%d %H:%M",
    "%Y/%m/%d %r",
    "%Y/%m/%d %I:%M %p",
    "%m.%d.%Y %T%.f",
    "%m.%d.%Y %I:%M:%S%.f %p",
    "%m.%d.%Y %T",
    "%m.%d.%Y %H:%M",
    "%m.%d.%Y %r",
    "%m.%d.%Y %I:%M %p",
    "%B %d %Y %T%.f",
    "%B %d %Y %I:%M:%S%.f %p",
    "%B %d %Y %T",
    "%B %d %Y %H:%M",
    "%B %d %Y %r",
    "%B %d %Y %I:%M %p",
    "%B %d, %Y %T%.f",
    "%B %d, %Y %I:%M:%S%.f %p",
    "%B %d, %Y %T",
    "%B %d, %Y %H:%M",
    "%B %d, %Y %r",
    "%B %d, %Y %I:%M %p",
    "%d %B %Y %T%.f",
    "%d %B %Y %I:%M:%S%.f %p",
    "%d %B %Y %T",
    "%d %B %Y %H:%M",
    "%d %B %Y %r",
    "%d %B %Y %I:%M %p",
    "%d %B, %Y %T%.f",
    "%d %B, %Y %I:%M:%S%.f %p",
    "%d %B, %Y %T",
    "%d %B, %Y %H:%M",
    "%d %B, %Y %r",
    "%d %B, %Y %I:%M %p",
    "%d %Y %B %T%.f",
    "%d %Y %B %I:%M:%S%.f %p",
    "%d %Y %B %T",
    "%d %Y %B %H:%M",
    "%d %Y %B %r",
    "%d %Y %B %I:%M %p",
    "%Y %B %d %T%.f",
    "%Y %B %d %I:%M:%S%.f %p",
    "%Y %B %d %T",
    "%Y %B %d %H:%M",
    "%Y %B %d %r",
    "%Y %B %d %I:%M %p",
    "%Y %d %B %T%.f",
    "%Y %d %B %I:%M:%S%.f %p",
    "%Y %d %B %T",
    "%Y %d %B %H:%M",
    "%Y %d %B %r",
    "%Y %d %B %I:%M %p",
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
        } else {
            if self.index == other.index {
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
}

impl fmt::Display for SQLType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut name = self.name.clone();
        if self.size > 0 {
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
    if value == "" {
        return Some(SQLType {
            name: "bit".to_string(),
            ..Default::default()
        });
    }
    let mut index = index.clone();
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
    let numeric = value.trim().replace(".", "");
    let length = numeric.len();
    if value.parse::<f64>().is_ok() && numeric.chars().all(char::is_numeric) && length <= 38 {
        if let Some(point) = value.find(".") {
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
    if value.parse::<f32>().is_ok() {
        Some(SQLType {
            name: "float(24)".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
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
    for i in (subindex..DATE_FORMATS.len()).chain(0..subindex) {
        let form = DATE_FORMATS[i];
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

fn check_time(value: &str, subindex: usize) -> Option<SQLType> {
    for i in (subindex..TIME_FORMATS.len()).chain(0..subindex) {
        let form = TIME_FORMATS[i];
        if let Ok(parsed) = NaiveTime::parse_from_str(&value, form) {
            return Some(SQLType {
                name: "time".to_string(),
                subindex,
                size: parsed
                    .nanosecond()
                    .to_string()
                    .trim_end_matches('0')
                    .len()
                    .min(7),
                ..Default::default()
            });
        }
    }
    None
}

fn check_datetimeoffset(value: &str, subindex: usize) -> Option<SQLType> {
    for i in (subindex..DATETIME_FORMATS.len()).chain(0..subindex) {
        let form = DATETIME_FORMATS[i];
        if let Ok(parsed) = DateTime::parse_from_str(&value, &format!("{} %z", form)) {
            return Some(SQLType {
                name: "datetimeoffset".to_string(),
                subindex,
                size: parsed
                    .nanosecond()
                    .to_string()
                    .trim_end_matches('0')
                    .len()
                    .min(7),
                ..Default::default()
            });
        }
    }
    None
}

fn check_datetime(value: &str, subindex: usize) -> Option<SQLType> {
    for i in (subindex..DATETIME_FORMATS.len()).chain(0..subindex) {
        let form = DATETIME_FORMATS[i];
        if let Ok(parsed) = NaiveTime::parse_from_str(&value, form) {
            return Some(SQLType {
                name: "datetime2".to_string(),
                subindex,
                size: parsed
                    .nanosecond()
                    .to_string()
                    .trim_end_matches('0')
                    .len()
                    .min(7),
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
