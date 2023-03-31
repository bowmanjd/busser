mod helper;

#[test]
fn test_schema_returns() {
    let table = busser::csv_schema(&helper::get_test_file("sample.csv"), "test");
    assert_eq!(
        table.unwrap_or("".to_string()),
        "DROP TABLE IF EXISTS test;\nCREATE TABLE test (CheckinTime time(3), Group_Num tinyint, \
        EmailAddress varchar(40), First_Name varchar(8), Last_Name varchar(11), unused bit, \
        StartDate date, EndDatetime datetime2(1), HawaiiTime datetimeoffset(1));"
    );
}
