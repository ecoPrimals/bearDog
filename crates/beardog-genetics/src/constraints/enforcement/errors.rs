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
    SignatureVerificationFailed { reason: String },

    /// Key has expired
    KeyExpired {
        expired_at: chrono::DateTime<chrono::Utc>,
    },

    /// Use count exceeded
    UseCountExceeded { max_uses: u64, current_uses: u64 },

    /// Operation violates scope constraint
    ScopeViolation {
        allowed: Vec<String>,
        attempted: String,
    },

    /// Operation in forbidden domain
    ForbiddenDomain { domain: String },

    /// Operation type not allowed
    OperationNotAllowed {
        operation: OperationType,
        allowed: Vec<OperationType>,
    },

    /// Data access denied
    DataAccessDenied {
        operation: String,
        path: String,
        reason: String,
    },

    /// Co-signer requirement not met
    CoSignerRequired {
        required: Vec<String>,
        present: Vec<String>,
    },

    /// Behavioral requirement not met
    BehavioralRequirementNotMet { requirement: String },
}

impl fmt::Display for ConstraintViolationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SignatureVerificationFailed { reason } => {
                write!(f, "🔒 Constraint signature verification failed: {}", reason)
            }
            Self::KeyExpired { expired_at } => {
                write!(f, "⏰ Key expired at {}", expired_at)
            }
            Self::UseCountExceeded {
                max_uses,
                current_uses,
            } => {
                write!(
                    f,
                    "🔢 Use count exceeded: {}/{} uses",
                    current_uses, max_uses
                )
            }
            Self::ScopeViolation { allowed, attempted } => {
                write!(
                    f,
                    "🚫 Scope violation: attempted '{}', allowed: {:?}",
                    attempted, allowed
                )
            }
            Self::ForbiddenDomain { domain } => {
                write!(f, "🚫 Operation in forbidden domain: '{}'", domain)
            }
            Self::OperationNotAllowed { operation, allowed } => {
                write!(
                    f,
                    "🚫 Operation '{:?}' not allowed. Allowed: {:?}",
                    operation, allowed
                )
            }
            Self::DataAccessDenied {
                operation,
                path,
                reason,
            } => {
                write!(
                    f,
                    "🚫 Data access denied: {} on '{}' - {}",
                    operation, path, reason
                )
            }
            Self::CoSignerRequired { required, present } => {
                write!(
                    f,
                    "🚫 Co-signer required. Required: {:?}, Present: {:?}",
                    required, present
                )
            }
            Self::BehavioralRequirementNotMet { requirement } => {
                write!(f, "🚫 Behavioral requirement not met: {}", requirement)
            }
        }
    }
}

impl std::error::Error for ConstraintViolationError {}

impl From<ConstraintViolationError> for BearDogError {
    fn from(err: ConstraintViolationError) -> Self {
        BearDogError::security(format!("Constraint violation: {}", err))
    }
}
