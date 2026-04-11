use once_cell::sync::Lazy;
use regex::Regex;

use crate::core::types::{RuleId, ValidationError};
use crate::rules::Rule;

static NON_EXPORT_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        // Variable declarations without export
        Regex::new(r"^\s*(?:const|let|var)\s+\w+\s*=").unwrap(),
        // Function declarations
        Regex::new(r"^\s*(?:function|async\s+function)\s+\w+\s*\(").unwrap(),
        // Class declarations
        Regex::new(r"^\s*class\s+\w+").unwrap(),
    ]
});

static EXPORT_LINE: Lazy<Regex> = Lazy::new(|| Regex::new(r"export").unwrap());

pub struct NoPlainTs;

impl Rule for NoPlainTs {
    fn check(&self, content: &str, file: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Only check .ts and .js files (not .tsx/.jsx)
        if file.ends_with(".tsx") || file.ends_with(".jsx") {
            return errors;
        }

        if !file.ends_with(".ts") && !file.ends_with(".js") {
            return errors;
        }

        // Allow index files
        let file_name = std::path::Path::new(file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        if file_name == "index.ts" || file_name == "index.js" {
            return errors;
        }

        // Check for non-export code line by line
        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            // Skip empty lines and comment-only lines
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            // Skip lines with exports
            if EXPORT_LINE.is_match(line) {
                continue;
            }

            // Check for non-export code patterns
            for regex in NON_EXPORT_PATTERNS.iter() {
                if regex.is_match(line) {
                    errors.push(ValidationError {
                        file: file.to_string(),
                        line: line_num + 1,
                        column: 1,
                        rule: RuleId::NoPlainTs.as_str().to_string(),
                        message: "Non-export code in .ts/.js file not allowed".to_string(),
                        fix:
                            "Use .tsx/.jsx for code, or use only export statements in .ts/.js files"
                                .to_string(),
                    });
                    break;
                }
            }
        }

        errors
    }
}
