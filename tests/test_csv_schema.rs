mod helper;

#[test]
fn test_schema_returns() {
    let table = busser::csv_schema(&helper::get_test_file("test_all_1.csv"), "test", false)
        .unwrap();
    assert_eq!(
        table,
        "DROP TABLE IF EXISTS test;\nCREATE TABLE test (unused bit, bit bit, tinyint \
        tinyint, smallint smallint, int int, bigint bigint, decimal numeric(8, 5), real float(24), \
        float float, date date, time time(0), datetimeoffset datetimeoffset(5), datetime \
        datetime2(2), char char(7), varchar varchar(5), varcharmax varchar(max));"
    );
}
