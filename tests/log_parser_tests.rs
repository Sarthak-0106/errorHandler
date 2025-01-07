// tests/log_parser_tests.rs
use error_handler::{parse_log, ParsedLog}; // Correct import path for the library

#[test]
fn test_valid_log_parsing() {
    let sample_content = "\
[ERROR] Missing file: data/config.ini
[ERROR] Corrupted file: assets/texture.png
[ERROR] Dependency error: Missing DirectX Runtime
[ERROR] Missing file: lib/library.dll
";

    let test_file = "test_valid_log.log";
    std::fs::write(test_file, sample_content).unwrap();

    let parsed_log = parse_log(test_file).unwrap();

    assert_eq!(
        parsed_log,
        ParsedLog {
            missing_files: vec![
                "data/config.ini".to_string(),
                "lib/library.dll".to_string()
            ],
            corrupt_files: vec![
                "assets/texture.png".to_string()
            ],
            dependency_errors: vec![
                "Missing DirectX Runtime".to_string()
            ],
            unknown_errors: vec![]
        }
    );

    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn test_empty_log_file() {
    let test_file = "test_empty_log.log";
    std::fs::write(test_file, "").unwrap();

    let parsed_log = parse_log(test_file).unwrap();

    assert_eq!(
        parsed_log,
        ParsedLog {
            missing_files: vec![],
            corrupt_files: vec![],
            dependency_errors: vec![],
            unknown_errors: vec![]
        }
    );

    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn test_ignore_non_error_lines() {
    let sample_content = "\
[INFO] Installation started successfully.
[WARN] Installation took longer than expected.
[ERROR] Missing file: data/config.ini
";

    let test_file = "test_ignore_non_error.log";
    std::fs::write(test_file, sample_content).unwrap();

    let parsed_log = parse_log(test_file).unwrap();

    assert_eq!(
        parsed_log,
        ParsedLog {
            missing_files: vec![
                "data/config.ini".to_string()
            ],
            corrupt_files: vec![],
            dependency_errors: vec![],
            unknown_errors: vec![]
        }
    );

    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn test_mixed_content() {
    let sample_content = "\
[INFO] Starting process...
[ERROR] Missing file: config.yml
Invalid line without error type
[ERROR] Corrupted file: assets/data.png
[WARN] Slow response detected
";

    let test_file = "test_mixed_content.log";
    std::fs::write(test_file, sample_content).unwrap();

    let parsed_log = parse_log(test_file).unwrap();

    assert_eq!(
        parsed_log,
        ParsedLog {
            missing_files: vec![
                "config.yml".to_string()
            ],
            corrupt_files: vec![
                "assets/data.png".to_string()
            ],
            dependency_errors: vec![],
            unknown_errors: vec![
                "Invalid line without error type".to_string()
            ]
        }
    );

    std::fs::remove_file(test_file).unwrap();
}

#[test]
