// SPDX-License-Identifier: AGPL-3.0-or-later

//! Recovery Test Helper Types

use beardog_errors::BearDogError;
use std::collections::{HashMap, HashSet};

// ============================================================================
// Helper Functions
// ============================================================================

pub fn compute_response(challenge: &[u8], secret: &[u8]) -> Vec<u8> {
    // Simple hash-based response for testing
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    challenge.hash(&mut hasher);
    secret.hash(&mut hasher);
    let hash = hasher.finish();

    hash.to_be_bytes().to_vec()
}

use std::time::{Duration, Instant};

/// Configuration for secret sharing
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ShardConfig {
    total_shards: usize,
    threshold: usize,
}

impl ShardConfig {
    pub fn new(total: usize, threshold: usize) -> Result<Self, BearDogError> {
        if total == 0 || threshold == 0 {
            return Err(BearDogError::security(
                "Shard configuration must have non-zero values".to_string(),
            ));
        }
        if threshold > total {
            return Err(BearDogError::security(
                "Threshold cannot exceed total shards".to_string(),
            ));
        }
        Ok(Self {
            total_shards: total,
            threshold,
        })
    }

    #[allow(dead_code)]
    pub fn threshold(&self) -> usize {
        self.threshold
    }
}

/// A single shard of a secret
#[derive(Debug, Clone)]
pub struct Shard {
    pub id: usize,
    pub data: Vec<u8>,
}

pub fn create_shards(secret: &[u8], config: &ShardConfig) -> Result<Vec<Shard>, BearDogError> {
    // Simple XOR-based secret sharing for testing
    let mut shards = Vec::new();

    for i in 0..config.total_shards {
        let shard_data = secret.iter().map(|b| b ^ (i as u8)).collect();
        shards.push(Shard {
            id: i,
            data: shard_data,
        });
    }

    Ok(shards)
}

pub fn reconstruct_from_shards(shards: &[Shard]) -> Result<Vec<u8>, BearDogError> {
    if shards.is_empty() {
        return Err(BearDogError::security(
            "Insufficient shards for reconstruction".to_string(),
        ));
    }

    // Check if we have enough shards (threshold is 2 minimum in our test setup)
    if shards.len() < 2 {
        return Err(BearDogError::security(
            "Insufficient shards for reconstruction (need at least 2)".to_string(),
        ));
    }

    // Simple reconstruction (XOR back)
    let result = shards[0]
        .data
        .iter()
        .map(|b| b ^ (shards[0].id as u8))
        .collect();

    Ok(result)
}

/// Guardian in social recovery
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Guardian {
    id: String,
    contact: String,
}

impl Guardian {
    pub fn new(id: &str, contact: &str) -> Self {
        Self {
            id: id.to_string(),
            contact: contact.to_string(),
        }
    }
}

/// Social recovery configuration
#[derive(Debug, Clone)]
pub struct SocialRecoveryConfig {
    guardians: Vec<Guardian>,
    threshold: usize,
}

impl SocialRecoveryConfig {
    pub fn new(guardians: Vec<Guardian>, threshold: usize) -> Self {
        Self {
            guardians,
            threshold,
        }
    }

    pub fn guardian_count(&self) -> usize {
        self.guardians.len()
    }

    pub fn threshold(&self) -> usize {
        self.threshold
    }
}

/// Recovery request with guardian approvals
#[derive(Debug, Clone)]
pub struct RecoveryRequest {
    user_id: String,
    approved_by: HashSet<String>,
}

impl RecoveryRequest {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            approved_by: HashSet::new(),
        }
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn approvals(&self) -> usize {
        self.approved_by.len()
    }

    pub fn add_approval(&mut self, guardian: &Guardian) -> Result<(), BearDogError> {
        if self.approved_by.contains(&guardian.id) {
            return Err(BearDogError::security(
                "Guardian has already approved this request".to_string(),
            ));
        }
        self.approved_by.insert(guardian.id.clone());
        Ok(())
    }

    pub fn is_complete(&self, config: &SocialRecoveryConfig) -> bool {
        self.approved_by.len() >= config.threshold
    }
}

/// Ephemeral key for temporary recovery
#[derive(Debug, Clone)]
pub struct EphemeralKey {
    key: Vec<u8>,
    created_at: Instant,
    expires_after: Duration,
}

impl EphemeralKey {
    pub fn generate(expires_after: Duration) -> Self {
        // Generate 32-byte random key (simplified for testing)
        // Use a combination of timing and current instant to ensure uniqueness
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
        let now = Instant::now();

        // Use SystemTime to get actual elapsed time since UNIX_EPOCH for better entropy
        use std::time::SystemTime;
        let system_nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        // Combine counter and system time for uniqueness
        let key: Vec<u8> = (0..32)
            .map(|i| {
                let byte_offset = (counter.wrapping_mul(31).wrapping_add(i as u64)) as u8;
                let time_byte = ((system_nanos >> (i * 4)) & 0xFF) as u8;
                (i as u8 ^ 0xAA) ^ time_byte ^ byte_offset
            })
            .collect();

        Self {
            key,
            created_at: now,
            expires_after,
        }
    }

