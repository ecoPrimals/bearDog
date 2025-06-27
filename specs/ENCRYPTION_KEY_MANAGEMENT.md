# BearDog Encryption & Key Management Specification

**Version:** 1.0  
**Date:** January 2025  
**Status:** SPECIFICATION  
**Priority:** CRITICAL  

## 🎯 **Overview**

BearDog's encryption and key management system provides enterprise-grade cryptographic services with:
- **Hardware Security Module (HSM)** integration
- **Post-quantum cryptography** readiness
- **Zero-trust key management**
- **Multi-party approval workflows**
- **Secure-by-default** configuration
- **Agnostic HSM provider** support

## 🔐 **Cryptographic Architecture**

### **Supported Algorithms**

#### **Symmetric Encryption (Secure Defaults)**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymmetricCipher {
    // Current secure defaults
    Aes256Gcm,         // Primary default
    ChaCha20Poly1305,  // Alternative for performance
    Aes256Cbc,         // Legacy support only
    
    // Post-quantum ready
    Kyber1024,         // NIST PQC finalist
    Dilithium5,        // Digital signatures
}

impl Default for SymmetricCipher {
    fn default() -> Self {
        Self::Aes256Gcm // Secure by default
    }
}
```

#### **Asymmetric Encryption**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AsymmetricCipher {
    // Current secure defaults
    Rsa4096,           // Traditional RSA
    EccP384,           // Elliptic curve (NIST P-384)
    Ed25519,           // EdDSA signatures
    X25519,            // ECDH key exchange
    
    // Post-quantum ready
    Kyber1024,         // Key encapsulation
    Dilithium5,        // Digital signatures
    Falcon1024,        // Alternative signatures
}

impl Default for AsymmetricCipher {
    fn default() -> Self {
        Self::EccP384 // Secure by default
    }
}
```

#### **Key Derivation**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyDerivationFunction {
    // Secure defaults
    Pbkdf2Sha512 {
        iterations: u32,
        salt_size: usize,
    },
    Scrypt {
        log_n: u8,
        r: u32,
        p: u32,
        salt_size: usize,
    },
    Argon2id {
        memory_cost: u32,
        time_cost: u32,
        parallelism: u32,
        salt_size: usize,
    },
    Hkdf {
        hash: HashAlgorithm,
        salt_size: usize,
    },
}

impl Default for KeyDerivationFunction {
    fn default() -> Self {
        Self::Argon2id {
            memory_cost: 65536,    // 64MB
            time_cost: 3,          // 3 iterations
            parallelism: 4,        // 4 threads
            salt_size: 32,         // 256-bit salt
        }
    }
}
```

## 🔑 **Key Management Architecture**

### **Core Key Manager**
```rust
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

pub struct BearDogKeyManager {
    config: Arc<KeyManagementConfig>,
    hsm_provider: Option<Arc<dyn HsmProvider>>,
    key_store: Arc<dyn KeyStore>,
    audit_logger: Arc<dyn AuditLogger>,
    workflow_engine: Arc<MultiPartyWorkflowEngine>,
    
    // Key caching and performance
    key_cache: Arc<RwLock<LruCache<String, CachedKey>>>,
    crypto_provider: Arc<dyn CryptoProvider>,
}

impl BearDogKeyManager {
    pub async fn new(config: KeyManagementConfig) -> Result<Self> {
        let hsm_provider = if config.hsm.enabled {
            Some(Self::initialize_hsm(&config.hsm).await?)
        } else {
            None
        };
        
        let key_store = Self::initialize_key_store(&config.storage).await?;
        let audit_logger = Self::initialize_audit_logger(&config.audit).await?;
        
        Ok(Self {
            config: Arc::new(config),
            hsm_provider,
            key_store,
            audit_logger,
            workflow_engine: Arc::new(MultiPartyWorkflowEngine::new()),
            key_cache: Arc::new(RwLock::new(LruCache::new(1000))),
            crypto_provider: Arc::new(RustCryptoProvider::new()),
        })
    }
    
