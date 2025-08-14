use clap::Parser;
use std::process;
use convertix::{Args, ConfigManager, OutputFormatter, QueryConverter};

fn main() {
    let args = Args::parse();

    // Parse query - support reading from file if starts with @
    let query = args.parse_query_input();

    // Load configuration
    let config_manager = match ConfigManager::from_file(&args.config) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("ERROR: Failed to load configuration file: {}", e);
            eprintln!("Please ensure configuration file '{}' exists and is properly formatted", args.config.display());
            process::exit(1);
        }
    };

    // Create converter
    let converter = QueryConverter::new(config_manager);
    let supported_platforms = converter.get_supported_platforms();

    // Validate platform support
    if !supported_platforms.contains(&args.platform) {
        eprintln!("ERROR: Unsupported platform: {}", args.platform);
        eprintln!("Supported platforms: {}", supported_platforms.join(", "));
        process::exit(1);
    }

    // Validate query syntax for source platform
    if let Err(e) = converter.validate_query_syntax(&query, &args.platform) {
        eprintln!("ERROR: {}", e);
        process::exit(1);
    }

    // Perform conversions
    let mut conversions = Vec::new();

    for target_platform in &supported_platforms {
        if target_platform != &args.platform {
            // Syntax validation completed at program start, conversion should always succeed here
            match converter.convert(&query, &args.platform, target_platform) {
                Ok(converted_query) => {
                    conversions.push((target_platform.clone(), converted_query));
                }
                Err(e) => {
                    // This should theoretically not happen since syntax has been validated
                    eprintln!("INTERNAL ERROR: Failed to convert to {}: {}", target_platform, e);
                    process::exit(1);
                }
            }
        }
    }

    // Generate and output results
    let output_content = OutputFormatter::format_output(&args.format, &args.platform, &query, &conversions);
    OutputFormatter::write_output(output_content, args.output);
}

