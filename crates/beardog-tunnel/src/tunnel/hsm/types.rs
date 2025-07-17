//! HSM Types Module
//!
//! This module contains all the types and structures used across the HSM system.
//! It includes HSM tiers, key types, configurations, and other supporting types.
//!
//! ## Module Structure
//!
//! - `tier`: HSM tier definitions and related types
//! - `key`: Key types, metadata, and key-related structures
//! - `config`: HSM configuration structures
//! - `algorithm`: Cryptographic algorithm definitions
//! - `status`: HSM health and status monitoring types

/// HSM type enumeration
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

pub mod tier;
pub mod key;
pub mod config;
pub mod algorithm;
pub mod status;

// Re-export commonly used types for convenience
pub use tier::{
    HsmTier, SmartphoneType, SecureEnclaveType, StrongBoxImplementation,
    SoftwareHsmType, KeyStorageType, MemoryProtectionLevel, HsmVendor,
    CertificationLevel, TamperResistanceLevel, KeyHierarchy, FallbackStrategy,
    AttestationLevel,
};

pub use key::{
    KeyType, KeyMetadata, KeyUsagePolicy, HsmKey, HsmKeyInfo, KeyAttestation,
    KeyHealthStatus, WarningSeverity, GenerateKeyRequest, KeyMaterial,
    KeyPerformanceMetrics, HsmOperation,
};

pub use config::{
    HsmConfig, HsmTierConfig, SoftwareHsmConfig, HardwareHsmConfig,
    SmartphoneHsmConfig, IosHsmConfig, AndroidHsmConfig, HybridHsmConfig,
    SecurityConfig, PerformanceConfig, MonitoringConfig, AlertThresholds,
    HsmConnectionConfig, ConnectionType, AuthConfig, AuthMethod, AuthCredentials,
    MfaConfig, HaConfig, ClusterConfig, ClusterNode, FailoverConfig,
    FailoverStrategy, LoadBalancingStrategy, HealthCheckConfig, KeyRotationPolicy,
    AuditConfig, AuditLogFormat, AuditLogDestination, AuditEvent,
    SecureEnclaveConfig, KeychainConfig, KeychainAccessibility, KeystoreConfig,
    AttestationConfig, KeyStoreConfig, KeySource, FileStorageConfig,
    DatabaseConfig, MemoryConfig, CryptoBackend,
};

pub use algorithm::{
    CryptographicAlgorithm, AlgorithmParameters, AlgorithmSpecificParameters,
    AesMode, RsaPadding, HashAlgorithm, EllipticCurveType, AlgorithmCapability,
    PerformanceCharacteristics, Algorithm,
};

pub use status::{
    HsmHealthStatus, PerformanceMetrics, HsmOperationalStatus, DegradationSeverity,
    ResourceUtilization, HsmCapacity, HsmError, ErrorSeverity, ErrorContext,
    HsmStatistics, ErrorStatistics, HsmAuditLogEntry, AuditEventType,
    OperationResult, HsmStatusSummary, HsmClusterStatus, HsmNodeStatus,
    NodeRole, ClusterHealth, LoadBalancingStatus, FailoverStatus, FailoverEvent,
    HsmInfo, HsmCapability,
}; 