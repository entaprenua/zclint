use once_cell::sync::Lazy;
use regex::Regex;

use crate::core::types::{RuleId, ValidationError};
use crate::rules::Rule;

static REACTIVE_PRIMITIVES: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"\bcreateSignal\s*\(").unwrap(),
        Regex::new(r"\bcreateEffect\s*\(").unwrap(),
        Regex::new(r"\bcreateMemo\s*\(").unwrap(),
        Regex::new(r"\bcreateStore\s*\(").unwrap(),
        Regex::new(r"\bcreateContext\s*\(").unwrap(),
        Regex::new(r"\bcreateRoot\s*\(").unwrap(),
        Regex::new(r"\bbatch\s*\(").unwrap(),
        Regex::new(r"\buntrack\s*\(").unwrap(),
        Regex::new(r"\bon\s*\(").unwrap(),
        Regex::new(r"\bcreateResource\s*\(").unwrap(),
        Regex::new(r"\bcreateDeferred\s*\(").unwrap(),
        Regex::new(r"\bcreateUniqueId\s*\(").unwrap(),
        Regex::new(r"\bmergeProps\s*\(").unwrap(),
        Regex::new(r"\bsplitProps\s*\(").unwrap(),
        Regex::new(r"\buseContext\s*\(").unwrap(),
    ]
});

pub struct NoReactivePrimitives;

impl Rule for NoReactivePrimitives {
    fn check(&self, content: &str, file: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for regex in REACTIVE_PRIMITIVES.iter() {
            for mat in regex.find_iter(content) {
                let line = content[..mat.start()].matches('\n').count() + 1;
                let column = mat.start() - content[..mat.start()].rfind('\n').unwrap_or(0);

                errors.push(ValidationError {
                    file: file.to_string(),
                    line,
                    column,
                    rule: RuleId::NoReactivePrimitives.as_str().to_string(),
                    message: format!("Reactive primitive not allowed: '{}'", mat.as_str()),
                    fix: RuleId::NoReactivePrimitives.fix().to_string(),
                });
            }
        }

        errors
    }
}
