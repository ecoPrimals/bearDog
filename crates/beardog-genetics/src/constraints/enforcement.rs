// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Constraint enforcement - verify operations against key constraints

use super::types::*;
use beardog_errors::BearDogError;
use chrono::Utc;
use std::fmt;

/// Enforces constraints on key operations
pub struct ConstraintEnforcer;

impl ConstraintEnforcer {
    /// Verify an operation against signed constraints
    ///
    /// This is the core enforcement mechanism. Every key operation must pass
    /// through this verification.
    pub fn verify_operation(
        signed_constraints: &SignedConstraints,
        operation: &KeyOperation,
        public_key: &[u8], // For signature verification
    ) -> Result<(), ConstraintViolationError> {
        // 1. Verify signature (detect tampering)
        Self::verify_signature(signed_constraints, public_key)?;

        // 2. Check scope constraints
        Self::check_scope(&signed_constraints.constraints.scope, operation)?;

        // 3. Check lifetime constraints
        Self::check_lifetime(&signed_constraints.constraints.lifetime)?;

        // 4. Check data access constraints
        Self::check_data_access(&signed_constraints.constraints.data_access, operation)?;

        // 5. Check co-signer requirements
        Self::check_co_signers(&signed_constraints.constraints.co_signers, operation)?;

        // 6. Check behavioral constraints
        Self::check_behavioral(&signed_constraints.constraints.behavioral)?;

        Ok(())
    }

    /// Verify cryptographic signature on constraints
    fn verify_signature(
        signed: &SignedConstraints,
        public_key: &[u8],
    ) -> Result<(), ConstraintViolationError> {
        use ed25519_dalek::{Signature, Verifier, VerifyingKey};

        // Compute expected hash
        let hash = SignedConstraints::compute_hash(&signed.constraints);

        // Parse public key (32 bytes)
        let key_bytes: [u8; 32] = public_key
            .get(..32)
            .ok_or_else(|| ConstraintViolationError::SignatureVerificationFailed {
                reason: "Public key must be 32 bytes".to_string(),
            })?
            .try_into()
            .map_err(|_| ConstraintViolationError::SignatureVerificationFailed {
                reason: "Failed to parse public key bytes".to_string(),
            })?;

        let pk = VerifyingKey::from_bytes(&key_bytes).map_err(|e| {
            ConstraintViolationError::SignatureVerificationFailed {
                reason: format!("Invalid public key: {}", e),
            }
        })?;

        // Parse signature (64 bytes)
        let sig_bytes: [u8; 64] = signed
            .signature
            .get(..64)
            .ok_or_else(|| ConstraintViolationError::SignatureVerificationFailed {
                reason: "Signature must be 64 bytes".to_string(),
            })?
            .try_into()
            .map_err(|_| ConstraintViolationError::SignatureVerificationFailed {
                reason: "Failed to parse signature bytes".to_string(),
            })?;

        let sig = Signature::from_bytes(&sig_bytes);

        // Verify
        pk.verify(&hash, &sig).map_err(|e| {
            ConstraintViolationError::SignatureVerificationFailed {
                reason: format!("Signature verification failed: {}", e),
            }
        })?;

        Ok(())
    }

    /// Check scope constraints
    fn check_scope(
        scope: &ScopeConstraint,
        operation: &KeyOperation,
    ) -> Result<(), ConstraintViolationError> {
        match scope {
            ScopeConstraint::Unrestricted => Ok(()),

            ScopeConstraint::Limited { domains } => {
                let op_domain = operation.domain();
                match op_domain {
                    Some(domain) => {
                        if domains.contains(&domain.to_string()) {
                            Ok(())
                        } else {
                            Err(ConstraintViolationError::ScopeViolation {
                                allowed: domains.clone(),
                                attempted: domain.to_string(),
                            })
                        }
                    }
                    None => {
                        // Operation has no domain - check if operation type is allowed
                        Ok(())
                    }
                }
            }

            ScopeConstraint::Forbidden { domains } => {
                let op_domain = operation.domain();
                match op_domain {
                    Some(domain) => {
                        if domains.contains(&domain.to_string()) {
                            Err(ConstraintViolationError::ForbiddenDomain {
                                domain: domain.to_string(),
                            })
                        } else {
                            Ok(())
                        }
                    }
                    None => Ok(()),
                }
            }

            ScopeConstraint::OperationSpecific { allowed_operations } => {
                let op_type = operation.operation_type();
                if allowed_operations.contains(&op_type) {
                    Ok(())
                } else {
                    Err(ConstraintViolationError::OperationNotAllowed {
                        operation: op_type,
                        allowed: allowed_operations.iter().copied().collect(),
                    })
                }
            }
        }
    }

