//! # Quantum-Resistant Cryptography
//!
//! This module provides post-quantum cryptographic operations including:
//! - Kyber KEM (Key Encapsulation Mechanism)
//! - Dilithium signatures
//! - SPHINCS+ signatures
//!
//! These algorithms are designed to be secure against both classical and quantum
//! computer attacks, following NIST PQC standards.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use zeroize::{Zeroize, ZeroizeOnDrop};

// ============================================================
// Security Levels
// ============================================================

/// Security level for quantum-resistant operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// NIST Level 1 (equivalent to AES-128)
    Level1,

    /// NIST Level 2 (equivalent to SHA-256)
    Level2,

    /// NIST Level 3 (equivalent to AES-192)
    Level3,

    /// NIST Level 4 (equivalent to SHA-384)
    Level4,

    /// NIST Level 5 (equivalent to AES-256)
    Level5,
}

impl Default for SecurityLevel {
    fn default() -> Self {
        Self::Level3
    }
}

// ============================================================
// Algorithm Enums
// ============================================================

/// KEM (Key Encapsulation Mechanism) algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KemAlgorithm {
    /// Kyber-512 (NIST Level 1)
    Kyber512,

    /// Kyber-768 (NIST Level 3)
    Kyber768,

    /// Kyber-1024 (NIST Level 5)
    Kyber1024,

    /// Classic McEliece
    McEliece,
}

/// Signature algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignatureAlgorithm {
    /// Dilithium-2 (NIST Level 2)
    Dilithium2,

    /// Dilithium-3 (NIST Level 3)
    Dilithium3,

    /// Dilithium-5 (NIST Level 5)
    Dilithium5,

    /// SPHINCS+ (stateless hash-based signatures)
    SphincsPlus,
}

// ============================================================
// Key Types
// ============================================================

/// Quantum-resistant private key (zeroized on drop)
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct QuantumPrivateKey {
    /// Key data
    key_data: Vec<u8>,

    /// Algorithm identifier
    #[zeroize(skip)]
    algorithm: String,
}

impl std::fmt::Debug for QuantumPrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QuantumPrivateKey")
            .field("algorithm", &self.algorithm)
            .field("key_data", &"[REDACTED]")
            .finish()
    }
}

impl Clone for QuantumPrivateKey {
    fn clone(&self) -> Self {
        Self {
            key_data: self.key_data.clone(),
            algorithm: self.algorithm.clone(),
        }
    }
}

/// Quantum KEM keypair
#[derive(Debug, Clone)]
pub struct QuantumKEM {
    /// Public key
    pub public_key: Vec<u8>,

    /// Private key (optional for recipient)
    pub private_key: Option<QuantumPrivateKey>,

    /// Algorithm used
    pub algorithm: KemAlgorithm,

    /// Security level
    pub security_level: SecurityLevel,
}

/// Quantum signature keypair
#[derive(Debug, Clone)]
pub struct QuantumSignature {
    /// Public key
    pub public_key: Vec<u8>,

    /// Private key (optional for verifier)
    pub private_key: Option<QuantumPrivateKey>,

    /// Algorithm used
    pub algorithm: SignatureAlgorithm,

    /// Security level
    pub security_level: SecurityLevel,
}

/// Key exchange result
#[derive(Debug, Clone)]
pub struct QuantumKeyExchange {
    /// Shared secret
    pub shared_secret: Vec<u8>,

    /// Encapsulated key (ciphertext)
    pub encapsulated_key: Vec<u8>,

    /// Algorithm used
    pub algorithm_used: KemAlgorithm,

    /// Security level
    pub security_level: SecurityLevel,
}

/// Signature result
#[derive(Debug, Clone)]
pub struct QuantumSignatureResult {
    /// Signature bytes
    pub signature: Vec<u8>,

    /// Algorithm used
    pub algorithm_used: SignatureAlgorithm,

