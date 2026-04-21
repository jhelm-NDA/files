// TODO 3:
// Define a struct Student { name: String, age: i32 }
// Serialize it to JSON using serde_json::to_string
// Write the JSON to student.json

use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct Student {
    pub name: String,
    pub age: i32,
}

pub fn write_student(file: &str) -> std::io::Result<()> {
    let s = Student {
        name: "Alice".to_string(),
        age: 18,
    };

    let json = serde_json::to_string(&s).unwrap();
    fs::write(file, json)?;
    Ok(())

}