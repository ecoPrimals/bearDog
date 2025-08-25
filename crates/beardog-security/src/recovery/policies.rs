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


/// Recovery Policies
///
/// This module handles recovery policies and trust boundaries.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::types::*;
use beardog_errors::{BearDogError, BearDogResult};
/// Recovery policy settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryPolicy {
    /// Maximum recovery attempts per day
    pub max_attempts_per_day: u32,
    /// Cooldown period between attempts (hours)
    pub cooldown_hours: u32,
    /// Whether to require additional verification
    pub require_additional_verification: bool,
    /// Types of additional verification required
    pub additional_verification_types: Vec<VerificationType>,
}
impl Default for RecoveryPolicy {}


    fn default() -> Self {
        Self {
            max_attempts_per_day: 3,
            cooldown_hours: 1,
            require_additional_verification: false,
            additional_verification_types: Vec::new(),
        }
    }
/// User-controlled recovery policy configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserRecoveryPolicy {
    /// User ID this policy belongs to
    pub user_id: String,
    /// Whether to allow mixed recovery (multiple methods)
    pub allow_mixed_recovery: bool,
    /// Trust boundaries configuration
    pub trust_boundaries: UserTrustBoundaries,
    /// Recovery contexts (family, work, emergency, etc.)
    pub recovery_contexts: Vec<RecoveryContext>,
    /// Global recovery settings
    pub global_settings: HashMap<String, String>,
/// User-defined trust boundaries
pub struct UserTrustBoundaries {
    /// Maximum number of recovery attempts per context
    pub max_attempts_per_context: u32,
    /// Require multiple contexts for high-value operations
    pub require_multiple_contexts: bool,
    /// Minimum trust level required
    pub min_trust_level: u8,
    /// Trust decay settings
    pub trust_decay_enabled: bool,
/// Recovery context defined by user (family, work, emergency, etc.)
pub struct RecoveryContext {
    /// Context ID
    pub id: String,
    /// Context name
    pub name: String,
    /// Context description
    pub description: String,
    /// Shard allocation for this context
    pub shard_allocation: ShardAllocation,
    /// Trust requirements for this context
    pub trust_requirements: ContextTrustRequirements,
/// How shards are allocated to a recovery context
pub struct ShardAllocation {
    /// Number of shards allocated to this context
    pub shard_count: u32,
    /// Minimum shards required from this context
    pub min_required: u32,
    /// Backup strategies for this context
    pub backup_strategies: Vec<BackupStrategy>,
/// Context-specific trust requirements
pub struct ContextTrustRequirements {
    /// Require identity verification
    pub require_identity_verification: bool,
    /// Verification methods allowed
    pub allowed_verification_methods: Vec<VerificationMethod>,}


impl Default for UserTrustBoundaries {
            max_attempts_per_context: 3,
            require_multiple_contexts: false,
            min_trust_level: 50,
            trust_decay_enabled: true,
