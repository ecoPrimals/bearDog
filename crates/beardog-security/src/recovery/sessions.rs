//! Recovery Session Management
//!
//! This module handles the creation and management of recovery sessions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::challenges::*;
use super::types::*;

/// Active recovery session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverySession {
    /// Session ID
    pub id: String,
    /// User ID attempting recovery
    pub user_id: String,
    /// Type of recovery being attempted
    pub recovery_type: RecoveryType,
    /// Session status
    pub status: RecoveryStatus,
    /// When the session was created
    pub created_at: DateTime<Utc>,
    /// When the session expires
    pub expires_at: DateTime<Utc>,
    /// Challenges that need to be completed
    pub challenges: Vec<RecoveryChallenge>,
    /// Responses received so far
    pub responses: Vec<ChallengeResponse>,
    /// Metadata about the recovery attempt
    pub metadata: HashMap<String, String>,
}

impl RecoverySession {
    /// Create a new recovery session
    pub fn new(id: String, user_id: String, recovery_type: RecoveryType) -> Self {
        let now = Utc::now();
        Self {
            id,
            user_id,
            recovery_type,
            status: RecoveryStatus::Active,
            created_at: now,
            expires_at: now + chrono::Duration::hours(24), // 24 hour expiry
            challenges: Vec::new(),
            responses: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Check if the session has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Add a challenge to the session
    pub fn add_challenge(&mut self, challenge: RecoveryChallenge) {
        self.challenges.push(challenge);
    }

    /// Add a response to the session
    pub fn add_response(&mut self, response: ChallengeResponse) {
        self.responses.push(response);
    }

    /// Check if all challenges have been completed
    pub fn is_complete(&self) -> bool {
        self.challenges.len() == self.responses.len()
    }

    /// Get completion percentage
    pub fn completion_percentage(&self) -> f64 {
        if self.challenges.is_empty() {
            0.0
        } else {
            (self.responses.len() as f64 / self.challenges.len() as f64) * 100.0
        }
    }

    /// Update session status
    pub fn update_status(&mut self, status: RecoveryStatus) {
        self.status = status;
    }

    /// Add metadata to the session
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Extend session expiry
    pub fn extend_expiry(&mut self, hours: i64) {
        self.expires_at += chrono::Duration::hours(hours);
    }

    /// Get remaining time until expiry
    pub fn remaining_time(&self) -> chrono::Duration {
        self.expires_at - Utc::now()
    }
}

/// Mixed recovery session combining multiple methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedRecoverySession {
    /// Session ID
    pub id: String,
    /// User ID attempting recovery
    pub user_id: String,
    /// Overall session status
    pub status: MixedRecoveryStatus,
    /// When the session was created
    pub created_at: DateTime<Utc>,
    /// When the session expires
    pub expires_at: DateTime<Utc>,
    /// Active recovery methods
    pub active_methods: Vec<ActiveRecoveryMethod>,
    /// Collected shards from various methods
    pub collected_shards: Vec<CollectedShard>,
    /// Minimum shards required for recovery
    pub min_shards_required: u32,
    /// Recovery progress tracking
    pub progress: RecoveryProgress,
    /// Session metadata
    pub metadata: HashMap<String, String>,
}

impl MixedRecoverySession {
    /// Create a new mixed recovery session
    pub fn new(id: String, user_id: String, min_shards_required: u32) -> Self {
        let now = Utc::now();
        Self {
            id,
            user_id,
            status: MixedRecoveryStatus::Initializing,
            created_at: now,
            expires_at: now + chrono::Duration::hours(48), // 48 hour expiry for mixed recovery
            active_methods: Vec::new(),
            collected_shards: Vec::new(),
            min_shards_required,
            progress: RecoveryProgress::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a recovery method to the session
    pub fn add_recovery_method(&mut self, method: ActiveRecoveryMethod) {
        self.active_methods.push(method);
    }

    /// Collect a shard from a recovery method
    pub fn collect_shard(&mut self, shard: CollectedShard) {
        self.collected_shards.push(shard);
        self.progress
            .update_shard_count(self.collected_shards.len() as u32);
    }

    /// Check if enough shards have been collected
    pub fn has_enough_shards(&self) -> bool {
        self.collected_shards.len() as u32 >= self.min_shards_required
    }

    /// Get completion percentage
    pub fn completion_percentage(&self) -> f64 {
        if self.min_shards_required == 0 {
            0.0
        } else {
            (self.collected_shards.len() as f64 / self.min_shards_required as f64) * 100.0
        }
    }

    /// Check if the session has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Update session status
    pub fn update_status(&mut self, status: MixedRecoveryStatus) {
        self.status = status;
    }
}

/// Active recovery method in a mixed session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveRecoveryMethod {
    /// Method ID
    pub id: String,
    /// Type of recovery method
    pub method_type: RecoveryType,
    /// Current status of the method
    pub status: MethodStatus,
    /// When the method was started
    pub started_at: DateTime<Utc>,
    /// Expected completion time
    pub expected_completion: DateTime<Utc>,
    /// Progress information
    pub progress: f64,
    /// Method-specific metadata
    pub metadata: HashMap<String, String>,
}

impl ActiveRecoveryMethod {
    /// Create a new active recovery method
    pub fn new(id: String, method_type: RecoveryType) -> Self {
        let now = Utc::now();
        Self {
            id,
            method_type,
            status: MethodStatus::Active,
            started_at: now,
            expected_completion: now + chrono::Duration::hours(24),
            progress: 0.0,
            metadata: HashMap::new(),
        }
    }

