// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Canonical Encryption Configuration
///
/// **UNIFIED ENCRYPTION CONFIGURATION** - Single source of truth for all encryption settings
/// This module consolidates encryption configuration from:
/// - beardog-security/src/encryption.rs::UnifiedSecurityConfig
/// - beardog-security/src/types/crypto_types.rs::ContextAwareKeyConfig
/// - Various scattered encryption settings across the ecosystem

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **CANONICAL ENCRYPTION CONFIGURATION** - Main encryption settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Enable encryption globally
    pub enable_encryption: bool,
    /// Default encryption algorithm
    pub algorithm: EncryptionAlgorithm,
    /// Key size in bits
    pub key_size: u32,
    /// Encryption mode
    pub mode: EncryptionMode,
    /// Key derivation configuration
    pub key_derivation: KeyDerivationConfig,
    /// Context-aware key management
    pub context_aware_keys: ContextAwareKeyConfig,
    /// Entropy adjustment settings
    pub entropy_adjustment: EntropyAdjustmentConfig,
    /// Key rotation settings
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

/// **CANONICAL ENCRYPTION ALGORITHMS** - Supported encryption algorithms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    /// AES-256 in GCM mode (recommended for most use cases)
    Aes256Gcm,
    /// AES-256 in CBC mode
    Aes256Cbc,
    /// AES-128 in GCM mode
    Aes128Gcm,
    /// ChaCha20-Poly1305
    ChaCha20Poly1305,
    /// XChaCha20-Poly1305
    XChaCha20Poly1305,
}

/// **CANONICAL ENCRYPTION MODES** - Supported encryption modes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EncryptionMode {
    /// Galois/Counter Mode (authenticated encryption)
    Gcm,
    /// Cipher Block Chaining mode
    Cbc,
    /// Counter mode
    Ctr,
    /// Electronic Codebook mode (not recommended for production)
    Ecb,
}

/// **CANONICAL KEY DERIVATION CONFIGURATION** - Key derivation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDerivationConfig {
    /// Key derivation function
    pub kdf: KeyDerivationFunction,
    /// Number of iterations for PBKDF2
    pub iterations: u32,
    /// Salt size in bytes
    pub salt_size: u32,
    /// Memory cost for Argon2 (in KB)
    pub memory_cost: u32,
    /// Parallelism for Argon2
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

/// **CANONICAL KEY DERIVATION FUNCTIONS** - Supported KDFs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyDerivationFunction {
    /// PBKDF2 with SHA-256
    Pbkdf2Sha256,
    /// Argon2i
    Argon2i,
    /// Argon2d
    Argon2d,
    /// Argon2id (recommended)
    Argon2id,
    /// scrypt
    Scrypt,
}

/// **CANONICAL CONTEXT-AWARE KEY CONFIGURATION** - Context-based key management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAwareKeyConfig {
    /// Enable context-aware key generation
    pub enabled: bool,
    /// Context factors to consider
    pub context_factors: Vec<ContextFactor>,
    /// Key versioning settings
    pub versioning: KeyVersioningConfig,
    /// Context-specific key policies
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
            policies: HashMap::new(),
        }
    }
}

/// **CANONICAL CONTEXT FACTORS** - Factors considered in context-aware keys
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContextFactor {
    /// User identifier
    UserId,
    /// Device identifier
    DeviceId,
    /// Timestamp
    Timestamp,
    /// Geographic location
    Location,
    /// Application context
    Application,
    /// Security level
    SecurityLevel,
}

/// **CANONICAL KEY VERSIONING CONFIGURATION** - Key version management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyVersioningConfig {
    /// Enable key versioning
    pub enabled: bool,
    /// Maximum number of key versions to retain
    pub max_versions: u32,
    /// Version retention period
    pub retention_period: Duration,
    /// Automatic version cleanup
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

/// **CANONICAL KEY POLICY** - Security policies for keys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPolicy {
    /// Minimum key strength
    pub min_strength: KeyStrength,
    /// Key usage restrictions
    pub usage_restrictions: Vec<KeyUsage>,
    /// Expiration policy
    pub expiration: KeyExpirationPolicy,
    /// Access control
    pub access_control: KeyAccessControl,
}

/// **CANONICAL ENTROPY ADJUSTMENT CONFIGURATION** - Entropy management settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyAdjustmentConfig {
    /// Enable entropy adjustment
    pub enabled: bool,
    /// Minimum entropy threshold
    pub min_entropy: f64,
    /// Entropy sources
    pub sources: Vec<EntropySource>,
    /// Adjustment strategies
    pub adjustment_strategies: Vec<EntropyAdjustmentStrategy>,
    /// Quality assessment settings
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

/// **CANONICAL KEY ROTATION CONFIGURATION** - Automatic key rotation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotationConfig {
    /// Enable automatic key rotation
    pub enabled: bool,
    /// Rotation frequency
    pub frequency: KeyRotationFrequency,
    /// Rotation triggers
    pub triggers: Vec<RotationTrigger>,
    /// Overlap period for smooth transition
    pub overlap_period: Duration,
    /// Notification settings
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

// Supporting enums and structs

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
                EntropyQualityTest::FrequencyTest,
                EntropyQualityTest::RunsTest,
                EntropyQualityTest::SerialTest,
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntropyQualityTest {
    FrequencyTest,
    RunsTest,
    SerialTest,
    ApproximateEntropyTest,
    CumulativeSumTest,
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