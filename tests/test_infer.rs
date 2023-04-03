use busser::infer::infer;

#[test]
fn zero_is_bit() {
    assert_eq!(infer("0", 0, 0).unwrap().name, "bit");
}

#[test]
fn one_is_bit() {
    assert_eq!(infer("1", 0, 0).unwrap().name, "bit");
}

#[test]
fn two_is_tinyint() {
    assert_eq!(infer("2", 0, 0).unwrap().name, "tinyint");
}

#[test]
fn utc_datetimeoffset() {
    assert_eq!(
        infer("2002-11-09T07:18:21Z", 0, 0).unwrap().name,
        "datetimeoffset"
    );
}

#[test]
fn datetimeoffset_if_has_tz() {
    assert_eq!(
        infer("2002-11-09T07:18:21+05:00", 0, 0).unwrap().name,
        "datetimeoffset"
    );
}

#[test]
fn datetime_if_no_tz() {
    assert_eq!(
        infer("2004-05-07T09:38:01", 0, 0).unwrap().name,
        "datetime2"
    );
}

#[test]
fn precision_count() {
    assert_eq!(3, infer("01:00:00.012", 0, 0).unwrap().size);
    assert_eq!(0, infer("01:00:00.0", 0, 0).unwrap().size);
    assert_eq!(7, infer("01:00:00.012345678", 0, 0).unwrap().size);
}
