# BearDog HSM Integration Specification

**Version**: 1.0  
**Date**: January 2025  
**Status**: SPECIFICATION  
**Priority**: HIGH  

## 🎯 Executive Summary

BearDog's Hardware Security Module (HSM) integration provides enterprise-grade key management across multiple HSM form factors:

- **Smartphone HSM**: Leveraging mobile device secure enclaves (iOS Secure Enclave, Android StrongBox)
- **Software HSM**: Rust-based software HSM with secure key storage and operations
- **Hardware HSM**: Traditional HSM integration (CloudHSM, Luna, nShield)
- **Hybrid HSM**: Multi-tier HSM architecture for optimal security and performance

## 🔐 HSM Architecture Overview

### 1.1 Multi-Tier HSM Strategy

```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HsmTier {
    /// Tier 1: Smartphone HSM (Most Accessible)
    /// - iOS Secure Enclave, Android StrongBox
    /// - Always available, user-controlled
    /// - Good for personal keys, authentication
    SmartphoneHsm {
        device_type: SmartphoneType,
        secure_enclave: SecureEnclaveType,
        attestation_level: AttestationLevel,
        user_presence_required: bool,
    },
    
    /// Tier 2: Software HSM (Scalable)
    /// - Rust-based implementation
    /// - Encrypted key storage
    /// - Good for development, testing, light production
    SoftwareHsm {
        implementation: SoftwareHsmType,
        key_storage: KeyStorageType,
        encryption_at_rest: bool,
        memory_protection: MemoryProtectionLevel,
    },
    
    /// Tier 3: Hardware HSM (Highest Security)
    /// - FIPS 140-2 Level 3+ certified
    /// - Tamper-resistant hardware
    /// - Good for root keys, high-value operations
    HardwareHsm {
        vendor: HsmVendor,
        model: String,
        certification: CertificationLevel,
        tamper_resistance: TamperResistanceLevel,
    },
    
    /// Tier 4: Hybrid HSM (Best of All Worlds)
    /// - Combines multiple HSM types
    /// - Hierarchical key management
    /// - Optimal security and performance
    HybridHsm {
        tiers: Vec<HsmTier>,
        key_hierarchy: KeyHierarchy,
        fallback_strategy: FallbackStrategy,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SmartphoneType {
    IPhone {
        model: String,
        ios_version: String,
        secure_enclave_version: String,
    },
    Android {
        manufacturer: String,
        model: String,
        android_version: String,
        strongbox_version: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecureEnclaveType {
    /// iOS Secure Enclave
    IosSecureEnclave {
        chip_type: String, // A-series, M-series
        biometric_support: bool,
        key_attestation: bool,
    },
    
    /// Android StrongBox
    AndroidStrongBox {
        implementation: StrongBoxImplementation,
        hardware_backed: bool,
        key_attestation: bool,
    },
    
    /// Generic trusted execution environment
    TrustedExecutionEnvironment {
        vendor: String,
        tee_type: String,
        certification: Option<String>,
    },
}
```

### 1.2 HSM Provider Interface

```rust
/// Universal HSM interface for all HSM types
#[async_trait]
pub trait HsmProvider: Send + Sync {
    /// Initialize the HSM connection
    async fn initialize(&self, config: HsmConfig) -> BearDogResult<()>;
    
    /// Generate a new key in the HSM
    async fn generate_key(&self, request: GenerateKeyRequest) -> BearDogResult<HsmKey>;
    
    /// Import an existing key into the HSM
    async fn import_key(&self, key_data: &[u8], metadata: KeyMetadata) -> BearDogResult<HsmKey>;
    
    /// Encrypt data using HSM key
    async fn encrypt(&self, key_id: &str, plaintext: &[u8]) -> BearDogResult<Vec<u8>>;
    
    /// Decrypt data using HSM key
    async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> BearDogResult<Vec<u8>>;
    
    /// Sign data using HSM key
    async fn sign(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>>;
    
    /// Verify signature using HSM key
    async fn verify(&self, key_id: &str, data: &[u8], signature: &[u8]) -> BearDogResult<bool>;
    
    /// Derive key using HSM-based KDF
    async fn derive_key(&self, master_key_id: &str, derivation_data: &[u8]) -> BearDogResult<HsmKey>;
    
    /// Get HSM information and capabilities
    async fn get_info(&self) -> BearDogResult<HsmInfo>;
    
    /// List keys stored in HSM
    async fn list_keys(&self) -> BearDogResult<Vec<HsmKeyInfo>>;
    
    /// Delete key from HSM
    async fn delete_key(&self, key_id: &str) -> BearDogResult<()>;
    
    /// Backup HSM state (if supported)
    async fn backup(&self) -> BearDogResult<Option<Vec<u8>>>;
    
    /// Restore HSM state (if supported)
    async fn restore(&self, backup_data: &[u8]) -> BearDogResult<()>;
}
```

