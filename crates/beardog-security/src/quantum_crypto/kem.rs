// SPDX-License-Identifier: AGPL-3.0-only

//! Kyber Key Encapsulation Mechanism (KEM)
//!
//! Implements ML-KEM (formerly Kyber) for quantum-resistant key exchange.
//!
//! ## Security Note
//!
//! This is a **simulation implementation** for API design. Production use
//! requires integration with actual PQC libraries when stable.

use super::types::{
    kyber_sizes::*, KemAlgorithm, QuantumKEM, QuantumKeyExchange, QuantumPrivateKey, SecurityLevel,
};
use beardog_errors::BearDogError;

/// Kyber KEM engine
///
/// Provides key encapsulation for quantum-resistant key exchange.
pub struct KyberEngine {
    security_level: SecurityLevel,
}

impl KyberEngine {
    /// Create a new Kyber engine with specified security level
    pub fn new(security_level: SecurityLevel) -> Result<Self, BearDogError> {
        Ok(Self { security_level })
    }

    /// Generate a Kyber keypair
    ///
    /// Returns a keypair suitable for key encapsulation.
    pub fn generate_keypair(&self) -> Result<QuantumKEM, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;

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
                format!("{:?}", algorithm),
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
    pub fn encapsulate(&self, _peer_public_key: &[u8]) -> Result<QuantumKeyExchange, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;

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
    pub fn decapsulate(
        &self,
        _ciphertext: &[u8],
        _private_key: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;

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
    pub fn security_level(&self) -> SecurityLevel {
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
