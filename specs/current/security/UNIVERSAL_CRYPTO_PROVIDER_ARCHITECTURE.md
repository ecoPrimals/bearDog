# Universal Crypto Provider Architecture

**Version**: 1.0  
**Date**: November 5, 2025  
**Status**: 🎯 **ARCHITECTURAL SPECIFICATION**  
**Pattern**: Vendor-Agnostic Crypto (mirrors Universal HSM success)

---

## 🎯 Executive Summary

Apply the same **vendor-agnostic pattern** that eliminated HSM lock-in to crypto providers. Instead of hardcoding support for specific crypto libraries, create a universal abstraction that supports ANY crypto implementation through runtime discovery and capability-based selection.

### Key Innovation
**Stop thinking**: "Which crypto library should we use?"  
**Start thinking**: "What crypto capabilities do we need, and let the system find them."

---

## 🏗️ Architecture Overview

### Design Philosophy

Just like our Universal HSM Architecture eliminated vendor lock-in, the Universal Crypto Provider eliminates **crypto library lock-in**.

```
┌─────────────────────────────────────────────────────────────┐
│                    HSM OPERATIONS                           │
│  • encrypt(key, data)                                      │
│  • decrypt(key, data)                                      │
│  • sign(key, data)                                         │
│  • verify(key, signature, data)                            │
└─────────────────┬───────────────────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────────────────┐
│         UNIVERSAL CRYPTO PROVIDER MANAGER                   │
│  • Capability Discovery Engine                             │
│  • Runtime Provider Registry                               │
│  • Algorithm-Based Selection                               │
│  • Fallback Chain                                          │
└─────────────────┬───────────────────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────────────────┐
│              VENDOR-AGNOSTIC TRAITS                        │
│  • UniversalCryptoProvider                                 │
│  • CryptoCapabilityDiscovery                               │
│  • AlgorithmSupport                                        │
└─────────────────┬───────────────────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────────────────┐
│           RUNTIME PROVIDER IMPLEMENTATIONS                 │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐          │
│  │ RustCrypto  │ │    Ring     │ │   OpenSSL   │          │
│  │  Provider   │ │  Provider   │ │  Provider   │          │
│  └─────────────┘ └─────────────┘ └─────────────┘          │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐          │
│  │   BoringSSL │ │   libsodium │ │   Custom    │          │
│  │  Provider   │ │  Provider   │ │  Provider   │          │
│  └─────────────┘ └─────────────┘ └─────────────┘          │
└─────────────────────────────────────────────────────────────┘
```

---

## 🔧 Core Trait System

### UniversalCryptoProvider Trait

```rust
#[async_trait]
pub trait UniversalCryptoProvider: Send + Sync + std::fmt::Debug {
    /// Provider identification
    fn provider_name(&self) -> &str;
    fn provider_version(&self) -> &str;
    
    /// Discover what algorithms this provider supports
    async fn discover_capabilities(&self) -> BearDogResult<CryptoCapabilities>;
    
    /// Check if this provider supports a specific algorithm
    async fn supports_algorithm(&self, algorithm: &CryptoAlgorithm) -> bool;
    
    /// Symmetric Encryption
    async fn encrypt_symmetric(
        &self,
        algorithm: SymmetricAlgorithm,
        key: &[u8],
        plaintext: &[u8],
        options: &EncryptionOptions,
    ) -> BearDogResult<EncryptedData>;
    
    async fn decrypt_symmetric(
        &self,
        algorithm: SymmetricAlgorithm,
        key: &[u8],
        ciphertext: &EncryptedData,
        options: &DecryptionOptions,
    ) -> BearDogResult<Vec<u8>>;
    
    /// Asymmetric Encryption
    async fn encrypt_asymmetric(
        &self,
        algorithm: AsymmetricAlgorithm,
        public_key: &[u8],
        plaintext: &[u8],
        options: &EncryptionOptions,
    ) -> BearDogResult<EncryptedData>;
    
    async fn decrypt_asymmetric(
        &self,
        algorithm: AsymmetricAlgorithm,
        private_key: &[u8],
        ciphertext: &EncryptedData,
        options: &DecryptionOptions,
    ) -> BearDogResult<Vec<u8>>;
    
    /// Digital Signatures
    async fn sign(
        &self,
        algorithm: SignatureAlgorithm,
        private_key: &[u8],
        message: &[u8],
        options: &SigningOptions,
    ) -> BearDogResult<Signature>;
    
    async fn verify(
        &self,
        algorithm: SignatureAlgorithm,
        public_key: &[u8],
        message: &[u8],
        signature: &Signature,
        options: &VerificationOptions,
    ) -> BearDogResult<bool>;
    
    /// Key Derivation
    async fn derive_key(
        &self,
        algorithm: KdfAlgorithm,
        input_key: &[u8],
        salt: &[u8],
        info: &[u8],
        output_length: usize,
    ) -> BearDogResult<Vec<u8>>;
    
    /// Hashing
    async fn hash(
        &self,
        algorithm: HashAlgorithm,
        data: &[u8],
    ) -> BearDogResult<Vec<u8>>;
    
    /// Performance characteristics
    async fn get_performance_profile(&self) -> BearDogResult<PerformanceProfile>;
}
```