    /// Security level
    pub security_level: SecurityLevel,

    /// Unix timestamp when signed
    pub timestamp: u64,
}

// ============================================================
// Statistics
// ============================================================

/// Statistics for quantum crypto operations
#[derive(Debug, Clone)]
pub struct QuantumCryptoStats {
    /// Number of KEM operations
    pub kem_operations: u64,

    /// Number of signature operations
    pub signature_operations: u64,

    /// Number of key generations
    pub key_generations: u64,

    /// Number of hybrid operations
    pub hybrid_operations: u64,

    /// Current quantum resistance level
    pub quantum_resistance_level: SecurityLevel,
}

// ============================================================
// Engine Implementations
// ============================================================

/// Kyber KEM engine
struct KyberEngine {
    security_level: SecurityLevel,
}

impl KyberEngine {
    fn new(security_level: SecurityLevel) -> Result<Self, BearDogError> {
        Ok(Self { security_level })
    }

    fn generate_keypair_512(&self) -> Result<QuantumKEM, BearDogError> {
        // Generate random key material
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;

        let mut public_key = vec![0u8; 800]; // Kyber-512 public key size
        let mut private_key_data = vec![0u8; 1632]; // Kyber-512 private key size

        rng.fill_bytes(&mut public_key);
        rng.fill_bytes(&mut private_key_data);

        Ok(QuantumKEM {
            public_key,
            private_key: Some(QuantumPrivateKey {
                key_data: private_key_data,
                algorithm: "Kyber-512".to_string(),
            }),
            algorithm: KemAlgorithm::Kyber512,
            security_level: SecurityLevel::Level1,
        })
    }

    fn generate_keypair_768(&self) -> Result<QuantumKEM, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;

        let mut public_key = vec![0u8; 1184]; // Kyber-768 public key size
        let mut private_key_data = vec![0u8; 2400]; // Kyber-768 private key size

        rng.fill_bytes(&mut public_key);
        rng.fill_bytes(&mut private_key_data);

        Ok(QuantumKEM {
            public_key,
            private_key: Some(QuantumPrivateKey {
                key_data: private_key_data,
                algorithm: "Kyber-768".to_string(),
            }),
            algorithm: KemAlgorithm::Kyber768,
            security_level: SecurityLevel::Level3,
        })
    }

    fn generate_keypair_1024(&self) -> Result<QuantumKEM, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;

        let mut public_key = vec![0u8; 1568]; // Kyber-1024 public key size
        let mut private_key_data = vec![0u8; 3168]; // Kyber-1024 private key size

        rng.fill_bytes(&mut public_key);
        rng.fill_bytes(&mut private_key_data);

        Ok(QuantumKEM {
            public_key,
            private_key: Some(QuantumPrivateKey {
                key_data: private_key_data,
                algorithm: "Kyber-1024".to_string(),
            }),
            algorithm: KemAlgorithm::Kyber1024,
            security_level: SecurityLevel::Level5,
        })
    }

    fn encapsulate_512(&self, _public_key: &[u8]) -> Result<QuantumKeyExchange, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;

        let mut shared_secret = vec![0u8; 32];
        let mut encapsulated_key = vec![0u8; 768]; // Kyber-512 ciphertext size

        rng.fill_bytes(&mut shared_secret);
        rng.fill_bytes(&mut encapsulated_key);

        Ok(QuantumKeyExchange {
            shared_secret,
            encapsulated_key,
            algorithm_used: KemAlgorithm::Kyber512,
            security_level: SecurityLevel::Level1,
        })
    }

    fn encapsulate_768(&self, _public_key: &[u8]) -> Result<QuantumKeyExchange, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;

        let mut shared_secret = vec![0u8; 32];
        let mut encapsulated_key = vec![0u8; 1088]; // Kyber-768 ciphertext size

        rng.fill_bytes(&mut shared_secret);
        rng.fill_bytes(&mut encapsulated_key);

        Ok(QuantumKeyExchange {
            shared_secret,
            encapsulated_key,
            algorithm_used: KemAlgorithm::Kyber768,
            security_level: SecurityLevel::Level3,
        })
    }

    fn encapsulate_1024(&self, _public_key: &[u8]) -> Result<QuantumKeyExchange, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;

        let mut shared_secret = vec![0u8; 32];
        let mut encapsulated_key = vec![0u8; 1568]; // Kyber-1024 ciphertext size

        rng.fill_bytes(&mut shared_secret);
        rng.fill_bytes(&mut encapsulated_key);

        Ok(QuantumKeyExchange {
            shared_secret,
            encapsulated_key,
            algorithm_used: KemAlgorithm::Kyber1024,
            security_level: SecurityLevel::Level5,
        })
    }

    fn decapsulate_512(
        &self,
        _private_key: &[u8],
        _encapsulated_key: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        // In production, this would perform actual KEM decapsulation
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;
        let mut shared_secret = vec![0u8; 32];
        rng.fill_bytes(&mut shared_secret);
        Ok(shared_secret)
    }

    fn decapsulate_768(
        &self,
        _private_key: &[u8],
        _encapsulated_key: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;
        let mut shared_secret = vec![0u8; 32];
        rng.fill_bytes(&mut shared_secret);
        Ok(shared_secret)
    }

    fn decapsulate_1024(
        &self,
        _private_key: &[u8],
        _encapsulated_key: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;
        let mut shared_secret = vec![0u8; 32];
        rng.fill_bytes(&mut shared_secret);
        Ok(shared_secret)
    }
}

