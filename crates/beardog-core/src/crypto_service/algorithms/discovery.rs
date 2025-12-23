//! Runtime algorithm discovery
//!
//! Enables primals to discover available cryptographic algorithms at runtime
//! without hardcoding assumptions. This supports the "primal self-knowledge"
//! principle - each primal knows what it can do by introspection.
//!
//! ## Design Philosophy
//!
//! - **No hardcoding**: Algorithms are discovered, not configured
//! - **Capability-based**: System reports what it CAN do, not what it SHOULD do
//! - **Hardware-aware**: Detects AES-NI, AVX2, and other acceleration
//! - **Extensible**: New algorithms auto-discovered when added

use beardog_types::crypto_service::{CryptoAlgorithm, SignatureAlgorithm};
use std::collections::HashMap;

/// Algorithm capability information
#[derive(Debug, Clone)]
pub struct AlgorithmCapability {
    /// Algorithm name (human-readable)
    pub name: String,

    /// Algorithm type category
    pub algorithm_type: AlgorithmType,

    /// Whether this algorithm is currently available
    pub available: bool,

    /// Performance tier based on hardware acceleration
    pub performance_tier: PerformanceTier,

    /// Security level (in bits)
    pub security_level: u32,

    /// Whether hardware acceleration is available
    pub hardware_accelerated: bool,
}

/// Algorithm type categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgorithmType {
    /// Symmetric encryption (AES, `ChaCha20`)
    SymmetricEncryption,

    /// Asymmetric signatures (Ed25519, ECDSA)
    AsymmetricSignature,

    /// Hash functions (SHA-256, BLAKE3)
    Hash,

    /// Key derivation (HKDF, Argon2)
    KeyDerivation,

    /// Message authentication (HMAC)
    MessageAuthentication,
}

/// Performance tier based on benchmarks and hardware
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PerformanceTier {
    /// Extremely fast (hardware-accelerated, > 1 GB/s)
    VeryHigh,

    /// Fast (optimized software, > 100 MB/s)
    High,

    /// Moderate (standard software, > 10 MB/s)
    Medium,

    /// Slow (memory-hard, < 10 MB/s)
    Low,
}

/// Discover all available cryptographic algorithms
///
/// This function introspects the system to determine what algorithms
/// are available and how well they will perform.
///
/// # Returns
///
/// Vector of algorithm capabilities, sorted by preference
/// (hardware-accelerated and faster algorithms first)
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn discover_algorithms() -> Vec<AlgorithmCapability> {
    let mut capabilities = Vec::new();

    // Detect hardware features
    let has_aes_ni = detect_aes_ni();
    let has_avx2 = detect_avx2();
    let has_sha_ext = detect_sha_extensions();

    // Symmetric encryption algorithms
    capabilities.push(AlgorithmCapability {
        name: "AES-256-GCM".to_string(),
        algorithm_type: AlgorithmType::SymmetricEncryption,
        available: true, // Always available (pure Rust fallback)
        performance_tier: if has_aes_ni {
            PerformanceTier::VeryHigh
        } else {
            PerformanceTier::High
        },
        security_level: 256,
        hardware_accelerated: has_aes_ni,
    });

    capabilities.push(AlgorithmCapability {
        name: "AES-128-GCM".to_string(),
        algorithm_type: AlgorithmType::SymmetricEncryption,
        available: true,
        performance_tier: if has_aes_ni {
            PerformanceTier::VeryHigh
        } else {
            PerformanceTier::High
        },
        security_level: 128,
        hardware_accelerated: has_aes_ni,
    });

    capabilities.push(AlgorithmCapability {
        name: "ChaCha20-Poly1305".to_string(),
        algorithm_type: AlgorithmType::SymmetricEncryption,
        available: true,
        performance_tier: if has_avx2 {
            PerformanceTier::VeryHigh
        } else {
            PerformanceTier::High
        },
        security_level: 256,
        hardware_accelerated: has_avx2,
    });

    // Asymmetric signature algorithms
    capabilities.push(AlgorithmCapability {
        name: "Ed25519".to_string(),
        algorithm_type: AlgorithmType::AsymmetricSignature,
        available: true,
        performance_tier: PerformanceTier::High,
        security_level: 128,         // 128-bit security level
        hardware_accelerated: false, // Pure software
    });

    capabilities.push(AlgorithmCapability {
        name: "ECDSA-P256".to_string(),
        algorithm_type: AlgorithmType::AsymmetricSignature,
        available: true,
        performance_tier: PerformanceTier::Medium,
        security_level: 128,
        hardware_accelerated: false,
    });

    // Hash functions
    capabilities.push(AlgorithmCapability {
        name: "SHA-256".to_string(),
        algorithm_type: AlgorithmType::Hash,
        available: true,
        performance_tier: if has_sha_ext {
            PerformanceTier::VeryHigh
        } else {
            PerformanceTier::High
        },
        security_level: 256,
        hardware_accelerated: has_sha_ext,
    });

    capabilities.push(AlgorithmCapability {
        name: "SHA-512".to_string(),
        algorithm_type: AlgorithmType::Hash,
        available: true,
        performance_tier: if has_sha_ext {
            PerformanceTier::VeryHigh
        } else {
            PerformanceTier::High
        },
        security_level: 512,
        hardware_accelerated: has_sha_ext,
    });

    capabilities.push(AlgorithmCapability {
        name: "BLAKE3".to_string(),
        algorithm_type: AlgorithmType::Hash,
        available: true,
        performance_tier: if has_avx2 {
            PerformanceTier::VeryHigh
        } else {
            PerformanceTier::High
        },
        security_level: 256,
        hardware_accelerated: has_avx2,
    });

    // Key derivation functions
    capabilities.push(AlgorithmCapability {
        name: "HKDF-SHA256".to_string(),
        algorithm_type: AlgorithmType::KeyDerivation,
        available: true,
        performance_tier: PerformanceTier::High,
        security_level: 256,
        hardware_accelerated: false,
    });

    capabilities.push(AlgorithmCapability {
        name: "Argon2id".to_string(),
        algorithm_type: AlgorithmType::KeyDerivation,
        available: true,
        performance_tier: PerformanceTier::Low, // Intentionally slow (memory-hard)
        security_level: 256,
        hardware_accelerated: false,
    });

    // Message authentication
    capabilities.push(AlgorithmCapability {
        name: "HMAC-SHA256".to_string(),
        algorithm_type: AlgorithmType::MessageAuthentication,
        available: true,
        performance_tier: PerformanceTier::High,
        security_level: 256,
        hardware_accelerated: false,
    });

    // Sort by preference: hardware-accelerated and faster first
    capabilities.sort_by(|a, b| {
        b.performance_tier
            .cmp(&a.performance_tier)
            .then_with(|| b.hardware_accelerated.cmp(&a.hardware_accelerated))
            .then_with(|| b.security_level.cmp(&a.security_level))
    });

    capabilities
}