---

## 📋 Algorithm Abstraction

### Universal Algorithm Types

```rust
/// Symmetric encryption algorithms
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymmetricAlgorithm {
    Aes { mode: AesMode, key_size: u32 },
    ChaCha20Poly1305,
    ChaCha20 { key_size: u32 },
    Aes256Gcm,
    Aes128Gcm,
    Custom { name: String, spec: AlgorithmSpec },
}

/// Asymmetric encryption algorithms
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AsymmetricAlgorithm {
    RsaOaep { key_size: u32, hash: HashAlgorithm },
    RsaPkcs1v15 { key_size: u32 },
    EciesP256,
    EciesP384,
    Custom { name: String, spec: AlgorithmSpec },
}

/// Signature algorithms
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SignatureAlgorithm {
    Ed25519,
    EcdsaP256 { hash: HashAlgorithm },
    EcdsaP384 { hash: HashAlgorithm },
    RsaPss { key_size: u32, hash: HashAlgorithm },
    RsaPkcs1v15 { key_size: u32, hash: HashAlgorithm },
    Custom { name: String, spec: AlgorithmSpec },
}

/// Hash algorithms
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HashAlgorithm {
    Sha256,
    Sha384,
    Sha512,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Blake2b { output_size: usize },
    Blake2s { output_size: usize },
    Blake3,
    Custom { name: String, spec: AlgorithmSpec },
}

/// Key derivation algorithms
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KdfAlgorithm {
    HkdfSha256,
    HkdfSha384,
    HkdfSha512,
    Pbkdf2 { hash: HashAlgorithm, iterations: u32 },
    Scrypt { n: u64, r: u32, p: u32 },
    Argon2 { variant: Argon2Variant },
    Custom { name: String, spec: AlgorithmSpec },
}
```

### Key Feature: Custom Algorithm Support

Notice the `Custom` variants - this allows adding **any** algorithm without code changes!

```rust
pub struct AlgorithmSpec {
    pub name: String,
    pub category: AlgorithmCategory,
    pub key_sizes: Vec<u32>,
    pub block_sizes: Vec<u32>,
    pub parameters: HashMap<String, String>,
}
```

---

## 🎯 Capability Discovery

### CryptoCapabilities Structure

```rust
pub struct CryptoCapabilities {
    pub provider_name: String,
    pub provider_version: String,
    
    /// What this provider can do
    pub symmetric_algorithms: Vec<SymmetricAlgorithm>,
    pub asymmetric_algorithms: Vec<AsymmetricAlgorithm>,
    pub signature_algorithms: Vec<SignatureAlgorithm>,
    pub hash_algorithms: Vec<HashAlgorithm>,
    pub kdf_algorithms: Vec<KdfAlgorithm>,
    
    /// Performance characteristics
    pub performance_profile: PerformanceProfile,
    
    /// Security features
    pub side_channel_resistance: SideChannelResistance,
    pub constant_time_ops: Vec<String>,
    
    /// Platform support
    pub supported_platforms: Vec<Platform>,
    pub hardware_acceleration: Vec<HardwareFeature>,
}

pub struct PerformanceProfile {
    pub throughput_mbps: HashMap<String, f64>,
    pub latency_us: HashMap<String, f64>,
    pub memory_overhead_bytes: usize,
}

pub enum SideChannelResistance {
    None,
    Partial,
    Full,
}
```

---

## 🔄 Runtime Provider Selection

### Selection Strategy

