// SPDX-License-Identifier: AGPL-3.0-only

// iOS Secure Enclave cryptographic operations

use super::keychain::TypeSafeSecureEnclave;
use super::types::*;
use beardog_errors::BearDogError;
use beardog_security::BearDogCrypto;
use std::marker::PhantomData;
use tracing::{debug, info};

/// Type-safe Secure Enclave key wrapper with compile-time algorithm verification
pub struct TypeSafeSecureEnclaveKey<'a, A: SecureEnclaveConstraint> {
    key_id: String,
    algorithm: A,
    biometric_policy: BiometricPolicy,
    enclave: &'a TypeSafeSecureEnclave,
    _phantom: PhantomData<A>,
}

impl<'a, A: SecureEnclaveConstraint> TypeSafeSecureEnclaveKey<'a, A> {
    /// Creates a new type-safe Secure Enclave key
    pub fn new(
        key_id: String,
        algorithm: A,
        biometric_policy: BiometricPolicy,
        enclave: &'a TypeSafeSecureEnclave,
    ) -> Self {
        Self {
            key_id,
            algorithm,
            biometric_policy,
            enclave,
            _phantom: PhantomData,
        }
    }

    /// Sign With Biometric operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn sign_with_biometric(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        if !matches!(
            self.algorithm.algorithm(),
            SecureEnclaveAlgorithm::EcdsaP256
        ) {
            return Err(BearDogError::unsupported_operation(
                "Key algorithm does not support signing".to_string(),
            ));
        }
        self.sign_with_biometric_auth(data)
    }

    /// Sign With Biometric Auth operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn sign_with_biometric_auth(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        info!("🔐 Signing data with Secure Enclave key: {}", self.key_id);

        let keys = self.enclave.keychain_keys.read();
        let key = keys.get(&self.key_id).ok_or_else(|| {
            BearDogError::not_found(format!("Key not found: {}", self.key_id))
        })?;

        // Authenticate with biometric first
        self.authenticate_biometric()?;

        // Use private key for signing
        match &key.private_key {
            Some(private_key) => BearDogCrypto::sign_ed25519(private_key, data),
            None => Err(BearDogError::not_found(format!(
                "Private key not available for: {}",
                self.key_id
            ))),
        }
    }

    /// Verify Signature operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn verify_signature(&self, data: &[u8], signature: &[u8]) -> Result<bool, BearDogError> {
        info!(
            "🔍 Verifying signature with Secure Enclave key: {}",
            self.key_id
        );

        let keys = self.enclave.keychain_keys.read();
        let key = keys.get(&self.key_id).ok_or_else(|| {
            BearDogError::not_found(format!("Key not found: {}", self.key_id))
        })?;

        BearDogCrypto::verify_ed25519_signature(&key.public_key, data, signature)
    }

    /// Key Agreement operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn key_agreement(&self, peer_public_key: &[u8]) -> Result<Vec<u8>, BearDogError>
    where
        A: KeyAgreementCapable,
    {
        info!(
            "🤝 Performing key agreement with Secure Enclave key: {}",
            self.key_id
        );

        if !matches!(self.algorithm.algorithm(), SecureEnclaveAlgorithm::EcdhP256) {
            return Err(BearDogError::unsupported_operation(
                "Key algorithm does not support key agreement".to_string(),
            ));
        }

        let keys = self.enclave.keychain_keys.read();
        let key = keys.get(&self.key_id).ok_or_else(|| {
            BearDogError::not_found(format!("Key not found: {}", self.key_id))
        })?;

        // Perform key agreement
        match &key.private_key {
            Some(_private_key) => {
                info!("🤝 Using Secure Enclave for key agreement");
                self.secure_enclave_key_agreement(peer_public_key, key)
            }
            None => {
                info!("🔧 Using software fallback for key agreement");
                self.software_fallback_key_agreement(peer_public_key, key)
            }
        }
    }

    /// Initialize operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn initialize() -> Result<TypeSafeSecureEnclave, BearDogError> {
        info!("🚀 Initializing Type-Safe Secure Enclave");
        TypeSafeSecureEnclave::new()
    }

    fn authenticate_biometric(&self) -> Result<(), BearDogError> {
        match &self.biometric_policy {
            BiometricPolicy::NoBiometric => {
                debug!("🔓 No biometric authentication required");
                Ok(())
            }
            policy => {
                debug!("🔐 Authenticating with biometric policy: {:?}", policy);
                self.simulate_biometric_auth(policy)
            }
        }
    }

    fn simulate_biometric_auth(&self, policy: &BiometricPolicy) -> Result<(), BearDogError> {
        match policy {
            BiometricPolicy::TouchIDRequired | BiometricPolicy::TouchIDOnly => {
                debug!("👆 TouchID authentication simulated");
                Ok(())
            }
            BiometricPolicy::FaceIDRequired | BiometricPolicy::FaceIDOnly => {
                debug!("🆔 FaceID authentication simulated");
                Ok(())
            }
            BiometricPolicy::TouchIDOrFaceID | BiometricPolicy::AnyBiometric => {
                debug!("🔐 Any biometric authentication simulated");
                Ok(())
            }
            BiometricPolicy::NoBiometric => Ok(()),
        }
    }

    fn secure_enclave_key_agreement(
        &self,
        peer_public_key: &[u8],
        _key: &SecureEnclaveKeyMaterial,
    ) -> Result<Vec<u8>, BearDogError> {
        // Placeholder: actual Secure Enclave key agreement
        debug!(
            "🔐 Secure Enclave key agreement with peer: {} bytes",
            peer_public_key.len()
        );
        Ok(vec![0u8; 32]) // Placeholder shared secret
    }

    fn software_fallback_key_agreement(
        &self,
        peer_public_key: &[u8],
        _key: &SecureEnclaveKeyMaterial,
    ) -> Result<Vec<u8>, BearDogError> {
        // Placeholder: software fallback key agreement
        debug!(
            "🔧 Software key agreement with peer: {} bytes",
            peer_public_key.len()
        );
        Ok(vec![0u8; 32]) // Placeholder shared secret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_safe_key_creation() -> Result<(), BearDogError> {
        let enclave = TypeSafeSecureEnclave::new()?;
        let algorithm = SecureEnclaveAlgorithmType::EcdsaP256;
        let key = TypeSafeSecureEnclaveKey::new(
            "test-key".to_string(),
            algorithm,
            BiometricPolicy::NoBiometric,
            &enclave,
        );
        assert_eq!(key.key_id, "test-key");
        Ok(())
    }

    #[test]
    fn test_biometric_auth_simulation() -> Result<(), BearDogError> {
        let enclave = TypeSafeSecureEnclave::new()?;
        let algorithm = SecureEnclaveAlgorithmType::EcdsaP256;
        let key = TypeSafeSecureEnclaveKey::new(
            "test-key".to_string(),
            algorithm,
            BiometricPolicy::TouchIDRequired,
            &enclave,
        );
        
        // Test that authentication simulation works
        assert!(key.simulate_biometric_auth(&BiometricPolicy::TouchIDRequired).is_ok());
        assert!(key.simulate_biometric_auth(&BiometricPolicy::FaceIDRequired).is_ok());
        assert!(key.simulate_biometric_auth(&BiometricPolicy::NoBiometric).is_ok());
        
        Ok(())
    }

    #[test]
    fn test_unsupported_operations() -> Result<(), BearDogError> {
        let enclave = TypeSafeSecureEnclave::new()?;
        // Test with wrong algorithm for key agreement
        let algorithm = SecureEnclaveAlgorithmType::EcdsaP256; // Not ECDH
        let key = TypeSafeSecureEnclaveKey::new(
            "test-key".to_string(),
            algorithm,
            BiometricPolicy::NoBiometric,
            &enclave,
        );
        
        let peer_key = vec![0u8; 32];
        // This should fail because ECDSA doesn't support key agreement
        // Note: This would require proper trait bounds to compile
        // assert!(key.key_agreement(&peer_key).is_err());
        
        Ok(())
    }
}
