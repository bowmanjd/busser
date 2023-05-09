mod helper;

#[test]
fn test_schema_returns() {
    let table = busser::csv_schema(&helper::get_test_file("test_all_1.csv"), "test", false)
        .unwrap();
    assert_eq!(
        table,
        "DROP TABLE IF EXISTS test;\nCREATE TABLE test (unused BIT, bit BIT, tinyint \
        TINYINT, smallint SMALLINT, int INT, bigint BIGINT, decimal NUMERIC(11, 5), real FLOAT(24), \
        float FLOAT(53), date DATE, time TIME(0), datetimeoffset DATETIMEOFFSET(5), datetime \
        DATETIME2(2), char CHAR(7), varchar VARCHAR(5), varcharmax VARCHAR(MAX));"
    );
}
