use chrono::{NaiveDate, NaiveTime};
use std::fmt;

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

#[derive(Clone, Debug, Default)]
pub struct SQLType {
    pub name: String,
    pub size: usize,
    pub index: usize,
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
                self.size = other.size.max(self.size);
                self.scale = other.scale.max(self.scale);
            } else if self.index < other.index {
                self.name = other.name.clone();
                self.index = other.index;
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

pub fn infer(value: &str, index: usize) -> Option<SQLType> {
    if value == "" {
        return Some(SQLType {
            name: "bit".to_string(),
            index: 0,
            ..Default::default()
        });
    }
    let checks: Vec<&dyn Fn(&str) -> Option<SQLType>> = vec![
        &check_bit,
        &check_tinyint,
        &check_smallint,
        &check_int,
        &check_bigint,
        &check_decimal,
        &check_real,
        &check_float,
        &check_date,
        &check_time,
        &check_char,
        &check_varchar,
        &check_varcharmax,
    ];
    let mut index = index.clone();
    while index < checks.len() {
        if let Some(mut typesize) = checks[index](value) {
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

fn check_bit(value: &str) -> Option<SQLType> {
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

fn check_tinyint(value: &str) -> Option<SQLType> {
    if value.parse::<u8>().is_ok() {
        Some(SQLType {
            name: "tinyint".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_smallint(value: &str) -> Option<SQLType> {
    if value.parse::<i16>().is_ok() {
        Some(SQLType {
            name: "smallint".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_int(value: &str) -> Option<SQLType> {
    if value.parse::<i32>().is_ok() {
        Some(SQLType {
            name: "int".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_bigint(value: &str) -> Option<SQLType> {
    if value.parse::<i64>().is_ok() {
        Some(SQLType {
            name: "bigint".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_decimal(value: &str) -> Option<SQLType> {
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

fn check_real(value: &str) -> Option<SQLType> {
    if value.parse::<f32>().is_ok() {
        Some(SQLType {
            name: "float(24)".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_float(value: &str) -> Option<SQLType> {
    if value.parse::<f64>().is_ok() {
        Some(SQLType {
            name: "float".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_date(value: &str) -> Option<SQLType> {
    for form in DATE_FORMATS {
        if NaiveDate::parse_from_str(value, form).is_ok() {
            return Some(SQLType {
                name: "date".to_string(),
                ..Default::default()
            });
        }
    }
    None
}

fn check_time(value: &str) -> Option<SQLType> {
    for form in TIME_FORMATS {
        if NaiveTime::parse_from_str(value, form).is_ok() {
            return Some(SQLType {
                name: "time".to_string(),
                ..Default::default()
            });
        }
    }
    None
}

fn check_char(value: &str) -> Option<SQLType> {
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

fn check_varchar(value: &str) -> Option<SQLType> {
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

fn check_varcharmax(value: &str) -> Option<SQLType> {
    if value.len() > 8000 {
        Some(SQLType {
            name: "varchar(max)".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}
