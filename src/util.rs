use std::fs::read_to_string;

use crate::error::Result;

pub fn read_lines(filename: &str) -> Result<Vec<String>> {
    let mut lines = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        lines.push(line.to_string());
    }

    Ok(lines)
}
