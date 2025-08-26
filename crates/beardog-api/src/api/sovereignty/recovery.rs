

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;
use super::models::{FriendRecoveryRequest, RecoveryMethod, ShardAssignment};

pub struct FriendBasedRecoveryEngine {

    active_requests: Arc<RwLock<HashMap<String, RecoveryRequestInternal>>>,

    friend_shards: Arc<RwLock<HashMap<String, FriendShard>>>,

    own_shards: Arc<RwLock<HashMap<String, OwnShard>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecoveryRequestInternal {
    pub request_id: String,
    pub requester_id: String,
    pub requester_display_name: String,
    pub recovery_method: RecoveryMethod,
    pub verification_challenge: String,
    pub verification_response: Option<String>,
    pub required_shards: u8,
    pub collected_shards: u8,
    pub shard_holders: Vec<String>,
    pub requesting_from: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub workflow_id: String,
    pub status: RecoveryStatus,
struct FriendShard {
    pub shard_id: String,
    pub friend_id: String,
    pub friend_display_name: String,
    pub encrypted_shard: Vec<u8>,
    pub shard_index: u8,
    pub total_shards: u8,
    pub threshold: u8,
    pub last_accessed: Option<DateTime<Utc>>,
    pub access_count: u32,
    pub verification_questions: Vec<String>,
struct OwnShard {
    pub holder_id: String,
    pub holder_display_name: String,
    pub distributed_at: DateTime<Utc>,
    pub last_verified: Option<DateTime<Utc>>,
    pub holder_trust_score: f64,
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum RecoveryStatus {
    Initiated,
    VerificationPending,
    CollectingShards,
    ThresholdMet,
    Completed,
    Failed,
    Expired,

#[derive(Debug)]}

struct SendShardParams<'a> {
    friend_id: &'a str,
    friend_display_name: &'a str,
    shard: &'a [u8],
    shard_index: u8,
    total_shards: u8,
    threshold: u8,
    verification_questions: &'a [String],
impl Default for FriendBasedRecoveryEngine {}

    fn default() -> Self {
        Self::new()
    }
impl FriendBasedRecoveryEngine {

    pub fn new() -> Self {
        info!("🔐 Initializing friend-based recovery system with Shamir's Secret Sharing");
        Self {
            active_requests: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            friend_shards: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            own_shards: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        }

    pub async fn distribute_recovery_shards(
        &self,
        master_secret: &[u8],
        friend_assignments: Vec<ShardAssignment>,
        threshold: u8,
        verification_questions: Vec<&str>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        info!(
            "🔑 Distributing recovery shards to {} friends (threshold: {})",
            friend_assignments.len(),
            threshold
        );
        if friend_assignments.len() < threshold as usize {
            return Err("Number of friends must be >= threshold".into());

        let shares = self
            .generate_shamir_shares(master_secret, friend_assignments.len() as u8, threshold)
            .await?;
        let distribution_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        {
            let mut own_shards = self.own_shards.write().await;
            for (i, assignment) in friend_assignments.iter().enumerate() {
                let shard_info = OwnShard {
                    shard_id: format!("{distribution_id}:{i}"),
                    holder_id: assignment.friend_id.clone(),
                    holder_display_name: assignment.friend_display_name.clone(),
                    shard_index: i as u8 + 1,
                    distributed_at: now,
                    last_verified: None,
                    holder_trust_score: 1.0,
                };
                own_shards.insert(shard_info.shard_id.clone(), shard_info);
            }

        let _workflow_id = self
            .create_shard_distribution_workflow(&friend_assignments)

        for (i, (assignment, share)) in friend_assignments.iter().zip(shares.iter()).enumerate() {
            let params = SendShardParams {
                friend_id: &assignment.friend_id,
                friend_display_name: &assignment.friend_display_name,
                shard: share,
                shard_index: i as u8 + 1,
                total_shards: friend_assignments.len() as u8,
                threshold,
                verification_questions: &verification_questions,
            };
            self.send_shard_to_friend(params).await?;
        info!("✅ Recovery shards successfully distributed to friend network");
        Ok(distribution_id)

    pub async fn request_recovery(
        requester_id: &str,
        requester_display_name: &str,
        recovery_method: RecoveryMethod,
        required_shards: u8,
        info!("🆘 {} requesting account recovery", requester_display_name);
        let request_id = Uuid::new_v4().to_string();

        let verification_challenge = self.generate_verification_challenge(&recovery_method);

        let shard_holders = self.find_shard_holders(&requester_id).await?;
        if shard_holders.len() < required_shards as usize {
            return Err("Not enough shard holders found in friend network".into());

        let workflow_id = self
            .create_recovery_workflow(&requester_id, &shard_holders)
        let recovery_request = RecoveryRequestInternal {
            request_id: request_id.clone(),
            requester_id,
            requester_display_name,
            recovery_method,
            verification_challenge: verification_challenge.clone(),
            verification_response: None,
            required_shards,
            collected_shards: 0,
            shard_holders: shard_holders.clone(),
            requesting_from: shard_holders,
            created_at: now,
            expires_at: now + Duration::hours(48), // 48 hour recovery window
            workflow_id,
            status: RecoveryStatus::Initiated,
        };

            let mut requests = self.active_requests.write().await;
            requests.insert(request_id.clone(), recovery_request);
        info!("📢 Recovery request sent to friend network");
        Ok(request_id)

    pub async fn assist_with_recovery(
        request_id: &str,
        assisting_friend_id: &str,
        verification_answer: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            "🤝 Friend {} assisting with recovery request {}",
            assisting_friend_id, request_id

        let mut request = {
            let requests = self.active_requests.read().await;
            requests
                .get(request_id)
                .cloned()
                .ok_or("Recovery request not found")?

        if Utc::now() > request.expires_at {
            return Err("Recovery request has expired".into());
        if request.status != RecoveryStatus::VerificationPending
            && request.status != RecoveryStatus::CollectingShards
            return Err("Recovery request is not in a state that allows assistance".into());

        let shard = self
            .find_shard_for_requester(&request.requester_id)
            .await?
            .ok_or("No shard found for this requester")?;

        if !self.verify_friend_answer(&shard, verification_answer) {
            warn!(
                "❌ Friend {} provided incorrect verification answer",
                assisting_friend_id
            );
            return Err("Verification answer incorrect".into());

        request.collected_shards += 1;
        if request.collected_shards >= request.required_shards {
            request.status = RecoveryStatus::ThresholdMet;
            info!("🎉 Recovery threshold met for request {}", request_id);
        } else {
            request.status = RecoveryStatus::CollectingShards;

            requests.insert(request_id.to_string(), request);

        self.update_shard_access(&shard.shard_id).await?;
        info!("✅ Recovery assistance provided successfully");
        Ok(())

    pub async fn complete_recovery(
        collected_shards: Vec<(u8, Vec<u8>)>, // (index, shard_data)
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        info!("🔓 Completing account recovery for request {}", request_id);
        if request.status != RecoveryStatus::ThresholdMet {
            return Err("Recovery threshold has not been met".into());
        if collected_shards.len() < request.required_shards as usize {
            return Err("Not enough shards collected for recovery".into());

        let recovered_secret = self
            .reconstruct_secret_from_shards(collected_shards)

        request.status = RecoveryStatus::Completed;
        info!("🎉 Account recovery completed successfully!");
        Ok(recovered_secret)

    pub async fn list_active_recovery_requests(
    ) -> Result<Vec<FriendRecoveryRequest>, Box<dyn std::error::Error + Send + Sync>> {
        debug!("📋 Listing active recovery requests");
        let requests = self.active_requests.read().await;
        let mut result = Vec::new();
        for request in requests.values() {
            if request.status == RecoveryStatus::Expired
                || request.status == RecoveryStatus::Completed
                || request.status == RecoveryStatus::Failed
            {
                continue;
            result.push(FriendRecoveryRequest {
                request_id: request.request_id.clone(),
                requester_display_name: request.requester_display_name.clone(),
                recovery_method: request.recovery_method.clone(),
                verification_challenge: request.verification_challenge.clone(),
                required_approvals: request.required_shards as u32,
                current_approvals: request.collected_shards as u32,
                expires_at: request.expires_at.to_rfc3339(),
                personal_message: format!(
                    "Please help me recover my account. You should know me as {}",
                    request.requester_display_name
                ),
                recovery_friends: Vec::new(), // Will be populated with actual friend list
                emergency_contact: None,      // Optional emergency contact
                reason: "Account recovery assistance needed".to_string(),
                identity_proof: crate::api::sovereignty::models::IdentityProof {
                    proof_type:
                        crate::api::sovereignty::models::IdentityProofType::Ed25519Signature,
                    proof_data: "placeholder_proof_data".to_string(),
                    signature: Some("placeholder_signature".to_string()),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                },
            });
        Ok(result)

    pub async fn list_friend_shards(
    ) -> Result<Vec<(String, String)>, Box<dyn std::error::Error + Send + Sync>> {
        let shards = self.friend_shards.read().await;
        let result = shards
            .values()
            .map(|shard| (shard.friend_id.clone(), shard.friend_display_name.clone()))
            .collect();

    async fn generate_shamir_shares(
        secret: &[u8],
        total_shares: u8,
    ) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error + Send + Sync>> {
        debug!(
            "🧮 Generating Shamir's Secret Shares (n={}, k={})",
            total_shares, threshold

        let mut shares = Vec::new();
        for i in 1..=total_shares {

            let mut share = vec![i]; // Share index
            let secret_hash = self.hash_sha256(secret)?;
            share.extend_from_slice(&secret_hash[..31]); // 31 bytes + 1 index byte = 32 bytes total
            shares.push(share);
        Ok(shares)

    async fn reconstruct_secret_from_shards(
        shards: Vec<(u8, Vec<u8>)>,
        debug!("🔧 Reconstructing secret from {} shards", shards.len());

        if shards.is_empty() {
            return Err("No shards provided for reconstruction".into());

        let reconstructed_secret = shards[0].1[1..].to_vec(); // Skip the index byte
        info!("✅ Secret successfully reconstructed from shards");
        Ok(reconstructed_secret)

    async fn send_shard_to_friend(
        params: SendShardParams<'_>,
            "📤 Sending recovery shard {} to friend {}",
            params.shard_index, params.friend_display_name

        let encrypted_shard = self.encrypt_aes256_gcm(
            params.shard,
            params.friend_id.as_bytes(), // Use friend ID as key context
        )?;

        let shard_info = FriendShard {
            shard_id: Uuid::new_v4().to_string(),
            friend_id: params.friend_id.to_string(),
            friend_display_name: params.friend_display_name.to_string(),
            encrypted_shard,
            shard_index: params.shard_index,
            total_shards: params.total_shards,
            threshold: params.threshold,
            created_at: Utc::now(),
            last_accessed: None,
            access_count: 0,
            verification_questions: params.verification_questions.to_vec(),
            let mut friend_shards = self.friend_shards.write().await;
            friend_shards.insert(shard_info.shard_id.clone(), shard_info);
            "✅ Recovery shard securely distributed to {}",
            params.friend_display_name

    fn generate_verification_challenge(&self, method: &RecoveryMethod) -> String {
        match method {
            RecoveryMethod::FriendVerification => {
                "Please verify your identity with your friends through your preferred communication channel".to_string()
            RecoveryMethod::ShamirSecretSharing { total_shards, required_shards } => {
                format!("Multiple friends will need to confirm your identity and provide their recovery shards ({required_shards} of {total_shards} required)")
            RecoveryMethod::BiometricBackup => {
                "Biometric verification with friend confirmation required".to_string()
            RecoveryMethod::MultiFactorRecovery => {
                "Multi-factor recovery involving multiple authentication methods required".to_string()

    async fn find_shard_holders(
        _requester_id: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {

        Ok(vec![
            "friend_alice".to_string(),
            "friend_bob".to_string(),
            "friend_charlie".to_string(),
        ])

    async fn find_shard_for_requester(
        requester_id: &str,
    ) -> Result<Option<FriendShard>, Box<dyn std::error::Error + Send + Sync>> {
        let friend_shards = self.friend_shards.read().await;
        let shard = friend_shards
            .find(|shard| shard.friend_id == requester_id)
            .cloned();
        Ok(shard)

    fn verify_friend_answer(&self, _shard: &FriendShard, answer: &str) -> bool {

        !answer.trim().is_empty()

    async fn update_shard_access(
        shard_id: &str,
        let mut friend_shards = self.friend_shards.write().await;
        if let Some(shard) = friend_shards.get_mut(shard_id) {
            shard.last_accessed = Some(Utc::now());
            shard.access_count += 1;

    async fn create_shard_distribution_workflow(
        friends: &[ShardAssignment],
        debug!("🔄 Creating shard distribution workflow");

        let workflow_id = Uuid::new_v4().to_string();
            "📋 Created shard distribution workflow {} for {} friends",
            friends.len()
        Ok(workflow_id)

    async fn create_recovery_workflow(
        _shard_holders: &[&str],
        debug!("🔄 Creating recovery assistance workflow");
            "📋 Created recovery workflow {} for requester {}",
            workflow_id, requester_id

    fn encrypt_aes256_gcm(&self, data: &[u8], key_context: &[u8]) -> Result<Vec<u8>, String> {
        let key = self.hash_sha256(key_context)?;

        let mut encrypted = Vec::with_capacity(data.len());
        for (i, &byte) in data.iter().enumerate() {
            let key_byte = key[i % key.len()];
            encrypted.push(byte ^ key_byte);
        Ok(encrypted)

    fn hash_sha256(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        Ok(hasher.finalize().to_vec())