## 📱 Smartphone HSM Integration

### 2.1 iOS Secure Enclave Integration

```rust
/// iOS Secure Enclave HSM provider
pub struct IosSecureEnclaveHsm {
    config: IosHsmConfig,
    keychain: SecKeychain,
    attestation_service: AttestationService,
}

impl IosSecureEnclaveHsm {
    pub async fn new(config: IosHsmConfig) -> BearDogResult<Self> {
        // Initialize keychain access
        let keychain = SecKeychain::default();
        
        // Initialize attestation service
        let attestation_service = AttestationService::new(&config.attestation_config)?;
        
        Ok(Self {
            config,
            keychain,
            attestation_service,
        })
    }
    
    /// Generate key in iOS Secure Enclave
    async fn generate_secure_enclave_key(
        &self,
        request: &GenerateKeyRequest,
    ) -> BearDogResult<HsmKey> {
        // Configure key attributes for Secure Enclave
        let key_attributes = self.configure_secure_enclave_attributes(request)?;
        
        // Generate key in Secure Enclave
        let key_ref = unsafe {
            SecKeyCreateRandomKey(
                key_attributes.as_dictionary(),
                std::ptr::null_mut(),
            )
        };
        
        if key_ref.is_null() {
            return Err(BearDogError::HsmKeyGenerationFailed {
                hsm_type: "iOS Secure Enclave".to_string(),
                error: "SecKeyCreateRandomKey failed".to_string(),
            });
        }
        
        // Store key in keychain with appropriate access control
        let key_id = self.store_key_in_keychain(key_ref, request).await?;
        
        // Generate key attestation
        let attestation = self.attestation_service.attest_key(&key_id).await?;
        
        Ok(HsmKey {
            id: key_id,
            hsm_type: HsmTier::SmartphoneHsm {
                device_type: SmartphoneType::IPhone {
                    model: self.config.device_model.clone(),
                    ios_version: self.config.ios_version.clone(),
                    secure_enclave_version: self.config.secure_enclave_version.clone(),
                },
                secure_enclave: SecureEnclaveType::IosSecureEnclave {
                    chip_type: self.config.chip_type.clone(),
                    biometric_support: self.config.biometric_support,
                    key_attestation: true,
                },
                attestation_level: AttestationLevel::Hardware,
                user_presence_required: request.require_user_presence,
            },
            key_type: request.key_type.clone(),
            attestation: Some(attestation),
            created_at: Utc::now(),
            metadata: request.metadata.clone(),
        })
    }
    
    /// Configure key attributes for Secure Enclave storage
    fn configure_secure_enclave_attributes(
        &self,
        request: &GenerateKeyRequest,
    ) -> BearDogResult<CFDictionary> {
        let mut attributes = CFMutableDictionary::new();
        
        // Key type
        match request.key_type {
            KeyType::EccP256 => {
                attributes.set(kSecAttrKeyType, kSecAttrKeyTypeECSECPrimeRandom);
                attributes.set(kSecAttrKeySizeInBits, CFNumber::from(256));
            },
            KeyType::EccP384 => {
                attributes.set(kSecAttrKeyType, kSecAttrKeyTypeECSECPrimeRandom);
                attributes.set(kSecAttrKeySizeInBits, CFNumber::from(384));
            },
            _ => return Err(BearDogError::UnsupportedKeyType {
                key_type: request.key_type.clone(),
                hsm_type: "iOS Secure Enclave".to_string(),
            }),
        }
        
        // Require Secure Enclave
        attributes.set(kSecAttrTokenID, kSecAttrTokenIDSecureEnclave);
        
        // Access control
        let access_control = if request.require_user_presence {
            SecAccessControlCreateWithFlags(
                kCFAllocatorDefault,
                kSecAttrAccessibleWhenUnlockedThisDeviceOnly,
                kSecAccessControlBiometryAny,
                std::ptr::null_mut(),
            )
        } else {
            SecAccessControlCreateWithFlags(
                kCFAllocatorDefault,
                kSecAttrAccessibleWhenUnlockedThisDeviceOnly,
                0,
                std::ptr::null_mut(),
            )
        };
        
        if !access_control.is_null() {
            attributes.set(kSecAttrAccessControl, access_control);
        }
        
        // Private key attributes
        let private_key_attrs = CFMutableDictionary::new();
        private_key_attrs.set(kSecAttrIsPermanent, CFBoolean::true_value());
        private_key_attrs.set(kSecAttrApplicationTag, CFString::new(&request.key_id));
        
        attributes.set(kSecPrivateKeyAttrs, private_key_attrs);
        
        Ok(attributes.to_immutable())
    }
}

#[async_trait]
impl HsmProvider for IosSecureEnclaveHsm {
    async fn initialize(&self, config: HsmConfig) -> BearDogResult<()> {
        // Verify Secure Enclave availability
        if !self.is_secure_enclave_available() {
            return Err(BearDogError::HsmUnavailable {
                hsm_type: "iOS Secure Enclave".to_string(),
                reason: "Secure Enclave not available on this device".to_string(),
            });
        }
        
        // Test keychain access
        self.test_keychain_access().await?;
        
        // Initialize attestation service
        self.attestation_service.initialize().await?;
        
        Ok(())
    }
    
    async fn generate_key(&self, request: GenerateKeyRequest) -> BearDogResult<HsmKey> {
        self.generate_secure_enclave_key(&request).await
    }
    
    async fn encrypt(&self, key_id: &str, plaintext: &[u8]) -> BearDogResult<Vec<u8>> {
        // Retrieve key from keychain
        let key_ref = self.retrieve_key_from_keychain(key_id).await?;
        
        // Encrypt using Secure Enclave
        let ciphertext = unsafe {
            SecKeyCreateEncryptedData(
                key_ref,
                kSecKeyAlgorithmECIESEncryptionCofactorVariableIVX963SHA256AESGCM,
                CFData::from_buffer(plaintext),
                std::ptr::null_mut(),
            )
        };
        
        if ciphertext.is_null() {
            return Err(BearDogError::HsmEncryptionFailed {
                hsm_type: "iOS Secure Enclave".to_string(),
                key_id: key_id.to_string(),
                error: "SecKeyCreateEncryptedData failed".to_string(),
            });
        }
        
        Ok(ciphertext.to_vec())
    }
    
    async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> BearDogResult<Vec<u8>> {
        // Retrieve key from keychain
        let key_ref = self.retrieve_key_from_keychain(key_id).await?;
        
        // Decrypt using Secure Enclave
        let plaintext = unsafe {
            SecKeyCreateDecryptedData(
                key_ref,
                kSecKeyAlgorithmECIESEncryptionCofactorVariableIVX963SHA256AESGCM,
                CFData::from_buffer(ciphertext),
                std::ptr::null_mut(),
            )
        };
        
        if plaintext.is_null() {
            return Err(BearDogError::HsmDecryptionFailed {
                hsm_type: "iOS Secure Enclave".to_string(),
                key_id: key_id.to_string(),
                error: "SecKeyCreateDecryptedData failed".to_string(),
            });
        }
        
        Ok(plaintext.to_vec())
    }
    
    async fn sign(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        // Retrieve key from keychain
        let key_ref = self.retrieve_key_from_keychain(key_id).await?;
        
        // Sign using Secure Enclave
        let signature = unsafe {
            SecKeyCreateSignature(
                key_ref,
                kSecKeyAlgorithmECDSASignatureMessageX962SHA256,
                CFData::from_buffer(data),
                std::ptr::null_mut(),
            )
        };
        
        if signature.is_null() {
            return Err(BearDogError::HsmSigningFailed {
                hsm_type: "iOS Secure Enclave".to_string(),
                key_id: key_id.to_string(),
                error: "SecKeyCreateSignature failed".to_string(),
            });
        }
        
        Ok(signature.to_vec())
    }
    
    async fn get_info(&self) -> BearDogResult<HsmInfo> {
        Ok(HsmInfo {
            hsm_type: "iOS Secure Enclave".to_string(),
            vendor: "Apple".to_string(),
            model: self.config.device_model.clone(),
            firmware_version: self.config.ios_version.clone(),
            capabilities: vec![
                HsmCapability::KeyGeneration,
                HsmCapability::Encryption,
                HsmCapability::Decryption,
                HsmCapability::Signing,
                HsmCapability::KeyAttestation,
                HsmCapability::BiometricAuthentication,
            ],
            supported_algorithms: vec![
                Algorithm::EccP256,
                Algorithm::EccP384,
                Algorithm::EcdsaSha256,
                Algorithm::EciesGcm,
            ],
            max_key_size: Some(384),
            certification: Some("CC EAL4+".to_string()),
            tamper_resistance: TamperResistanceLevel::Hardware,
        })
    }
}
```

