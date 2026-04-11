mod cli;
mod core;
mod rules;

use clap::Parser;
use std::path::Path;
use tracing_subscriber::EnvFilter;

use crate::cli::commands::Cli;
use crate::cli::commands::Commands;
use crate::cli::output::{format_json, format_text};
use crate::core::linter::Linter;
use crate::core::types::ValidationResult;

fn main() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Check { files, json } => match run_check(&files) {
            Ok(result) => {
                if json {
                    println!("{}", format_json(&result));
                } else {
                    format_text(&result);
                }

                if !result.valid {
                    std::process::exit(1);
                }
            }
            Err(msg) => {
                eprintln!("Error: {}", msg);
                std::process::exit(2);
            }
        },
        Commands::Install => {
            if let Err(e) = cli::commands::install_hook() {
                eprintln!("Error: {}", e);
                std::process::exit(2);
            }
        }
        Commands::Init => {
            if let Err(e) = cli::commands::init_config() {
                eprintln!("Error: {}", e);
                std::process::exit(2);
            }
        }
    }
}

fn run_check(files: &[PathBuf]) -> Result<ValidationResult, String> {
    let mut linter = Linter::new();
    let mut all_errors = Vec::new();
    let mut total_files = 0;

    let search_paths: Vec<PathBuf> = if files.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        files.to_vec()
    };

    for search_path in search_paths {
        if !search_path.exists() {
            return Err(format!("File not found: {}", search_path.display()));
        } else if search_path.is_file() {
            // Direct file - lint it
            total_files += 1;
            match linter.lint_file(&search_path) {
                Ok(errors) => all_errors.extend(errors),
                Err(e) => return Err(e),
            }
        } else {
            // Directory - search for files
            let dir_str = search_path.to_string_lossy();
            let extensions = ["tsx", "jsx"];

            for ext in extensions {
                let pattern = format!("{}/**/*.{}", dir_str, ext);
                let entries = glob::glob(&pattern).map_err(|e| e.to_string())?;

                for entry in entries.flatten() {
                    if is_valid_file(&entry) {
                        total_files += 1;
                        match linter.lint_file(&entry) {
                            Ok(errors) => all_errors.extend(errors),
                            Err(e) => return Err(e),
                        }
                    }
                }
            }
        }
    }

    let result = ValidationResult::with_errors(all_errors, total_files);
    Ok(result)
}

fn is_valid_file(path: &Path) -> bool {
    let path_str = path.to_string_lossy();

    // Exclude common non-source directories
    !path_str.contains("node_modules")
        && !path_str.contains("dist")
        && !path_str.contains("build")
        && !path_str.contains(".git")
        && !path_str.contains(".next")
        && !path_str.contains("zclint.config")
        && !path_str.contains("target")
        // Exclude protected directories
        && !path_str.contains("components/ui/")
        && !path_str.contains("components\\ui\\")
        && !path_str.contains("/lib/")
        && !path_str.contains("\\lib\\")
}

use std::path::PathBuf;
