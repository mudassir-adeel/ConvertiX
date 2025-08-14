use crate::cli::OutputFormat;
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use std::process;

/// Output formatter for conversion results
pub struct OutputFormatter;

impl OutputFormatter {
    /// Generate raw text output
    pub fn generate_raw_output(
        platform: &str,
        query: &str,
        conversions: &[(String, String)],
    ) -> String {
        let mut output = String::new();
        output.push_str(&format!("Source platform: {}\n", platform));
        output.push_str(&format!("Original query: {}\n", query));
        output.push_str("\n");

        for (platform, converted_query) in conversions {
            output.push_str(&format!(
                "[-] {}:\n{}\n\n",
                platform.to_uppercase(),
                converted_query
            ));
        }

        output
    }

    /// Generate JSON output
    pub fn generate_json_output(
        source_platform: &str,
        query: &str,
        conversions: &[(String, String)],
    ) -> String {
        let mut converted_queries = serde_json::Map::new();

        for (platform, query) in conversions {
            converted_queries.insert(platform.clone(), json!(query));
        }

        let result = json!({
            "source_platform": source_platform,
            "original_query": query,
            "converted_queries": converted_queries,
        });

        serde_json::to_string_pretty(&result).unwrap_or_else(|e| {
            eprintln!("ERROR: Failed to serialize JSON output: {}", e);
            process::exit(1);
        })
    }

    /// Write output to file or stdout
    pub fn write_output(content: String, output_path: Option<PathBuf>) {
        match output_path {
            Some(path) => {
                if let Err(e) = fs::write(&path, content) {
                    eprintln!(
                        "ERROR: Failed to write to output file '{}': {}",
                        path.display(),
                        e
                    );
                    process::exit(1);
                }
                println!("Output written to: {}", path.display());
            }
            None => {
                print!("{}", content);
            }
        }
    }

    /// Format output based on the specified format
    pub fn format_output(
        format: &OutputFormat,
        source_platform: &str,
        query: &str,
        conversions: &[(String, String)],
    ) -> String {
        match format {
            OutputFormat::Raw => Self::generate_raw_output(source_platform, query, conversions),
            OutputFormat::Json => Self::generate_json_output(source_platform, query, conversions),
        }
    }
}
