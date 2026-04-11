use std::path::Path;

use crate::core::types::ValidationError;
use crate::rules::Rule;

pub struct Linter {
    rules: Vec<Box<dyn Rule>>,
}

impl Linter {
    pub fn new() -> Self {
        let rules: Vec<Box<dyn Rule>> = vec![
            Box::new(crate::rules::NoEventHandlers),
            Box::new(crate::rules::NoReactivePrimitives),
            Box::new(crate::rules::NoTernary),
            Box::new(crate::rules::NoLogicalAnd),
            Box::new(crate::rules::NoPlainTs),
        ];

        Self { rules }
    }

    pub fn lint_file(&mut self, path: &Path) -> Result<Vec<ValidationError>, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {} - {}", path.display(), e.kind()))?;

        let path_str = path.to_string_lossy().to_string();

        let mut errors = Vec::new();

        for rule in &self.rules {
            let rule_errors = rule.check(&content, &path_str);
            errors.extend(rule_errors);
        }

        Ok(errors)
    }
}

impl Default for Linter {
    fn default() -> Self {
        Self::new()
    }
}
