// SPDX-License-Identifier: AGPL-3.0-or-later

//! Kyber Key Encapsulation Mechanism (KEM)
//!
//! Implements ML-KEM (formerly Kyber) for quantum-resistant key exchange.
//!
//! ## Security Note
//!
//! This is a **simulation implementation** for API design. Production use
//! requires integration with actual PQC libraries when stable.

use super::types::{
    KemAlgorithm, QuantumKEM, QuantumKeyExchange, QuantumPrivateKey, SecurityLevel, kyber_sizes::*,
};
use beardog_errors::BearDogError;
use rand::RngCore;

/// Kyber KEM engine
///
/// Provides key encapsulation for quantum-resistant key exchange.
pub struct KyberEngine {
    security_level: SecurityLevel,
}

impl KyberEngine {
    /// Create a new Kyber engine with specified security level
    ///
    /// # Errors
    ///
    /// Currently always returns `Ok`; the `Result` is reserved for future validation.
    pub const fn new(security_level: SecurityLevel) -> Result<Self, BearDogError> {
        Ok(Self { security_level })
    }

    /// Generate a Kyber keypair
    ///
    /// Returns a keypair suitable for key encapsulation.
    ///
    /// # Errors
    ///
    /// Currently infallible; reserved for production KEM integration errors.
    pub fn generate_keypair(&self) -> Result<QuantumKEM, BearDogError> {
        let mut rng = rand::rng();

        let (pk_size, sk_size, algorithm) = match self.security_level {
            SecurityLevel::Level1 | SecurityLevel::Level2 => {
                (KYBER512_PK_SIZE, KYBER512_SK_SIZE, KemAlgorithm::Kyber512)
            }
            SecurityLevel::Level3 => (KYBER768_PK_SIZE, KYBER768_SK_SIZE, KemAlgorithm::Kyber768),
            SecurityLevel::Level4 | SecurityLevel::Level5 => (
                KYBER1024_PK_SIZE,
                KYBER1024_SK_SIZE,
                KemAlgorithm::Kyber1024,
            ),
        };

        let mut public_key = vec![0u8; pk_size];
        let mut private_key_data = vec![0u8; sk_size];

        rng.fill_bytes(&mut public_key);
        rng.fill_bytes(&mut private_key_data);

        Ok(QuantumKEM {
            public_key,
            private_key: Some(QuantumPrivateKey::new(
                private_key_data,
                format!("{algorithm:?}"),
            )),
            algorithm,
            security_level: self.security_level,
        })
    }

    /// Encapsulate a shared secret using peer's public key
    ///
    /// # Arguments
    /// * `peer_public_key` - The recipient's public key
    ///
    /// # Returns
    /// * `QuantumKeyExchange` containing shared secret and ciphertext
    ///
    /// # Errors
    ///
    /// Currently infallible in this simulation; production may fail on invalid keys.
    pub fn encapsulate(&self, _peer_public_key: &[u8]) -> Result<QuantumKeyExchange, BearDogError> {
        let mut rng = rand::rng();

        let (ct_size, ss_size, algorithm) = match self.security_level {
            SecurityLevel::Level1 | SecurityLevel::Level2 => {
                (KYBER512_CT_SIZE, KYBER512_SS_SIZE, KemAlgorithm::Kyber512)
            }
            SecurityLevel::Level3 => (KYBER768_CT_SIZE, KYBER768_SS_SIZE, KemAlgorithm::Kyber768),
            SecurityLevel::Level4 | SecurityLevel::Level5 => (
                KYBER1024_CT_SIZE,
                KYBER1024_SS_SIZE,
                KemAlgorithm::Kyber1024,
            ),
        };

        let mut shared_secret = vec![0u8; ss_size];
        let mut ciphertext = vec![0u8; ct_size];

        rng.fill_bytes(&mut shared_secret);
        rng.fill_bytes(&mut ciphertext);

        Ok(QuantumKeyExchange {
            shared_secret,
            ciphertext,
            algorithm,
        })
    }

    /// Decapsulate a shared secret using own private key
    ///
    /// # Arguments
    /// * `ciphertext` - The ciphertext from encapsulation
    /// * `private_key` - Own private key
    ///
    /// # Returns
    /// * The derived shared secret
    ///
    /// # Errors
    ///
    /// Currently infallible in this simulation; production may fail on invalid ciphertext or keys.
    pub fn decapsulate(
        &self,
        _ciphertext: &[u8],
        _private_key: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        let mut rng = rand::rng();

        let ss_size = match self.security_level {
            SecurityLevel::Level1 | SecurityLevel::Level2 => KYBER512_SS_SIZE,
            SecurityLevel::Level3 => KYBER768_SS_SIZE,
            SecurityLevel::Level4 | SecurityLevel::Level5 => KYBER1024_SS_SIZE,
        };

        let mut shared_secret = vec![0u8; ss_size];
        rng.fill_bytes(&mut shared_secret);

        Ok(shared_secret)
    }

    /// Get the security level
    #[must_use]
    pub const fn security_level(&self) -> SecurityLevel {
        self.security_level
    }
}

impl std::fmt::Debug for KyberEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KyberEngine")
            .field("security_level", &self.security_level)
            .finish()
    }
}
