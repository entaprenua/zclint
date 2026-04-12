use once_cell::sync::Lazy;
use regex::Regex;

use crate::core::types::{RuleId, ValidationError};
use crate::rules::Rule;

static INLINE_FUNCTION_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        // Arrow function returning JSX or block: {() => <...>} or {() => { ... return ... }}
        Regex::new(r"\{\s*\(\s*\)\s*=>\s*(?:\{[^}]*?return|<[A-Za-z][^>]*>)").unwrap(),
        // Anonymous function expression: {(function() { ... })()}
        Regex::new(r"\{\s*\(?\s*function\s*\([^)]*\)\s*\{[^}]*?\}\s*\(\)\s*\}").unwrap(),
    ]
});

pub struct NoInlineFunctions;

impl Rule for NoInlineFunctions {
    fn check(&self, content: &str, file: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for regex in INLINE_FUNCTION_PATTERNS.iter() {
            for mat in regex.find_iter(content) {
                let line = content[..mat.start()].matches('\n').count() + 1;
                let column = mat.start() - content[..mat.start()].rfind('\n').unwrap_or(0);

                errors.push(ValidationError {
                    file: file.to_string(),
                    line,
                    column,
                    rule: RuleId::NoInlineFunctions.as_str().to_string(),
                    message: "Inline function in JSX not allowed".to_string(),
                    fix: RuleId::NoInlineFunctions.fix().to_string(),
                });
            }
        }

        errors
    }
}
