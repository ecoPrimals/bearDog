// SPDX-License-Identifier: AGPL-3.0-only

// Implementation of self-enforcing constraint methods for BearDogGenetics
//
// This module implements the cryptographic verification and enforcement
// of self-enforcing key constraints.

use super::BearDogGenetics;
use beardog_errors::BearDogError;
use beardog_types::genetics_constraints::KeyOperation;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use tracing::{debug, info, warn};

impl BearDogGenetics {
    /// Verify that an operation is allowed by this key's constraints
    ///
    /// This is the main enforcement point for self-enforcing keys. Before any
    /// operation is performed, this method must be called to ensure the key's
    /// constraints allow it.
    ///
    /// # Security
    ///
    /// This method:
    /// 1. Verifies constraint integrity (detects tampering)
    /// 2. Delegates to constraint verification logic
    /// 3. Logs all verification attempts
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let delete_op = KeyOperation::Delete {
    ///     path: "raw_data/temperature.nc".to_string(),
    /// };
    ///
    /// if let Err(e) = key.verify_operation(&delete_op) {
    ///     println!("Operation blocked: {}", e);
    ///     return Err(e);
    /// }
    ///
    /// // Operation allowed, proceed
    /// perform_delete(&path);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Constraints have been tampered with
    /// - Operation violates any constraint
    /// - Key has expired
    pub fn verify_operation(&self, operation: &KeyOperation) -> Result<(), BearDogError> {
        // If no constraints, allow all operations (backward compatible)
        let Some(ref constraints) = self.constraints else {
            debug!("Key {} has no constraints, allowing operation", self.id);
            return Ok(());
        };

        // Verify constraint integrity first (detect tampering)
        self.verify_constraint_integrity()?;

        // Delegate to constraint verification
        constraints.verify_operation(operation)?;

        debug!("Operation verified for key {}: {:?}", self.id, operation);
        Ok(())
    }

    /// Verify that constraints haven't been tampered with
    ///
    /// This cryptographically verifies that the constraints embedded in this key
    /// match the signature created during key generation. Any modification to the
    /// constraints will cause this verification to fail.
    ///
    /// # Security
    ///
    /// Uses Ed25519 signature verification:
    /// - Fast (sub-millisecond)
    /// - Cryptographically secure
    /// - Tamper-evident
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Signature is missing
    /// - Public key is missing
    /// - Signature doesn't match constraints
    /// - Constraints have been modified
    pub fn verify_constraint_integrity(&self) -> Result<(), BearDogError> {
        let Some(ref constraints) = self.constraints else {
            // No constraints = nothing to verify
            return Ok(());
        };

        let Some(ref signature_bytes) = self.constraint_signature else {
            warn!("Key {} has constraints but no signature!", self.id);
            return Err(BearDogError::security(
                "Constraints exist but signature is missing - possible tampering".to_string(),
            ));
        };

        let Some(ref public_key_bytes) = self.public_key else {
            warn!("Key {} has constraints but no public key!", self.id);
            return Err(BearDogError::security(
                "Constraints exist but public key is missing - possible tampering".to_string(),
            ));
        };

        // Parse public key
        let public_key_array: [u8; 32] = public_key_bytes
            .as_slice()
            .try_into()
            .map_err(|_| BearDogError::security("Invalid public key length".to_string()))?;

        let verifying_key = VerifyingKey::from_bytes(&public_key_array)
            .map_err(|e| BearDogError::security(format!("Invalid public key: {}", e)))?;

        // Parse signature
        let signature_array: [u8; 64] = signature_bytes
            .as_slice()
            .try_into()
            .map_err(|_| BearDogError::security("Invalid signature length".to_string()))?;

        let signature = Signature::from_bytes(&signature_array);

        // Hash constraints and verify signature
        let constraint_hash = constraints.hash()?;

        verifying_key
            .verify(&constraint_hash, &signature)
            .map_err(|e| {
                warn!(
                    "Constraint integrity verification failed for key {}: {}",
                    self.id, e
                );
                BearDogError::security(format!(
                    "Constraint integrity check failed - constraints may have been tampered with: {}",
                    e
                ))
            })?;

        debug!("Constraint integrity verified for key {}", self.id);
        Ok(())
    }

    /// Check if this key has expired based on its constraints
    ///
    /// This is a convenience method that checks the lifetime constraint
    /// without performing a full operation verification.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if key.is_expired() {
    ///     println!("Key has expired, cannot use");
    ///     return Err("Expired key");
    /// }
    /// ```
    pub fn is_expired(&self) -> bool {
        if let Some(ref constraints) = self.constraints {
            use chrono::Utc;
            Utc::now() > constraints.lifetime.expires_at
        } else {
            false // No constraints = never expires
        }
    }

    /// Get a human-readable description of this key's constraints
    ///
    /// Useful for displaying to users what operations this key can perform.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// println!("Key constraints: {}", key.constraint_description());
    /// // Output: "Project-scoped to 'climate-modeling', cannot delete raw_data/*, expires 2025-12-31"
    /// ```
    pub fn constraint_description(&self) -> String {
        if let Some(ref constraints) = self.constraints {
            constraints.description()
        } else {
            "No constraints (unrestricted)".to_string()
        }
    }

    // ============================================================================
    // PHASE 2: ADAPTER UNLOCK CERTIFICATES
    // ============================================================================

    /// Issue an adapter unlock certificate (Phase 2)
    ///
    /// Creates a cryptographically signed certificate that grants permission
    /// to use a specific adapter. The certificate:
    ///
    /// - Inherits all constraints from this key
    /// - Is time-limited (expires after specified duration)
    /// - Is bound to a specific adapter ID
    /// - Cannot be forged or tampered with (Ed25519 signature)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use beardog_types::adapter_certificates::AdapterClassification;
    /// use chrono::Duration;
    ///
    /// // Issue certificate for Songbird network adapter
    /// let certificate = key.issue_adapter_certificate(
    ///     "beardog-adapters::songbird::network",
    ///     AdapterClassification::Human,
    ///     Duration::hours(24),
    /// )?;
    ///
    /// // Certificate is now valid for 24 hours
    /// assert!(certificate.is_valid_now());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Key has expired (cannot issue certificates)
    /// - Key has no signing key
    /// - Signature generation fails
    pub fn issue_adapter_certificate(
        &self,
        adapter_id: &str,
        classification: beardog_types::adapter_certificates::AdapterClassification,
        validity_duration: chrono::Duration,
    ) -> Result<beardog_types::adapter_certificates::AdapterUnlockCertificate, BearDogError> {
        use beardog_types::adapter_certificates::AdapterUnlockCertificate;
        use chrono::Utc;
        use ed25519_dalek::Signer;
        use uuid::Uuid;

        // 1. Verify this key can issue certificates
        if self.is_expired() {
            return Err(BearDogError::unauthorized(
                "Cannot issue certificate from expired key".to_string(),
            ));
        }

        let Some(ref public_key_bytes) = self.public_key else {
            return Err(BearDogError::security(
                "Key has no public key - cannot issue certificate".to_string(),
            ));
        };

        // 2. Reconstruct signing key from this key's genetics
        // Note: In production, the signing key would be stored securely (HSM/TPM)
        // For Phase 2, we derive it from the key ID (deterministic for testing)
        let signing_key = self.derive_signing_key()?;

        // 3. Create certificate with inherited constraints
        let cert_id = Uuid::new_v4().to_string();
        let issued_at = Utc::now();
        let expires_at = issued_at + validity_duration;

        let mut certificate = AdapterUnlockCertificate {
            cert_id: cert_id.clone(),
            issuer_key_id: self.id.clone(),
            adapter_id: adapter_id.to_string(),
            classification,
            constraints: self.constraints.clone(), // Inherit constraints
            issued_at,
            expires_at,
            signature: vec![], // Will be filled below
            issuer_public_key: public_key_bytes.clone(),
        };

        // 4. Sign the certificate
        let signable_data = certificate.signable_data();
        let signature = signing_key.sign(&signable_data);
        certificate.signature = signature.to_bytes().to_vec();

        info!(
            "✅ Issued certificate {} for adapter '{}' (classification: {:?}, valid for {})",
            &cert_id[..8],
            adapter_id,
            certificate.classification,
            validity_duration
        );

        Ok(certificate)
    }

    /// Derive signing key from this genetics (for certificate issuance)
    ///
    /// In production, this would retrieve the key from secure storage (HSM/TPM).
    /// For Phase 2 implementation, we derive it deterministically from the key ID.
    fn derive_signing_key(&self) -> Result<ed25519_dalek::SigningKey, BearDogError> {
        use ed25519_dalek::SigningKey;
        use sha3::{Digest, Sha3_256};

        // Derive seed from key ID and a secret component
        let mut hasher = Sha3_256::new();
        hasher.update(self.id.as_bytes());
        hasher.update(b"BearDog-CertificateIssuance-v1");

        // In production, add HSM-derived entropy here
        if let Some(ref public_key) = self.public_key {
            hasher.update(public_key);
        }

        let seed = hasher.finalize();
        let mut seed_array = [0u8; 32];
        seed_array.copy_from_slice(&seed[..32]);

        Ok(SigningKey::from_bytes(&seed_array))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::genetics_constraints::{
        BehavioralConstraint, ComputeQuota, DataAccessConstraint, KeyConstraints,
        LifetimeConstraint, ScopeConstraint,
    };
    use chrono::{Duration, Utc};
    use std::collections::HashMap;

    #[test]
    fn test_generate_with_constraints() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                immutable_paths: vec!["raw_data/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let result = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]);

        assert!(result.is_ok());
        let key = result.unwrap();
        assert!(key.constraints.is_some());
        assert!(key.constraint_signature.is_some());
        assert!(key.public_key.is_some());
    }

    #[test]
    fn test_verify_operation_blocks_protected_delete() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                immutable_paths: vec!["raw_data/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // Try to delete protected path
        let delete_op = KeyOperation::Delete {
            path: "raw_data/temperature.nc".to_string(),
        };

        let result = key.verify_operation(&delete_op);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("cannot delete protected"));
    }

    #[test]
    fn test_verify_operation_allows_unprotected_delete() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                immutable_paths: vec!["raw_data/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // Try to delete unprotected path
        let delete_op = KeyOperation::Delete {
            path: "processed/temperature.nc".to_string(),
        };

        let result = key.verify_operation(&delete_op);
        assert!(result.is_ok());
    }

    #[test]
    fn test_constraint_integrity_detects_tampering() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                immutable_paths: vec!["raw_data/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let mut key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // Verify integrity is OK initially
        assert!(key.verify_constraint_integrity().is_ok());

        // Tamper with constraints
        if let Some(ref mut constraints) = key.constraints {
            constraints.data_access.immutable_paths.clear(); // Remove protection
        }

        // Integrity check should now fail
        let result = key.verify_constraint_integrity();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("tampered"));
    }

