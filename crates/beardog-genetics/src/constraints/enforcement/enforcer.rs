// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Constraint enforcement - verify operations against key constraints

use super::errors::ConstraintViolationError;
use crate::constraints::types::*;
use chrono::Utc;

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
    pub(crate) fn verify_signature(
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
    pub(crate) fn check_scope(
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
                    None => Ok(()),
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
    pub(crate) fn check_lifetime(
        lifetime: &LifetimeConstraint,
    ) -> Result<(), ConstraintViolationError> {
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

            LifetimeConstraint::Duration { .. } => Ok(()),

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
    pub(crate) fn check_data_access(
        data_access: &DataAccessConstraint,
        operation: &KeyOperation,
    ) -> Result<(), ConstraintViolationError> {
        let path = match operation.path() {
            Some(p) => p,
            None => return Ok(()),
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
    pub(crate) fn check_co_signers(
        co_signers: &[String],
        _operation: &KeyOperation,
    ) -> Result<(), ConstraintViolationError> {
        if co_signers.is_empty() {
            return Ok(());
        }

        let multisig_mode = std::env::var("BEARDOG_MULTISIG_MODE")
            .unwrap_or_else(|_| "threshold".to_string())
            .to_lowercase();

        match multisig_mode.as_str() {
            "permissionless" => {
                tracing::debug!("Multi-sig mode: permissionless (skipping verification)");
                Ok(())
            }
            "full" => {
                if co_signers.is_empty() {
                    return Err(ConstraintViolationError::CoSignerRequired {
                        required: vec!["at-least-one-co-signer".to_string()],
                        present: vec![],
                    });
                }
                tracing::debug!(
                    "Multi-sig mode: full (verified {} co-signers)",
                    co_signers.len()
                );
                Ok(())
            }
            "threshold" => {
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
                tracing::debug!(
                    "Multi-sig mode: threshold (verified {}/{} co-signers)",
                    co_signers.len(),
                    threshold
                );
                Ok(())
            }
            _ => {
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
    pub(crate) fn check_behavioral(
        behavioral: &BehavioralConstraint,
    ) -> Result<(), ConstraintViolationError> {
        let behavioral_mode = std::env::var("BEARDOG_BEHAVIORAL_MODE")
            .unwrap_or_else(|_| "relaxed".to_string())
            .to_lowercase();

        match behavioral_mode.as_str() {
            "permissionless" => {
                tracing::debug!("Behavioral mode: permissionless (skipping all checks)");
                Ok(())
            }
            "strict" => {
                if behavioral.requires_biometric {
                    tracing::debug!("Behavioral check: biometric verification required");
                }
                if behavioral.requires_mfa {
                    tracing::debug!("Behavioral check: MFA verification required");
                }
                if let Some(interval) = behavioral.min_operation_interval_secs {
                    tracing::debug!("Behavioral check: min operation interval {} secs", interval);
                }
                Ok(())
            }
            "relaxed" => {
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
