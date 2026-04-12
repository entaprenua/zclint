use once_cell::sync::Lazy;
use regex::Regex;

use crate::core::types::{RuleId, ValidationError};
use crate::rules::Rule;

static EVENT_HANDLERS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\bon\w+\s*=").unwrap());

pub struct NoEventHandlers;

impl Rule for NoEventHandlers {
    fn check(&self, content: &str, file: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for mat in EVENT_HANDLERS.find_iter(content) {
            let line = content[..mat.start()].matches('\n').count() + 1;
            let column = mat.start() - content[..mat.start()].rfind('\n').unwrap_or(0);

            errors.push(ValidationError {
                file: file.to_string(),
                line,
                column,
                rule: RuleId::NoEventHandlers.as_str().to_string(),
                message: format!("Event handler not allowed: '{}'", mat.as_str()),
                fix: RuleId::NoEventHandlers.fix().to_string(),
            });
        }

        errors
    }
}