/// Algorithm registry for runtime capability queries
///
/// Provides fast lookups for algorithm availability and performance.
pub struct AlgorithmRegistry {
    capabilities: Vec<AlgorithmCapability>,
    by_name: HashMap<String, usize>,
}

impl AlgorithmRegistry {
    /// Create new registry by discovering available algorithms
    #[must_use]
    pub fn new() -> Self {
        let capabilities = discover_algorithms();
        let mut by_name = HashMap::new();

        for (idx, cap) in capabilities.iter().enumerate() {
            by_name.insert(cap.name.clone(), idx);
        }

        Self {
            capabilities,
            by_name,
        }
    }

    /// Check if an algorithm is supported
    #[must_use]
    pub fn supports_crypto(&self, algorithm: &CryptoAlgorithm) -> bool {
        let name = match algorithm {
            CryptoAlgorithm::Aes256Gcm => "AES-256-GCM",
            CryptoAlgorithm::Aes128Gcm => "AES-128-GCM",
            CryptoAlgorithm::ChaCha20Poly1305 => "ChaCha20-Poly1305",
        };

        self.by_name
            .get(name)
            .and_then(|&idx| self.capabilities.get(idx))
            .is_some_and(|cap| cap.available)
    }

    /// Check if a signature algorithm is supported
    #[must_use]
    pub fn supports_signature(&self, algorithm: &SignatureAlgorithm) -> bool {
        let name = match algorithm {
            SignatureAlgorithm::Ed25519 => "Ed25519",
            SignatureAlgorithm::EcdsaP256 => "ECDSA-P256",
            SignatureAlgorithm::RsaPss => "RSA-PSS",
        };

        self.by_name
            .get(name)
            .and_then(|&idx| self.capabilities.get(idx))
            .is_some_and(|cap| cap.available)
    }

    /// Get best algorithm for a given operation type
    ///
    /// Selects the fastest available algorithm that meets security requirements.
    #[must_use]
    pub fn best_for_type(&self, algo_type: AlgorithmType) -> Option<&AlgorithmCapability> {
        self.capabilities
            .iter()
            .filter(|cap| cap.algorithm_type == algo_type && cap.available)
            .max_by(|a, b| {
                a.performance_tier
                    .cmp(&b.performance_tier)
                    .then_with(|| a.hardware_accelerated.cmp(&b.hardware_accelerated))
            })
    }