    pub async fn generate_master_key(&self, request: GenerateKeyRequest) -> Result<MasterKey> {
        // Validate request
        self.validate_key_generation_request(&request).await?;
        
        // Check if multi-party approval required
        if self.requires_approval(&request).await? {
            return self.initiate_key_generation_approval(request).await;
        }
        
        // Generate key using appropriate provider
        let key = if let Some(ref hsm) = self.hsm_provider {
            self.generate_hsm_key(hsm.as_ref(), &request).await?
        } else {
            self.generate_software_key(&request).await?
        };
        
        // Store key with metadata
        self.store_key_with_metadata(&key).await?;
        
        // Audit log
        self.audit_logger.log_key_generation(&key).await?;
        
        Ok(key)
    }
    
    pub async fn encrypt_data(&self, request: EncryptionRequest) -> Result<EncryptedData> {
        let key = self.get_key(&request.key_id).await?;
        
        // Validate permissions
        self.validate_encryption_permissions(&request, &key).await?;
        
        let encrypted_data = if key.location == KeyLocation::Hsm {
            self.hsm_encrypt(&key, &request.plaintext).await?
        } else {
            self.software_encrypt(&key, &request.plaintext).await?
        };
        
        // Audit log
        self.audit_logger.log_encryption_operation(&request, &encrypted_data).await?;
        
        Ok(encrypted_data)
    }
    
    pub async fn decrypt_data(&self, request: DecryptionRequest) -> Result<Vec<u8>> {
        let key = self.get_key(&request.key_id).await?;
        
        // Validate permissions (owner-only decryption)
        self.validate_decryption_permissions(&request, &key).await?;
        
        let plaintext = if key.location == KeyLocation::Hsm {
            self.hsm_decrypt(&key, &request.encrypted_data).await?
        } else {
            self.software_decrypt(&key, &request.encrypted_data).await?
        };
        
        // Audit log
        self.audit_logger.log_decryption_operation(&request).await?;
        
        Ok(plaintext)
    }
    
    pub async fn rotate_key(&self, request: KeyRotationRequest) -> Result<KeyRotationResult> {
        // Multi-party approval required for rotation
        if !self.has_rotation_approval(&request).await? {
            return self.initiate_rotation_approval(request).await;
        }
        
        let old_key = self.get_key(&request.key_id).await?;
        
        // Generate new key with same parameters
        let new_key_request = GenerateKeyRequest {
            key_type: old_key.key_type.clone(),
            algorithm: old_key.algorithm.clone(),
            owner_id: old_key.owner_id.clone(),
            purpose: old_key.purpose.clone(),
            metadata: old_key.metadata.clone(),
        };
        
        let new_key = self.generate_master_key(new_key_request).await?;
        
        // Update key references atomically
        self.atomic_key_rotation(&old_key, &new_key).await?;
        
        // Schedule old key deletion (with grace period)
        self.schedule_key_deletion(&old_key.id, Duration::from_days(30)).await?;
        
        // Audit log
        self.audit_logger.log_key_rotation(&old_key, &new_key).await?;
        
        Ok(KeyRotationResult {
            old_key_id: old_key.id,
            new_key_id: new_key.id,
            rotation_timestamp: Utc::now(),
        })
    }
}
```

### **Key Types and Specifications**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterKey {
    pub id: String,
    pub key_type: KeyType,
    pub algorithm: SymmetricCipher,
    pub key_size: usize,
    pub owner_id: String,
    pub purpose: KeyPurpose,
    pub location: KeyLocation,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub rotation_policy: RotationPolicy,
    pub metadata: KeyMetadata,
    
    // Security attributes
    pub min_approval_count: u32,
    pub allowed_operations: Vec<KeyOperation>,
    pub access_control: AccessControlPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyType {
    Master,                    // Root encryption key
    DataEncryption,           // Data-at-rest encryption
    TransportEncryption,      // Data-in-transit encryption
    BackupEncryption,         // Backup-specific encryption
    FederationEncryption,     // Federation/sharing encryption
    ComplianceEncryption,     // Compliance-specific encryption
    SigningKey,               // Digital signatures
    KeyEncryption,            // Key-wrapping key (KEK)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyLocation {
    Software,                 // Software-based key storage
    Hsm,                      // Hardware Security Module
    CloudHsm,                 // Cloud HSM (AWS KMS, Azure Key Vault, etc.)
    Distributed,              // Distributed across multiple HSMs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyPurpose {
    GeneralEncryption,
    DatabaseEncryption,
    FileSystemEncryption,
    BackupEncryption,
    FederationSharing,
    ComplianceArchival,
    DigitalSignatures,
    KeyWrapping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    pub description: String,
    pub tags: HashMap<String, String>,
    pub compliance_labels: Vec<ComplianceLabel>,
    pub retention_policy: RetentionPolicy,
    pub backup_policy: BackupPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationPolicy {
    pub enabled: bool,
    pub rotation_interval: Duration,
    pub auto_rotate: bool,
    pub require_approval: bool,
    pub grace_period: Duration, // Keep old key for this duration
}

impl Default for RotationPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            rotation_interval: Duration::from_days(90), // 90 days default
            auto_rotate: false, // Manual rotation by default
            require_approval: true, // Require approval by default
            grace_period: Duration::from_days(30), // 30 day grace period
        }
    }
}
```

