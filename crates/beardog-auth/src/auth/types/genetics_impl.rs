// SPDX-License-Identifier: AGPL-3.0-or-later

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
            .map_err(|e| BearDogError::security(format!("Invalid public key: {e}")))?;

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
                    "Constraint integrity check failed - constraints may have been tampered with: {e}"
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
    ///     "beardog-adapters::{discovered_adapter}::network",
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
#[path = "genetics_impl_tests.rs"]
mod tests;
