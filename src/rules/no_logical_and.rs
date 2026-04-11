use once_cell::sync::Lazy;
use regex::Regex;

use crate::core::types::{RuleId, ValidationError};
use crate::rules::Rule;

static LOGICAL_AND: Lazy<Regex> = Lazy::new(|| {
    // Match && followed by JSX-like content (starts with < or whitespace + <)
    Regex::new(r"&&\s*(?:[^{]*<[A-Za-z][^>]*>|<[^>]*\/>|[\s\S]*?<)").unwrap()
});

pub struct NoLogicalAnd;

impl Rule for NoLogicalAnd {
    fn check(&self, content: &str, file: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for mat in LOGICAL_AND.find_iter(content) {
            let line = content[..mat.start()].matches('\n').count() + 1;
            let column = mat.start() - content[..mat.start()].rfind('\n').unwrap_or(0);

            errors.push(ValidationError {
                file: file.to_string(),
                line,
                column,
                rule: RuleId::NoLogicalAnd.as_str().to_string(),
                message: "Logical AND operator not allowed".to_string(),
                fix: RuleId::NoLogicalAnd.fix().to_string(),
            });
        }

        errors
    }
}
