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
git config --global user.name "{your_username}"
git config --global user.email "{your_email}"
git remote set-url origin https://{your_username}:{your_token}@github.com/{your_username}/{your_repo}.gitgit config --global user.name "{your_username}"