## 🔧 **HSM Integration**

### **HSM Provider Trait**
```rust
#[async_trait]
pub trait HsmProvider: Send + Sync {
    async fn initialize(&self, config: &HsmConfig) -> Result<()>;
    async fn health_check(&self) -> Result<HsmHealthStatus>;
    
    // Key management operations
    async fn generate_key(&self, spec: HsmKeySpec) -> Result<HsmKey>;
    async fn import_key(&self, key_material: &[u8], spec: HsmKeySpec) -> Result<HsmKey>;
    async fn delete_key(&self, key_id: &str) -> Result<()>;
    async fn get_key_info(&self, key_id: &str) -> Result<HsmKeyInfo>;
    
    // Cryptographic operations
    async fn encrypt(&self, key_id: &str, plaintext: &[u8], algorithm: SymmetricCipher) -> Result<Vec<u8>>;
    async fn decrypt(&self, key_id: &str, ciphertext: &[u8], algorithm: SymmetricCipher) -> Result<Vec<u8>>;
    async fn sign(&self, key_id: &str, data: &[u8], algorithm: AsymmetricCipher) -> Result<Signature>;
    async fn verify(&self, key_id: &str, data: &[u8], signature: &Signature, algorithm: AsymmetricCipher) -> Result<bool>;
    
    // Key wrapping operations  
    async fn wrap_key(&self, kek_id: &str, key_to_wrap: &[u8]) -> Result<WrappedKey>;
    async fn unwrap_key(&self, kek_id: &str, wrapped_key: &WrappedKey) -> Result<Vec<u8>>;
    
    // Backup and recovery
    async fn backup_key(&self, key_id: &str) -> Result<KeyBackup>;
    async fn restore_key(&self, backup: &KeyBackup) -> Result<String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfig {
    pub provider: HsmProviderType,
    pub connection: HsmConnectionConfig,
    pub authentication: HsmAuthConfig,
    pub security: HsmSecurityConfig,
    pub performance: HsmPerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HsmProviderType {
    Pkcs11 {
        library_path: String,
        slot_id: u64,
    },
    AwsKms {
        region: String,
        key_spec: String,
    },
    AzureKeyVault {
        vault_url: String,
        key_size: u32,
    },
    HashiCorpVault {
        addr: String,
        mount_path: String,
    },
    YubiHsm {
        device_serial: Option<u64>,
        connector_url: String,
    },
    Tpm {
        tpm_device: String,
        hierarchy: TpmHierarchy,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConnectionConfig {
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
    pub retry_delay_ms: u64,
    pub connection_pool_size: u32,
    pub health_check_interval_seconds: u64,
}

impl Default for HsmConnectionConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            retry_attempts: 3,
            retry_delay_ms: 1000,
            connection_pool_size: 5,
            health_check_interval_seconds: 60,
        }
    }
}
```