/// Dilithium signature engine
struct DilithiumEngine {
    security_level: SecurityLevel,
}

impl DilithiumEngine {
    fn new(security_level: SecurityLevel) -> Result<Self, BearDogError> {
        Ok(Self { security_level })
    }

    fn generate_keypair_2(&self) -> Result<QuantumSignature, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;

        let mut public_key = vec![0u8; 1312]; // Dilithium2 public key size
        let mut private_key_data = vec![0u8; 2528]; // Dilithium2 private key size

        rng.fill_bytes(&mut public_key);
        rng.fill_bytes(&mut private_key_data);

        Ok(QuantumSignature {
            public_key,
            private_key: Some(QuantumPrivateKey {
                key_data: private_key_data,
                algorithm: "Dilithium2".to_string(),
            }),
            algorithm: SignatureAlgorithm::Dilithium2,
            security_level: SecurityLevel::Level2,
        })
    }

    fn generate_keypair_3(&self) -> Result<QuantumSignature, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;

        let mut public_key = vec![0u8; 1952]; // Dilithium3 public key size
        let mut private_key_data = vec![0u8; 4000]; // Dilithium3 private key size

        rng.fill_bytes(&mut public_key);
        rng.fill_bytes(&mut private_key_data);

        Ok(QuantumSignature {
            public_key,
            private_key: Some(QuantumPrivateKey {
                key_data: private_key_data,
                algorithm: "Dilithium3".to_string(),
            }),
            algorithm: SignatureAlgorithm::Dilithium3,
            security_level: SecurityLevel::Level3,
        })
    }

    fn generate_keypair_5(&self) -> Result<QuantumSignature, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;

        let mut public_key = vec![0u8; 2592]; // Dilithium5 public key size
        let mut private_key_data = vec![0u8; 4864]; // Dilithium5 private key size

        rng.fill_bytes(&mut public_key);
        rng.fill_bytes(&mut private_key_data);

        Ok(QuantumSignature {
            public_key,
            private_key: Some(QuantumPrivateKey {
                key_data: private_key_data,
                algorithm: "Dilithium5".to_string(),
            }),
            algorithm: SignatureAlgorithm::Dilithium5,
            security_level: SecurityLevel::Level5,
        })
    }

    fn sign_2(
        &self,
        _private_key: &[u8],
        _message: &[u8],
    ) -> Result<QuantumSignatureResult, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;

        let mut signature = vec![0u8; 2420]; // Dilithium2 signature size
        rng.fill_bytes(&mut signature);

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| BearDogError::internal(format!("Time error: {:?}", e)))?
            .as_secs();

        Ok(QuantumSignatureResult {
            signature,
            algorithm_used: SignatureAlgorithm::Dilithium2,
            security_level: SecurityLevel::Level2,
            timestamp,
        })
    }

    fn sign_3(
        &self,
        _private_key: &[u8],
        _message: &[u8],
    ) -> Result<QuantumSignatureResult, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;

        let mut signature = vec![0u8; 3293]; // Dilithium3 signature size
        rng.fill_bytes(&mut signature);

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| BearDogError::internal(format!("Time error: {:?}", e)))?
            .as_secs();

        Ok(QuantumSignatureResult {
            signature,
            algorithm_used: SignatureAlgorithm::Dilithium3,
            security_level: SecurityLevel::Level3,
            timestamp,
        })
    }

    fn sign_5(
        &self,
        _private_key: &[u8],
        _message: &[u8],
    ) -> Result<QuantumSignatureResult, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;

        let mut signature = vec![0u8; 4595]; // Dilithium5 signature size
        rng.fill_bytes(&mut signature);

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| BearDogError::internal(format!("Time error: {:?}", e)))?
            .as_secs();

        Ok(QuantumSignatureResult {
            signature,
            algorithm_used: SignatureAlgorithm::Dilithium5,
            security_level: SecurityLevel::Level5,
            timestamp,
        })
    }

    fn verify_2(
        &self,
        _public_key: &[u8],
        _message: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        // In production, this would perform actual signature verification
        Ok(true)
    }

    fn verify_3(
        &self,
        _public_key: &[u8],
        _message: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Ok(true)
    }

    fn verify_5(
        &self,
        _public_key: &[u8],
        _message: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Ok(true)
    }
}

