use std::env;
use std::fs;
use std::path::Path;

const BASE_PATH: &str = "data/";

pub fn read_data_file(filename: String) -> String {
    let path = Path::new(BASE_PATH);
    let path = path.join(filename);
    dbg!(&path);
    let contents = fs::read_to_string(path);
    contents.expect("Ohh no the file has not been found: {path}")
}
