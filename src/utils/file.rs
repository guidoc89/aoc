use std::fs;

pub fn read_file(input_path: &str) -> String {
    fs::read_to_string(input_path).expect("Failed to read file ")
}