```rust
pub struct CryptoProviderManager {
    providers: Vec<Box<dyn UniversalCryptoProvider>>,
    capabilities_cache: HashMap<String, CryptoCapabilities>,
}

impl CryptoProviderManager {
    /// Find the best provider for a specific operation
    pub async fn select_provider(
        &self,
        requirements: &CryptoRequirements,
    ) -> BearDogResult<&dyn UniversalCryptoProvider> {
        let candidates = self.providers
            .iter()
            .filter(|p| self.meets_requirements(p, requirements))
            .collect::<Vec<_>>();
        
        if candidates.is_empty() {
            return Err(BearDogError::no_suitable_provider(
                format!("No provider supports: {:?}", requirements)
            ));
        }
        
        // Select based on priorities
        self.rank_providers(candidates, requirements)
            .first()
            .ok_or_else(|| BearDogError::internal("Provider ranking failed"))
    }
    
    fn rank_providers(
        &self,
        candidates: Vec<&Box<dyn UniversalCryptoProvider>>,
        requirements: &CryptoRequirements,
    ) -> Vec<&dyn UniversalCryptoProvider> {
        let mut scored: Vec<_> = candidates
            .into_iter()
            .map(|p| {
                let score = self.calculate_score(p, requirements);
                (p, score)
            })
            .collect();
        
        // Sort by score (descending)
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        scored.into_iter().map(|(p, _)| p.as_ref()).collect()
    }
}
```

### CryptoRequirements Structure

```rust
pub struct CryptoRequirements {
    /// What operation needs to be performed
    pub operation: CryptoOperation,
    pub algorithm: Option<CryptoAlgorithm>,
    
    /// Security requirements
    pub min_security_level: SecurityLevel,
    pub require_constant_time: bool,
    pub side_channel_protection: bool,
    
    /// Performance requirements
    pub max_latency_us: Option<u64>,
    pub min_throughput_mbps: Option<f64>,
    
    /// Platform requirements
    pub require_hardware_accel: bool,
    pub platform_specific: Option<Platform>,
    
    /// Preference ordering
    pub prefer_performance: bool, // vs prefer_security
}
```

---

## 🔌 Provider Implementations

### Example: RustCrypto Provider

