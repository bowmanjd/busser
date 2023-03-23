use rust_decimal::prelude::*;

#[derive(Debug, Default)]
pub struct SQLType {
    pub name: String,
    pub size: u32,
    pub precision: u8,
}

pub fn check_integer(value: &str) -> Option<SQLType> {
    if value.parse::<u8>().is_ok() {
        Some(SQLType {
            name: "tinyint".to_string(),
            ..Default::default()
        })
    } else if value.parse::<i32>().is_ok() {
        Some(SQLType {
            name: "int".to_string(),
            ..Default::default()
        })
    } else if value.parse::<i64>().is_ok() {
        Some(SQLType {
            name: "bigint".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

pub fn check_decimal(value: &str) -> Option<String> {
    let dec = Decimal::from_str(value);
    if dec.is_ok() {
        Some("float(24)".to_string())
    } else if value.parse::<f64>().is_ok() {
        Some("float(53)".to_string())
    } else {
        None
    }
}

pub fn check_float(value: &str) -> Option<SQLType> {
    if value.parse::<f32>().is_ok() {
        Some(SQLType {
            name: "float(24)".to_string(),
            ..Default::default()
        })
    } else if value.parse::<f64>().is_ok() {
        Some(SQLType {
            name: "float(53)".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}
