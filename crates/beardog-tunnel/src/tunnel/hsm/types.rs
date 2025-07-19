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

pub mod algorithm;
pub mod config;
pub mod key;
pub mod status;
pub mod tier;

// Re-export commonly used types for convenience
pub use tier::{
    AttestationLevel, CertificationLevel, FallbackStrategy, HsmTier, HsmVendor, KeyHierarchy,
    KeyStorageType, MemoryProtectionLevel, SecureEnclaveType, SmartphoneType, SoftwareHsmType,
    StrongBoxImplementation, TamperResistanceLevel,
};

pub use key::{
    GenerateKeyRequest, HsmKey, HsmKeyInfo, HsmOperation, KeyAttestation, KeyHealthStatus,
    KeyMaterial, KeyMetadata, KeyPerformanceMetrics, KeyType, KeyUsagePolicy, WarningSeverity,
};

pub use config::{
    AlertThresholds, AndroidHsmConfig, AttestationConfig, AuditConfig, AuditEvent,
    AuditLogDestination, AuditLogFormat, AuthConfig, AuthCredentials, AuthMethod, ClusterConfig,
    ClusterNode, ConnectionType, CryptoBackend, DatabaseConfig, FailoverConfig, FailoverStrategy,
    FileStorageConfig, HaConfig, HardwareHsmConfig, HealthCheckConfig, HsmConfig,
    HsmConnectionConfig, HsmTierConfig, HybridHsmConfig, IosHsmConfig, KeyRotationPolicy,
    KeySource, KeyStoreConfig, KeychainAccessibility, KeychainConfig, KeystoreConfig,
    LoadBalancingStrategy, MemoryConfig, MfaConfig, MonitoringConfig, PerformanceConfig,
    SecureEnclaveConfig, SecurityConfig, SmartphoneHsmConfig, SoftwareHsmConfig,
};

pub use algorithm::{
    AesMode, Algorithm, AlgorithmCapability, AlgorithmParameters, AlgorithmSpecificParameters,
    CryptographicAlgorithm, EllipticCurveType, HashAlgorithm, PerformanceCharacteristics,
    RsaPadding,
};

pub use status::{
    AuditEventType, ClusterHealth, DegradationSeverity, ErrorContext, ErrorSeverity,
    ErrorStatistics, FailoverEvent, FailoverStatus, HsmAuditLogEntry, HsmCapability, HsmCapacity,
    HsmClusterStatus, HsmError, HsmHealthStatus, HsmInfo, HsmNodeStatus, HsmOperationalStatus,
    HsmStatistics, HsmStatusSummary, LoadBalancingStatus, NodeRole, OperationResult,
    PerformanceMetrics, ResourceUtilization,
};
