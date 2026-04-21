// TODO 2:
// Read the contents of output.txt and return it as a String

use std::fs;

pub fn read_message(file: &str) -> std::io::Result<String> {
    let contents = fs::read_to_string(file)?;
    Ok(contents)
}