### 2.2 Android StrongBox Integration

```rust
/// Android StrongBox HSM provider
pub struct AndroidStrongBoxHsm {
    config: AndroidHsmConfig,
    keystore: AndroidKeystore,
    attestation_service: AndroidAttestationService,
}

impl AndroidStrongBoxHsm {
    pub async fn new(config: AndroidHsmConfig) -> BearDogResult<Self> {
        // Initialize Android Keystore
        let keystore = AndroidKeystore::new(&config.keystore_config)?;
        
        // Initialize attestation service
        let attestation_service = AndroidAttestationService::new(&config.attestation_config)?;
        
        Ok(Self {
            config,
            keystore,
            attestation_service,
        })
    }
    
    /// Generate key in Android StrongBox
    async fn generate_strongbox_key(
        &self,
        request: &GenerateKeyRequest,
    ) -> BearDogResult<HsmKey> {
        // Configure key parameters for StrongBox
        let key_params = self.configure_strongbox_parameters(request)?;
        
        // Generate key in StrongBox
        let key_alias = format!("beardog_{}", request.key_id);
        let key_generator = self.keystore.get_key_generator(&key_params.algorithm)?;
        
        key_generator.init(&key_params)?;
        let secret_key = key_generator.generate_key(&key_alias)?;
        
        // Generate key attestation
        let attestation = self.attestation_service.attest_key(&key_alias).await?;
        
        Ok(HsmKey {
            id: key_alias,
            hsm_type: HsmTier::SmartphoneHsm {
                device_type: SmartphoneType::Android {
                    manufacturer: self.config.manufacturer.clone(),
                    model: self.config.model.clone(),
                    android_version: self.config.android_version.clone(),
                    strongbox_version: self.config.strongbox_version.clone(),
                },
                secure_enclave: SecureEnclaveType::AndroidStrongBox {
                    implementation: self.config.strongbox_implementation.clone(),
                    hardware_backed: true,
                    key_attestation: true,
                },
                attestation_level: AttestationLevel::Hardware,
                user_presence_required: request.require_user_presence,
            },
            key_type: request.key_type.clone(),
            attestation: Some(attestation),
            created_at: Utc::now(),
            metadata: request.metadata.clone(),
        })
    }
    
    /// Configure key parameters for StrongBox storage
    fn configure_strongbox_parameters(
        &self,
        request: &GenerateKeyRequest,
    ) -> BearDogResult<KeyGenParameterSpec> {
        let mut builder = KeyGenParameterSpec::builder(&request.key_id);
        
        // Key algorithm
        match request.key_type {
            KeyType::EccP256 => {
                builder = builder.set_key_algorithm(KeyProperties::KEY_ALGORITHM_EC);
                builder = builder.set_key_size(256);
            },
            KeyType::EccP384 => {
                builder = builder.set_key_algorithm(KeyProperties::KEY_ALGORITHM_EC);
                builder = builder.set_key_size(384);
            },
            KeyType::Aes256 => {
                builder = builder.set_key_algorithm(KeyProperties::KEY_ALGORITHM_AES);
                builder = builder.set_key_size(256);
            },
            _ => return Err(BearDogError::UnsupportedKeyType {
                key_type: request.key_type.clone(),
                hsm_type: "Android StrongBox".to_string(),
            }),
        }
        
        // Require StrongBox
        builder = builder.set_is_strongbox_backed(true);
        
        // Key purposes
        builder = builder.set_purposes(
            KeyProperties::PURPOSE_ENCRYPT |
            KeyProperties::PURPOSE_DECRYPT |
            KeyProperties::PURPOSE_SIGN |
            KeyProperties::PURPOSE_VERIFY
        );
        
        // User authentication
        if request.require_user_presence {
            builder = builder.set_user_authentication_required(true);
            builder = builder.set_user_authentication_validity_duration_seconds(30);
        }
        
        // Attestation
        builder = builder.set_attestation_challenge(request.attestation_challenge.as_ref());
        
        Ok(builder.build())
    }
}
```

