// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Constraint violation errors

use crate::constraints::types::OperationType;
use beardog_errors::BearDogError;

/// Errors that occur when constraints are violated
#[derive(Debug, Clone, thiserror::Error)]
pub enum ConstraintViolationError {
    /// Signature verification failed (constraints may be tampered)
    #[error("🔒 Constraint signature verification failed: {reason}")]
    SignatureVerificationFailed {
        /// Why the embedded constraint signature did not verify.
        reason: String,
    },

    /// Key has expired
    #[error("⏰ Key expired at {expired_at}")]
    KeyExpired {
        /// Wall-clock instant after which the key must not be used.
        expired_at: chrono::DateTime<chrono::Utc>,
    },

    /// Use count exceeded
    #[error("🔢 Use count exceeded: {current_uses}/{max_uses} uses")]
    UseCountExceeded {
        /// Maximum operations permitted under the constraint bundle.
        max_uses: u64,
        /// Operations already recorded against the key.
        current_uses: u64,
    },

    /// Operation violates scope constraint
    #[error("🚫 Scope violation: attempted '{attempted}', allowed: {allowed:?}")]
    ScopeViolation {
        /// Domains or scopes explicitly allowed by policy.
        allowed: Vec<String>,
        /// Domain or scope the caller attempted to use.
        attempted: String,
    },

    /// Operation in forbidden domain
    #[error("🚫 Operation in forbidden domain: '{domain}'")]
    ForbiddenDomain {
        /// Label of the forbidden domain (e.g. capability realm).
        domain: String,
    },

    /// Operation type not allowed
    #[error("🚫 Operation '{operation:?}' not allowed. Allowed: {allowed:?}")]
    OperationNotAllowed {
        /// Operation the caller requested.
        operation: OperationType,
        /// Operation types still permitted.
        allowed: Vec<OperationType>,
    },

    /// Data access denied
    #[error("🚫 Data access denied: {operation} on '{path}' - {reason}")]
    DataAccessDenied {
        /// High-level operation name (e.g. read, delete).
        operation: String,
        /// Resource path or identifier that was denied.
        path: String,
        /// Policy explanation for integrators.
        reason: String,
    },

    /// Co-signer requirement not met
    #[error("🚫 Co-signer required. Required: {required:?}, Present: {present:?}")]
    CoSignerRequired {
        /// Identifiers of principals that must co-sign.
        required: Vec<String>,
        /// Principals that actually participated in the attempt.
        present: Vec<String>,
    },

    /// Behavioral requirement not met
    #[error("🚫 Behavioral requirement not met: {requirement}")]
    BehavioralRequirementNotMet {
        /// Description of the unmet behavioral gate (e.g. biometric step-up).
        requirement: String,
    },
}

impl From<ConstraintViolationError> for BearDogError {
    fn from(err: ConstraintViolationError) -> Self {
        Self::security(format!("Constraint violation: {err}"))
    }
}