    /// Update the progress of the method
    pub fn update_progress(&mut self, progress: f64) {
        self.progress = progress.clamp(0.0, 100.0);
    }

    /// Update the status of the method
    pub fn update_status(&mut self, status: MethodStatus) {
        self.status = status;
    }

    /// Check if the method has completed
    pub fn is_complete(&self) -> bool {
        matches!(self.status, MethodStatus::Completed)
    }

    /// Check if the method has failed
    pub fn is_failed(&self) -> bool {
        matches!(self.status, MethodStatus::Failed)
    }
}

/// Collected shard information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectedShard {
    /// Shard ID
    pub id: String,
    /// Which recovery method provided this shard
    pub source_method: String,
    /// Shard data (encrypted)
    pub data: String,
    /// Verification status
    pub verification_status: ShardVerificationStatus,
    /// When the shard was collected
    pub collected_at: DateTime<Utc>,
    /// Shard metadata
    pub metadata: HashMap<String, String>,
}

impl CollectedShard {
    /// Create a new collected shard
    pub fn new(id: String, source_method: String, data: String) -> Self {
        Self {
            id,
            source_method,
            data,
            verification_status: ShardVerificationStatus::Pending,
            collected_at: Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Update verification status
    pub fn update_verification_status(&mut self, status: ShardVerificationStatus) {
        self.verification_status = status;
    }

    /// Check if the shard is valid
    pub fn is_valid(&self) -> bool {
        matches!(self.verification_status, ShardVerificationStatus::Valid)
    }
}

/// Recovery progress tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryProgress {
    /// Current step in the recovery process
    pub current_step: u32,
    /// Total steps in the recovery process
    pub total_steps: u32,
    /// Number of shards collected
    pub shards_collected: u32,
    /// Number of shards required
    pub shards_required: u32,
    /// Overall completion percentage
    pub completion_percentage: f64,
    /// Status message
    pub status_message: String,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

impl RecoveryProgress {
    /// Create a new recovery progress tracker
    pub fn new() -> Self {
        Self {
            current_step: 0,
            total_steps: 0,
            shards_collected: 0,
            shards_required: 0,
            completion_percentage: 0.0,
            status_message: "Initializing recovery process".to_string(),
            last_updated: Utc::now(),
        }
    }

    /// Update the current step
    pub fn update_step(&mut self, step: u32) {
        self.current_step = step;
        self.update_completion_percentage();
    }

    /// Update the shard count
    pub fn update_shard_count(&mut self, collected: u32) {
        self.shards_collected = collected;
        self.update_completion_percentage();
    }

    /// Update the status message
    pub fn update_status_message(&mut self, message: String) {
        self.status_message = message;
        self.last_updated = Utc::now();
    }

    /// Calculate and update completion percentage
    fn update_completion_percentage(&mut self) {
        if self.total_steps > 0 {
            self.completion_percentage =
                (self.current_step as f64 / self.total_steps as f64) * 100.0;
        } else if self.shards_required > 0 {
            self.completion_percentage =
                (self.shards_collected as f64 / self.shards_required as f64) * 100.0;
        }
        self.last_updated = Utc::now();
    }
}

impl Default for RecoveryProgress {
    fn default() -> Self {
        Self::new()
    }
}
