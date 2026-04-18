use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub rule: String,
    pub message: String,
    pub fix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ValidationSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub total_files: usize,
    pub total_errors: usize,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub errors_by_rule: HashMap<String, usize>,
}

impl ValidationResult {
    #[allow(dead_code)]
    pub fn valid() -> Self {
        Self {
            valid: true,
            errors: vec![],
            summary: None,
        }
    }

    pub fn with_errors(errors: Vec<ValidationError>, total_files: usize) -> Self {
        let valid = errors.is_empty();
        let mut errors_by_rule = HashMap::new();

        for err in &errors {
            *errors_by_rule.entry(err.rule.clone()).or_insert(0) += 1;
        }

        let total_errors = errors.len();

        Self {
            valid,
            errors,
            summary: Some(ValidationSummary {
                total_files,
                total_errors,
                errors_by_rule,
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleId {
    NoEventHandlers,
    NoInlineFunctions,
    NoDisallowedImports,
    NoDisallowedPatterns,
}

impl RuleId {
    pub fn as_str(&self) -> &'static str {
        match self {
            RuleId::NoEventHandlers => "no-event-handlers",
            RuleId::NoInlineFunctions => "no-inline-functions",
            RuleId::NoDisallowedImports => "no-disallowed-imports",
            RuleId::NoDisallowedPatterns => "no-disallowed-patterns",
        }
    }

    #[allow(dead_code)]
    pub fn message(&self) -> &'static str {
        match self {
            RuleId::NoEventHandlers => "Event handler not allowed",
            RuleId::NoInlineFunctions => "Inline function in JSX not allowed",
            RuleId::NoDisallowedImports => "Disallowed import",
            RuleId::NoDisallowedPatterns => "Disallowed pattern not allowed",
        }
    }

    pub fn fix(&self) -> &'static str {
        match self {
            RuleId::NoEventHandlers => "Use platform components instead of event handlers",
            RuleId::NoInlineFunctions => "Use JSX composition instead of inline functions",
            RuleId::NoDisallowedImports => "Use only allowed imports from whitelisted packages",
            RuleId::NoDisallowedPatterns => "Remove disallowed pattern",
        }
    }
}
