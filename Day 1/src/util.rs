use std::fs;
pub fn read_file_as_lines(file_path: &str)-> String{
    fs::read_to_string(file_path).unwrap()
}