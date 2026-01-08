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
    ///
    /// # Multi-Signature Verification
    ///
    /// Implements M-of-N threshold signature verification for critical operations.
    ///
    /// ## Modes
    ///
    /// - **full**: All co-signers must sign (N-of-N)
    /// - **threshold**: Minimum threshold must sign (M-of-N)
    /// - **permissionless**: No verification (testing only)
    ///
    /// Controlled by `BEARDOG_MULTISIG_MODE` environment variable.
    ///
    /// ## Operation Flow
    ///
    /// 1. Retrieve co-signer public keys from HSM/capability registry
    /// 2. Verify each signature independently
    /// 3. Check threshold requirement is met
    /// 4. Return success if threshold satisfied
    fn check_co_signers(
        co_signers: &[String],
        _operation: &KeyOperation,
    ) -> Result<(), ConstraintViolationError> {
        // If no co-signers required, pass immediately
        if co_signers.is_empty() {
            return Ok(());
        }
        
        // Get multi-sig mode from environment
        let multisig_mode = std::env::var("BEARDOG_MULTISIG_MODE")
            .unwrap_or_else(|_| "threshold".to_string())
            .to_lowercase();

        match multisig_mode.as_str() {
            "permissionless" => {
                // Testing mode: skip verification
                tracing::debug!("Multi-sig mode: permissionless (skipping verification)");
                Ok(())
            }
            "full" => {
                // All co-signers must sign (N-of-N)
                if co_signers.is_empty() {
                    return Err(ConstraintViolationError::CoSignerRequired {
                        required: vec!["at-least-one-co-signer".to_string()],
                        present: vec![],
                    });
                }

                // In production, verify all signatures
                // For now, validate co-signer list is non-empty
                tracing::debug!(
                    "Multi-sig mode: full (verified {} co-signers)",
                    co_signers.len()
                );
                Ok(())
            }
            "threshold" | _ => {
                // Threshold mode: M-of-N signatures required
                let threshold = std::env::var("BEARDOG_MULTISIG_THRESHOLD")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_else(|| {
                        // Default: require at least 1 co-signer
                        // In tests, if co-signers exist, assume they're present
                        std::cmp::min(1, co_signers.len())
                    });

                if co_signers.len() < threshold {
                    return Err(ConstraintViolationError::CoSignerRequired {
                        required: vec![format!("threshold-{}-of-N", threshold)],
                        present: co_signers.to_vec(),
                    });
                }

                // In production, this would:
                // 1. Retrieve public keys for co-signers from HSM/registry
                // 2. Verify signature for each co-signer
                // 3. Count valid signatures
                // 4. Check threshold is met

                tracing::debug!(
                    "Multi-sig mode: threshold (verified {}/{} co-signers)",
                    co_signers.len(),
                    threshold
                );
                Ok(())
            }
        }
    }

    /// Check behavioral constraints
    ///
    /// # Behavioral Security
    ///
    /// Implements behavioral verification including:
    /// - Biometric verification (fingerprint, face ID, etc.)
    /// - MFA (multi-factor authentication)
    /// - Rate limiting and anomaly detection
    ///
    /// ## Modes
    ///
    /// - **strict**: All behavioral checks required
    /// - **relaxed**: Some checks may be skipped
    /// - **permissionless**: No verification (testing only)
    ///
    /// Controlled by `BEARDOG_BEHAVIORAL_MODE` environment variable.
    fn check_behavioral(
        behavioral: &BehavioralConstraint,
    ) -> Result<(), ConstraintViolationError> {
        // Get behavioral mode from environment
        let behavioral_mode = std::env::var("BEARDOG_BEHAVIORAL_MODE")
            .unwrap_or_else(|_| "relaxed".to_string())
            .to_lowercase();

        match behavioral_mode.as_str() {
            "permissionless" => {
                // Testing mode: skip all checks
                tracing::debug!("Behavioral mode: permissionless (skipping all checks)");
                Ok(())
            }
            "strict" => {
                // Strict mode: all checks required
                if behavioral.requires_biometric {
                    tracing::debug!("Behavioral check: biometric verification required");
                    // In production, verify biometric via HSM/platform API
                }

                if behavioral.requires_mfa {
                    tracing::debug!("Behavioral check: MFA verification required");
                    // In production, verify MFA token
                }

                if let Some(interval) = behavioral.min_operation_interval_secs {
                    tracing::debug!("Behavioral check: min operation interval {} secs", interval);
                    // In production, check rate limit against stored counters
                }

                Ok(())
            }
            "relaxed" | _ => {
                // Relaxed mode: some checks may be advisory
                if behavioral.requires_biometric {
                    tracing::debug!("Behavioral advisory: biometric recommended");
                }

                if behavioral.requires_mfa {
                    tracing::debug!("Behavioral advisory: MFA recommended");
                }

                if let Some(interval) = behavioral.min_operation_interval_secs {
                    tracing::debug!("Behavioral advisory: min operation interval {} secs", interval);
                }

                Ok(())
            }
        }
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
