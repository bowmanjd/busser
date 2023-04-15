use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::fs;

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
fn test_output_misssing_filename() {
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

#[test]
fn test_output_infer_simple() {
    let temp = assert_fs::TempDir::new().unwrap().into_persistent();
    let output_file = temp.child("test_output.txt");
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    let assert = cmd
        .arg("output")
        .arg("-t")
        .arg("test")
        .arg("-i")
        .arg("-o")
        .arg(output_file.path())
        .arg(&helper::get_test_file("test_all_1.csv"))
        .assert();
    assert.success().stdout("DROP TABLE IF EXISTS test;\n\
                            CREATE TABLE test (unused bit, bit bit, \
                            tinyint tinyint, smallint smallint, int int, bigint bigint, \
                            decimal numeric(8, 5), real float(24), float float, date date, \
                            time time(0), datetimeoffset datetimeoffset(5), datetime \
                            datetime2(2), char char(7), varchar varchar(5), varcharmax varchar(max));\n");
    assert!(
        output_file.exists(),
        "Output path: {:?}",
        output_file.path()
    );
    let file_contents =
        fs::read_to_string(output_file.path()).expect("Should have been able to read the file");
    insta::assert_debug_snapshot!(file_contents);
}