    pub fn key_data(&self) -> &[u8] {
        &self.key
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.expires_after
    }

    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }

    pub fn rotate(&self) -> Self {
        // Create a new key with different seed
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1000); // Different counter for rotation

        let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
        let now = Instant::now();

        // Use SystemTime to get actual elapsed time since UNIX_EPOCH for better entropy
        use std::time::SystemTime;
        let system_nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        // Use a different base XOR pattern to ensure difference from generate()
        let key: Vec<u8> = (0..32)
            .map(|i| {
                let byte_offset = (counter.wrapping_mul(37).wrapping_add(i as u64)) as u8;
                let time_byte = ((system_nanos >> (i * 4)) & 0xFF) as u8;
                (i as u8 ^ 0x55) ^ time_byte ^ byte_offset // 0x55 instead of 0xAA
            })
            .collect();

        Self {
            key,
            created_at: now,
            expires_after: self.expires_after,
        }
    }

    pub fn derive_key(&self, context: &[u8]) -> Vec<u8> {
        self.key
            .iter()
            .zip(context.iter().cycle())
            .map(|(a, b)| a ^ b)
            .collect()
    }
}

/// Recovery policy
#[derive(Debug, Clone)]
pub struct RecoveryPolicy {
    cooldown: Duration,
    max_attempts: usize,
    guardian_threshold: usize,
}

impl RecoveryPolicy {
    pub fn new() -> Self {
        Self {
            cooldown: Duration::from_secs(0),
            max_attempts: 10,
            guardian_threshold: 1,
        }
    }

    pub fn with_cooldown(mut self, duration: Duration) -> Self {
        self.cooldown = duration;
        self
    }

    pub fn with_max_attempts(mut self, max: usize) -> Self {
        self.max_attempts = max;
        self
    }

    pub fn with_guardian_threshold(mut self, threshold: usize, _total: usize) -> Self {
        self.guardian_threshold = threshold;
        self
    }

    pub fn cooldown_duration(&self) -> Duration {
        self.cooldown
    }

    pub fn max_attempts(&self) -> usize {
        self.max_attempts
    }

    pub fn guardian_threshold(&self) -> usize {
        self.guardian_threshold
    }

    pub fn validate_request(&self, _request: &RecoveryRequest) -> Result<(), BearDogError> {
        Ok(())
    }
}

/// Tracks recovery attempts
#[derive(Debug)]
pub struct RecoveryAttemptTracker {
    attempts: HashMap<String, Vec<Instant>>,
}

impl RecoveryAttemptTracker {
    pub fn new() -> Self {
        Self {
            attempts: HashMap::new(),
        }
    }

    pub fn record_attempt(&mut self, user_id: &str) {
        self.attempts
            .entry(user_id.to_string())
            .or_default()
            .push(Instant::now());
    }

    pub fn attempt_count(&self, user_id: &str) -> usize {
        self.attempts.get(user_id).map_or(0, std::vec::Vec::len)
    }

    pub fn can_attempt(&self, user_id: &str, policy: &RecoveryPolicy) -> bool {
        if let Some(attempts) = self.attempts.get(user_id)
            && let Some(last_attempt) = attempts.last()
            && last_attempt.elapsed() < policy.cooldown
        {
            return false;
        }
        self.under_attempt_limit(user_id, policy)
    }

    pub fn under_attempt_limit(&self, user_id: &str, policy: &RecoveryPolicy) -> bool {
        self.attempt_count(user_id) < policy.max_attempts
    }
}

/// Federation member
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FederationMember {
    id: String,
    endpoint: String,
}

impl FederationMember {
    pub fn new(id: &str, endpoint: &str) -> Self {
        Self {
            id: id.to_string(),
            endpoint: endpoint.to_string(),
        }
    }
}

/// Federation for distributed recovery
#[derive(Debug, Clone)]
pub struct Federation {
    members: Vec<FederationMember>,
    consensus_threshold: usize,
}

impl Federation {
    pub fn new(members: Vec<FederationMember>, consensus_threshold: usize) -> Self {
        Self {
            members,
            consensus_threshold,
        }
    }

    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    pub fn consensus_threshold(&self) -> usize {
        self.consensus_threshold
    }
}

/// Federated recovery process
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FederatedRecovery {
    user_id: String,
    confirmed_by: HashSet<String>,
}

impl FederatedRecovery {
    pub fn new(user_id: &str, _federation: &Federation) -> Self {
        Self {
            user_id: user_id.to_string(),
            confirmed_by: HashSet::new(),
        }
    }

    pub fn confirmations(&self) -> usize {
        self.confirmed_by.len()
    }