```rust
pub struct RustCryptoProvider {
    capabilities: CryptoCapabilities,
}

#[async_trait]
impl UniversalCryptoProvider for RustCryptoProvider {
    fn provider_name(&self) -> &str {
        "RustCrypto"
    }
    
    fn provider_version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }
    
    async fn discover_capabilities(&self) -> BearDogResult<CryptoCapabilities> {
        Ok(CryptoCapabilities {
            provider_name: self.provider_name().to_string(),
            provider_version: self.provider_version().to_string(),
            
            symmetric_algorithms: vec![
                SymmetricAlgorithm::Aes { 
                    mode: AesMode::Gcm, 
                    key_size: 256 
                },
                SymmetricAlgorithm::ChaCha20Poly1305,
                // ... more algorithms
            ],
            
            signature_algorithms: vec![
                SignatureAlgorithm::Ed25519,
                SignatureAlgorithm::EcdsaP256 { 
                    hash: HashAlgorithm::Sha256 
                },
                // ... more algorithms
            ],
            
            hash_algorithms: vec![
                HashAlgorithm::Sha256,
                HashAlgorithm::Sha384,
                HashAlgorithm::Sha512,
                HashAlgorithm::Blake3,
                // ... more algorithms
            ],
            
            performance_profile: PerformanceProfile {
                throughput_mbps: [
                    ("aes-256-gcm".to_string(), 1500.0),
                    ("chacha20".to_string(), 2000.0),
                ].into_iter().collect(),
                latency_us: [
                    ("aes-256-gcm".to_string(), 5.0),
                    ("chacha20".to_string(), 3.0),
                ].into_iter().collect(),
                memory_overhead_bytes: 4096,
            },
            
            side_channel_resistance: SideChannelResistance::Partial,
            constant_time_ops: vec![
                "chacha20".to_string(),
                "ed25519".to_string(),
            ],
            
            supported_platforms: vec![
                Platform::Linux,
                Platform::MacOs,
                Platform::Windows,
            ],
            
            hardware_acceleration: vec![
                HardwareFeature::AesNi,
            ],
        })
    }
    
    async fn supports_algorithm(&self, algorithm: &CryptoAlgorithm) -> bool {
        match algorithm {
            CryptoAlgorithm::Symmetric(sym) => {
                matches!(sym,
                    SymmetricAlgorithm::Aes { mode: AesMode::Gcm, .. } |
                    SymmetricAlgorithm::ChaCha20Poly1305
                )
            }
            CryptoAlgorithm::Signature(sig) => {
                matches!(sig,
                    SignatureAlgorithm::Ed25519 |
                    SignatureAlgorithm::EcdsaP256 { .. }
                )
            }
            // ... more cases
            _ => false,
        }
    }
    
    async fn encrypt_symmetric(
        &self,
        algorithm: SymmetricAlgorithm,
        key: &[u8],
        plaintext: &[u8],
        options: &EncryptionOptions,
    ) -> BearDogResult<EncryptedData> {
        match algorithm {
            SymmetricAlgorithm::Aes { mode: AesMode::Gcm, key_size: 256 } => {
                use aes_gcm::{Aes256Gcm, Nonce, KeyInit};
                use aes_gcm::aead::Aead;
                
                let cipher = Aes256Gcm::new_from_slice(key)
                    .map_err(|e| BearDogError::crypto_error(format!("Invalid key: {}", e)))?;
                
                let nonce = options.nonce
                    .unwrap_or_else(|| self.generate_nonce(12));
                
                let ciphertext = cipher.encrypt(Nonce::from_slice(&nonce), plaintext)
                    .map_err(|e| BearDogError::crypto_error(format!("Encryption failed: {}", e)))?;
                
                Ok(EncryptedData {
                    algorithm: algorithm.to_string(),
                    ciphertext,
                    nonce: Some(nonce),
                    tag: None, // GCM includes tag in ciphertext
                })
            }
            
            SymmetricAlgorithm::ChaCha20Poly1305 => {
                use chacha20poly1305::{ChaCha20Poly1305, Nonce, KeyInit};
                use chacha20poly1305::aead::Aead;
                
                let cipher = ChaCha20Poly1305::new_from_slice(key)
                    .map_err(|e| BearDogError::crypto_error(format!("Invalid key: {}", e)))?;
                
                let nonce = options.nonce
                    .unwrap_or_else(|| self.generate_nonce(12));
                
                let ciphertext = cipher.encrypt(Nonce::from_slice(&nonce), plaintext)
                    .map_err(|e| BearDogError::crypto_error(format!("Encryption failed: {}", e)))?;
                
                Ok(EncryptedData {
                    algorithm: algorithm.to_string(),
                    ciphertext,
                    nonce: Some(nonce),
                    tag: None,
                })
            }
            
            _ => Err(BearDogError::unsupported_algorithm(
                format!("RustCrypto doesn't support: {:?}", algorithm)
            )),
        }
    }
    
    // ... implement other methods
}
```

---

## 🔗 Integration with HSM System

### Bridging HSM Keys to Crypto Operations

```rust
impl RustSoftwareHsm {
    pub async fn encrypt(
        &self,
        key_id: &str,
        plaintext: &[u8],
    ) -> BearDogResult<Vec<u8>> {
        // 1. Get the HSM key
        let key = self.key_store.get_key(key_id).await?;
        
        // 2. Determine crypto requirements from key metadata
        let requirements = CryptoRequirements::from_key_metadata(&key.metadata);
        
        // 3. Select appropriate crypto provider
        let crypto_provider = self.crypto_manager
            .select_provider(&requirements)
            .await?;
        
        // 4. Extract key material
        let key_material = self.memory_protector
            .unprotect(key.key_material.data())
            .await?;
        
        // 5. Perform encryption using selected provider
        let encrypted_data = crypto_provider.encrypt_symmetric(
            requirements.algorithm.as_symmetric()?,
            &key_material,
            plaintext,
            &EncryptionOptions::default(),
        ).await?;
        
        // 6. Zeroize key material
        self.memory_protector.zeroize(&key_material).await?;
        
        Ok(encrypted_data.ciphertext)
    }
}

impl CryptoRequirements {
    pub fn from_key_metadata(metadata: &KeyMetadata) -> Self {
        let (operation, algorithm) = match &metadata.key_type {
            KeyType::Aes { key_size } => (
                CryptoOperation::SymmetricEncryption,
                CryptoAlgorithm::Symmetric(
                    SymmetricAlgorithm::Aes {
                        mode: AesMode::Gcm,
                        key_size: *key_size,
                    }
                ),
            ),
            KeyType::Ed25519 => (
                CryptoOperation::Signing,
                CryptoAlgorithm::Signature(SignatureAlgorithm::Ed25519),
            ),
            KeyType::EccP256 => (
                CryptoOperation::Signing,
                CryptoAlgorithm::Signature(
                    SignatureAlgorithm::EcdsaP256 {
                        hash: HashAlgorithm::Sha256,
                    }
                ),
            ),
            // ... more types
        };
        
        Self {
            operation,
            algorithm: Some(algorithm),
            min_security_level: metadata.security_level.clone(),
            require_constant_time: true,
            side_channel_protection: true,
            max_latency_us: None,
            min_throughput_mbps: None,
            require_hardware_accel: false,
            platform_specific: None,
            prefer_performance: false,
        }
    }
}
```

