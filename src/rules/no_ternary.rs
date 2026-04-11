use once_cell::sync::Lazy;
use regex::Regex;

use crate::core::types::{RuleId, ValidationError};
use crate::rules::Rule;

static TERNARY: Lazy<Regex> = Lazy::new(|| Regex::new(r"\?\s*[^?]+\s*:\s*[^:?]+").unwrap());

pub struct NoTernary;

impl Rule for NoTernary {
    fn check(&self, content: &str, file: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for mat in TERNARY.find_iter(content) {
            let line = content[..mat.start()].matches('\n').count() + 1;
            let column = mat.start() - content[..mat.start()].rfind('\n').unwrap_or(0);

            errors.push(ValidationError {
                file: file.to_string(),
                line,
                column,
                rule: RuleId::NoTernary.as_str().to_string(),
                message: "Ternary operator not allowed".to_string(),
                fix: RuleId::NoTernary.fix().to_string(),
            });
        }

        errors
    }
}
