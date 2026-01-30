//! Graph Security Module for Collaborative Intelligence
//!
//! This module provides security validation for graph modifications, templates,
//! and origins as part of the biomeOS Collaborative Intelligence ecosystem.
//!
//! # Architecture
//!
//! The module implements a 5-layer security model:
//! 1. **Authentication**: User identity verification via HSM
//! 2. **Authorization**: RBAC and ownership validation
//! 3. **Validation**: Structure and safety checks
//! 4. **Threat Detection**: Malicious pattern detection
//! 5. **Audit**: Provenance and trust scoring
//!
//! # APIs
//!
//! - `authorize_modification`: Real-time authorization for graph edits
//! - `validate_template`: Template safety validation
//! - `audit_origin`: Template provenance verification

pub mod audit;
pub mod authorize;
pub mod collaboration_service;
pub mod permissions;
pub mod threats;
pub mod types;
pub mod validate;

// Internal helpers (not public API)
pub(crate) mod internal;

#[cfg(test)]
mod tests;

pub use audit::audit_origin;
pub use authorize::authorize_modification;
pub use types::*;
pub use validate::validate_template;