---

## 🎯 Benefits of This Architecture

### 1. **Zero Crypto Library Lock-In**
- Switch crypto backends without code changes
- Use multiple crypto libraries simultaneously
- Add new crypto libraries at runtime

### 2. **Automatic Best Selection**
- System picks the best crypto provider for each operation
- Based on capabilities, performance, security level
- Fallback chains for resilience

### 3. **Future-Proof**
- New algorithms via `Custom` variants
- New providers via trait implementation
- No breaking changes to API

### 4. **Consistent with HSM Architecture**
- Same patterns as Universal HSM
- Familiar to team
- Proven approach

### 5. **Testability**
- Mock providers for testing
- Multiple real providers for validation
- Clear interface contracts

### 6. **Performance Flexibility**
- Hardware-accelerated when available
- Pure software fallback
- Per-operation optimization

---

## 📊 Comparison with Previous Options

| Aspect | Option A<br/>(Extend KeyType) | Option B<br/>(Update Providers) | Option C<br/>(Adapter Layer) | **Universal Crypto<br/>(New Approach)** |
|--------|------|------|------|------|
| **Vendor Lock-in** | ❌ Still locked | ❌ Still locked | ❌ Still locked | ✅ **Zero lock-in** |
| **Add New Crypto** | ❌ Code changes | ❌ Code changes | ⚠️ Adapter update | ✅ **Runtime addition** |
| **Multiple Backends** | ❌ Hard to support | ❌ Hard to support | ⚠️ Complex | ✅ **Built-in** |
| **Best Selection** | ❌ Manual | ❌ Manual | ❌ Manual | ✅ **Automatic** |
| **Architecture Consistency** | ⚠️ Different pattern | ⚠️ Different pattern | ⚠️ Different pattern | ✅ **Mirrors HSM** |
| **Implementation Time** | 2-4 hours | 4-6 hours | 2-3 hours | **6-10 hours** |
| **Long-term Maintainability** | ⚠️ Medium | ⚠️ Low | ⚠️ Medium | ✅ **Excellent** |
| **Testability** | ⚠️ Medium | ⚠️ Medium | ⚠️ Medium | ✅ **Excellent** |

**Winner**: Universal Crypto Provider - worth the extra implementation time!

---

## 🗓️ Implementation Plan

### Phase 1: Core Abstractions (2-3 hours)
1. [ ] Define `UniversalCryptoProvider` trait
2. [ ] Define algorithm enums (Symmetric, Asymmetric, Signature, etc.)
3. [ ] Define `CryptoCapabilities` structure
4. [ ] Define `CryptoRequirements` structure
5. [ ] Create `CryptoProviderManager`

### Phase 2: First Provider (2-3 hours)
1. [ ] Implement `RustCryptoProvider`
2. [ ] Support AES-256-GCM
3. [ ] Support Ed25519
4. [ ] Support ECDSA P-256
5. [ ] Add capability discovery

### Phase 3: Integration (2-3 hours)
1. [ ] Wire up to `RustSoftwareHsm`
2. [ ] Update encrypt/decrypt methods
3. [ ] Update sign/verify methods
4. [ ] Add provider selection logic
5. [ ] Test integration

### Phase 4: Additional Providers (1-2 hours each)
1. [ ] Implement `RingProvider`
2. [ ] Implement `OpenSslProvider` (optional)
3. [ ] Add performance benchmarks
4. [ ] Document provider differences

