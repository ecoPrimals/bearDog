

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrivacyProtectionType {
    TrafficObfuscation,
    DataAnonymization,
    MetadataScrubbing,
    OnionRouting,
    NoiseInjection,
    EncryptionAtRest,
    EncryptionInTransit,
    AccessControlMatrix,
    ZeroKnowledgeProof,
    HomomorphicEncryption,
}

pub enum VulnerabilityType {
    MetadataLeakage,
    TrafficAnalysis,
    TimingAttack,
    SideChannelAttack,
    DataResidue,
    UnencryptedTransport,
    WeakAuthentication,
    ExcessivePermissions,

pub enum VulnerabilitySeverity {
    Critical,
    High,
    Medium,
    Low,
    Informational,

pub enum IndicatorType {
    NetworkScanning,
    DataExfiltration,
    UnusualTraffic,
    MetadataCollection,
    FingerprintingAttempt,
    CorrelationAttack,

pub use beardog_types::canonical::AuditEventType;

pub enum PrivacyImpactLevel {
    None,

pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    UnderReview,
    Remediated,