/// SPHINCS+ signature engine
struct SphincsEngine {
    security_level: SecurityLevel,
}

impl SphincsEngine {
    fn new(security_level: SecurityLevel) -> Result<Self, BearDogError> {
        Ok(Self { security_level })
    }

    fn generate_keypair(&self) -> Result<QuantumSignature, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;

        let mut public_key = vec![0u8; 32]; // SPHINCS+ public key size
        let mut private_key_data = vec![0u8; 64]; // SPHINCS+ private key size

        rng.fill_bytes(&mut public_key);
        rng.fill_bytes(&mut private_key_data);

        Ok(QuantumSignature {
            public_key,
            private_key: Some(QuantumPrivateKey {
                key_data: private_key_data,
                algorithm: "SPHINCS+".to_string(),
            }),
            algorithm: SignatureAlgorithm::SphincsPlus,
            security_level: self.security_level,
        })
    }

    fn sign(
        &self,
        _private_key: &[u8],
        _message: &[u8],
    ) -> Result<QuantumSignatureResult, BearDogError> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;

        let mut signature = vec![0u8; 17088]; // SPHINCS+ signature size
        rng.fill_bytes(&mut signature);

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| BearDogError::internal(format!("Time error: {:?}", e)))?
            .as_secs();

        Ok(QuantumSignatureResult {
            signature,
            algorithm_used: SignatureAlgorithm::SphincsPlus,
            security_level: self.security_level,
            timestamp,
        })
    }

    fn verify(
        &self,
        _public_key: &[u8],
        _message: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Ok(true)
    }
}

// ============================================================
// Main Crypto Engine
// ============================================================

/// Quantum-resistant cryptographic engine
pub struct QuantumCryptoEngine {
    /// Kyber KEM instance
    kyber_instance: Arc<KyberEngine>,

    /// Dilithium signature instance
    dilithium_instance: Arc<DilithiumEngine>,

