use once_cell::sync::Lazy;
use regex::Regex;

use crate::core::types::{RuleId, ValidationError};
use crate::rules::Rule;

static EVENT_HANDLERS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        // HTML event handlers
        Regex::new(r"\bonClick\s*=").unwrap(),
        Regex::new(r"\bonChange\s*=").unwrap(),
        Regex::new(r"\bonSubmit\s*=").unwrap(),
        Regex::new(r"\bonInput\s*=").unwrap(),
        Regex::new(r"\bonFocus\s*=").unwrap(),
        Regex::new(r"\bonBlur\s*=").unwrap(),
        Regex::new(r"\bonKeyDown\s*=").unwrap(),
        Regex::new(r"\bonKeyUp\s*=").unwrap(),
        Regex::new(r"\bonKeyPress\s*=").unwrap(),
        Regex::new(r"\bonMouseEnter\s*=").unwrap(),
        Regex::new(r"\bonMouseLeave\s*=").unwrap(),
        Regex::new(r"\bonMouseOver\s*=").unwrap(),
        Regex::new(r"\bonMouseOut\s*=").unwrap(),
        Regex::new(r"\bonScroll\s*=").unwrap(),
        Regex::new(r"\bonWheel\s*=").unwrap(),
        Regex::new(r"\bonDrag\s*=").unwrap(),
        Regex::new(r"\bonDrop\s*=").unwrap(),
        Regex::new(r"\bonTouchStart\s*=").unwrap(),
        Regex::new(r"\bonTouchMove\s*=").unwrap(),
        Regex::new(r"\bonTouchEnd\s*=").unwrap(),
        Regex::new(r"\bonContextMenu\s*=").unwrap(),
        Regex::new(r"\bonDblClick\s*=").unwrap(),
        Regex::new(r"\bonCopy\s*=").unwrap(),
        Regex::new(r"\bonPaste\s*=").unwrap(),
        Regex::new(r"\bonCut\s*=").unwrap(),
        Regex::new(r"\bonAnimationStart\s*=").unwrap(),
        Regex::new(r"\bonAnimationEnd\s*=").unwrap(),
        Regex::new(r"\bonAnimationIteration\s*=").unwrap(),
        Regex::new(r"\bonTransitionEnd\s*=").unwrap(),
        // Solid-specific event handlers
        Regex::new(r"\bon:\w+\s*=").unwrap(),
    ]
});

pub struct NoEventHandlers;

impl Rule for NoEventHandlers {
    fn check(&self, content: &str, file: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for regex in EVENT_HANDLERS.iter() {
            for mat in regex.find_iter(content) {
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
        }

        errors
    }
}
