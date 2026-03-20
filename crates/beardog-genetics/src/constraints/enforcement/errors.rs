// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Constraint violation errors

use crate::constraints::types::OperationType;
use beardog_errors::BearDogError;
use std::fmt;

/// Errors that occur when constraints are violated
#[derive(Debug, Clone)]
pub enum ConstraintViolationError {
    /// Signature verification failed (constraints may be tampered)
    SignatureVerificationFailed {
        /// Why the embedded constraint signature did not verify.
        reason: String,
    },

    /// Key has expired
    KeyExpired {
        /// Wall-clock instant after which the key must not be used.
        expired_at: chrono::DateTime<chrono::Utc>,
    },

    /// Use count exceeded
    UseCountExceeded {
        /// Maximum operations permitted under the constraint bundle.
        max_uses: u64,
        /// Operations already recorded against the key.
        current_uses: u64,
    },

    /// Operation violates scope constraint
    ScopeViolation {
        /// Domains or scopes explicitly allowed by policy.
        allowed: Vec<String>,
        /// Domain or scope the caller attempted to use.
        attempted: String,
    },

    /// Operation in forbidden domain
    ForbiddenDomain {
        /// Label of the forbidden domain (e.g. capability realm).
        domain: String,
    },

    /// Operation type not allowed
    OperationNotAllowed {
        /// Operation the caller requested.
        operation: OperationType,
        /// Operation types still permitted.
        allowed: Vec<OperationType>,
    },

    /// Data access denied
    DataAccessDenied {
        /// High-level operation name (e.g. read, delete).
        operation: String,
        /// Resource path or identifier that was denied.
        path: String,
        /// Policy explanation for integrators.
        reason: String,
    },

    /// Co-signer requirement not met
    CoSignerRequired {
        /// Identifiers of principals that must co-sign.
        required: Vec<String>,
        /// Principals that actually participated in the attempt.
        present: Vec<String>,
    },

    /// Behavioral requirement not met
    BehavioralRequirementNotMet {
        /// Description of the unmet behavioral gate (e.g. biometric step-up).
        requirement: String,
    },
}

impl fmt::Display for ConstraintViolationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SignatureVerificationFailed { reason } => {
                write!(f, "🔒 Constraint signature verification failed: {reason}")
            }
            Self::KeyExpired { expired_at } => {
                write!(f, "⏰ Key expired at {expired_at}")
            }
            Self::UseCountExceeded {
                max_uses,
                current_uses,
            } => {
                write!(f, "🔢 Use count exceeded: {current_uses}/{max_uses} uses")
            }
            Self::ScopeViolation { allowed, attempted } => {
                write!(
                    f,
                    "🚫 Scope violation: attempted '{attempted}', allowed: {allowed:?}"
                )
            }
            Self::ForbiddenDomain { domain } => {
                write!(f, "🚫 Operation in forbidden domain: '{domain}'")
            }
            Self::OperationNotAllowed { operation, allowed } => {
                write!(
                    f,
                    "🚫 Operation '{operation:?}' not allowed. Allowed: {allowed:?}"
                )
            }
            Self::DataAccessDenied {
                operation,
                path,
                reason,
            } => {
                write!(
                    f,
                    "🚫 Data access denied: {operation} on '{path}' - {reason}"
                )
            }
            Self::CoSignerRequired { required, present } => {
                write!(
                    f,
                    "🚫 Co-signer required. Required: {required:?}, Present: {present:?}"
                )
            }
            Self::BehavioralRequirementNotMet { requirement } => {
                write!(f, "🚫 Behavioral requirement not met: {requirement}")
            }
        }
    }
}

impl std::error::Error for ConstraintViolationError {}

impl From<ConstraintViolationError> for BearDogError {
    fn from(err: ConstraintViolationError) -> Self {
        Self::security(format!("Constraint violation: {err}"))
    }
}