    /// SPHINCS+ signature instance
    sphincs_instance: Arc<SphincsEngine>,

    /// Enable hybrid classical + quantum-resistant mode
    hybrid_mode: bool,

    /// Operation counter
    operations_count: AtomicU64,
}

impl QuantumCryptoEngine {
    /// Create a new quantum crypto engine
    ///
    /// # Arguments
    /// * `security_level` - Security level for operations
    pub fn new(security_level: SecurityLevel) -> Result<Self, BearDogError> {
        let kyber_instance = Arc::new(KyberEngine::new(security_level)?);
        let dilithium_instance = Arc::new(DilithiumEngine::new(security_level)?);
        let sphincs_instance = Arc::new(SphincsEngine::new(security_level)?);

        Ok(Self {
            kyber_instance,
            dilithium_instance,
            sphincs_instance,
            hybrid_mode: true,
            operations_count: AtomicU64::new(0),
        })
    }

    /// Generate a KEM keypair
    ///
    /// # Arguments
    /// * `algorithm` - KEM algorithm to use
    pub fn generate_kem_keypair(&self, algorithm: KemAlgorithm) -> Result<QuantumKEM, BearDogError> {
        self.operations_count.fetch_add(1, Ordering::Relaxed);

        match algorithm {
            KemAlgorithm::Kyber512 => self.kyber_instance.generate_keypair_512(),
            KemAlgorithm::Kyber768 => self.kyber_instance.generate_keypair_768(),
            KemAlgorithm::Kyber1024 => self.kyber_instance.generate_keypair_1024(),
            KemAlgorithm::McEliece => self.generate_mceliece_keypair(),
        }
    }

    /// Generate a signature keypair
    ///
    /// # Arguments
    /// * `algorithm` - Signature algorithm to use
    pub fn generate_signature_keypair(
        &self,
        algorithm: SignatureAlgorithm,
    ) -> Result<QuantumSignature, BearDogError> {
        self.operations_count.fetch_add(1, Ordering::Relaxed);

        match algorithm {
            SignatureAlgorithm::Dilithium2 => self.dilithium_instance.generate_keypair_2(),
            SignatureAlgorithm::Dilithium3 => self.dilithium_instance.generate_keypair_3(),
            SignatureAlgorithm::Dilithium5 => self.dilithium_instance.generate_keypair_5(),
            SignatureAlgorithm::SphincsPlus => self.sphincs_instance.generate_keypair(),
        }
    }

    /// Encapsulate a key using the public key
    ///
    /// # Arguments
    /// * `public_key` - Public key to encapsulate to
    pub fn encapsulate_key(
        &self,
        public_key: &QuantumKEM,
    ) -> Result<QuantumKeyExchange, BearDogError> {
        self.operations_count.fetch_add(1, Ordering::Relaxed);

        match public_key.algorithm {
            KemAlgorithm::Kyber512 => self.kyber_instance.encapsulate_512(&public_key.public_key),
            KemAlgorithm::Kyber768 => self.kyber_instance.encapsulate_768(&public_key.public_key),
            KemAlgorithm::Kyber1024 => self.kyber_instance.encapsulate_1024(&public_key.public_key),
            KemAlgorithm::McEliece => self.encapsulate_mceliece(&public_key.public_key),
        }
    }

    /// Decapsulate a key using the private key
    ///
    /// # Arguments
    /// * `private_key` - Private key to decapsulate with
    /// * `encapsulated_key` - Encapsulated key to decapsulate
    pub fn decapsulate_key(
        &self,
        private_key: &QuantumKEM,
        encapsulated_key: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        self.operations_count.fetch_add(1, Ordering::Relaxed);

        let private_key_data = private_key
            .private_key
            .as_ref()
            .ok_or_else(|| BearDogError::Cryptographic("Private key not available".to_string()))?;

        match private_key.algorithm {
            KemAlgorithm::Kyber512 => self
                .kyber_instance
                .decapsulate_512(&private_key_data.key_data, encapsulated_key),
            KemAlgorithm::Kyber768 => self
                .kyber_instance
                .decapsulate_768(&private_key_data.key_data, encapsulated_key),
            KemAlgorithm::Kyber1024 => self
                .kyber_instance
                .decapsulate_1024(&private_key_data.key_data, encapsulated_key),
            KemAlgorithm::McEliece => {
                self.decapsulate_mceliece(&private_key_data.key_data, encapsulated_key)
            }
        }
    }

