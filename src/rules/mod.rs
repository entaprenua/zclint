pub mod no_event_handlers;
pub mod no_logical_and;
pub mod no_plain_ts;
pub mod no_reactive_primitives;
pub mod no_ternary;

pub use no_event_handlers::NoEventHandlers;
pub use no_logical_and::NoLogicalAnd;
pub use no_plain_ts::NoPlainTs;
pub use no_reactive_primitives::NoReactivePrimitives;
pub use no_ternary::NoTernary;

use crate::core::types::ValidationError;

pub trait Rule: Send + Sync {
    fn check(&self, content: &str, file: &str) -> Vec<ValidationError>;
}
