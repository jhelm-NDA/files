mod write_file;
mod read_file;
mod json_write;
mod json_read;
mod csv_write;
mod binary_io;

fn main() {
    write_file::write_message("output.txt","Hello, world!").unwrap();
    println!("File says: {}", read_file::read_message("output.txt").unwrap());

    json_write::write_student("student.json").unwrap();
    println!("Student: {:?}", json_read::read_student("student.json").unwrap());

    csv_write::write_csv("students.csv").unwrap();
    println!("Wrote students.csv");

    binary_io::write_binary("data.bin").unwrap();
    println!("Binary read: {:?}", binary_io::read_binary("data.bin").unwrap());
}
