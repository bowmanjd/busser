use std::path::PathBuf;

pub fn get_test_file(filename: &str) -> PathBuf {
    // locate resources/test relative to crate base dir
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/test/");
    path.push(filename);
    path
}
