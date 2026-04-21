// TODO 4:
// Read student.json
// Deserialize it into a Student struct

use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Student {
    pub name:String,
    pub age: i32,
}

pub fn read_student(file: &str) -> std::io::Result<Student> {
    let json = fs::read_to_string(file)?;
    let student: Student = serde_json::from_str(&json).unwrap();

    Ok(student)
}