### **PKCS#11 Implementation**
```rust
pub struct Pkcs11HsmProvider {
    config: HsmConfig,
    ctx: Arc<Mutex<Pkcs11>>,
    session_pool: Arc<Mutex<Vec<Session>>>,
    health_monitor: Arc<Mutex<HsmHealthMonitor>>,
}

impl Pkcs11HsmProvider {
    pub async fn new(config: HsmConfig) -> Result<Self> {
        let ctx = Pkcs11::new(&config.connection.library_path)
            .map_err(|e| BearDogError::HsmInitializationError(e.to_string()))?;
        
        ctx.initialize(CInitializeArgs::OsLockingOk)
            .map_err(|e| BearDogError::HsmInitializationError(e.to_string()))?;
        
        // Initialize session pool
        let session_pool = Self::create_session_pool(&ctx, &config).await?;
        
        Ok(Self {
            config,
            ctx: Arc::new(Mutex::new(ctx)),
            session_pool: Arc::new(Mutex::new(session_pool)),
            health_monitor: Arc::new(Mutex::new(HsmHealthMonitor::new())),
        })
    }
    
    async fn create_session_pool(ctx: &Pkcs11, config: &HsmConfig) -> Result<Vec<Session>> {
        let mut sessions = Vec::new();
        
        for _ in 0..config.connection.connection_pool_size {
            let session = ctx.open_session_no_callback(
                Slot::try_from(config.connection.slot_id)?,
                SessionType::SerialSession,
                true, // Read/write session
            )?;
            
            sessions.push(session);
        }
        
        Ok(sessions)
    }
}

#[async_trait]
impl HsmProvider for Pkcs11HsmProvider {
    async fn initialize(&self, config: &HsmConfig) -> Result<()> {
        // Already initialized in new(), but verify health
        self.health_check().await?;
        Ok(())
    }
    
    async fn generate_key(&self, spec: HsmKeySpec) -> Result<HsmKey> {
        let session = self.get_session().await?;
        
        let key_template = self.build_key_template(&spec)?;
        
        let key_handle = match spec.key_type {
            HsmKeyType::Symmetric => {
                session.generate_key(&Mechanism::AesKeyGen, &key_template)?
            }
            HsmKeyType::Asymmetric => {
                let (public_template, private_template) = self.build_keypair_templates(&spec)?;
                let (public_key, private_key) = session.generate_key_pair(
                    &Mechanism::RsaPkcsKeyPairGen,
                    &public_template,
                    &private_template,
                )?;
                private_key // Return private key handle
            }
        };
        
        let key_id = format!("hsm-{}", Uuid::new_v4());
        let key_info = HsmKeyInfo {
            id: key_id.clone(),
            handle: key_handle.try_into()?,
            key_type: spec.key_type,
            algorithm: spec.algorithm,
            size: spec.key_size,
            created_at: Utc::now(),
            metadata: spec.metadata,
        };
        
        self.return_session(session).await;
        
        Ok(HsmKey {
            id: key_id,
            info: key_info,
            provider: HsmProviderType::Pkcs11 {
                library_path: self.config.connection.library_path.clone(),
                slot_id: self.config.connection.slot_id,
            },
        })
    }
    
    async fn encrypt(&self, key_id: &str, plaintext: &[u8], algorithm: SymmetricCipher) -> Result<Vec<u8>> {
        let session = self.get_session().await?;
        let key_handle = self.get_key_handle(key_id).await?;
        
        let mechanism = match algorithm {
            SymmetricCipher::Aes256Gcm => Mechanism::AesGcm,
            SymmetricCipher::ChaCha20Poly1305 => return Err(BearDogError::UnsupportedAlgorithm("ChaCha20Poly1305 not supported in PKCS#11".to_string())),
            _ => return Err(BearDogError::UnsupportedAlgorithm(format!("{:?}", algorithm))),
        };
        
        session.encrypt_init(&mechanism, key_handle)?;
        let ciphertext = session.encrypt(plaintext)?;
        
        self.return_session(session).await;
        
        Ok(ciphertext)
    }
    
    // ... implement other trait methods
}
```

## ⚙️ **Configuration**

### **Key Management Configuration**
```toml
[key_management]
# Default key configuration
default_algorithm = "aes-256-gcm"
default_key_size = 256
default_rotation_days = 90
require_hsm = false

[key_management.generation]
# Key generation security
min_entropy_bits = 256
use_hardware_random = true
key_derivation_iterations = 100000

[key_management.storage]
# Key storage configuration
storage_type = "encrypted_file"  # "encrypted_file", "database", "hsm_only"
storage_path = "./keys"
encryption_at_rest = true
backup_enabled = true
backup_interval_hours = 24

[key_management.policies]
# Security policies
require_multi_party_approval = true
min_approvers = 2
approval_timeout_hours = 24
auto_rotate_master_keys = false
grace_period_days = 30

[key_management.hsm]
# HSM configuration
enabled = false
provider = "pkcs11"
library_path = "/usr/lib/libpkcs11.so"
slot_id = 0
pin_env_var = "HSM_PIN"

# Connection settings
timeout_seconds = 30
retry_attempts = 3
connection_pool_size = 5
health_check_interval_seconds = 60

[key_management.hsm.pkcs11]
# PKCS#11 specific settings
use_os_locking = true
max_sessions = 10
session_timeout_minutes = 30

[key_management.compliance]
# Compliance settings
fips140_level = 2
common_criteria_level = "eal4"
audit_all_operations = true
export_control_compliance = true

[key_management.performance]
# Performance tuning
key_cache_size = 1000
key_cache_ttl_minutes = 60
concurrent_operations = 100
batch_size = 50
```

