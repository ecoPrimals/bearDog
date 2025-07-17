//! BearDog compliance module

pub mod audit;
pub mod compliance;

// Re-export commonly used types
pub use audit::{AuditEngine, AuditEvent, AuditEventType, AuditSeverity};
pub use compliance::*;