## 🖥️ Software HSM Implementation

### 3.1 Rust Software HSM

```rust
/// Pure Rust software HSM implementation
pub struct RustSoftwareHsm {
    config: SoftwareHsmConfig,
    key_store: Arc<RwLock<SoftwareKeyStore>>,
    crypto_provider: Arc<dyn CryptoProvider>,
    memory_protector: Arc<dyn MemoryProtector>,
}

impl RustSoftwareHsm {
    pub async fn new(config: SoftwareHsmConfig) -> BearDogResult<Self> {
        // Initialize encrypted key store
        let key_store = SoftwareKeyStore::new(&config.key_store_config)?;
        
        // Initialize crypto provider
        let crypto_provider = match config.crypto_backend {
            CryptoBackend::RustCrypto => Arc::new(RustCryptoProvider::new()),
            CryptoBackend::OpenSsl => Arc::new(OpenSslProvider::new()),
            CryptoBackend::Ring => Arc::new(RingProvider::new()),
        };
        
        // Initialize memory protector
        let memory_protector = Arc::new(MemoryProtector::new(&config.memory_config)?);
        
        Ok(Self {
            config,
            key_store: Arc::new(RwLock::new(key_store)),
            crypto_provider,
            memory_protector,
        })
    }
    
    /// Generate key in software HSM
    async fn generate_software_key(
        &self,
        request: &GenerateKeyRequest,
    ) -> BearDogResult<HsmKey> {
        // Generate key material
        let key_material = self.crypto_provider.generate_key_material(&request.key_type)?;
        
        // Protect key material in memory
        let protected_key = self.memory_protector.protect_key_material(&key_material)?;
        
        // Create software key
        let software_key = SoftwareKey {
            id: request.key_id.clone(),
            key_type: request.key_type.clone(),
            key_material: protected_key,
            created_at: Utc::now(),
            metadata: request.metadata.clone(),
        };
        
        // Encrypt and store key
        let mut key_store = self.key_store.write().await;
        key_store.store_key(&software_key).await?;
        
        Ok(HsmKey {
            id: request.key_id.clone(),
            hsm_type: HsmTier::SoftwareHsm {
                implementation: SoftwareHsmType::RustSoftwareHsm,
                key_storage: self.config.key_store_config.storage_type.clone(),
                encryption_at_rest: true,
                memory_protection: self.config.memory_config.protection_level.clone(),
            },
            key_type: request.key_type.clone(),
            attestation: None, // Software HSM doesn't provide hardware attestation
            created_at: Utc::now(),
            metadata: request.metadata.clone(),
        })
    }
}

#[async_trait]
impl HsmProvider for RustSoftwareHsm {
    async fn initialize(&self, config: HsmConfig) -> BearDogResult<()> {
        // Initialize key store
        let mut key_store = self.key_store.write().await;
        key_store.initialize().await?;
        
        // Initialize crypto provider
        self.crypto_provider.initialize()?;
        
        // Initialize memory protector
        self.memory_protector.initialize()?;
        
        Ok(())
    }
    
    async fn generate_key(&self, request: GenerateKeyRequest) -> BearDogResult<HsmKey> {
        self.generate_software_key(&request).await
    }
    
    async fn encrypt(&self, key_id: &str, plaintext: &[u8]) -> BearDogResult<Vec<u8>> {
        // Retrieve key from store
        let key_store = self.key_store.read().await;
        let software_key = key_store.get_key(key_id).await?;
        
        // Decrypt key material
        let key_material = self.memory_protector.unprotect_key_material(&software_key.key_material)?;
        
        // Encrypt using crypto provider
        let ciphertext = self.crypto_provider.encrypt(&key_material, plaintext)?;
        
        // Zero out key material
        self.memory_protector.zeroize_key_material(&key_material)?;
        
        Ok(ciphertext)
    }
    
    async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> BearDogResult<Vec<u8>> {
        // Retrieve key from store
        let key_store = self.key_store.read().await;
        let software_key = key_store.get_key(key_id).await?;
        
        // Decrypt key material
        let key_material = self.memory_protector.unprotect_key_material(&software_key.key_material)?;
        
        // Decrypt using crypto provider
        let plaintext = self.crypto_provider.decrypt(&key_material, ciphertext)?;
        
        // Zero out key material
        self.memory_protector.zeroize_key_material(&key_material)?;
        
        Ok(plaintext)
    }
    
    async fn get_info(&self) -> BearDogResult<HsmInfo> {
        Ok(HsmInfo {
            hsm_type: "Rust Software HSM".to_string(),
            vendor: "BearDog".to_string(),
            model: "RustSoftwareHsm".to_string(),
            firmware_version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities: vec![
                HsmCapability::KeyGeneration,
                HsmCapability::Encryption,
                HsmCapability::Decryption,
                HsmCapability::Signing,
                HsmCapability::KeyDerivation,
                HsmCapability::KeyImport,
                HsmCapability::KeyExport,
                HsmCapability::Backup,
                HsmCapability::Restore,
            ],
            supported_algorithms: vec![
                Algorithm::Aes256Gcm,
                Algorithm::ChaCha20Poly1305,
                Algorithm::EccP256,
                Algorithm::EccP384,
                Algorithm::Ed25519,
                Algorithm::X25519,
                Algorithm::Rsa2048,
                Algorithm::Rsa4096,
            ],
            max_key_size: Some(4096),
            certification: None,
            tamper_resistance: TamperResistanceLevel::Software,
        })
    }
}
```