    pub fn add_confirmation(&mut self, member: &FederationMember) -> Result<(), BearDogError> {
        if self.confirmed_by.contains(&member.id) {
            return Err(BearDogError::security(
                "Member has already confirmed".to_string(),
            ));
        }
        self.confirmed_by.insert(member.id.clone());
        Ok(())
    }

    pub fn has_consensus(&self, federation: &Federation) -> bool {
        self.confirmed_by.len() >= federation.consensus_threshold
    }

    pub fn is_complete(&self, federation: &Federation) -> bool {
        self.has_consensus(federation)
    }
}

/// Recovery challenge
#[derive(Debug, Clone)]
pub struct RecoveryChallenge {
    user_id: String,
    challenge_data: Vec<u8>,
    answered: bool,
    valid: bool,
    created_at: Instant,
    expires_after: Duration,
}

impl RecoveryChallenge {
    pub fn generate(user_id: &str) -> Self {
        let challenge_data: Vec<u8> = (0..32).map(|i| i as u8).collect();
        Self {
            user_id: user_id.to_string(),
            challenge_data,
            answered: false,
            valid: false,
            created_at: Instant::now(),
            expires_after: Duration::from_secs(300), // 5 minutes
        }
    }

    pub fn with_expiration(user_id: &str, expires_after: Duration) -> Self {
        let challenge_data: Vec<u8> = (0..32).map(|i| i as u8).collect();
        Self {
            user_id: user_id.to_string(),
            challenge_data,
            answered: false,
            valid: false,
            created_at: Instant::now(),
            expires_after,
        }
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn challenge_data(&self) -> &[u8] {
        &self.challenge_data
    }

    pub fn is_answered(&self) -> bool {
        self.answered
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.expires_after
    }

    pub fn submit_response(&mut self, response: &[u8]) -> Result<(), BearDogError> {
        // Simplified validation for testing
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        self.challenge_data.hash(&mut hasher);
        b"correct_secret".hash(&mut hasher);
        let expected_hash = hasher.finish();
        let expected = expected_hash.to_be_bytes().to_vec();

        self.answered = true;
        self.valid = response == expected;

        if self.valid {
            Ok(())
        } else {
            Err(BearDogError::security("Invalid response".to_string()))
        }
    }

    pub fn is_valid(&self) -> bool {
        self.valid && !self.is_expired()
    }
}

/// Multi-factor challenge
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum ChallengeFactor {
    Password,
    Biometric,
    Hardware,
    Social,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MultiFactorChallenge {
    user_id: String,
    factors: Vec<ChallengeFactor>,
    completed: HashSet<ChallengeFactor>,
}

impl MultiFactorChallenge {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            factors: Vec::new(),
            completed: HashSet::new(),
        }
    }

    pub fn add_factor(&mut self, factor: ChallengeFactor) {
        self.factors.push(factor);
    }

    pub fn factor_count(&self) -> usize {
        self.factors.len()
    }

    pub fn complete_factor(&mut self, factor: ChallengeFactor) -> Result<(), BearDogError> {
        if !self.factors.contains(&factor) {
            return Err(BearDogError::security(
                "Factor not in challenge".to_string(),
            ));
        }
        self.completed.insert(factor);
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        self.completed.len() == self.factors.len()
    }
}

/// Recovery session
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RecoverySession {
    session_id: String,
    user_id: String,
    created_at: Instant,
    expires_at: Instant,
    active: bool,
}

impl RecoverySession {
    pub fn new(user_id: &str, duration: Duration) -> Self {
        let now = Instant::now();
        // ✅ FIXED: Use thread-safe UUID for truly unique session IDs
        // The previous implementation using now.elapsed().as_nanos() was racy:
        // - elapsed() returns 0 immediately after now(), causing duplicate IDs
        // - This is a concurrency bug that would affect production
        use std::sync::atomic::{AtomicU64, Ordering};
        static SESSION_COUNTER: AtomicU64 = AtomicU64::new(0);
        let session_num = SESSION_COUNTER.fetch_add(1, Ordering::SeqCst);

        Self {
            session_id: format!("session_{}_{}", session_num, now.elapsed().as_nanos()),
            user_id: user_id.to_string(),
            created_at: now,
            expires_at: now + duration,
            active: true,
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn is_expired(&self) -> bool {
        Instant::now() >= self.expires_at
    }

    pub fn is_active(&self) -> bool {
        self.active && !self.is_expired()
    }

    pub fn validate(&self) -> Result<(), BearDogError> {
        if !self.active {
            return Err(BearDogError::security("Session is not active".to_string()));
        }
        if self.is_expired() {
            return Err(BearDogError::security("Session has expired".to_string()));
        }
        Ok(())
    }

    pub fn revoke(&mut self) {
        self.active = false;
    }

    pub fn expires_at(&self) -> Instant {
        self.expires_at
    }

    pub fn refresh(&mut self, duration: Duration) {
        self.expires_at = Instant::now() + duration;
    }
}
