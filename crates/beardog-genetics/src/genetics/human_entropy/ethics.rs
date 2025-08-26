

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyCollectionEthics {

    pub informed_consent: InformedConsent,

    pub privacy_protection: PrivacyProtection,

    pub data_minimization: DataMinimization,

    pub user_control: UserControl,

    pub purpose_limitation: PurposeLimitation,

    pub transparency: TransparencyPolicy,
}

pub struct InformedConsent {

    pub collection_description: String,

    pub usage_description: String,

    pub retention_period: chrono::Duration,

    pub withdrawal_rights: WithdrawalRights,

    pub consent_timestamp: DateTime<Utc>,

    pub consent_signature: DigitalSignature,

pub struct WithdrawalRights {

    pub can_withdraw: bool,

    pub withdrawal_process: String,

    pub data_deletion_timeline: chrono::Duration,

pub struct DigitalSignature {

    pub signature_bytes: Vec<u8>,

    pub algorithm: String,

    pub key_id: String,

pub struct PrivacyProtection {

    pub data_minimization: bool,

    pub immediate_raw_deletion: bool,

    pub anonymization_level: AnonymizationLevel,

    pub encryption_at_rest: bool,

pub enum AnonymizationLevel {

    None,

    Low,

    Medium,

    High,

    Maximum,

pub struct DataMinimization {

    pub collect_only_necessary: bool,

    pub feature_extraction_only: bool,

    pub raw_data_retention: chrono::Duration,

pub struct UserControl {

    pub can_pause_collection: bool,

    pub can_review_data: bool,

    pub can_delete_data: bool,

    pub granular_permissions: bool,

pub struct PurposeLimitation {

    pub primary_purpose: String,

    pub secondary_purposes: Vec<String>,

    pub purpose_change_notification: bool,

pub struct TransparencyPolicy {

    pub algorithm_description: String,

    pub data_flow_documentation: String,

    pub security_measures: String,

    pub audit_log_access: bool,

pub struct PrivacyMetadata {

    pub data_minimization_applied: bool,

    pub raw_data_deleted: bool,

    pub processing_timestamp: DateTime<Utc>,
