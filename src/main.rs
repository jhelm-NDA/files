mod write_file;
mod read_file;
mod json_write;
mod json_read;
mod csv_write;
mod binary_io;

fn main() {
    write_file::write_message().unwrap();
    println!("File says: {}", read_file::read_message().unwrap());

    json_write::write_student().unwrap();
    println!("Student: {:?}", json_read::read_student().unwrap());

    csv_write::write_csv().unwrap();
    println!("Wrote students.csv");

    binary_io::write_binary().unwrap();
    println!("Binary read: {:?}", binary_io::read_binary().unwrap());
}
