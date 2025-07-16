//! Ethical framework for human entropy collection
//!
//! This module defines comprehensive ethical guidelines and policies for
//! collecting human-generated entropy, ensuring privacy, consent, and user
//! control at all times.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Ethical framework for human entropy collection
///
/// Defines the comprehensive ethical guidelines and policies that must be
/// followed during human entropy collection, ensuring privacy, consent,
/// and user control at all times.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyCollectionEthics {
    /// Informed consent requirements and validation
    pub informed_consent: InformedConsent,
    /// Privacy protection policies and measures
    pub privacy_protection: PrivacyProtection,
    /// Data minimization principles and enforcement
    pub data_minimization: DataMinimization,
    /// User control capabilities and rights
    pub user_control: UserControl,
    /// Purpose limitation and scope definitions
    pub purpose_limitation: PurposeLimitation,
    /// Transparency policies and reporting
    pub transparency: TransparencyPolicy,
}

/// Informed consent for entropy collection
///
/// Represents the legally binding informed consent given by a user for
/// entropy collection activities. Includes full disclosure of collection
/// methods, usage, and retention policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InformedConsent {
    /// Detailed description of what data is collected and how
    pub collection_description: String,
    /// Explanation of how the collected data will be used
    pub usage_description: String,
    /// How long the processed data will be retained
    pub retention_period: chrono::Duration,
    /// User's withdrawal rights and procedures
    pub withdrawal_rights: WithdrawalRights,
    /// Timestamp when consent was granted
    pub consent_timestamp: DateTime<Utc>,
    /// Digital signature validating the consent
    pub consent_signature: DigitalSignature,
}

/// User withdrawal rights and procedures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalRights {
    /// Whether the user can withdraw consent
    pub can_withdraw: bool,
    /// Process for withdrawing consent
    pub withdrawal_process: String,
    /// Timeline for data deletion after withdrawal
    pub data_deletion_timeline: chrono::Duration,
}

/// Digital signature for consent validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalSignature {
    /// Raw signature bytes
    pub signature_bytes: Vec<u8>,
    /// Signature algorithm used
    pub algorithm: String,
    /// Identifier of the signing key
    pub key_id: String,
}

/// Privacy protection policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyProtection {
    /// Whether data minimization is applied
    pub data_minimization: bool,
    /// Whether raw data is immediately deleted after processing
    pub immediate_raw_deletion: bool,
    /// Level of anonymization applied to processed data
    pub anonymization_level: AnonymizationLevel,
    /// Whether data is encrypted at rest
    pub encryption_at_rest: bool,
}

/// Levels of data anonymization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnonymizationLevel {
    /// No anonymization applied
    None,
    /// Basic anonymization (identifiers removed)
    Low,
    /// Moderate anonymization (additional noise added)
    Medium,
    /// High anonymization (significant data transformation)
    High,
    /// Maximum anonymization (only aggregate patterns preserved)
    Maximum,
}

/// Data minimization principles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataMinimization {
    /// Only collect data that is strictly necessary
    pub collect_only_necessary: bool,
    /// Only extract features, not store raw data
    pub feature_extraction_only: bool,
    /// How long raw data is retained (should be minimal)
    pub raw_data_retention: chrono::Duration,
}

/// User control capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserControl {
    /// User can pause data collection at any time
    pub can_pause_collection: bool,
    /// User can review what data has been collected
    pub can_review_data: bool,
    /// User can delete their data
    pub can_delete_data: bool,
    /// User has granular control over what is collected
    pub granular_permissions: bool,
}

/// Purpose limitation for data collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurposeLimitation {
    /// Primary purpose for data collection
    pub primary_purpose: String,
    /// Any secondary purposes (requires explicit consent)
    pub secondary_purposes: Vec<String>,
    /// Whether users are notified of purpose changes
    pub purpose_change_notification: bool,
}

/// Transparency policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransparencyPolicy {
    /// Description of algorithms used for processing
    pub algorithm_description: String,
    /// Documentation of data flow and processing
    pub data_flow_documentation: String,
    /// Security measures in place
    pub security_measures: String,
    /// Whether users can access audit logs
    pub audit_log_access: bool,
}

/// Privacy metadata for collected data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyMetadata {
    /// Whether data minimization was applied
    pub data_minimization_applied: bool,
    /// Whether raw data was deleted
    pub raw_data_deleted: bool,
    /// Level of anonymization applied
    pub anonymization_level: AnonymizationLevel,
    /// When the data was processed
    pub processing_timestamp: DateTime<Utc>,
}
