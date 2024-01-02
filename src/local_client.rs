use std::fs::{read_to_string, File};
use std::io::{Result, Write};

pub fn read_file(filename: &str) -> Vec<String> {
    match read_to_string(filename) {
        Ok(value) => value.lines().map(str::to_string).collect(),
        Err(e) => panic!("Error reading file: {}", e),
    }
}

pub fn write_assembly_file(filename: &str, body: String) -> Result<()> {
    let mut file = File::create(format!("{}{}", filename.split(".").next().unwrap(), ".asm"))?;
    file.write_all(body.as_bytes())?;
    Ok(())
}
