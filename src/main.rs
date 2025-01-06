// src/main.rs
use error_handler::{parse_log};

fn main() {
    let log_file = "logfiles/sample_log.log";

    match parse_log(log_file) {
        Ok(parsed_log) => {
            println!("✅ Parsed Errors:");
            println!("{:#?}", parsed_log);
        },
        Err(e) => eprintln!("❌ Error reading log file: {}", e),
    }
}