### 3.2 Software HSM Key Storage

```rust
/// Encrypted key storage for software HSM
pub struct SoftwareKeyStore {
    storage_backend: Arc<dyn StorageBackend>,
    encryption_key: Arc<dyn EncryptionKey>,
    key_cache: Arc<RwLock<LruCache<String, SoftwareKey>>>,
}

impl SoftwareKeyStore {
    pub fn new(config: &KeyStoreConfig) -> BearDogResult<Self> {
        // Initialize storage backend
        let storage_backend = match config.storage_type {
            StorageType::FileSystem => Arc::new(FileSystemStorage::new(&config.file_config)?),
            StorageType::Database => Arc::new(DatabaseStorage::new(&config.db_config)?),
            StorageType::Memory => Arc::new(MemoryStorage::new()),
        };
        
        // Initialize encryption key
        let encryption_key = match config.encryption_config.key_source {
            KeySource::Derived => Arc::new(DerivedEncryptionKey::new(&config.encryption_config)?),
            KeySource::External => Arc::new(ExternalEncryptionKey::new(&config.encryption_config)?),
            KeySource::Hardware => Arc::new(HardwareEncryptionKey::new(&config.encryption_config)?),
        };
        
        Ok(Self {
            storage_backend,
            encryption_key,
            key_cache: Arc::new(RwLock::new(LruCache::new(config.cache_size))),
        })
    }
    
    pub async fn store_key(&self, key: &SoftwareKey) -> BearDogResult<()> {
        // Serialize key
        let serialized = bincode::serialize(key)?;
        
        // Encrypt key data
        let encrypted = self.encryption_key.encrypt(&serialized)?;
        
        // Store encrypted key
        self.storage_backend.store(&key.id, &encrypted).await?;
        
        // Cache key
        let mut cache = self.key_cache.write().await;
        cache.put(key.id.clone(), key.clone());
        
        Ok(())
    }
    
    pub async fn get_key(&self, key_id: &str) -> BearDogResult<SoftwareKey> {
        // Check cache first
        {
            let cache = self.key_cache.read().await;
            if let Some(key) = cache.get(key_id) {
                return Ok(key.clone());
            }
        }
        
        // Load from storage
        let encrypted = self.storage_backend.load(key_id).await?;
        
        // Decrypt key data
        let decrypted = self.encryption_key.decrypt(&encrypted)?;
        
        // Deserialize key
        let key: SoftwareKey = bincode::deserialize(&decrypted)?;
        
        // Cache key
        let mut cache = self.key_cache.write().await;
        cache.put(key_id.to_string(), key.clone());
        
        Ok(key)
    }
    
    pub async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        // Remove from storage
        self.storage_backend.delete(key_id).await?;
        
        // Remove from cache
        let mut cache = self.key_cache.write().await;
        cache.pop(key_id);
        
        Ok(())
    }
}
```

