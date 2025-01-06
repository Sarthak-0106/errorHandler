// src/lib.rs
use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq)] // Ensures ParsedLog can be printed using {:?}
pub struct ParsedLog {
    pub missing_files: Vec<String>,
    pub corrupt_files: Vec<String>,
    pub dependency_errors: Vec<String>,
    pub unknown_errors: Vec<String>,
}

// Parses the log file and identifies errors
pub fn parse_log(file_path: &str) -> Result<ParsedLog, std::io::Error> {
    let content = fs::read_to_string(file_path)?;

    let mut missing_files = Vec::new();
    let mut corrupt_files = Vec::new();
    let mut dependency_errors = Vec::new();
    let mut unknown_errors = Vec::new();

    let missing_file_regex = Regex::new(r"Missing file: (.+)").unwrap();
    let corrupt_file_regex = Regex::new(r"Corrupted file: (.+)").unwrap();
    let dependency_error_regex = Regex::new(r"Dependency error: (.+)").unwrap();

    for line in content.lines() {
        if line.contains("[INFO]") || line.contains("[WARN]") {
            continue; // Skip non-error messages
        }

        if let Some(caps) = missing_file_regex.captures(line) {
            missing_files.push(caps[1].to_string());
        } else if let Some(caps) = corrupt_file_regex.captures(line) {
            corrupt_files.push(caps[1].to_string());
        } else if let Some(caps) = dependency_error_regex.captures(line) {
            dependency_errors.push(caps[1].to_string());
        } else {
            unknown_errors.push(line.to_string());
        }
    }

    Ok(ParsedLog { 
        missing_files,
        corrupt_files,
        dependency_errors,
        unknown_errors, 
    })
}
