

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::challenges::*;
use super::types::*;
use beardog_errors::{BearDogError, BearDogResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverySession {

    pub id: String,

    pub user_id: String,

    pub recovery_type: RecoveryType,

    pub status: RecoveryStatus,

    pub created_at: DateTime<Utc>,

    pub expires_at: DateTime<Utc>,

    pub challenges: Vec<RecoveryChallenge>,

    pub responses: Vec<ChallengeResponse>,

    pub metadata: HashMap<String, String>,
}
impl RecoverySession {

    pub fn new(id: &str, user_id: &str, recovery_type: RecoveryType) -> Self {
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
            metadata: HashMap::with_capacity(16),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at

    pub fn add_challenge(&mut self, challenge: RecoveryChallenge) {
        self.challenges.push(challenge);

    pub fn add_response(&mut self, response: ChallengeResponse) {
        self.responses.push(response);

    pub fn is_complete(&self) -> bool {
        self.challenges.len() == self.responses.len()

    pub fn completion_percentage(&self) -> f64 {
        if self.challenges.is_empty() {
            0.0
        } else {
            (self.responses.len() as f64 / self.challenges.len() as f64) * 100.0

    pub fn update_status(&mut self, status: RecoveryStatus) {
        self.status = status;

    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key, value);

    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)

    pub fn extend_expiry(&mut self, hours: i64) {
        self.expires_at += chrono::Duration::hours(hours);

    pub fn remaining_time(&self) -> chrono::Duration {
        self.expires_at - Utc::now()

pub struct MixedRecoverySession {

    pub status: MixedRecoveryStatus,

    pub active_methods: Vec<ActiveRecoveryMethod>,

    pub collected_shards: Vec<CollectedShard>,

    pub min_shards_required: u32,

    pub progress: RecoveryProgress,

impl MixedRecoverySession {

    pub fn new(id: &str, user_id: &str, min_shards_required: u32) -> Self {
            status: MixedRecoveryStatus::Initializing,
            expires_at: now + chrono::Duration::hours(48), // 48 hour expiry for mixed recovery
            active_methods: Vec::new(),
            collected_shards: Vec::new(),
            min_shards_required,
            progress: RecoveryProgress::new(),

    pub fn add_recovery_method(&mut self, method: ActiveRecoveryMethod) {
        self.active_methods.push(method);

    pub fn collect_shard(&mut self, shard: CollectedShard) {
        self.collected_shards.push(shard);
        self.progress
            .update_shard_count(self.collected_shards.len() as u32);

    pub fn has_enough_shards(&self) -> bool {
        self.collected_shards.len() as u32 >= self.min_shards_required
        if self.min_shards_required == 0 {
            (self.collected_shards.len() as f64 / self.min_shards_required as f64) * 100.0
    pub fn update_status(&mut self, status: MixedRecoveryStatus) {

pub struct ActiveRecoveryMethod {

    pub method_type: RecoveryType,

    pub status: MethodStatus,

    pub started_at: DateTime<Utc>,

    pub expected_completion: DateTime<Utc>,

    pub progress: f64,

impl ActiveRecoveryMethod {

    pub fn new(id: &str, method_type: RecoveryType) -> Self {
            method_type,
            status: MethodStatus::Active,
            started_at: now,
            expected_completion: now + chrono::Duration::hours(24),
            progress: 0.0,

    pub fn update_progress(&mut self, progress: f64) {
        self.progress = progress.clamp(0.0, 100.0);

    pub fn update_status(&mut self, status: MethodStatus) {

        matches!(self.status, MethodStatus::Completed)

    pub fn is_failed(&self) -> bool {
        matches!(self.status, MethodStatus::Failed)

pub struct CollectedShard {

    pub source_method: String,

    pub data: String,

    pub verification_status: ShardVerificationStatus,

    pub collected_at: DateTime<Utc>,

impl CollectedShard {

    pub fn new(id: &str, source_method: &str, data: &str) -> Self {
            source_method,
            data,
            verification_status: ShardVerificationStatus::Pending,
            collected_at: Utc::now(),

    pub fn update_verification_status(&mut self, status: ShardVerificationStatus) {
        self.verification_status = status;

    pub fn is_valid(&self) -> bool {
        matches!(self.verification_status, ShardVerificationStatus::Valid)

pub struct RecoveryProgress {

    pub current_step: u32,

    pub total_steps: u32,

    pub shards_collected: u32,

    pub shards_required: u32,

    pub completion_percentage: f64,

    pub status_message: String,

    pub last_updated: DateTime<Utc>,}

impl RecoveryProgress {

    pub fn new() -> Self {
            current_step: 0,
            total_steps: 0,
            shards_collected: 0,
            shards_required: 0,
            completion_percentage: 0.0,
            status_message: "Initializing recovery process".to_string(),
            last_updated: Utc::now(),

    pub fn update_step(&mut self, step: u32) {
        self.current_step = step;
        self.update_completion_percentage();

    pub fn update_shard_count(&mut self, collected: u32) {
        self.shards_collected = collected;

    pub fn update_status_message(&mut self, message: &str) {
        self.status_message = message;
        self.last_updated = Utc::now();

    fn update_completion_percentage(&mut self) {
        if self.total_steps > 0 {
            self.completion_percentage =
                (self.current_step as f64 / self.total_steps as f64) * 100.0;
        } else if self.shards_required > 0 {
                (self.shards_collected as f64 / self.shards_required as f64) * 100.0;
impl Default for RecoveryProgress {}

    fn default() -> Self {
        Self::new()
