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
fn two_is_tinyint() {
    assert_eq!(infer(b"2", 0, 0).unwrap().name, SQLTypeName::Tinyint);
}

#[test]
fn utc_datetimeoffset() {
    assert_eq!(
        infer(b"2002-11-09T07:18:21Z", 0, 0).unwrap().name,
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
fn precision_count() {
    println!("{:?}", infer(b"01:00:00.012", 0, 0));
    assert_eq!(3, infer(b"01:00:00.012", 0, 0).unwrap().size);
    assert_eq!(0, infer(b"01:00:00.0", 0, 0).unwrap().size);
    assert_eq!(7, infer(b"01:00:00.012345678", 0, 0).unwrap().size);
}