    /// Check lifetime constraints
    fn check_lifetime(lifetime: &LifetimeConstraint) -> Result<(), ConstraintViolationError> {
        match lifetime {
            LifetimeConstraint::Permanent => Ok(()),

            LifetimeConstraint::ExpiresAt { timestamp } => {
                let now = Utc::now();
                if now > *timestamp {
                    Err(ConstraintViolationError::KeyExpired {
                        expired_at: *timestamp,
                    })
                } else {
                    Ok(())
                }
            }

            LifetimeConstraint::Duration { .. } => {
                // Duration is checked against creation time by caller
                Ok(())
            }

            LifetimeConstraint::UseCount {
                max_uses,
                current_uses,
            } => {
                if current_uses >= max_uses {
                    Err(ConstraintViolationError::UseCountExceeded {
                        max_uses: *max_uses,
                        current_uses: *current_uses,
                    })
                } else {
                    Ok(())
                }
            }
        }
    }

    /// Check data access constraints
    fn check_data_access(
        data_access: &DataAccessConstraint,
        operation: &KeyOperation,
    ) -> Result<(), ConstraintViolationError> {
        let path = match operation.path() {
            Some(p) => p,
            None => return Ok(()), // No path, no data access constraints apply
        };

        match operation {
            KeyOperation::Delete { .. } => {
                if DataAccessConstraint::path_matches(path, &data_access.cannot_delete) {
                    return Err(ConstraintViolationError::DataAccessDenied {
                        operation: "delete".to_string(),
                        path: path.to_string(),
                        reason: "Path matches cannot_delete pattern".to_string(),
                    });
                }
            }

            KeyOperation::Modify { .. } => {
                if DataAccessConstraint::path_matches(path, &data_access.cannot_modify) {
                    return Err(ConstraintViolationError::DataAccessDenied {
                        operation: "modify".to_string(),
                        path: path.to_string(),
                        reason: "Path matches cannot_modify pattern".to_string(),
                    });
                }
                if DataAccessConstraint::path_matches(path, &data_access.immutable_paths) {
                    return Err(ConstraintViolationError::DataAccessDenied {
                        operation: "modify".to_string(),
                        path: path.to_string(),
                        reason: "Path is immutable".to_string(),
                    });
                }
            }

            _ => {}
        }

        Ok(())
    }

    /// Check co-signer requirements
    fn check_co_signers(
        _co_signers: &[String],
        _operation: &KeyOperation,
    ) -> Result<(), ConstraintViolationError> {
        // TODO: Implement multi-signature verification
        // This requires coordination with other keys in the network
        Ok(())
    }

    /// Check behavioral constraints
    fn check_behavioral(
        _behavioral: &BehavioralConstraint,
    ) -> Result<(), ConstraintViolationError> {
        // TODO: Implement behavioral checks (biometric, MFA, rate limiting)
        // This requires integration with tunnel/HSM systems
        Ok(())
    }
}

/// Errors that occur when constraints are violated
#[derive(Debug, Clone)]
pub enum ConstraintViolationError {
    /// Signature verification failed (constraints may be tampered)
    SignatureVerificationFailed { reason: String },

    /// Key has expired
    KeyExpired { expired_at: chrono::DateTime<Utc> },

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_violation() {
        let scope = ScopeConstraint::Limited {
            domains: vec!["climate_modeling".to_string()],
        };

        let op = KeyOperation::Sign {
            domain: Some("medical_research".to_string()),
        };

        let result = ConstraintEnforcer::check_scope(&scope, &op);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::ScopeViolation { .. }
        ));
    }

    #[test]
    fn test_data_access_denial() {
        let data_access = DataAccessConstraint {
            cannot_delete: vec!["raw_data/*".to_string()],
            ..Default::default()
        };

        let op = KeyOperation::Delete {
            path: "raw_data/temperature.nc".to_string(),
        };

        let result = ConstraintEnforcer::check_data_access(&data_access, &op);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::DataAccessDenied { .. }
        ));
    }

    #[test]
    fn test_expired_key() {
        use chrono::Duration;

        let past = Utc::now() - Duration::days(1);
        let lifetime = LifetimeConstraint::ExpiresAt { timestamp: past };

        let result = ConstraintEnforcer::check_lifetime(&lifetime);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::KeyExpired { .. }
        ));
    }
}