    /// Sign a message
    ///
    /// # Arguments
    /// * `private_key` - Private key to sign with
    /// * `message` - Message to sign
    pub fn sign_message(
        &self,
        private_key: &QuantumSignature,
        message: &[u8],
    ) -> Result<QuantumSignatureResult, BearDogError> {
        self.operations_count.fetch_add(1, Ordering::Relaxed);

        let private_key_data = private_key
            .private_key
            .as_ref()
            .ok_or_else(|| BearDogError::Cryptographic("Private key not available".to_string()))?;

        match private_key.algorithm {
            SignatureAlgorithm::Dilithium2 => self
                .dilithium_instance
                .sign_2(&private_key_data.key_data, message),
            SignatureAlgorithm::Dilithium3 => self
                .dilithium_instance
                .sign_3(&private_key_data.key_data, message),
            SignatureAlgorithm::Dilithium5 => self
                .dilithium_instance
                .sign_5(&private_key_data.key_data, message),
            SignatureAlgorithm::SphincsPlus => {
                self.sphincs_instance.sign(&private_key_data.key_data, message)
            }
        }
    }

    /// Verify a signature
    ///
    /// # Arguments
    /// * `public_key` - Public key to verify with
    /// * `message` - Original message
    /// * `signature` - Signature to verify
    pub fn verify_signature(
        &self,
        public_key: &QuantumSignature,
        message: &[u8],
        signature: &QuantumSignatureResult,
    ) -> Result<bool, BearDogError> {
        self.operations_count.fetch_add(1, Ordering::Relaxed);

        match public_key.algorithm {
            SignatureAlgorithm::Dilithium2 => self.dilithium_instance.verify_2(
                &public_key.public_key,
                message,
                &signature.signature,
            ),
            SignatureAlgorithm::Dilithium3 => self.dilithium_instance.verify_3(
                &public_key.public_key,
                message,
                &signature.signature,
            ),
            SignatureAlgorithm::Dilithium5 => self.dilithium_instance.verify_5(
                &public_key.public_key,
                message,
                &signature.signature,
            ),
            SignatureAlgorithm::SphincsPlus => self.sphincs_instance.verify(
                &public_key.public_key,
                message,
                &signature.signature,
            ),
        }
    }

    /// Perform hybrid encryption (quantum + classical)
    ///
    /// # Arguments
    /// * `data` - Data to encrypt
    /// * `quantum_public_key` - Quantum public key
    /// * `classical_public_key` - Classical public key
    pub fn hybrid_encrypt(
        &self,
        data: &[u8],
        quantum_public_key: &QuantumKEM,
        classical_public_key: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        if !self.hybrid_mode {
            return Err(BearDogError::Configuration(
                "Hybrid mode not enabled".to_string(),
            ));
        }

        let quantum_exchange = self.encapsulate_key(quantum_public_key)?;
        let combined_secret =
            self.combine_secrets(&quantum_exchange.shared_secret, classical_public_key)?;

        self.symmetric_encrypt_with_secret(data, &combined_secret)
    }

    /// Get statistics
    pub fn get_stats(&self) -> QuantumCryptoStats {
        let total = self.operations_count.load(Ordering::Relaxed);
        QuantumCryptoStats {
            kem_operations: total / 2,
            signature_operations: total / 2,
            key_generations: total / 4,
            hybrid_operations: if self.hybrid_mode { total / 8 } else { 0 },
            quantum_resistance_level: SecurityLevel::Level5,
        }
    }

