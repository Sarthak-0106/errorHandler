// src/main.rs
use error_handler::{parse_log};
use clap::{Parser, ValueEnum};
use serde_json;
use serde_yaml;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(short = 'f', long)]
    file: String,

    /// Output format (json, yaml, plain)
    #[arg(short = 'o', long, default_value = "plain")]
    format: OutputFormat,
}

#[derive(ValueEnum, Debug, Clone)]
enum OutputFormat {
    Json,
    Yaml,
    Plain,
}

fn main() {
    let args = Args::parse();

    match parse_log(&args.file) {
        Ok(parsed_log) => match args.format {
            OutputFormat::Json => {
                let json_output = serde_json::to_string_pretty(&parsed_log).unwrap();
                println!("{}", json_output);
            }
            OutputFormat::Yaml => {
                let yaml_output = serde_yaml::to_string(&parsed_log).unwrap();
                println!("{}", yaml_output);
            }
            OutputFormat::Plain => {
                println!("✅ Parsed Log:");
                println!("{:#?}", parsed_log);
            }
        },
        Err(e) => eprintln!("❌ Error reading log file: {}", e),
    }
}