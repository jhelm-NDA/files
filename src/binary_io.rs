// TODO 6A:
// Create a Vec<u8> containing some bytes (e.g., [1, 2, 3, 4])
// Write it to a file named "data.bin"

// TODO 6B:
// Read the bytes back from "data.bin"
// Return them as Vec<u8>

use std::fs::File;
use std::io::{Read,Write};

    pub fn write_binary(path: &str) -> std::io::Result<()> {
        let data: Vec<u8> = vec![1,2,3,4,5];

        let mut file = File::create(path)?;
        file.write_all(&data)?;

        Ok(())
    }

    pub fn read_binary(path: &str) -> std::io::Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }
