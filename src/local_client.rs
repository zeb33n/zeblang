use serde_json;
use std::fs::{read_to_string, File};
use std::io::{Result, Write};

use crate::parser::StatementNode;

pub fn read_file(filename: &str) -> Vec<String> {
    match read_to_string(filename) {
        Ok(value) => value.lines().map(|line| line.to_string()).collect(),
        Err(e) => panic!("Error reading file: {}", e),
    }
}

pub fn write_json(filename: &str, program: Vec<StatementNode>) -> Result<()> {
    let mut file = File::create(format!(
        "{}{}",
        filename.split(".").next().unwrap(),
        ".json"
    ))?;
    let json = serde_json::to_string_pretty(&program)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn write_assembly_file(filename: &str, body: String) -> Result<()> {
    let mut file = File::create(format!("{}{}", filename.split(".").next().unwrap(), ".asm"))?;
    file.write_all(body.as_bytes())?;
    Ok(())
}
