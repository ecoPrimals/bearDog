// BearDog Compliance Framework
//
// This crate provides comprehensive compliance and auditing capabilities for BearDog
// applications, ensuring adherence to security standards and regulatory requirements.

pub mod audit;
/// Core compliance framework and types
pub mod compliance;

pub use audit::{AuditEngine, AuditEvent, AuditEventType, AuditSeverity};
pub use compliance::*;
