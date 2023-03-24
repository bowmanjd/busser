#[derive(Debug, Default)]
pub struct SQLType {
    pub name: String,
    pub index: usize,
    pub size: usize,
    pub scale: usize,
    pub fixed: bool,
}

// compare function should check if return value is greater or less than
// previous, return greater
// if none then advance

pub fn check_bit(value: &str) -> Option<SQLType> {
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

pub fn check_tinyint(value: &str) -> Option<SQLType> {
    if value.parse::<u8>().is_ok() {
        Some(SQLType {
            name: "tinyint".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

pub fn check_smallint(value: &str) -> Option<SQLType> {
    if value.parse::<i16>().is_ok() {
        Some(SQLType {
            name: "smallint".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

pub fn check_int(value: &str) -> Option<SQLType> {
    if value.parse::<i32>().is_ok() {
        Some(SQLType {
            name: "int".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

pub fn check_bigint(value: &str) -> Option<SQLType> {
    if value.parse::<i64>().is_ok() {
        Some(SQLType {
            name: "bigint".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

pub fn check_decimal(value: &str) -> Option<SQLType> {
    if value.parse::<f64>().is_ok() {
        if let Some(point) = value.find(".") {
            Some(SQLType {
                name: "numeric".to_string(),
                size: value.trim().replace(".", "").len(),
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

pub fn check_real(value: &str) -> Option<SQLType> {
    if value.parse::<f32>().is_ok() {
        Some(SQLType {
            name: "float(24)".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

pub fn check_float(value: &str) -> Option<SQLType> {
    if value.parse::<f64>().is_ok() {
        Some(SQLType {
            name: "float(53)".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

pub fn check_char(value: &str) -> Option<SQLType> {
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


pub fn check_varchar(value: &str) -> Option<SQLType> {
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

pub fn check_varcharmax(value: &str) -> Option<SQLType> {
    if value.len() > 8000 {
        Some(SQLType {
            name: "varchar(max)".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

