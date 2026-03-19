// SPDX-License-Identifier: AGPL-3.0-only

//! # `BearDog` Compliance Framework
//!
//! Comprehensive compliance and auditing capabilities for `BearDog` applications,
//! ensuring adherence to security standards and regulatory requirements.
//!
//! ## Features
//!
//! - **Regulatory Compliance**: GDPR, HIPAA, SOC 2, and custom regulations
//! - **Audit Logging**: Comprehensive audit trail with tamper detection
//! - **Policy Enforcement**: Automated compliance policy validation
//! - **Reporting**: Compliance reports and violation detection
//! - **Real-Time Monitoring**: Continuous compliance validation
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_compliance::{AuditEngine, AuditEvent, AuditEventType};
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Initialize audit engine
//! let audit = AuditEngine::new()?;
//!
//! // Log compliance event
//! let event = AuditEvent::new(AuditEventType::AccessControl);
//! audit.log_event(event).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! The compliance system provides multi-layered validation:
//! - **Policy Engine**: Validates actions against compliance policies
//! - **Audit Trail**: Immutable log of all compliance-relevant events
//! - **Violation Detection**: Automated detection of compliance violations
//! - **Remediation**: Suggested actions for compliance issues
//!
//! ## Safety
//!
//! All compliance operations are memory-safe with zero unsafe code.

/// Audit logging and event tracking
///
/// Provides comprehensive audit trail functionality with tamper detection.
pub mod audit;

/// Core compliance framework and types
///
/// Policy enforcement, validation, and compliance management.
pub mod compliance;

pub use audit::{AuditEngine, AuditEvent, AuditEventType, AuditSeverity};
pub use compliance::*;

// October 27, 2025: Comprehensive test expansion
#[cfg(test)]
mod audit_comprehensive_tests;

// October 31, 2025: Week 3 Test Expansion - Compliance Validation
#[cfg(test)]
mod compliance_validation_tests;
