#[derive(Clone, Debug, Default)]
pub struct SQLType {
    pub name: String,
    pub size: usize,
    pub index: usize,
    pub scale: usize,
    pub fixed: bool,
}

/*
pub fn infer_many(values: Vec<&str>, index: usize) -> Option<SQLType> {
    let checks: Vec<&dyn Fn(&str) -> Option<SQLType>> = vec![
        &check_bit,
        &check_tinyint,
        &check_smallint,
        &check_int,
        &check_bigint,
        &check_decimal,
        &check_real,
        &check_float,
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
*/

pub fn infer(value: &str, index: usize) -> Option<SQLType> {
    if value == "" {
        return None;
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