## 🔧 Genetic Spawning HSM Integration

### 4.1 HSM-Aware Genetic Spawning

```rust
/// HSM-integrated genetic spawning engine
pub struct HsmGeneticSpawner {
    hsm_manager: Arc<HsmManager>,
    genetic_engine: Arc<GeneticSpawningEngine>,
    key_hierarchy: Arc<KeyHierarchy>,
}

impl HsmGeneticSpawner {
    pub async fn spawn_with_hsm_hierarchy(
        &self,
        parent_nodes: Vec<BearDogNode>,
        spawn_request: SpawnRequest,
        hsm_requirements: HsmRequirements,
    ) -> BearDogResult<SpawnedChild> {
        // Determine appropriate HSM tier for child
        let hsm_tier = self.determine_hsm_tier(&spawn_request, &hsm_requirements).await?;
        
        // Generate child root key in appropriate HSM
        let child_root_key = self.generate_child_root_key(&hsm_tier, &spawn_request).await?;
        
        // Derive child genetics key from root key
        let genetics_key = self.derive_genetics_key(&child_root_key, &spawn_request).await?;
        
        // Perform genetic spawning with HSM-protected keys
        let child_genetics = self.genetic_engine.spawn_with_protected_keys(
            &parent_nodes,
            &genetics_key,
            &spawn_request,
        ).await?;
        
        // Initialize child with HSM configuration
        let child = self.initialize_child_with_hsm(
            child_genetics,
            child_root_key,
            hsm_tier,
            &spawn_request,
        ).await?;
        
        Ok(child)
    }
    
    /// Determine appropriate HSM tier based on requirements
    async fn determine_hsm_tier(
        &self,
        spawn_request: &SpawnRequest,
        requirements: &HsmRequirements,
    ) -> BearDogResult<HsmTier> {
        // Check security requirements
        if requirements.security_level >= SecurityLevel::High {
            // High security requires hardware HSM
            return Ok(HsmTier::HardwareHsm {
                vendor: requirements.preferred_vendor.clone(),
                model: requirements.preferred_model.clone(),
                certification: CertificationLevel::Fips140Level3,
                tamper_resistance: TamperResistanceLevel::Hardware,
            });
        }
        
        // Check if user interaction is required
        if spawn_request.requires_user_interaction {
            // User interaction suggests smartphone HSM
            return Ok(HsmTier::SmartphoneHsm {
                device_type: self.detect_smartphone_type().await?,
                secure_enclave: self.detect_secure_enclave().await?,
                attestation_level: AttestationLevel::Hardware,
                user_presence_required: true,
            });
        }
        
        // Check availability and cost constraints
        if requirements.cost_optimization && self.is_software_hsm_sufficient(&requirements).await? {
            return Ok(HsmTier::SoftwareHsm {
                implementation: SoftwareHsmType::RustSoftwareHsm,
                key_storage: KeyStorageType::Encrypted,
                encryption_at_rest: true,
                memory_protection: MemoryProtectionLevel::High,
            });
        }
        
        // Default to hybrid approach
        Ok(HsmTier::HybridHsm {
            tiers: vec![
                HsmTier::SoftwareHsm {
                    implementation: SoftwareHsmType::RustSoftwareHsm,
                    key_storage: KeyStorageType::Encrypted,
                    encryption_at_rest: true,
                    memory_protection: MemoryProtectionLevel::High,
                },
                HsmTier::SmartphoneHsm {
                    device_type: self.detect_smartphone_type().await.unwrap_or_default(),
                    secure_enclave: self.detect_secure_enclave().await.unwrap_or_default(),
                    attestation_level: AttestationLevel::Hardware,
                    user_presence_required: false,
                },
            ],
            key_hierarchy: KeyHierarchy::Tiered,
            fallback_strategy: FallbackStrategy::Graceful,
        })
    }
}
```

