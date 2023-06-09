use std::fs;
use std::io;

pub fn load(filepath: String) -> io::Result<String> {
    fs::read_to_string(filepath)
}