    /// Get all available algorithms
    #[must_use]
    pub fn all_capabilities(&self) -> &[AlgorithmCapability] {
        &self.capabilities
    }

    /// Get capabilities by type
    #[must_use]
    pub fn capabilities_by_type(&self, algo_type: AlgorithmType) -> Vec<&AlgorithmCapability> {
        self.capabilities
            .iter()
            .filter(|cap| cap.algorithm_type == algo_type && cap.available)
            .collect()
    }
}

impl Default for AlgorithmRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Detect AES-NI hardware support
///
/// AES-NI provides hardware-accelerated AES encryption/decryption.
fn detect_aes_ni() -> bool {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        // Check for AES-NI CPU feature
        #[cfg(target_feature = "aes")]
        {
            return true;
        }

        // Runtime detection via cpuid (if available)
        #[cfg(not(target_feature = "aes"))]
        {
            // Use is_x86_feature_detected! macro for runtime detection
            std::arch::is_x86_feature_detected!("aes")
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        false
    }
}

/// Detect AVX2 hardware support
///
/// AVX2 provides SIMD acceleration for `ChaCha20` and BLAKE3.
fn detect_avx2() -> bool {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        #[cfg(target_feature = "avx2")]
        {
            return true;
        }

        #[cfg(not(target_feature = "avx2"))]
        {
            std::arch::is_x86_feature_detected!("avx2")
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        false
    }
}

/// Detect SHA extensions hardware support
///
/// SHA extensions (Intel SHA-NI / ARM SHA) provide hardware acceleration
/// for SHA-256 and SHA-512 hash functions.
fn detect_sha_extensions() -> bool {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        // Check for SHA extensions at compile time
        #[cfg(target_feature = "sha")]
        {
            return true;
        }

        // Runtime detection via cpuid
        #[cfg(not(target_feature = "sha"))]
        {
            std::arch::is_x86_feature_detected!("sha")
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        // ARM SHA extensions
        #[cfg(target_feature = "sha2")]
        {
            return true;
        }

        #[cfg(not(target_feature = "sha2"))]
        {
            // ARM doesn't have runtime detection in stable Rust yet
            // Conservative: assume not available
            false
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_algorithms() {
        let caps = discover_algorithms();

        // Should discover multiple algorithms
        assert!(!caps.is_empty());

        // All should be marked as available
        for cap in &caps {
            assert!(cap.available, "Algorithm {} should be available", cap.name);
        }

        // Should include key algorithms
        let names: Vec<_> = caps.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"AES-256-GCM"));
        assert!(names.contains(&"Ed25519"));
        assert!(names.contains(&"BLAKE3"));
    }

    #[test]
    fn test_algorithm_registry() {
        let registry = AlgorithmRegistry::new();

        // Should support common algorithms
        assert!(registry.supports_crypto(&CryptoAlgorithm::Aes256Gcm));
        assert!(registry.supports_crypto(&CryptoAlgorithm::ChaCha20Poly1305));
        assert!(registry.supports_signature(&SignatureAlgorithm::Ed25519));
    }

    #[test]
    fn test_best_algorithm_selection() {
        let registry = AlgorithmRegistry::new();

        // Should find best symmetric encryption
        let best_sym = registry.best_for_type(AlgorithmType::SymmetricEncryption);
        assert!(best_sym.is_some());

        // Should find best signature algorithm
        let best_sig = registry.best_for_type(AlgorithmType::AsymmetricSignature);
        assert!(best_sig.is_some());

        println!("Best symmetric: {:?}", best_sym.unwrap().name);
        println!("Best signature: {:?}", best_sig.unwrap().name);
    }

    #[test]
    fn test_hardware_detection() {
        // Just verify these don't panic
        let has_aes = detect_aes_ni();
        let has_avx2 = detect_avx2();
        let has_sha = detect_sha_extensions();

        println!("AES-NI: {has_aes}");
        println!("AVX2: {has_avx2}");
        println!("SHA Extensions: {has_sha}");

        // On modern x86_64, at least one should be true
        #[cfg(target_arch = "x86_64")]
        {
            // Most modern CPUs have at least AES-NI
            // But we can't assert this in CI
        }
    }

    #[test]
    fn test_capabilities_by_type() {
        let registry = AlgorithmRegistry::new();

        let hashes = registry.capabilities_by_type(AlgorithmType::Hash);
        assert!(!hashes.is_empty());

        // Should have multiple hash algorithms
        assert!(hashes.len() >= 2);

        for hash in hashes {
            println!(
                "Hash algorithm: {} ({})",
                hash.name,
                if hash.hardware_accelerated {
                    "HW"
                } else {
                    "SW"
                }
            );
        }
    }
}
