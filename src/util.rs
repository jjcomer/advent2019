use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input(path: &str) -> Result<Vec<String>> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    Ok(buffered
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>())
}