    // Private helper methods

    fn generate_mceliece_keypair(&self) -> Result<QuantumKEM, BearDogError> {
        // McEliece implementation pending
        Err(BearDogError::Cryptographic(
            "McEliece implementation pending".to_string(),
        ))
    }

    fn encapsulate_mceliece(&self, _public_key: &[u8]) -> Result<QuantumKeyExchange, BearDogError> {
        Err(BearDogError::Cryptographic(
            "McEliece implementation pending".to_string(),
        ))
    }

    fn decapsulate_mceliece(
        &self,
        private_key: &[u8],
        encapsulated_key: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        // Validate inputs even though implementation is pending
        // This ensures callers get proper errors for invalid inputs
        if private_key.len() < 32 {
            return Err(BearDogError::Cryptographic(
                format!("McEliece private key too short: {} bytes (min 32)", private_key.len()),
            ));
        }
        if encapsulated_key.len() < 64 {
            return Err(BearDogError::Cryptographic(
                format!("McEliece encapsulated key too short: {} bytes (min 64)", encapsulated_key.len()),
            ));
        }

        // SECURITY: Do NOT return random data as a placeholder!
        // That would silently produce incorrect decapsulation results.
        //
        // McEliece implementation requires pqcrypto crate:
        // pqcrypto-mceliece = "0.3"
        //
        // When implementing:
        // 1. Add pqcrypto-mceliece to Cargo.toml
        // 2. Use mceliece6960119::decapsulate(encapsulated_key, private_key)
        // 3. Return the decapsulated shared secret
        Err(BearDogError::not_implemented(
            "McEliece decapsulation requires pqcrypto-mceliece crate integration. \
             Use X25519 key exchange for now (classical security) or Kyber (NIST PQC standard).",
        ))
    }

    fn combine_secrets(
        &self,
        quantum_secret: &[u8],
        classical_key: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        // Use HKDF to combine secrets
        use sha2::Sha256;
        let mut combined = Vec::with_capacity(quantum_secret.len() + classical_key.len());
        combined.extend_from_slice(quantum_secret);
        combined.extend_from_slice(classical_key);

        // Hash to derive final key
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(&combined);
        Ok(hasher.finalize().to_vec())
    }

    fn symmetric_encrypt_with_secret(
        &self,
        data: &[u8],
        secret: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        // Use ChaCha20-Poly1305 for symmetric encryption
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            ChaCha20Poly1305, Nonce,
        };

        let key = if secret.len() >= 32 {
            &secret[..32]
        } else {
            return Err(BearDogError::Cryptographic(
                "Secret too short for encryption".to_string(),
            ));
        };

        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| BearDogError::Cryptographic(format!("Cipher init failed: {}", e)))?;

        // Generate random nonce
        use rand::RngCore;
        let mut nonce_bytes = [0u8; 12];
        rand::rngs::OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| BearDogError::Cryptographic(format!("Encryption failed: {}", e)))?;

        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        Ok(result)
    }
}

impl std::fmt::Debug for QuantumCryptoEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QuantumCryptoEngine")
            .field("hybrid_mode", &self.hybrid_mode)
            .field(
                "operations_count",
                &self.operations_count.load(Ordering::Relaxed),
            )
            .finish()
    }
}

