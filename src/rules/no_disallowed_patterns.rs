use once_cell::sync::Lazy;
use regex::Regex;

use crate::core::types::{RuleId, ValidationError};
use crate::rules::Rule;

static DISALLOWED_PATTERNS: Lazy<Vec<(&'static str, Regex)>> = Lazy::new(|| {
    vec![
        // Browser APIs
        ("window", Regex::new(r"\bwindow\b").unwrap()),
        ("document", Regex::new(r"\bdocument\b").unwrap()),
        ("localStorage", Regex::new(r"\blocalStorage\b").unwrap()),
        ("sessionStorage", Regex::new(r"\bsessionStorage\b").unwrap()),
        (
            "document.cookie",
            Regex::new(r"\bdocument\.cookie\b").unwrap(),
        ),
        // Networking
        ("fetch", Regex::new(r"\bfetch\s*\(").unwrap()),
        ("WebSocket", Regex::new(r"\bWebSocket\s*\(").unwrap()),
        ("postMessage", Regex::new(r"\bpostMessage\s*\(").unwrap()),
        // Dynamic code
        ("eval", Regex::new(r"\beval\s*\(").unwrap()),
        (
            "new Function",
            Regex::new(r"\bnew\s+Function\s*\(").unwrap(),
        ),
        ("setTimeout", Regex::new(r"\bsetTimeout\s*\(").unwrap()),
        ("setInterval", Regex::new(r"\bsetInterval\s*\(").unwrap()),
        ("import()", Regex::new(r"\bimport\s*\(").unwrap()),
        ("with statement", Regex::new(r"\bwith\s*\(").unwrap()),
        // HTML injection
        ("innerHTML", Regex::new(r"\.innerHTML\b").unwrap()),
        ("outerHTML", Regex::new(r"\.outerHTML\b").unwrap()),
        (
            "dangerouslySetInnerHTML",
            Regex::new(r"dangerouslySetInnerHTML").unwrap(),
        ),
        (
            "javascript:",
            Regex::new(r#"href\s*=\s*["']javascript:[^"']*["']"#).unwrap(),
        ),
        (
            "data URI",
            Regex::new(r#"href\s*=\s*["']data:[^"']*["']"#).unwrap(),
        ),
        // Script/embed tags
        ("<script", Regex::new(r"<script").unwrap()),
        ("<iframe", Regex::new(r"<iframe").unwrap()),
        ("<embed", Regex::new(r"<embed").unwrap()),
        ("<object", Regex::new(r"<object").unwrap()),
        // Debugging/logging
        ("console", Regex::new(r"\bconsole\.").unwrap()),
        ("debugger", Regex::new(r"\bdebugger;").unwrap()),
        // Event target
        (".target", Regex::new(r"\.\btarget\b").unwrap()),
        (
            ".currentTarget",
            Regex::new(r"\.\bcurrentTarget\b").unwrap(),
        ),
    ]
});

pub struct NoDisallowedPatterns;

impl Rule for NoDisallowedPatterns {
    fn check(&self, content: &str, file: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for (name, pattern) in DISALLOWED_PATTERNS.iter() {
            for mat in pattern.find_iter(content) {
                let line = content[..mat.start()].matches('\n').count() + 1;
                let column = mat.start() - content[..mat.start()].rfind('\n').unwrap_or(0);

                errors.push(ValidationError {
                    file: file.to_string(),
                    line,
                    column,
                    rule: RuleId::NoDisallowedPatterns.as_str().to_string(),
                    message: format!("Disallowed pattern not allowed: '{}'", name),
                    fix: RuleId::NoDisallowedPatterns.fix().to_string(),
                });
            }
        }

        errors
    }
}
