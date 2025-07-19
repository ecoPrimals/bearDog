//! Recovery Shards
//!
//! This module handles Shamir's Secret Sharing for recovery operations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types::*;

/// Recovery shard using Shamir's Secret Sharing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryShard {
    /// Shard ID
    pub id: String,
    /// User ID this shard belongs to
    pub user_id: String,
    /// Shard index (used in SSS)
    pub index: u32,
    /// Encrypted shard data
    pub encrypted_data: String,
    /// Shard holder information
    pub holder: ShardHolderInfo,
    /// When the shard was created
    pub created_at: DateTime<Utc>,
    /// When the shard expires
    pub expires_at: DateTime<Utc>,
    /// Usage restrictions
    pub usage_restrictions: ShardUsageRestrictions,
    /// Shard metadata
    pub metadata: HashMap<String, String>,
}

/// Information about who holds a recovery shard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardHolderInfo {
    /// Holder ID
    pub id: String,
    /// Holder name
    pub name: String,
    /// Type of holder
    pub holder_type: ShardHolderType,
    /// Verification status
    pub verification_status: HolderVerificationStatus,
    /// Contact information
    pub contact_info: String,
    /// Trust level (0-100)
    pub trust_level: u8,
    /// When the holder was last verified
    pub last_verified: Option<DateTime<Utc>>,
    /// Holder metadata
    pub metadata: HashMap<String, String>,
}

/// Usage restrictions for recovery shards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardUsageRestrictions {
    /// Maximum number of times this shard can be used
    pub max_uses: u32,
    /// Number of times this shard has been used
    pub times_used: u32,
    /// Time window for usage
    pub time_window: TimeWindow,
    /// Require additional verification
    pub require_verification: bool,
    /// Allowed verification methods
    pub allowed_verification_methods: Vec<VerificationMethod>,
}

/// Time window restrictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    /// Start time for the window
    pub start: DateTime<Utc>,
    /// End time for the window
    pub end: DateTime<Utc>,
    /// Days of the week when usage is allowed
    pub allowed_days: Vec<u8>, // 0=Sunday, 6=Saturday
    /// Hours of the day when usage is allowed
    pub allowed_hours: Vec<u8>, // 0-23
}

impl RecoveryShard {
    /// Create a new recovery shard
    pub fn new(
        id: String,
        user_id: String,
        index: u32,
        encrypted_data: String,
        holder: ShardHolderInfo,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            user_id,
            index,
            encrypted_data,
            holder,
            created_at: now,
            expires_at: now + chrono::Duration::days(365), // 1 year expiry
            usage_restrictions: ShardUsageRestrictions::default(),
            metadata: HashMap::new(),
        }
    }

    /// Check if the shard is still valid
    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at
    }

    /// Check if the shard can be used now
    pub fn can_use(&self) -> bool {
        self.is_valid() && self.usage_restrictions.times_used < self.usage_restrictions.max_uses
    }

    /// Mark the shard as used
    pub fn mark_used(&mut self) {
        self.usage_restrictions.times_used += 1;
    }
}

impl Default for ShardUsageRestrictions {
    fn default() -> Self {
        Self {
            max_uses: 10,
            times_used: 0,
            time_window: TimeWindow::default(),
            require_verification: false,
            allowed_verification_methods: Vec::new(),
        }
    }
}

impl Default for TimeWindow {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            start: now,
            end: now + chrono::Duration::days(365),
            allowed_days: vec![0, 1, 2, 3, 4, 5, 6], // All days
            allowed_hours: (0..24).collect(),        // All hours
        }
    }
}

/// Demonstration of key worthlessness principle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyWorthinessDemo {
    /// Demo ID
    pub id: String,
    /// User ID this demo belongs to
    pub user_id: String,
    /// Demo scenario
    pub scenario: String,
    /// Key information (safe to share)
    pub key_info: HashMap<String, String>,
    /// Demonstration results
    pub results: Vec<String>,
    /// When the demo was created
    pub created_at: DateTime<Utc>,
}

impl KeyWorthinessDemo {
    /// Create a new key worthiness demonstration
    pub fn new(id: String, user_id: String, scenario: String) -> Self {
        Self {
            id,
            user_id,
            scenario,
            key_info: HashMap::new(),
            results: Vec::new(),
            created_at: Utc::now(),
        }
    }

    /// Add a demonstration result
    pub fn add_result(&mut self, result: String) {
        self.results.push(result);
    }
}
