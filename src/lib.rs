use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use std::path::Path;

pub fn read_int_lines(input: impl AsRef<str>) -> Result<Vec<u64>, ParseIntError> {
    input
        .as_ref()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<u64>())
        .collect()
}

pub fn read_lines(input: impl AsRef<str>) -> Vec<String> {
    input
        .as_ref()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect()
}

pub fn read_binary_map(input: impl AsRef<str>, true_value: char) -> Vec<Vec<bool>> {
    input
        .as_ref()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c == true_value).collect())
        .collect()
}