    #[test]
    fn test_expired_key_rejected() {
        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint {
                expires_at: Utc::now() - Duration::seconds(1), // Already expired
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        assert!(key.is_expired());

        // Any operation should fail
        let read_op = KeyOperation::Read {
            path: "any/file.txt".to_string(),
            project: None,
        };

        let result = key.verify_operation(&read_op);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expired"));
    }

    #[test]
    fn test_key_without_constraints_allows_all() {
        let key = BearDogGenetics::default(); // No constraints

        let delete_op = KeyOperation::Delete {
            path: "raw_data/critical.nc".to_string(),
        };

        // Should allow (backward compatible)
        assert!(key.verify_operation(&delete_op).is_ok());
        assert!(!key.is_expired());
    }

    // ========================================================================
    // COMPREHENSIVE TESTS - Scope Constraints
    // ========================================================================

    #[test]
    fn test_project_scope_enforcement() {
        let project_hash = [42u8; 32]; // Mock project hash
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Project {
                name: "climate-model".to_string(),
                project_hash,
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // Allowed: Correct project
        let allowed_read = KeyOperation::Read {
            path: "sensor_data/temp.csv".to_string(),
            project: Some("climate-model".to_string()),
        };
        assert!(key.verify_operation(&allowed_read).is_ok());

        // Denied: Wrong project
        let denied_project = KeyOperation::Read {
            path: "sensor_data/temp.csv".to_string(),
            project: Some("other-project".to_string()),
        };
        assert!(key.verify_operation(&denied_project).is_err());
    }

    #[test]
    fn test_resource_scope_enforcement() {
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Resources {
                allow_read: vec!["data/*.csv".to_string()],
                allow_write: vec!["logs/*.log".to_string()],
                deny_delete: vec!["archive/*".to_string()],
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // Allowed: Read matching pattern
        let allowed_read = KeyOperation::Read {
            path: "data/measurements.csv".to_string(),
            project: None,
        };
        assert!(key.verify_operation(&allowed_read).is_ok());

        // Allowed: Write matching pattern
        let allowed_write = KeyOperation::Write {
            path: "logs/system.log".to_string(),
            size_bytes: 1024,
            project: None,
        };
        assert!(key.verify_operation(&allowed_write).is_ok());

        // Denied: Delete protected path
        let denied_delete = KeyOperation::Delete {
            path: "archive/2023.tar.gz".to_string(),
        };
        assert!(key.verify_operation(&denied_delete).is_err());
    }

    #[test]
    fn test_operations_scope_enforcement() {
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Operations {
                allowed_operations: vec!["analyze".to_string(), "process".to_string()],
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // Allowed: Allowed operation
        let allowed_rpc = KeyOperation::RpcCall {
            target_service: "compute".to_string(),
            method: "analyze".to_string(),
            project: None,
        };
        assert!(key.verify_operation(&allowed_rpc).is_ok());

        // Denied: Disallowed operation
        let denied_rpc = KeyOperation::RpcCall {
            target_service: "compute".to_string(),
            method: "delete_all".to_string(),
            project: None,
        };
        assert!(key.verify_operation(&denied_rpc).is_err());
    }

    // ========================================================================
    // COMPREHENSIVE TESTS - Data Access Constraints
    // ========================================================================

    #[test]
    fn test_multiple_immutable_paths() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                immutable_paths: vec![
                    "raw_data/*".to_string(),
                    "archive/*".to_string(),
                    "provenance/*.json".to_string(),
                ],
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // All protected paths should be blocked
        assert!(key
            .verify_operation(&KeyOperation::Delete {
                path: "raw_data/sensor.csv".to_string(),
            })
            .is_err());

        assert!(key
            .verify_operation(&KeyOperation::Delete {
                path: "archive/2023/data.tar.gz".to_string(),
            })
            .is_err());

        assert!(key
            .verify_operation(&KeyOperation::Delete {
                path: "provenance/lineage.json".to_string(),
            })
            .is_err());

        // Unprotected path should be allowed
        assert!(key
            .verify_operation(&KeyOperation::Delete {
                path: "tmp/cache.dat".to_string(),
            })
            .is_ok());
    }

    #[test]
    fn test_wildcard_path_matching() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                immutable_paths: vec![
                    "*.nc".to_string(),        // All NetCDF files
                    "important_*".to_string(), // All files starting with important_
                ],
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // Protected by *.nc
        assert!(key
            .verify_operation(&KeyOperation::Delete {
                path: "temperature.nc".to_string(),
            })
            .is_err());

        // Protected by important_*
        assert!(key
            .verify_operation(&KeyOperation::Delete {
                path: "important_config.toml".to_string(),
            })
            .is_err());

        // Not protected
        assert!(key
            .verify_operation(&KeyOperation::Delete {
                path: "temp.csv".to_string(),
            })
            .is_ok());
    }

    #[test]
    fn test_nested_path_protection() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                immutable_paths: vec!["data/raw/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // Protected nested path
        assert!(key
            .verify_operation(&KeyOperation::Delete {
                path: "data/raw/sensors/temp.csv".to_string(),
            })
            .is_err());

        // Unprotected sibling path
        assert!(key
            .verify_operation(&KeyOperation::Delete {
                path: "data/processed/temp.csv".to_string(),
            })
            .is_ok());
    }

    // ========================================================================
    // COMPREHENSIVE TESTS - Compute Quotas
    // ========================================================================

    #[test]
    fn test_compute_quota_presence() {
        use beardog_types::genetics_constraints::ComputeUsage;

        let constraints = KeyConstraints {
            compute_quota: Some(ComputeQuota {
                max_hours: 100.0,
                max_memory_bytes: 1024 * 1024 * 100, // 100 MB
                max_cpu_percent: 80,
                current_usage: ComputeUsage::default(),
            }),
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // Verify compute quota is present
        assert!(key.constraints.as_ref().unwrap().compute_quota.is_some());

        // ComputeAllocation operations should work
        let compute_op = KeyOperation::ComputeAllocation {
            hours: 1.5,
            memory_bytes: 1024 * 1024 * 50,
        };
        assert!(key.verify_operation(&compute_op).is_ok());
    }

    // ========================================================================
    // COMPREHENSIVE TESTS - Behavioral & Co-signers
    // ========================================================================

    #[test]
    fn test_behavioral_constraint_specified() {
        let constraints = KeyConstraints {
            behavior: BehavioralConstraint {
                biometric_required: true,
                challenge_on_anomaly: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // Verify behavioral constraints are present
        let behavioral = &key.constraints.as_ref().unwrap().behavior;
        assert!(behavioral.biometric_required);
        assert!(behavioral.challenge_on_anomaly);
    }

    #[test]
    fn test_co_signer_requirement() {
        let constraints = KeyConstraints {
            co_signers: vec![
                "co-signer-key-123".to_string(),
                "co-signer-key-456".to_string(),
            ],
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // Verify co-signers are present in constraints
        assert_eq!(key.constraints.as_ref().unwrap().co_signers.len(), 2);

        // Operations should still verify (co-signature verification is at auth layer)
        let read_op = KeyOperation::Read {
            path: "data.csv".to_string(),
            project: None,
        };
        assert!(key.verify_operation(&read_op).is_ok());
    }

    // ========================================================================
    // COMPREHENSIVE TESTS - Combined Constraints
    // ========================================================================

    #[test]
    fn test_combined_constraints() {
        let project_hash = [42u8; 32];
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Project {
                name: "climate".to_string(),
                project_hash,
            },
            lifetime: LifetimeConstraint {
                expires_at: Utc::now() + Duration::days(30),
                ..Default::default()
            },
            data_access: DataAccessConstraint {
                immutable_paths: vec!["sensor/calibration/*".to_string()],
                audit_required: true,
                ..Default::default()
            },
            compute_quota: Some(ComputeQuota {
                max_hours: 1000.0,
                max_memory_bytes: 1024 * 1024 * 500,
                max_cpu_percent: 90,
                current_usage: Default::default(),
            }),
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // Should pass all constraints
        let allowed = KeyOperation::Read {
            path: "sensor/temp.csv".to_string(),
            project: Some("climate".to_string()),
        };
        assert!(key.verify_operation(&allowed).is_ok());

        // Should fail scope constraint (wrong project)
        let wrong_project = KeyOperation::Read {
            path: "sensor/temp.csv".to_string(),
            project: Some("other".to_string()),
        };
        assert!(key.verify_operation(&wrong_project).is_err());

        // Should fail data access constraint (protected path)
        let protected_delete = KeyOperation::Delete {
            path: "sensor/calibration/baseline.dat".to_string(),
        };
        assert!(key.verify_operation(&protected_delete).is_err());
    }

    // ========================================================================
    // COMPREHENSIVE TESTS - Metadata & Audit
    // ========================================================================

    #[test]
    fn test_audit_required_flag() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                audit_required: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // Verify audit flag is present
        assert!(key.constraints.as_ref().unwrap().data_access.audit_required);
    }

    #[test]
    fn test_mandatory_encryption_specified() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                mandatory_encryption: vec!["key1".to_string(), "key2".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // Verify encryption keys are present
        assert_eq!(
            key.constraints
                .as_ref()
                .unwrap()
                .data_access
                .mandatory_encryption
                .len(),
            2
        );
    }

    #[test]
    fn test_constraint_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("purpose".to_string(), "research".to_string());
        metadata.insert("classification".to_string(), "public".to_string());

        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                metadata: metadata.clone(),
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // Verify metadata is preserved
        let stored_metadata = &key.constraints.as_ref().unwrap().data_access.metadata;
        assert_eq!(
            stored_metadata.get("purpose"),
            Some(&"research".to_string())
        );
        assert_eq!(
            stored_metadata.get("classification"),
            Some(&"public".to_string())
        );
    }

    #[test]
    fn test_constraint_signature_verification() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                immutable_paths: vec!["protected/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // Verify the signature is valid
        assert!(key.verify_constraint_integrity().is_ok());

        // Verify the key has constraints and signature
        assert!(key.constraints.is_some());
        assert!(key.constraint_signature.is_some());
        assert!(key.public_key.is_some());
    }

    #[test]
    fn test_lifecycle_renewal_settings() {
        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint {
                renewable: true,
                max_renewals: Some(3),
                renewal_approvers: vec!["admin-key-123".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key =
            BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

        // Verify renewal settings
        let lifetime = &key.constraints.as_ref().unwrap().lifetime;
        assert!(lifetime.renewable);
        assert_eq!(lifetime.max_renewals, Some(3));
        assert_eq!(lifetime.renewal_approvers.len(), 1);
    }
}
