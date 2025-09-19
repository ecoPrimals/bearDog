

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::challenges::*;
use super::types::*;
use beardog_errors::BearDogError;

#[derive(Debug, Clone)]
    pub user_id: String,

    /// The recovery type value
    pub recovery_type: RecoveryType,

    /// Current status of the component
    pub status: RecoveryStatus,

    /// The created at value
    pub created_at: DateTime<Utc>,

    /// The expires at value
    pub expires_at: DateTime<Utc>,

    /// Collection of challenges
    pub challenges: Vec<RecoveryChallenge>,

    /// Collection of responses
    pub responses: Vec<ChallengeResponse>,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}
impl RecoverySession {

/// New operation.
    /// Creates a new instance
    pub fn new(&str, user_id: &str, recovery_type: RecoveryType) -> Self {
        let now = Utc::now();
        Self {
            id: id.to_string(),
            expires_at: now + chrono::Duration::hours(24), // 24 hour expiry
            challenges: Vec::new(),
            responses: Vec::new(),
            metadata: HashMap::with_capacity(16),
        }
    }

/// Is Expired operation.
    /// Checks if expired
    /// Checks if expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at

/// Add Challenge operation.
    pub fn add_challenge(&mut self, challenge: RecoveryChallenge) {
        self.challenges.push(challenge);

/// Add Response operation.
    pub fn add_response(&mut self, response: ChallengeResponse) {
        self.responses.push(response);

/// Is Complete operation.
    /// Checks if complete
    /// Checks if complete
    pub fn is_complete(&self) -> bool {
        self.challenges.len() == self.responses.len()

/// Completion Percentage operation.
    pub fn completion_percentage(&self) -> f64 {
        if self.challenges.is_empty() {
            0.0
        } else {
            (self.responses.len() as f64 / self.challenges.len() as f64) * 100.0

/// Update Status operation.
    /// Updates status
    /// Updates status
    pub fn update_status(&mut self, status: RecoveryStatus) {
        self.status = status;

/// Add Metadata operation.
    pub fn add_metadata(&str, value: &str) {
        self.metadata.insert(key.to_string(), value.into());

/// Get Metadata operation.
    /// Gets metadata
    /// Gets metadata
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)

/// Extend Expiry operation.
    pub fn extend_expiry(&mut self, hours: i64) {
        self.expires_at += chrono::Duration::hours(hours);

/// Remaining Time operation.
    pub fn remaining_time(&self) -> chrono::Duration {
        self.expires_at - Utc::now(MixedRecoveryStatus,

    /// Collection of active methods
    pub active_methods: Vec<ActiveRecoveryMethod>,

    /// Collection of collected shards
    pub collected_shards: Vec<CollectedShard>,

    /// Number of min_shards_required
    pub min_shards_required: u32,

    /// The progress value
    pub progress: RecoveryProgress,

impl MixedRecoverySession {

/// New operation.
    /// Creates a new instance
    pub fn new(&str, user_id: &str, min_shards_required: u32) -> Self {
            status: MixedRecoveryStatus::Initializing,
            expires_at: now + chrono::Duration::hours(48), // 48 hour expiry for mixed recovery
            active_methods: Vec::new(),
            collected_shards: Vec::new(),
            min_shards_required,
            progress: RecoveryProgress::new(),

/// Add Recovery Method operation.
    pub fn add_recovery_method(&mut self, method: ActiveRecoveryMethod) {
        self.active_methods.push(method);

/// Collect Shard operation.
    pub fn collect_shard(&mut self, shard: CollectedShard) {
        self.collected_shards.push(shard);
        self.progress
            .update_shard_count(self.collected_shards.len() as u32);

/// Has Enough Shards operation.
    /// Checks if enough shards
    /// Checks if enough shards
    pub fn has_enough_shards(&self) -> bool {
        self.collected_shards.len() as u32 >= self.min_shards_required
        if self.min_shards_required == 0 {
            (self.collected_shards.len() as f64 / self.min_shards_required as f64) * 100.0
/// Update Status operation.
    /// Updates status
    /// Updates status
    pub fn update_status(&mut self, status: MixedRecoveryStatus) {

pub struct ActiveRecoveryMethod {

    /// The method type value
    pub method_type: RecoveryType,

    /// Current status of the component
    pub status: MethodStatus,

    /// The started at value
    pub started_at: DateTime<Utc>,

    /// The expected completion value
    pub expected_completion: DateTime<Utc>,

    /// The progress value
    pub progress: f64,

impl ActiveRecoveryMethod {

/// New operation.
    /// Creates a new instance
    pub fn new(&str, method_type: RecoveryType) -> Self {
            method_type,
            status: MethodStatus::Active,
            started_at: now,
            expected_completion: now + chrono::Duration::hours(0.0,

/// Update Progress operation.
    /// Updates progress
    /// Updates progress
    pub fn update_progress(&mut self, progress: f64) {
        self.progress = progress.clamp(0.0, 100.0);

/// Update Status operation.
    /// Updates status
    /// Updates status
    pub fn update_status(&mut self, status: MethodStatus) {

        matches!(self.status, MethodStatus::Completed)

/// Is Failed operation.
    /// Checks if failed
    /// Checks if failed
    pub fn is_failed(&self) -> bool {
        matches!(self.status, MethodStatus::Failed)

pub struct CollectedShard {

    /// The source method value
    pub source_method: String,

    /// The data value
    pub data: String,

    /// Current status of the verification
    pub verification_status: ShardVerificationStatus,

    /// The collected at value
    pub collected_at: DateTime<Utc>,

impl CollectedShard {

/// New operation.
    /// Creates a new instance
    pub fn new(&str, source_method: &str, data: &str) -> Self {
            source_method,
            data,
            verification_status: ShardVerificationStatus::Pending,
            collected_at: Utc::now(),

/// Update Verification Status operation.
    /// Updates verification_status
    /// Updates verification_status
    pub fn update_verification_status(&mut self, status: ShardVerificationStatus) {
        self.verification_status = status;

/// Is Valid operation.
    /// Checks if valid
    /// Checks if valid
    pub fn is_valid(&self) -> bool {
        matches!(self.verification_status, ShardVerificationStatus::Valid)

pub struct RecoveryProgress {

    /// Number of current_step
    pub current_step: u32,

    /// Number of total_steps
    pub total_steps: u32,

    /// Number of shards_collected
    pub shards_collected: u32,

    /// Number of shards_required
    pub shards_required: u32,

    /// The completion percentage value
    pub completion_percentage: f64,

    /// Current status of the component_message
    pub status_message: String,

    /// The last updated value
    pub last_updated: DateTime<Utc>,}

impl RecoveryProgress {

/// New operation.
    /// Creates a new instance
    pub fn new(0,
            total_steps: 0,
            shards_collected: 0,
            shards_required: 0,
            completion_percentage: 0.0,
            status_message: "Initializing recovery process".to_string(),
            last_updated: Utc::now(),

/// Update Step operation.
    /// Updates step
    /// Updates step
    pub fn update_step(&mut self, step: u32) {
        self.current_step = step;
        self.update_completion_percentage();

/// Update Shard Count operation.
    /// Updates shard_count
    /// Updates shard_count
    pub fn update_shard_count(&mut self, collected: u32) {
        self.shards_collected = collected;

/// Update Status Message operation.
    /// Updates status_message
    /// Updates status_message
    pub fn update_status_message(&mut self, message: &str) {
        self.status_message = message;
        self.last_updated = Utc::now();

    /// Updates completion_percentage
    fn update_completion_percentage(&mut self) {
        if self.total_steps > 0 {
            self.completion_percentage =
                (self.current_step as f64 / self.total_steps as f64) * 100.0;
        } else if self.shards_required > 0 {
                (self.shards_collected as f64 / self.shards_required as f64) * 100.0;
impl Default for RecoveryProgress {}

    fn default() -> Self {
        Self::new()
