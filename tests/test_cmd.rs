use assert_cmd::Command;
use predicates::prelude::*;

mod helper;

#[test]
fn test_help_if_no_command() {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("help"));
}

#[test]
fn test_columns() {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    let assert = cmd
        .arg("columns")
        .arg(&helper::get_test_file("test_all_1.csv"))
        .assert();
    assert.success().stdout(
        "unused, bit, tinyint, smallint, int, \
                            bigint, decimal, real, float, date, time, \
                            datetimeoffset, datetime, char, varchar, varcharmax\n",
    );
}

#[test]
fn test_schema() {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    let assert = cmd
        .arg("schema")
        .arg("-t")
        .arg("test")
        .arg(&helper::get_test_file("test_all_1.csv"))
        .assert();
    assert.success().stdout("DROP TABLE IF EXISTS test;\n\
                            CREATE TABLE test (unused bit, bit bit, \
                            tinyint tinyint, smallint smallint, int int, bigint bigint, \
                            decimal numeric(8, 5), real float(24), float float, date date, \
                            time time(0), datetimeoffset datetimeoffset(5), datetime \
                            datetime2(2), char char(7), varchar varchar(5), varcharmax varchar(max));\n");
}

#[test]
fn test_fail_misssing_filename() {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    let assert = cmd
        .arg("output")
        .arg("-t")
        .arg("test")
        .arg("non_existent_file.csv")
        .assert();
    assert
        .failure()
        .code(1)
        .stderr("Failed to read csv from \"non_existent_file.csv\"\n");
}
