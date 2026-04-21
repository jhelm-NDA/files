// TODO 1:
// Write "Hello, world!" to a file named output.txt

// Use std::fs::write

use std::fs;

pub fn write_message(path:&str, msg:&str) -> std::io::Result<()> {
    fs::write(path,msg)?;
    Ok(())
}
