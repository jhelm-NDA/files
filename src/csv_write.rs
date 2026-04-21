// TODO 5:
// Write a list of students to students.csv
// Use csv::Writer

use serde::Serialize;
use std::fs::File;

#[derive(Serialize)]
pub struct Student {
    pub name: String,
    pub age: i32,
}

pub fn write_csv(path: &str) -> csv::Result<()> {
    let file = File::create(path)?;
    let mut wtr = csv::Writer::from_writer(file);

    let students = vec![
        Student {name: "Alice".into(), age: 18},
        Student {name: "Bob".into(), age: 19},
    ];

    for s in students {
        wtr.serialize(s)?;
    }

    wtr.flush()?;

    Ok(())

}