## 🔍 HSM Management & Orchestration

### 5.1 HSM Manager

```rust
/// Central HSM management component
pub struct HsmManager {
    hsm_providers: HashMap<String, Arc<dyn HsmProvider>>,
    config: HsmManagerConfig,
    health_monitor: Arc<HsmHealthMonitor>,
    failover_manager: Arc<FailoverManager>,
}

impl HsmManager {
    pub async fn new(config: HsmManagerConfig) -> BearDogResult<Self> {
        let mut hsm_providers = HashMap::new();
        
        // Initialize configured HSM providers
        for hsm_config in &config.hsm_configs {
            let provider = Self::create_hsm_provider(hsm_config).await?;
            hsm_providers.insert(hsm_config.id.clone(), provider);
        }
        
        // Initialize health monitor
        let health_monitor = Arc::new(HsmHealthMonitor::new(&config.health_config)?);
        
        // Initialize failover manager
        let failover_manager = Arc::new(FailoverManager::new(&config.failover_config)?);
        
        Ok(Self {
            hsm_providers,
            config,
            health_monitor,
            failover_manager,
        })
    }
    
    /// Create HSM provider based on configuration
    async fn create_hsm_provider(config: &HsmConfig) -> BearDogResult<Arc<dyn HsmProvider>> {
        match config.hsm_type {
            HsmType::SmartphoneIos => {
                let provider = IosSecureEnclaveHsm::new(config.ios_config.clone().unwrap()).await?;
                Ok(Arc::new(provider))
            },
            HsmType::SmartphoneAndroid => {
                let provider = AndroidStrongBoxHsm::new(config.android_config.clone().unwrap()).await?;
                Ok(Arc::new(provider))
            },
            HsmType::SoftwareRust => {
                let provider = RustSoftwareHsm::new(config.software_config.clone().unwrap()).await?;
                Ok(Arc::new(provider))
            },
            HsmType::HardwareAws => {
                let provider = AwsCloudHsm::new(config.aws_config.clone().unwrap()).await?;
                Ok(Arc::new(provider))
            },
            HsmType::HardwareLuna => {
                let provider = SafeNetLunaHsm::new(config.luna_config.clone().unwrap()).await?;
                Ok(Arc::new(provider))
            },
        }
    }
    
    /// Get best available HSM provider for requirements
    pub async fn get_best_provider(&self, requirements: &HsmRequirements) -> BearDogResult<Arc<dyn HsmProvider>> {
        // Get candidate providers
        let candidates = self.get_candidate_providers(requirements).await?;
        
        // Check health of candidates
        let healthy_candidates = self.health_monitor.filter_healthy_providers(candidates).await?;
        
        // Select best provider based on requirements
        let best_provider = self.select_best_provider(healthy_candidates, requirements).await?;
        
        Ok(best_provider)
    }
    
    /// Perform HSM operation with automatic failover
    pub async fn perform_operation<T, F, Fut>(
        &self,
        requirements: &HsmRequirements,
        operation: F,
    ) -> BearDogResult<T>
    where
        F: Fn(Arc<dyn HsmProvider>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = BearDogResult<T>> + Send + 'static,
        T: Send + 'static,
    {
        // Get primary provider
        let primary_provider = self.get_best_provider(requirements).await?;
        
        // Attempt operation on primary provider
        match operation(primary_provider.clone()).await {
            Ok(result) => Ok(result),
            Err(error) => {
                // Primary failed, attempt failover
                self.failover_manager.handle_provider_failure(&primary_provider, &error).await?;
                
                // Get failover provider
                let failover_provider = self.get_failover_provider(requirements, &primary_provider).await?;
                
                // Retry operation on failover provider
                operation(failover_provider).await
            }
        }
    }
}
```

