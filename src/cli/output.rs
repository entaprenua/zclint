use colored::Colorize;

use crate::core::types::ValidationResult;

pub fn format_json(result: &ValidationResult) -> String {
    serde_json::to_string_pretty(result).unwrap_or_else(|_| {
        r#"{"valid": false, "errors": [{"message": "Failed to format output"}]}"#.to_string()
    })
}

pub fn format_text(result: &ValidationResult) {
    if result.errors.is_empty() {
        println!("{}", "✓ No violations found".green());
        return;
    }

    // Group errors by file
    let mut errors_by_file: std::collections::HashMap<String, Vec<_>> =
        std::collections::HashMap::new();
    for error in &result.errors {
        errors_by_file
            .entry(error.file.clone())
            .or_default()
            .push(error);
    }

    // Print errors
    for (file, errors) in errors_by_file {
        println!("{}", file.bold().underline());
        for error in errors {
            let line_col = format!("{}:{}", error.line, error.column);
            println!(
                "  {} {} {}",
                "✗".red(),
                format!("{}:{}", file, line_col).red(),
                error.message.yellow()
            );
            println!("    {} {}", "Fix:".cyan(), error.fix);
        }
        println!();
    }

    // Print summary
    if let Some(summary) = &result.summary {
        println!("{}", "-".dimmed().repeat(50));
        println!(
            "Found {} {} in {} {}",
            summary.total_errors.to_string().red(),
            if summary.total_errors == 1 {
                "violation"
            } else {
                "violations"
            },
            summary.total_files,
            if summary.total_files == 1 {
                "file"
            } else {
                "files"
            }
        );

        if !summary.errors_by_rule.is_empty() {
            println!("\n{}", "By rule:".dimmed());
            for (rule, count) in &summary.errors_by_rule {
                println!("  {}: {}", rule.red(), count);
            }
        }
    }
}