## 🔒 **Security Features**

### **Access Control**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlPolicy {
    pub owner_only_decrypt: bool,
    pub allowed_users: Vec<String>,
    pub allowed_roles: Vec<String>,
    pub allowed_operations: Vec<KeyOperation>,
    pub time_restrictions: Option<TimeRestriction>,
    pub ip_restrictions: Option<IpRestriction>,
    pub require_mfa: bool,
}

impl Default for AccessControlPolicy {
    fn default() -> Self {
        Self {
            owner_only_decrypt: true, // Secure default
            allowed_users: Vec::new(),
            allowed_roles: Vec::new(),
            allowed_operations: Vec::new(),
            time_restrictions: None,
            ip_restrictions: None,
            require_mfa: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyOperation {
    Encrypt,
    Decrypt,
    Sign,
    Verify,
    Wrap,
    Unwrap,
    Derive,
    Rotate,
    Backup,
    Export,
    Delete,
}
```

### **Audit and Compliance**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyAuditEvent {
    pub event_id: String,
    pub timestamp: DateTime<Utc>,
    pub key_id: String,
    pub operation: KeyOperation,
    pub user_id: String,
    pub result: OperationResult,
    pub metadata: HashMap<String, serde_json::Value>,
    pub compliance_labels: Vec<ComplianceLabel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceLabel {
    GDPR,
    HIPAA,
    SOX,
    PCI,
    FedRAMP,
    FIPS140_2,
    CommonCriteria,
}
```

## 🚀 **Performance Optimization**

### **Key Caching**
```rust
use lru::LruCache;

pub struct KeyCache {
    cache: Arc<RwLock<LruCache<String, CachedKey>>>,
    config: KeyCacheConfig,
}

#[derive(Debug, Clone)]
pub struct CachedKey {
    pub key: Key,
    pub cached_at: DateTime<Utc>,
    pub access_count: u64,
    pub last_accessed: DateTime<Utc>,
}

impl KeyCache {
    pub fn new(config: KeyCacheConfig) -> Self {
        Self {
            cache: Arc::new(RwLock::new(LruCache::new(config.max_size))),
            config,
        }
    }
    
    pub async fn get(&self, key_id: &str) -> Option<Key> {
        let mut cache = self.cache.write().await;
        
        if let Some(cached_key) = cache.get_mut(key_id) {
            // Check if still valid
            if cached_key.cached_at + self.config.ttl > Utc::now() {
                cached_key.access_count += 1;
                cached_key.last_accessed = Utc::now();
                return Some(cached_key.key.clone());
            } else {
                // Expired, remove from cache
                cache.pop(key_id);
            }
        }
        
        None
    }
    
    pub async fn put(&self, key_id: String, key: Key) {
        let cached_key = CachedKey {
            key,
            cached_at: Utc::now(),
            access_count: 0,
            last_accessed: Utc::now(),
        };
        
        let mut cache = self.cache.write().await;
        cache.put(key_id, cached_key);
    }
}
```

## 🧪 **Testing Strategy**

### **Unit Tests**
- Algorithm implementation correctness
- Key generation randomness
- Encryption/decryption round-trip
- HSM provider mocking

### **Integration Tests**
- HSM connectivity and operations
- Multi-party workflow integration
- Cross-system key sharing
- Performance benchmarks

### **Security Tests**  
- Side-channel attack resistance
- Key material leakage prevention
- Access control enforcement
- Audit trail completeness

### **Compliance Tests**
- FIPS 140-2 validation
- Common Criteria evaluation
- Regulatory compliance verification
- Third-party security audits

---

**Next Steps**: Implement HSM providers, multi-party workflows, and compliance reporting. 