// SPDX-License-Identifier: AGPL-3.0-or-later



// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
    /// The privacy protection value
    pub privacy_protection: PrivacyProtection,

    /// The data minimization value
    pub data_minimization: DataMinimization,

    /// The user control value
    pub user_control: UserControl,

    /// The purpose limitation value
    pub purpose_limitation: PurposeLimitation,

    /// The transparency value
    pub transparency: TransparencyPolicy,
}

pub struct InformedConsent {

    /// The collection description value
    pub collection_description: String,

    /// The usage description value
    pub usage_description: String,

    /// The retention period value
    pub retention_period: chrono::Duration,

    /// The withdrawal rights value
    pub withdrawal_rights: WithdrawalRights,


    pub consent_timestamp: DateTime<Utc>,

    /// The consent signature value
    pub consent_signature: DigitalSignature,
}

pub struct WithdrawalRights {

    /// Whether can_withdraw is enabled
    pub can_withdraw: bool,

    /// The withdrawal process value
    pub withdrawal_process: String,


    pub data_deletion_timeline: chrono::Duration,
}

pub struct DigitalSignature {

    /// Collection of signature bytes
    pub signature_bytes: Vec<u8>,

    /// The algorithm value
    pub algorithm: String,


    pub key_id: String,
}

pub struct PrivacyProtection {

    /// Whether data_minimization is enabled
    pub data_minimization: bool,

    /// Whether immediate_raw_deletion is enabled
    pub immediate_raw_deletion: bool,

    /// The anonymization level value
    pub anonymization_level: AnonymizationLevel,

    /// Whether encryption_at_rest is enabled
    pub encryption_at_rest: bool,
}

pub enum AnonymizationLevel {


    /// No none specified
    None,


    /// Represents low variant
    Low,


    /// Represents medium variant
    Medium,


    /// Represents high variant
    High,


    /// Represents maximum variant
    Maximum,
}

pub struct DataMinimization {

    /// Whether collect_only_necessary is enabled
    pub collect_only_necessary: bool,

    /// Whether feature_extraction_only is enabled
    pub feature_extraction_only: bool,

    /// The raw data retention value
    pub raw_data_retention: chrono::Duration,
}

pub struct UserControl {

    /// Whether can_pause_collection is enabled
    pub can_pause_collection: bool,

    /// Whether can_review_data is enabled
    pub can_review_data: bool,

    /// Whether can_delete_data is enabled
    pub can_delete_data: bool,

    /// Whether granular_permissions is enabled
    pub granular_permissions: bool,
}

pub struct PurposeLimitation {

    /// The primary purpose value
    pub primary_purpose: String,

    /// Collection of secondary purposes
    pub secondary_purposes: Vec<String>,

    /// Whether purpose_change_notification is enabled
    pub purpose_change_notification: bool,
}

pub struct TransparencyPolicy {

    /// The algorithm description value
    pub algorithm_description: String,

    /// The data flow documentation value
    pub data_flow_documentation: String,

    /// The security measures value
    pub security_measures: String,

    /// Whether audit_log_access is enabled
    pub audit_log_access: bool,
}

pub struct PrivacyMetadata {

    /// Whether data_minimization_applied is enabled
    pub data_minimization_applied: bool,

    /// Whether raw_data_deleted is enabled
    pub raw_data_deleted: bool,


    pub processing_timestamp: DateTime<Utc>,
}
