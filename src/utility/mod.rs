use std::env;
use std::fs;
use std::path::Path;

pub fn read_data_file(filename: String) -> String {
    let path = Path::new(filename.as_str());
    let contents = fs::read_to_string(path);
    contents.expect("Ohh no the file has not been found: {path}")
}