**Total Time**: 6-10 hours (initial implementation)  
**Future Cost**: Near-zero (add providers via trait implementation)

---

## 🧪 Testing Strategy

### Provider Tests
```rust
#[tokio::test]
async fn test_rustcrypto_provider_discovery() {
    let provider = RustCryptoProvider::new();
    let caps = provider.discover_capabilities().await.unwrap();
    
    assert!(caps.symmetric_algorithms.contains(&SymmetricAlgorithm::Aes {
        mode: AesMode::Gcm,
        key_size: 256,
    }));
    assert!(caps.signature_algorithms.contains(&SignatureAlgorithm::Ed25519));
}

#[tokio::test]
async fn test_provider_selection() {
    let manager = CryptoProviderManager::new();
    manager.register_provider(Box::new(RustCryptoProvider::new()));
    manager.register_provider(Box::new(RingProvider::new()));
    
    let requirements = CryptoRequirements {
        operation: CryptoOperation::SymmetricEncryption,
        algorithm: Some(CryptoAlgorithm::Symmetric(
            SymmetricAlgorithm::ChaCha20Poly1305
        )),
        prefer_performance: true,
        ..Default::default()
    };
    
    let provider = manager.select_provider(&requirements).await.unwrap();
    // Should select the fastest provider for ChaCha20
}

#[tokio::test]
async fn test_multiple_providers_same_algorithm() {
    // Test that both RustCrypto and Ring can do AES-GCM
    // and system picks the best one
}
```

---

## 📚 Documentation Requirements

### For Users
- Clear guide on selecting providers
- Performance comparison table
- Security comparison table
- Migration guide from hardcoded crypto

### For Developers
- How to implement a new provider
- Capability discovery best practices
- Algorithm naming conventions
- Testing guidelines

---

## 🌟 Success Criteria

### Technical
- [ ] Zero hardcoded crypto library dependencies in HSM code
- [ ] Can switch providers via configuration
- [ ] Can run multiple providers simultaneously
- [ ] All 4 failing tests pass
- [ ] Performance within 10% of direct library use

### Architectural
- [ ] Consistent with Universal HSM pattern
- [ ] Clear trait boundaries
- [ ] Runtime discovery working
- [ ] Selection logic validated

### Quality
- [ ] 100% test coverage of trait implementations
- [ ] Documentation complete
- [ ] Migration path clear
- [ ] Team understands pattern

---

## 💡 Future Enhancements

### Possible Extensions
1. **Provider Plugins**: Load crypto providers as dynamic libraries
2. **Cloud KMS Integration**: Treat cloud KMS as crypto providers
3. **Hardware Crypto**: Integrate hardware crypto accelerators
4. **Streaming Operations**: Support streaming encrypt/decrypt
5. **Batch Operations**: Optimize batch crypto operations

---

## 🎓 Learning from HSM Evolution

### What We Learned from Universal HSM
1. **Trait-based abstraction** eliminates vendor lock-in ✅
2. **Runtime discovery** provides flexibility ✅
3. **Capability-based selection** is powerful ✅
4. **Zero hardcoding** enables future growth ✅

### Applying to Crypto Providers
1. **Same trait pattern** for consistency ✅
2. **Same discovery pattern** for crypto capabilities ✅
3. **Same selection logic** for algorithm requirements ✅
4. **Same zero-hardcoding principle** for new algorithms ✅

**Result**: A crypto system that's as vendor-agnostic as our HSM system!

---

## 📋 Decision

**Recommended Approach**: **Universal Crypto Provider Architecture**

**Rationale**:
- Eliminates all 3 issues with previous options
- Consistent with proven Universal HSM pattern
- Future-proofs the entire crypto stack
- Worth the extra implementation time
- Establishes pattern for future abstractions

**Next Steps**:
1. Review and approve this specification
2. Begin Phase 1 implementation (core abstractions)
3. Implement Phase 2 (RustCrypto provider)
4. Integrate Phase 3 (wire to HSM)
5. Expand Phase 4 (additional providers)

---

**Status**: 🎯 **READY FOR IMPLEMENTATION**  
**Pattern**: Proven (mirrors Universal HSM success)  
**Timeline**: 6-10 hours initial, near-zero future cost  
**Impact**: Eliminates crypto library lock-in forever

**Last Updated**: November 5, 2025  
**Author**: BearDog Development Team