## 📊 Implementation Roadmap

### Phase 1: Foundation (Months 1-2)
- [ ] HSM provider interface design
- [ ] Software HSM implementation
- [ ] Basic key management

### Phase 2: Smartphone Integration (Months 3-4)
- [ ] iOS Secure Enclave integration
- [ ] Android StrongBox integration
- [ ] Mobile key attestation

### Phase 3: Hardware HSM (Months 5-6)
- [ ] AWS CloudHSM integration
- [ ] SafeNet Luna integration
- [ ] Hardware key management

### Phase 4: Genetic Integration (Months 7-8)
- [ ] HSM-aware genetic spawning
- [ ] Key hierarchy management
- [ ] Entropy-HSM integration

### Phase 5: Production (Months 9-10)
- [ ] Performance optimization
- [ ] Monitoring and alerting
- [ ] Documentation and training

## 🎛️ Configuration

```toml
[hsm]
# HSM Manager Configuration
default_provider = "software"
failover_enabled = true
health_check_interval = "30s"

# Software HSM Configuration
[hsm.software]
enabled = true
key_storage = "encrypted_file"
memory_protection = "high"
backup_enabled = true

# Smartphone HSM Configuration
[hsm.smartphone]
enabled = true
prefer_hardware_backed = true
require_user_presence = false
attestation_required = true

# Hardware HSM Configuration
[hsm.hardware]
enabled = false
vendor = "aws"
region = "us-east-1"
high_availability = true

# Genetic Spawning HSM Integration
[hsm.genetic_spawning]
inherit_hsm_tier = true
child_key_hierarchy = "tiered"
entropy_hsm_mixing = true
```

This comprehensive HSM integration specification provides BearDog with enterprise-grade key management capabilities across multiple device types and security levels. 