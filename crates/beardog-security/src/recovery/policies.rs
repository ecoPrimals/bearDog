

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::types::*;
use beardog_errors::BearDogError;

#[derive(Debug, Clone)]
    /// Number of cooldown_hours
    pub cooldown_hours: u32,

    /// Whether require_additional_verification is enabled
    pub require_additional_verification: bool,

    /// Collection of additional verification types
    pub additional_verification_types: Vec<VerificationType>,
}
impl Default for RecoveryPolicy {}

    fn default(3,
            cooldown_hours: 1,
            require_additional_verification: false,
            additional_verification_types: Vec::new(String,

    /// Whether allow_mixed_recovery is enabled
    pub allow_mixed_recovery: bool,

    /// The trust boundaries value
    pub trust_boundaries: UserTrustBoundaries,

    /// Collection of recovery contexts
    pub recovery_contexts: Vec<RecoveryContext>,

    /// Mapping of global settings
    pub global_settings: HashMap<String, String>,

pub struct UserTrustBoundaries {

    /// Number of max_attempts_per_context
    pub max_attempts_per_context: u32,

    /// Whether require_multiple_contexts is enabled
    pub require_multiple_contexts: bool,

    /// Number of min_trust_level
    pub min_trust_level: u8,

    /// Whether trust_decay is enabled
    pub trust_decay_enabled: bool,

pub struct RecoveryContext {


    pub id: String,

    /// Name of the item
    pub name: String,

    /// The description value
    pub description: String,

    /// The shard allocation value
    pub shard_allocation: ShardAllocation,

    /// The trust requirements value
    pub trust_requirements: ContextTrustRequirements,

pub struct ShardAllocation {

    /// Number of shard
    pub shard_count: u32,

    /// Number of min_required
    pub min_required: u32,

    /// Collection of backup strategies
    pub backup_strategies: Vec<BackupStrategy>,

pub struct ContextTrustRequirements {


    pub require_identity_verification: bool,

    /// Collection of allowed verification methods
    pub allowed_verification_methods: Vec<VerificationMethod>,}

impl Default for UserTrustBoundaries {
            max_attempts_per_context: 3,
            require_multiple_contexts: false,
            min_trust_level: 50,
            trust_decay_enabled: true,
