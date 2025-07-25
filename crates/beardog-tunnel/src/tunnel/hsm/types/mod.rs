pub mod key;
pub mod status;
pub mod config;
pub mod tier;
pub mod capability;

// Re-export all types for convenience
pub use key::*;
pub use status::*;
pub use config::*;
pub use tier::*;
pub use capability::*;

// Import and re-export selected types from beardog-core (commenting out conflicting ones)
pub use beardog_core::{
    // KeyType, HsmTier, HsmHealthStatus, HsmCapabilities, HsmKey,  // Conflicts with local definitions
    ComponentStatus, CoreState, HealthStatus, HealthCheck, SystemMetrics,
    // HsmCapability,  // May conflict
    BearDogSecurityProvider, SystemMonitor, GeneticOptimizer
};

// Re-export the new Android types we've added
pub use tier::{AndroidKeyAlgorithm, StrongBoxImplementation};

// Re-export utility types (removing conflicting re-exports that cause multiple definition errors)
// pub use self::{PerformanceMetrics, StrongBoxCapabilities, CapabilityRequirements, AndroidHsmConfig, DeviceModel, MemoryProtectionLevel};

// HSM type enumeration from the standalone types.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HsmType {
    /// iOS-based smartphone HSM
    SmartphoneIos,
    /// Android-based smartphone HSM
    SmartphoneAndroid,
    /// Rust software HSM
    SoftwareRust,
    /// AWS hardware HSM
    HardwareAws,
    /// Luna hardware HSM
    HardwareLuna,
    /// Thales hardware HSM
    HardwareThales,
    /// Utimaco hardware HSM
    HardwareUtimaco,
    /// Custom HSM type
    Custom(String),
}

// Cryptographic Algorithm enumeration (most common missing type)
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Algorithm {
    /// AES-256-GCM encryption
    Aes256Gcm,
    /// ChaCha20-Poly1305 encryption
    ChaCha20Poly1305,
    /// ECC P-256 curve
    EccP256,
    /// ECC P-384 curve
    EccP384,
    /// ECDSA with SHA-256
    EcdsaSha256,
    /// RSA with SHA-256
    RsaSha256,
    /// HKDF with SHA-256
    HkdfSha256,
    /// Ed25519 signature algorithm
    Ed25519,
    /// Custom algorithm
    Custom(String),
}

