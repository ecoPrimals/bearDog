use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub enable_encryption: bool,

    pub algorithm: EncryptionAlgorithm,

    pub key_size: u32,

    pub mode: EncryptionMode,

    pub key_derivation: KeyDerivationConfig,

    pub context_aware_keys: ContextAwareKeyConfig,

    pub entropy_adjustment: EntropyAdjustmentConfig,

    pub key_rotation: KeyRotationConfig,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            enable_encryption: true,
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            key_size: 256,
            mode: EncryptionMode::Gcm,
            key_derivation: KeyDerivationConfig::default(),
            context_aware_keys: ContextAwareKeyConfig::default(),
            entropy_adjustment: EntropyAdjustmentConfig::default(),
            key_rotation: KeyRotationConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    Aes256Gcm,

    Aes256Cbc,

    Aes128Gcm,

    ChaCha20Poly1305,

    XChaCha20Poly1305,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EncryptionMode {
    Gcm,

    Cbc,

    Ctr,

    Ecb,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDerivationConfig {
    pub kdf: KeyDerivationFunction,

    pub iterations: u32,

    pub salt_size: u32,

    pub memory_cost: u32,

    pub parallelism: u32,
}

impl Default for KeyDerivationConfig {
    fn default() -> Self {
        Self {
            kdf: KeyDerivationFunction::Argon2id,
            iterations: 100_000,
            salt_size: 32,
            memory_cost: 65536, // 64MB
            parallelism: 4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyDerivationFunction {
    Pbkdf2Sha256,

    Argon2i,

    Argon2d,

    Argon2id,

    Scrypt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAwareKeyConfig {
    pub enabled: bool,

    pub context_factors: Vec<ContextFactor>,

    pub versioning: KeyVersioningConfig,

    pub policies: HashMap<String, KeyPolicy>,
}

impl Default for ContextAwareKeyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            context_factors: vec![
                ContextFactor::UserId,
                ContextFactor::DeviceId,
                ContextFactor::Timestamp,
            ],
            versioning: KeyVersioningConfig::default(),
            policies: HashMap::with_capacity(16),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContextFactor {
    UserId,

    DeviceId,

    Timestamp,

    Location,

    Application,

    SecurityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyVersioningConfig {
    pub enabled: bool,

    pub max_versions: u32,

    pub retention_period: Duration,

    pub auto_cleanup: bool,
}

impl Default for KeyVersioningConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_versions: 10,
            retention_period: Duration::from_secs(7_776_000), // 90 days
            auto_cleanup: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPolicy {
    pub min_strength: KeyStrength,

    pub usage_restrictions: Vec<KeyUsage>,

    pub expiration: KeyExpirationPolicy,

    pub access_control: KeyAccessControl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyAdjustmentConfig {
    pub enabled: bool,

    pub min_entropy: f64,

    pub sources: Vec<EntropySource>,

    pub adjustment_strategies: Vec<EntropyAdjustmentStrategy>,

    pub quality_assessment: EntropyQualityConfig,
}

impl Default for EntropyAdjustmentConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_entropy: 128.0, // bits
            sources: vec![
                EntropySource::SystemRandom,
                EntropySource::HardwareRng,
                EntropySource::UserInput,
            ],
            adjustment_strategies: vec![
                EntropyAdjustmentStrategy::Whitening,
                EntropyAdjustmentStrategy::Conditioning,
            ],
            quality_assessment: EntropyQualityConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotationConfig {
    pub enabled: bool,

    pub frequency: KeyRotationFrequency,

    pub triggers: Vec<RotationTrigger>,

    pub overlap_period: Duration,

    pub notifications: RotationNotifications,
}

impl Default for KeyRotationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            frequency: KeyRotationFrequency::Monthly,
            triggers: vec![
                RotationTrigger::TimeBasedRotation,
                RotationTrigger::UsageThreshold,
            ],
            overlap_period: Duration::from_secs(86400), // 24 hours
            notifications: RotationNotifications::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyStrength {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyUsage {
    Encryption,
    Decryption,
    Signing,
    Verification,
    KeyDerivation,
    KeyWrapping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyExpirationPolicy {
    pub enabled: bool,
    pub default_lifetime: Duration,
    pub max_lifetime: Duration,
    pub warning_period: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyAccessControl {
    pub allowed_users: Vec<String>,
    pub allowed_applications: Vec<String>,
    pub required_permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntropySource {
    SystemRandom,
    HardwareRng,
    UserInput,
    NetworkJitter,
    DiskActivity,
    CpuJitter,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntropyAdjustmentStrategy {
    Whitening,
    Conditioning,
    Pooling,
    Mixing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyQualityConfig {
    pub assessment_enabled: bool,
    pub min_quality_score: f64,
    pub quality_tests: Vec<EntropyQualityTest>,
}

impl Default for EntropyQualityConfig {
    fn default() -> Self {
        Self {
            assessment_enabled: true,
            min_quality_score: 0.8,
            quality_tests: vec![
                EntropyQualityTest::Frequency,
                EntropyQualityTest::Runs,
                EntropyQualityTest::Serial,
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntropyQualityTest {
    Frequency,
    Runs,
    Serial,
    ApproximateEntropy,
    CumulativeSum,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyRotationFrequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annually,
    OnDemand,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RotationTrigger {
    TimeBasedRotation,
    UsageThreshold,
    SecurityEvent,
    AdminRequest,
    ComplianceRequirement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationNotifications {
    pub enabled: bool,
    pub advance_warning_hours: u32,
    pub email_recipients: Vec<String>,
    pub webhook_urls: Vec<String>,
}

impl Default for RotationNotifications {
    fn default() -> Self {
        Self {
            enabled: true,
            advance_warning_hours: 24,
            email_recipients: Vec::new(),
            webhook_urls: Vec::new(),
        }
    }
}
