use busser::infer::{infer, SQLTypeName};

#[test]
fn zero_is_bit() {
    assert_eq!(infer(b"0", 0, 0).unwrap().name, SQLTypeName::Bit);
}

#[test]
fn one_is_bit() {
    assert_eq!(infer(b"1", 0, 0).unwrap().name, SQLTypeName::Bit);
}

#[test]
fn blank_is_bit() {
    assert_eq!(infer(b"", 0, 0).unwrap().name, SQLTypeName::Bit);
}

#[test]
fn dot_is_char() {
    assert_eq!(infer(b".", 0, 0).unwrap().name, SQLTypeName::Char);
}

#[test]
fn leading_zero_is_char() {
    assert_eq!(infer(b"0123", 0, 0).unwrap().name, SQLTypeName::Char);
}

#[test]
fn zero_dot_zero_is_numeric() {
    assert_eq!(infer(b"0.00", 0, 0).unwrap().name, SQLTypeName::Numeric);
}

#[test]
fn two_is_tinyint() {
    assert_eq!(infer(b"2", 0, 0).unwrap().name, SQLTypeName::Tinyint);
}

#[test]
fn negative_thousand_is_smallint() {
    assert_eq!(infer(b"-1000", 0, 0).unwrap().name, SQLTypeName::Smallint);
}

#[test]
fn negative_million_is_int() {
    assert_eq!(infer(b"-1000000", 0, 0).unwrap().name, SQLTypeName::Int);
}

#[test]
fn utc_datetime() {
    assert_eq!(
        infer(b"2002-11-09T07:18:21Z", 0, 0).unwrap().name,
        SQLTypeName::Datetime2
    );
}

#[test]
fn utc_datetimeoffset() {
    assert_eq!(
        infer(b"2002-11-09T07:18:21+00:00", 0, 0).unwrap().name,
        SQLTypeName::Datetimeoffset
    );
}

#[test]
fn datetimeoffset_if_has_tz() {
    assert_eq!(
        infer(b"2002-11-09T07:18:21+05:00", 0, 0).unwrap().name,
        SQLTypeName::Datetimeoffset
    );
}

#[test]
fn datetime_if_no_tz() {
    assert_eq!(
        infer(b"2004-05-07T09:38:01", 0, 0).unwrap().name,
        SQLTypeName::Datetime2
    );
}

#[test]
fn zero_is_int_after_merge() {
    let mut num = infer(b"123456789", 0, 0).unwrap();
    let zero = infer(b"0", 0, 0).unwrap();
    num.merge(&zero);
    assert_eq!(
        num.name,
        SQLTypeName::Int
    );
}

#[test]
fn accurate_length_merge_integer_char() {
    let mut num = infer(b"123456789", 0, 0).unwrap();
    let char = infer(b"abcd", 0, 0).unwrap();
    num.merge(&char);
    assert_eq!(
        num.size,
        9
    );
}

#[test]
fn precision_count() {
    println!("{:?}", infer(b"01:00:00.012", 0, 0));
    assert_eq!(3, infer(b"01:00:00.012", 0, 0).unwrap().size);
    assert_eq!(0, infer(b"01:00:00.0", 0, 0).unwrap().size);
    assert_eq!(7, infer(b"01:00:00.012345678", 0, 0).unwrap().size);
}