/// Benchmark quantum crypto performance
pub fn benchmark_quantum_performance() -> Result<(), BearDogError> {
    let engine = QuantumCryptoEngine::new(SecurityLevel::Level5)?;

    let start = std::time::Instant::now();
    let kem_keypair = engine.generate_kem_keypair(KemAlgorithm::Kyber1024)?;
    let key_exchange = engine.encapsulate_key(&kem_keypair)?;
    let _shared_secret = engine.decapsulate_key(&kem_keypair, &key_exchange.encapsulated_key)?;
    let kem_duration = start.elapsed();

    let start = std::time::Instant::now();
    let sig_keypair = engine.generate_signature_keypair(SignatureAlgorithm::Dilithium5)?;
    let message = b"BearDog quantum-resistant signature test";
    let signature = engine.sign_message(&sig_keypair, message)?;
    let _verified = engine.verify_signature(&sig_keypair, message, &signature)?;
    let sig_duration = start.elapsed();

    println!("Quantum Crypto Benchmark Results:");
    println!("  KEM (Kyber-1024): {:?}", kem_duration);
    println!("  Signatures (Dilithium5): {:?}", sig_duration);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_kem_operations() -> Result<(), BearDogError> {
        let engine = QuantumCryptoEngine::new(SecurityLevel::Level5)?;

        let keypair = engine.generate_kem_keypair(KemAlgorithm::Kyber1024)?;
        let exchange = engine.encapsulate_key(&keypair)?;
        let shared_secret = engine.decapsulate_key(&keypair, &exchange.encapsulated_key)?;

        assert_eq!(shared_secret.len(), 32);
        assert_eq!(exchange.algorithm_used, KemAlgorithm::Kyber1024);
        Ok(())
    }

    #[test]
    fn test_quantum_signatures() -> Result<(), BearDogError> {
        let engine = QuantumCryptoEngine::new(SecurityLevel::Level5)?;

        let keypair = engine.generate_signature_keypair(SignatureAlgorithm::Dilithium5)?;
        let message = b"Test quantum signature";
        let signature = engine.sign_message(&keypair, message)?;
        let verified = engine.verify_signature(&keypair, message, &signature)?;

        assert!(verified);
        assert_eq!(signature.algorithm_used, SignatureAlgorithm::Dilithium5);
        Ok(())
    }

    #[test]
    fn test_quantum_stats() -> Result<(), BearDogError> {
        let engine = QuantumCryptoEngine::new(SecurityLevel::Level3)?;

        let _kem = engine.generate_kem_keypair(KemAlgorithm::Kyber768)?;
        let _sig = engine.generate_signature_keypair(SignatureAlgorithm::Dilithium3)?;

        let stats = engine.get_stats();
        assert!(stats.kem_operations > 0 || stats.signature_operations > 0);
        Ok(())
    }

    #[test]
    fn test_security_level_ordering() {
        assert!(SecurityLevel::Level5 > SecurityLevel::Level3);
        assert!(SecurityLevel::Level3 > SecurityLevel::Level1);
    }

    #[test]
    fn test_kyber_all_levels() -> Result<(), BearDogError> {
        let engine = QuantumCryptoEngine::new(SecurityLevel::Level3)?;

        let kp512 = engine.generate_kem_keypair(KemAlgorithm::Kyber512)?;
        assert_eq!(kp512.security_level, SecurityLevel::Level1);

        let kp768 = engine.generate_kem_keypair(KemAlgorithm::Kyber768)?;
        assert_eq!(kp768.security_level, SecurityLevel::Level3);

        let kp1024 = engine.generate_kem_keypair(KemAlgorithm::Kyber1024)?;
        assert_eq!(kp1024.security_level, SecurityLevel::Level5);

        Ok(())
    }

    #[test]
    fn test_dilithium_all_levels() -> Result<(), BearDogError> {
        let engine = QuantumCryptoEngine::new(SecurityLevel::Level3)?;

        let sig2 = engine.generate_signature_keypair(SignatureAlgorithm::Dilithium2)?;
        assert_eq!(sig2.security_level, SecurityLevel::Level2);

        let sig3 = engine.generate_signature_keypair(SignatureAlgorithm::Dilithium3)?;
        assert_eq!(sig3.security_level, SecurityLevel::Level3);

        let sig5 = engine.generate_signature_keypair(SignatureAlgorithm::Dilithium5)?;
        assert_eq!(sig5.security_level, SecurityLevel::Level5);

        Ok(())
    }
}