// Android-specific types (commonly missing)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AndroidKeyPurpose {
    Sign,
    Verify,
    Encrypt,
    Decrypt,
    WrapKey,
    UnwrapKey,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AndroidEcCurve {
    P256,
    P384,
    P521,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AndroidKeyParams {
    pub purposes: Vec<AndroidKeyPurpose>,
    pub algorithm: Algorithm,
    pub key_size: Option<u32>,
    pub ec_curve: Option<AndroidEcCurve>,
    pub user_authentication_required: bool,
    pub user_presence_required: bool,
    pub strongbox_required: bool,
}

// AndroidDeviceInfo is defined in android_strongbox::types - use that one

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum VerifiedBootState {
    Green,      // Locked bootloader, verified boot
    Yellow,     // Locked bootloader, custom OS
    Orange,     // Unlocked bootloader
    Red,        // Verification failed
}

// Platform-specific enums
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SmartphoneType {
    Android { device_info: String }, // Simplified to avoid import complexity
    iOS { device_info: String }, // Simplified for now
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SecureEnclaveType {
    AndroidStrongBox { implementation: StrongBoxImplementation },
    IOSSecureEnclave,
    SoftwareFallback,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AttestationLevel {
    SoftwareImplemented,
    TrustedEnvironment,
    StrongBox,
    CertifiedHardware,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SoftwareHsmType {
    RustSoftwareHsm,
    OpenSSLHsm,
    CustomHsm(String),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KeyStorageType {
    InMemory,
    EncryptedFile,
    SystemKeychain,
    Hardware,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MemoryProtectionLevel {
    None,
    Low,
    Medium,
    High,
    Maximum,
}

// Configuration types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AndroidHsmConfig {
    pub strongbox_enabled: bool,
    pub key_params: AndroidKeyParams,
    pub attestation_level: AttestationLevel,
    pub security_level: u8,
}

impl Default for AndroidHsmConfig {
    fn default() -> Self {
        Self {
            strongbox_enabled: true,
            key_params: AndroidKeyParams {
                purposes: vec![AndroidKeyPurpose::Sign, AndroidKeyPurpose::Verify],
                algorithm: Algorithm::EcdsaSha256,
                key_size: Some(256),
                ec_curve: Some(AndroidEcCurve::P256),
                user_authentication_required: false,
                user_presence_required: false,
                strongbox_required: false,
            },
            attestation_level: AttestationLevel::StrongBox,
            security_level: 100,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Pixel8GrapheneOSConfig {
    pub strongbox_optimization: bool,
    pub graphene_hardening: bool,
    pub verified_boot_enforced: bool,
}

impl Default for Pixel8GrapheneOSConfig {
    fn default() -> Self {
        Self {
            strongbox_optimization: true,
            graphene_hardening: true,
            verified_boot_enforced: true,
        }
    }
}

// HSM configuration type
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HsmConfig {
    pub hsm_type: HsmType,
    pub android_config: Option<AndroidHsmConfig>,
    pub security_level: u8,
    pub performance_mode: bool,
}

impl Default for HsmConfig {
    fn default() -> Self {
        Self {
            hsm_type: HsmType::SoftwareRust,
            android_config: None,
            security_level: 80,
            performance_mode: false,
        }
    }
}

// Capability requirements (commonly referenced)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CapabilityRequirements {
    pub minimum_security_level: u8,
    pub required_algorithms: Vec<Algorithm>,
    pub attestation_required: bool,
    pub biometric_support: bool,
    pub hardware_backed: bool,
}

impl Default for CapabilityRequirements {
    fn default() -> Self {
        Self {
            minimum_security_level: 50,
            required_algorithms: vec![Algorithm::EcdsaSha256],
            attestation_required: false,
            biometric_support: false,
            hardware_backed: false,
        }
    }
}

// Add the missing PerformanceMetrics type
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PerformanceMetrics {
    pub operations_per_second: f64,
    pub average_latency_ms: f64,
    pub success_rate: f64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub error_count: u64,
    pub uptime_seconds: u64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            operations_per_second: 0.0,
            average_latency_ms: 0.0,
            success_rate: 100.0,
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            error_count: 0,
            uptime_seconds: 0,
        }
    }
}

// Additional missing types that are referenced in the codebase
#[derive(Debug, Clone)]
pub struct StrongBoxCapabilities {
    pub hardware_backed: bool,
    pub secure_key_import: bool,
    pub attestation_support: bool,
}

impl Default for StrongBoxCapabilities {
    fn default() -> Self {
        Self {
            hardware_backed: false,
            secure_key_import: false,
            attestation_support: false,
        }
    }
}

// Android service types (commonly missing)
#[derive(Debug, Clone)]
pub struct AndroidKeystore {
    pub alias_prefix: String,
    pub strongbox_enabled: bool,
    pub config: AndroidHsmConfig,
}

impl Default for AndroidKeystore {
    fn default() -> Self {
        Self {
            alias_prefix: "beardog_".to_string(),
            strongbox_enabled: true,
            config: AndroidHsmConfig::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AndroidAttestationService {
    pub enabled: bool,
    pub attestation_level: AttestationLevel,
}

impl Default for AndroidAttestationService {
    fn default() -> Self {
        Self {
            enabled: true,
            attestation_level: AttestationLevel::StrongBox,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AndroidHealthMonitor {
    pub monitoring_enabled: bool,
    pub check_interval_ms: u64,
}

impl Default for AndroidHealthMonitor {
    fn default() -> Self {
        Self {
            monitoring_enabled: true,
            check_interval_ms: 30000,
        }
    }
}

// Main Android StrongBox HSM implementation type
#[derive(Debug, Clone)]
pub struct AndroidStrongBoxHsm {
    pub keystore: AndroidKeystore,
    pub attestation_service: AndroidAttestationService,
    pub health_monitor: AndroidHealthMonitor,
    pub config: AndroidHsmConfig,
    pub capabilities: StrongBoxCapabilities,
}

impl Default for AndroidStrongBoxHsm {
    fn default() -> Self {
        Self {
            keystore: AndroidKeystore::default(),
            attestation_service: AndroidAttestationService::default(),
            health_monitor: AndroidHealthMonitor::default(),
            config: AndroidHsmConfig::default(),
            capabilities: StrongBoxCapabilities::default(),
        }
    }
}

// Crypto provider types
#[derive(Debug, Clone)]
pub struct RustCryptoProvider {
    pub supported_algorithms: Vec<Algorithm>,
    pub hardware_acceleration: bool,
}

impl Default for RustCryptoProvider {
    fn default() -> Self {
        Self {
            supported_algorithms: vec![
                Algorithm::EcdsaSha256,
                Algorithm::Ed25519,
                Algorithm::Aes256Gcm,
                Algorithm::ChaCha20Poly1305,
            ],
            hardware_acceleration: false,
        }
    }
}

// Crypto provider trait (referenced in several places)
#[async_trait::async_trait]
pub trait CryptoProvider: Send + Sync {
    fn supported_algorithms(&self) -> &[Algorithm];
    fn supports_algorithm(&self, algorithm: &Algorithm) -> bool;
    fn hardware_accelerated(&self) -> bool;
    
    // Additional methods that implementations expect
    async fn initialize(&self) -> Result<(), String>;
    async fn generate_key_material(&self, key_type: &KeyType) -> Result<Vec<u8>, String>;
    async fn encrypt(&self, data: &[u8], key: &[u8], algorithm: &Algorithm) -> Result<Vec<u8>, String>;
    async fn decrypt(&self, data: &[u8], key: &[u8], algorithm: &Algorithm) -> Result<Vec<u8>, String>;
    async fn sign(&self, data: &[u8], key: &[u8], algorithm: &Algorithm) -> Result<Vec<u8>, String>;
    async fn verify(&self, data: &[u8], signature: &[u8], key: &[u8], algorithm: &Algorithm) -> Result<bool, String>;
    async fn derive_key(&self, base_key: &[u8], salt: &[u8], info: &[u8]) -> Result<Vec<u8>, String>;
}

// CryptoProvider implementation for RustCryptoProvider is in software_hsm/crypto_providers/rust_crypto.rs

// Key caching types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CachedKeyInfo {
    pub key_id: String,
    pub algorithm: Algorithm,
    pub created_at: std::time::SystemTime,
    pub last_used: std::time::SystemTime,
    pub access_count: u64,
}

impl Default for CachedKeyInfo {
    fn default() -> Self {
        Self {
            key_id: String::new(),
            algorithm: Algorithm::EcdsaSha256,
            created_at: std::time::SystemTime::now(),
            last_used: std::time::SystemTime::now(),
                         access_count: 0,
         }
     }
}

impl Default for MemoryProtectionLevel {
    fn default() -> Self {
        MemoryProtectionLevel::Medium
    }
} 