// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Self-Enforcing Key Constraints
//!
//! This module implements cryptographically-enforced constraints that are embedded
//! directly into BearDog keys, making them tamper-proof and self-enforcing.
//!
//! ## Philosophy
//!
//! Keys are not just identity - they are **architectural law**. When a key is created
//! with constraints, those constraints are cryptographically signed and embedded in
//! the key itself. Any attempt to use the key must pass constraint verification first.
//!
//! ## Core Concepts
//!
//! - **ScopeConstraint**: What domains the key can operate in
//! - **LifetimeConstraint**: Time-based expiry and evolution triggers
//! - **DataAccessConstraint**: Fine-grained data access rules
//! - **BehavioralConstraint**: Biometric verification, co-signing requirements
//! - **ConstraintEvolution**: Constraints adapt based on key behavior
//!
//! ## Security Model
//!
//! ```text
//! ┌────────────────┐
//! │  Key Creation  │
//! └────────┬───────┘
//!          │
//!          ▼
//! ┌────────────────────┐
//! │ Define Constraints │ (Scope, Lifetime, Data Access, etc.)
//! └────────┬───────────┘
//!          │
//!          ▼
//! ┌────────────────────┐
//! │ Sign Constraints   │ (Private key signs constraint hash)
//! └────────┬───────────┘
//!          │
//!          ▼
//! ┌────────────────────┐
//! │ Embed in Key       │ (Constraints + signature stored in key)
//! └────────┬───────────┘
//!          │
//!          ▼
//! ┌────────────────────┐
//! │ Key Usage Attempt  │
//! └────────┬───────────┘
//!          │
//!          ▼
//! ┌────────────────────┐
//! │ Verify Signature   │ (Detect tampering)
//! └────────┬───────────┘
//!          │
//!          ▼
//! ┌────────────────────┐
//! │ Check Constraints  │ (Operation allowed?)
//! └────────┬───────────┘
//!          │
//!    ┌─────┴─────┐
//!    ▼           ▼
//! ✅ Allow    ❌ Deny
//! ```
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_genetics::constraints::*;
//! use beardog_genetics::BearDogGenetics;
//!
//! // Create a collaboration key with constraints
//! let constraints = KeyConstraints {
//!     scope: ScopeConstraint::Limited {
//!         domains: vec!["climate_modeling".to_string()],
//!     },
//!     lifetime: LifetimeConstraint::Duration {
//!         months: 18,
//!         evolution_trigger: Some(12),
//!     },
//!     data_access: DataAccessConstraint {
//!         cannot_delete: vec!["raw_data/*".to_string()],
//!         cannot_modify: vec!["published/*".to_string()],
//!         must_encrypt_to: vec!["martinez_key_id".to_string(), "kowalski_key_id".to_string()],
//!     },
//!     co_signers: vec!["martinez_key_id".to_string()],
//!     behavioral: BehavioralConstraint::default(),
//! };
//!
//! // Key is created with cryptographically-signed constraints
//! let key = BearDogGenetics::generate_with_constraints(
//!     &entropy,
//!     constraints,
//!     vec![],
//! )?;
//!
//! // Later: try to delete raw data
//! let operation = KeyOperation::Delete {
//!     path: "raw_data/2020-temperature.nc".to_string(),
//! };
//!
//! // This will FAIL because constraint forbids deleting raw_data/*
//! match key.verify_operation(&operation) {
//!     Ok(_) => { /* operation allowed */ },
//!     Err(e) => {
//!         // ConstraintViolation: Cannot delete path matching 'raw_data/*'
//!         assert!(matches!(e, ConstraintViolationError::DataAccessDenied { .. }));
//!     }
//! }
//! ```

pub mod enforcement;
pub mod evolution;
pub mod types;

pub use enforcement::{ConstraintEnforcementPolicy, ConstraintEnforcer, ConstraintViolationError};
pub use evolution::{ConstraintEvolutionEngine, EvolutionTrigger};
pub use types::{
    BehavioralConstraint, DataAccessConstraint, KeyConstraints, KeyOperation, LifetimeConstraint,
    ScopeConstraint, SignedConstraints,
};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod enforcement_comprehensive_tests;
