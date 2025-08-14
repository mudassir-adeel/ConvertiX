use clap::{Parser, ValueEnum};
use std::fs;
use std::path::PathBuf;
use std::process;

/// Command line arguments
#[derive(Parser)]
#[command(name = "ConvertiX")]
#[command(author = "key")]
#[command(about = "[Cyberspace Asset Mapping Platform Query Statement Conversion Tool]")]
#[command(version = "0.1.0")]
pub struct Args {
    /// Configuration file path
    #[arg(short = 'c', long = "config", default_value = "config.json")]
    pub config: PathBuf,

    /// Query statement (use @filename to read from file)
    #[arg(short = 'q', long = "query")]
    pub query: String,

    /// Source platform of the query statement
    #[arg(short = 'p', long = "platform")]
    pub platform: String,

    /// Output format
    #[arg(short = 'f', long = "format", default_value = "raw")]
    pub format: OutputFormat,

    /// Output file path (optional, defaults to stdout)
    #[arg(short = 'o', long = "output")]
    pub output: Option<PathBuf>,
}

/// Output format options
#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    /// Raw text output
    Raw,
    /// JSON format output
    Json,
}

impl Args {
    /// Parse query input, supporting file input with @ prefix
    pub fn parse_query_input(&self) -> String {
        if self.query.starts_with('@') {
            // Read from file
            let file_path = &self.query[1..]; // Remove @ prefix
            match fs::read_to_string(file_path) {
                Ok(content) => content.trim().to_string(),
                Err(e) => {
                    eprintln!("ERROR: Failed to read query file '{}': {}", file_path, e);
                    process::exit(1);
                }
            }
        } else {
            // Use input directly
            self.query.clone()
        }
    }
}
