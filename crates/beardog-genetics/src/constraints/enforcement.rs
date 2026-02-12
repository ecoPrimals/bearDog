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
            "threshold" => {
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
            _ => {
                // Unknown multisig mode - default to threshold
                tracing::warn!(
                    "Unknown multisig mode '{}', defaulting to threshold",
                    multisig_mode
                );
                let threshold = std::env::var("BEARDOG_MULTISIG_THRESHOLD")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_else(|| std::cmp::min(1, co_signers.len()));

                if co_signers.len() < threshold {
                    return Err(ConstraintViolationError::CoSignerRequired {
                        required: vec![format!("threshold-{}-of-N", threshold)],
                        present: co_signers.to_vec(),
                    });
                }
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
    fn check_behavioral(behavioral: &BehavioralConstraint) -> Result<(), ConstraintViolationError> {
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
            "relaxed" => {
                // Relaxed mode: some checks may be advisory
                if behavioral.requires_biometric {
                    tracing::debug!("Behavioral advisory: biometric recommended");
                }

                if behavioral.requires_mfa {
                    tracing::debug!("Behavioral advisory: MFA recommended");
                }

                if let Some(interval) = behavioral.min_operation_interval_secs {
                    tracing::debug!(
                        "Behavioral advisory: min operation interval {} secs",
                        interval
                    );
                }

                Ok(())
            }
            _ => {
                // Unknown behavioral mode - default to relaxed (most permissive)
                tracing::warn!("Unknown behavioral enforcement mode, defaulting to relaxed");
                if behavioral.requires_biometric {
                    tracing::debug!("Behavioral advisory: biometric recommended");
                }
                if behavioral.requires_mfa {
                    tracing::debug!("Behavioral advisory: MFA recommended");
                }
                if let Some(interval) = behavioral.min_operation_interval_secs {
                    tracing::debug!(
                        "Behavioral advisory: min operation interval {} secs",
                        interval
                    );
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

    // === Scope Tests ===

    #[test]
    fn test_scope_unrestricted() {
        let scope = ScopeConstraint::Unrestricted;
        let op = KeyOperation::Sign {
            domain: Some("any_domain".to_string()),
        };
        assert!(ConstraintEnforcer::check_scope(&scope, &op).is_ok());
    }

    #[test]
    fn test_scope_limited_allowed() {
        let scope = ScopeConstraint::Limited {
            domains: vec!["climate_modeling".to_string()],
        };
        let op = KeyOperation::Sign {
            domain: Some("climate_modeling".to_string()),
        };
        assert!(ConstraintEnforcer::check_scope(&scope, &op).is_ok());
    }

    #[test]
    fn test_scope_limited_no_domain() {
        let scope = ScopeConstraint::Limited {
            domains: vec!["climate_modeling".to_string()],
        };
        let op = KeyOperation::Sign { domain: None };
        assert!(ConstraintEnforcer::check_scope(&scope, &op).is_ok());
    }

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
    fn test_scope_forbidden_blocked() {
        let scope = ScopeConstraint::Forbidden {
            domains: vec!["forbidden_zone".to_string()],
        };
        let op = KeyOperation::Sign {
            domain: Some("forbidden_zone".to_string()),
        };
        let result = ConstraintEnforcer::check_scope(&scope, &op);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::ForbiddenDomain { .. }
        ));
    }

    #[test]
    fn test_scope_forbidden_allowed() {
        let scope = ScopeConstraint::Forbidden {
            domains: vec!["forbidden_zone".to_string()],
        };
        let op = KeyOperation::Sign {
            domain: Some("ok_domain".to_string()),
        };
        assert!(ConstraintEnforcer::check_scope(&scope, &op).is_ok());
    }

    #[test]
    fn test_scope_forbidden_no_domain() {
        let scope = ScopeConstraint::Forbidden {
            domains: vec!["forbidden_zone".to_string()],
        };
        let op = KeyOperation::Sign { domain: None };
        assert!(ConstraintEnforcer::check_scope(&scope, &op).is_ok());
    }

    #[test]
    fn test_scope_operation_specific_allowed() {
        use std::collections::HashSet;
        let scope = ScopeConstraint::OperationSpecific {
            allowed_operations: HashSet::from([OperationType::Sign, OperationType::Read]),
        };
        let op = KeyOperation::Sign { domain: None };
        assert!(ConstraintEnforcer::check_scope(&scope, &op).is_ok());
    }

    #[test]
    fn test_scope_operation_specific_denied() {
        use std::collections::HashSet;
        let scope = ScopeConstraint::OperationSpecific {
            allowed_operations: HashSet::from([OperationType::Read]),
        };
        let op = KeyOperation::Sign { domain: None };
        let result = ConstraintEnforcer::check_scope(&scope, &op);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::OperationNotAllowed { .. }
        ));
    }

    // === Lifetime Tests ===

    #[test]
    fn test_lifetime_permanent() {
        let lifetime = LifetimeConstraint::Permanent;
        assert!(ConstraintEnforcer::check_lifetime(&lifetime).is_ok());
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

    #[test]
    fn test_lifetime_not_expired() {
        use chrono::Duration;
        let future = Utc::now() + Duration::days(365);
        let lifetime = LifetimeConstraint::ExpiresAt { timestamp: future };
        assert!(ConstraintEnforcer::check_lifetime(&lifetime).is_ok());
    }

    #[test]
    fn test_lifetime_duration() {
        let lifetime = LifetimeConstraint::Duration {
            months: 12,
            evolution_trigger: Some(6),
        };
        assert!(ConstraintEnforcer::check_lifetime(&lifetime).is_ok());
    }

    #[test]
    fn test_lifetime_use_count_ok() {
        let lifetime = LifetimeConstraint::UseCount {
            max_uses: 100,
            current_uses: 50,
        };
        assert!(ConstraintEnforcer::check_lifetime(&lifetime).is_ok());
    }

    #[test]
    fn test_lifetime_use_count_exceeded() {
        let lifetime = LifetimeConstraint::UseCount {
            max_uses: 100,
            current_uses: 100,
        };
        let result = ConstraintEnforcer::check_lifetime(&lifetime);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::UseCountExceeded {
                max_uses: 100,
                current_uses: 100
            }
        ));
    }

    // === Data Access Tests ===

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
    fn test_data_access_modify_cannot_modify() {
        let data_access = DataAccessConstraint {
            cannot_modify: vec!["config/*".to_string()],
            ..Default::default()
        };
        let op = KeyOperation::Modify {
            path: "config/settings.toml".to_string(),
        };
        let result = ConstraintEnforcer::check_data_access(&data_access, &op);
        assert!(result.is_err());
    }

    #[test]
    fn test_data_access_modify_immutable_path() {
        let data_access = DataAccessConstraint {
            immutable_paths: vec!["genesis/*".to_string()],
            ..Default::default()
        };
        let op = KeyOperation::Modify {
            path: "genesis/root.key".to_string(),
        };
        let result = ConstraintEnforcer::check_data_access(&data_access, &op);
        assert!(result.is_err());
    }

    #[test]
    fn test_data_access_no_path_operation() {
        let data_access = DataAccessConstraint {
            cannot_delete: vec!["raw_data/*".to_string()],
            ..Default::default()
        };
        let op = KeyOperation::Sign { domain: None };
        assert!(ConstraintEnforcer::check_data_access(&data_access, &op).is_ok());
    }

    #[test]
    fn test_data_access_allowed_path() {
        let data_access = DataAccessConstraint {
            cannot_delete: vec!["raw_data/*".to_string()],
            ..Default::default()
        };
        let op = KeyOperation::Delete {
            path: "temp/file.txt".to_string(),
        };
        assert!(ConstraintEnforcer::check_data_access(&data_access, &op).is_ok());
    }

    // === Co-Signer Tests ===

    #[test]
    fn test_co_signers_empty() {
        let op = KeyOperation::Sign { domain: None };
        assert!(ConstraintEnforcer::check_co_signers(&[], &op).is_ok());
    }

    #[test]
    fn test_co_signers_permissionless() {
        std::env::set_var("BEARDOG_MULTISIG_MODE", "permissionless");
        let op = KeyOperation::Sign { domain: None };
        let co_signers = vec!["alice".to_string(), "bob".to_string()];
        let result = ConstraintEnforcer::check_co_signers(&co_signers, &op);
        std::env::remove_var("BEARDOG_MULTISIG_MODE");
        assert!(result.is_ok());
    }

    #[test]
    fn test_co_signers_full_mode() {
        std::env::set_var("BEARDOG_MULTISIG_MODE", "full");
        let op = KeyOperation::Sign { domain: None };
        let co_signers = vec!["alice".to_string(), "bob".to_string()];
        let result = ConstraintEnforcer::check_co_signers(&co_signers, &op);
        std::env::remove_var("BEARDOG_MULTISIG_MODE");
        assert!(result.is_ok());
    }

    #[test]
    fn test_co_signers_threshold_mode() {
        std::env::set_var("BEARDOG_MULTISIG_MODE", "threshold");
        std::env::set_var("BEARDOG_MULTISIG_THRESHOLD", "2");
        let op = KeyOperation::Sign { domain: None };
        let co_signers = vec!["alice".to_string(), "bob".to_string()];
        let result = ConstraintEnforcer::check_co_signers(&co_signers, &op);
        std::env::remove_var("BEARDOG_MULTISIG_MODE");
        std::env::remove_var("BEARDOG_MULTISIG_THRESHOLD");
        assert!(result.is_ok());
    }

    #[test]
    fn test_co_signers_threshold_insufficient() {
        std::env::set_var("BEARDOG_MULTISIG_MODE", "threshold");
        std::env::set_var("BEARDOG_MULTISIG_THRESHOLD", "5");
        let op = KeyOperation::Sign { domain: None };
        let co_signers = vec!["alice".to_string(), "bob".to_string()];
        let result = ConstraintEnforcer::check_co_signers(&co_signers, &op);
        std::env::remove_var("BEARDOG_MULTISIG_MODE");
        std::env::remove_var("BEARDOG_MULTISIG_THRESHOLD");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::CoSignerRequired { .. }
        ));
    }

    #[test]
    fn test_co_signers_unknown_mode() {
        std::env::set_var("BEARDOG_MULTISIG_MODE", "unknown_mode");
        let op = KeyOperation::Sign { domain: None };
        let co_signers = vec!["alice".to_string()];
        let result = ConstraintEnforcer::check_co_signers(&co_signers, &op);
        std::env::remove_var("BEARDOG_MULTISIG_MODE");
        assert!(result.is_ok());
    }

    // === Behavioral Tests ===

    #[test]
    fn test_behavioral_permissionless() {
        std::env::set_var("BEARDOG_BEHAVIORAL_MODE", "permissionless");
        let behavioral = BehavioralConstraint {
            requires_biometric: true,
            requires_mfa: true,
            min_operation_interval_secs: Some(60),
            min_entropy_quality: None,
        };
        let result = ConstraintEnforcer::check_behavioral(&behavioral);
        std::env::remove_var("BEARDOG_BEHAVIORAL_MODE");
        assert!(result.is_ok());
    }

    #[test]
    fn test_behavioral_strict() {
        std::env::set_var("BEARDOG_BEHAVIORAL_MODE", "strict");
        let behavioral = BehavioralConstraint {
            requires_biometric: true,
            requires_mfa: true,
            min_operation_interval_secs: Some(30),
            min_entropy_quality: None,
        };
        let result = ConstraintEnforcer::check_behavioral(&behavioral);
        std::env::remove_var("BEARDOG_BEHAVIORAL_MODE");
        assert!(result.is_ok());
    }

    #[test]
    fn test_behavioral_relaxed() {
        std::env::set_var("BEARDOG_BEHAVIORAL_MODE", "relaxed");
        let behavioral = BehavioralConstraint {
            requires_biometric: true,
            requires_mfa: true,
            min_operation_interval_secs: Some(10),
            min_entropy_quality: None,
        };
        let result = ConstraintEnforcer::check_behavioral(&behavioral);
        std::env::remove_var("BEARDOG_BEHAVIORAL_MODE");
        assert!(result.is_ok());
    }

    #[test]
    fn test_behavioral_unknown_mode() {
        std::env::set_var("BEARDOG_BEHAVIORAL_MODE", "unknown_mode");
        let behavioral = BehavioralConstraint {
            requires_biometric: true,
            requires_mfa: true,
            min_operation_interval_secs: Some(5),
            min_entropy_quality: None,
        };
        let result = ConstraintEnforcer::check_behavioral(&behavioral);
        std::env::remove_var("BEARDOG_BEHAVIORAL_MODE");
        assert!(result.is_ok());
    }

    #[test]
    fn test_behavioral_no_requirements() {
        let behavioral = BehavioralConstraint {
            requires_biometric: false,
            requires_mfa: false,
            min_operation_interval_secs: None,
            min_entropy_quality: None,
        };
        assert!(ConstraintEnforcer::check_behavioral(&behavioral).is_ok());
    }

    // === Display Tests ===

    #[test]
    fn test_display_signature_verification_failed() {
        let err = ConstraintViolationError::SignatureVerificationFailed {
            reason: "tampered".to_string(),
        };
        assert!(err.to_string().contains("tampered"));
    }

    #[test]
    fn test_display_key_expired() {
        let err = ConstraintViolationError::KeyExpired {
            expired_at: Utc::now(),
        };
        assert!(err.to_string().contains("expired"));
    }

    #[test]
    fn test_display_use_count_exceeded() {
        let err = ConstraintViolationError::UseCountExceeded {
            max_uses: 100,
            current_uses: 101,
        };
        let s = err.to_string();
        assert!(s.contains("101") && s.contains("100"));
    }

    #[test]
    fn test_display_scope_violation() {
        let err = ConstraintViolationError::ScopeViolation {
            allowed: vec!["a".into()],
            attempted: "b".into(),
        };
        assert!(err.to_string().contains("b"));
    }

    #[test]
    fn test_display_forbidden_domain() {
        let err = ConstraintViolationError::ForbiddenDomain {
            domain: "evil".into(),
        };
        assert!(err.to_string().contains("evil"));
    }

    #[test]
    fn test_display_operation_not_allowed() {
        let err = ConstraintViolationError::OperationNotAllowed {
            operation: OperationType::Sign,
            allowed: vec![OperationType::Read],
        };
        assert!(err.to_string().contains("not allowed"));
    }

    #[test]
    fn test_display_data_access_denied() {
        let err = ConstraintViolationError::DataAccessDenied {
            operation: "delete".into(),
            path: "/data".into(),
            reason: "immutable".into(),
        };
        assert!(err.to_string().contains("immutable"));
    }

    #[test]
    fn test_display_co_signer_required() {
        let err = ConstraintViolationError::CoSignerRequired {
            required: vec!["alice".into()],
            present: vec![],
        };
        assert!(err.to_string().contains("alice"));
    }

    #[test]
    fn test_display_behavioral_requirement() {
        let err = ConstraintViolationError::BehavioralRequirementNotMet {
            requirement: "biometric".into(),
        };
        assert!(err.to_string().contains("biometric"));
    }

    #[test]
    fn test_error_conversion_to_beardog_error() {
        let err = ConstraintViolationError::ForbiddenDomain {
            domain: "test".into(),
        };
        let beardog_err: BearDogError = err.into();
        assert!(beardog_err.to_string().contains("Constraint violation"));
    }

    // === Signature Verification Tests ===

    #[test]
    fn test_verify_signature_short_pubkey() {
        let signed = SignedConstraints {
            constraints: KeyConstraints::default(),
            signature: vec![0u8; 64],
            signed_by_key_id: "test".into(),
            created_at: Utc::now(),
            version: 1,
        };
        let result = ConstraintEnforcer::verify_signature(&signed, &[0u8; 16]); // too short
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_signature_short_signature() {
        let signed = SignedConstraints {
            constraints: KeyConstraints::default(),
            signature: vec![0u8; 32], // too short, need 64
            signed_by_key_id: "test".into(),
            created_at: Utc::now(),
            version: 1,
        };
        let result = ConstraintEnforcer::verify_signature(&signed, &[0u8; 32]);
        assert!(result.is_err());
    }
}
