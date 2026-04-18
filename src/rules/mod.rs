pub mod no_disallowed_imports;
pub mod no_disallowed_patterns;
pub mod no_event_handlers;
pub mod no_inline_functions;

pub use no_disallowed_imports::NoDisallowedImports;
pub use no_disallowed_patterns::NoDisallowedPatterns;
pub use no_event_handlers::NoEventHandlers;
pub use no_inline_functions::NoInlineFunctions;

use crate::core::types::ValidationError;

pub trait Rule: Send + Sync {
    fn check(&self, content: &str, file: &str) -> Vec<ValidationError>;
}
