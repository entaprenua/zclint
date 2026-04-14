use once_cell::sync::Lazy;
use regex::Regex;

use crate::core::types::{RuleId, ValidationError};
use crate::rules::Rule;

pub static ALLOWED_IMPORTS: Lazy<Vec<(&'static str, Vec<&'static str>)>> = Lazy::new(|| {
    vec![
        ("lucide-solid", vec!["*"]), // Any import allowed
        ("solid-js", vec!["splitProps", "mergeProps", "Suspense"]),
        (
            "@solidjs/meta",
            vec!["MetaProvider", "Title", "Meta", "Link", "Base"],
        ),
        ("@solidjs/router", vec!["Router", "Routes", "A"]),
        ("@solidjs/start", vec!["clientOnly"]),
        ("@solidjs/start/router", vec!["FileRoutes"]),
    ]
});

static IMPORT_STATEMENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"import\s+(?:\{[^}]+\}|\*\s+as\s+\w+|\w+)\s+from\s+['"]([^'"]+)['"]"#).unwrap()
});

pub struct NoDisallowedImports;

impl NoDisallowedImports {
    fn check_import(&self, content: &str, file: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for cap in IMPORT_STATEMENT.captures_iter(content) {
            let import_path = cap.get(1).map(|m| m.as_str()).unwrap_or("");

            // Skip local imports
            if import_path.starts_with("./") || import_path.starts_with("../") {
                continue;
            }
            // Skip path aliases like ~/ and @/
            if import_path.starts_with("~/") || import_path.starts_with("@/") {
                continue;
            }

            // Check if this import is in our allowed list
            let is_allowed = ALLOWED_IMPORTS.iter().any(|(package, allowed)| {
                if import_path == *package || import_path.starts_with(&format!("{}/", *package)) {
                    if allowed.contains(&"*") {
                        return true; // Any import from this package is allowed
                    }
                    // Check specific named imports
                    let line = cap.get(0).map(|m| m.as_str()).unwrap_or("");
                    // Extract named imports from { Import1, Import2 } pattern
                    if let Some(named_cap) = Regex::new(r"\{([^}]+)\}").unwrap().captures(line) {
                        let imports_str = named_cap.get(1).map(|m| m.as_str()).unwrap_or("");
                        for import_name in imports_str.split(',') {
                            let name = import_name.trim().split(" as ").next().unwrap_or("").trim();
                            if !allowed.contains(&name) {
                                return false;
                            }
                        }
                    }
                    true
                } else {
                    false
                }
            });

            if !is_allowed {
                let full_match = cap.get(0).unwrap();
                let line = content[..full_match.start()].matches('\n').count() + 1;
                let column =
                    full_match.start() - content[..full_match.start()].rfind('\n').unwrap_or(0);

                errors.push(ValidationError {
                    file: file.to_string(),
                    line,
                    column,
                    rule: RuleId::NoDisallowedImports.as_str().to_string(),
                    message: format!("Disallowed import: '{}'", import_path),
                    fix: RuleId::NoDisallowedImports.fix().to_string(),
                });
            }
        }

        errors
    }
}

impl Rule for NoDisallowedImports {
    fn check(&self, content: &str, file: &str) -> Vec<ValidationError> {
        // Only check .tsx and .jsx files
        if !file.ends_with(".tsx") && !file.ends_with(".jsx") {
            return Vec::new();
        }

        self.check_import(content, file)
    }
}
