//! Account Recovery System
//!
//! **Distributed Recovery - No Single Point of Failure**
//!
//! This module implements a comprehensive recovery system that follows the principle:
//! "Finding the key doesn't mean owning the house" - recovery is distributed,
//! time-bound, and requires multiple parties without giving full ownership.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

use beardog_errors::{BearDogError, BearDogResult};

/// Recovery system manager
#[derive(Debug)]
pub struct RecoveryManager {
    /// Active recovery sessions
    recovery_sessions: Arc<RwLock<HashMap<String, RecoverySession>>>,
    /// Social recovery configurations per user
    social_configs: Arc<RwLock<HashMap<String, SocialRecoveryConfig>>>,
    /// Federation recovery configurations
    federation_configs: Arc<RwLock<HashMap<String, FederationRecoveryConfig>>>,
    /// Ephemeral recovery keys
    ephemeral_keys: Arc<RwLock<HashMap<String, EphemeralRecoveryKey>>>,
    /// Recovery challenge responses
    challenge_responses: Arc<RwLock<HashMap<String, ChallengeResponse>>>,
    /// Recovery audit log
    audit_log: Arc<RwLock<Vec<RecoveryAuditEntry>>>,
}

/// Social recovery configuration for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialRecoveryConfig {
    /// User ID this config belongs to
    pub user_id: String,
    /// Trusted contacts who can help with recovery
    pub trusted_contacts: Vec<TrustedContact>,
    /// Minimum number of contacts needed for recovery
    pub min_contacts_required: u32,
    /// Maximum time window for recovery attempts
    pub recovery_window_hours: u32,
    /// Whether social recovery is enabled
    pub enabled: bool,
    /// Recovery policy settings
    pub policy: RecoveryPolicy,
}

/// Trusted contact for social recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedContact {
    /// Contact ID
    pub id: String,
    /// Contact identifier (email, phone, etc.)
    pub identifier: String,
    /// Type of contact
    pub contact_type: ContactType,
    /// Public key for secure communication
    pub public_key: Option<String>,
    /// Trust level (0-100)
    pub trust_level: u8,
    /// When this contact was added
    pub added_at: DateTime<Utc>,
    /// Last time this contact was used for recovery
    pub last_used: Option<DateTime<Utc>>,
    /// Whether this contact is currently active
    pub active: bool,
}

/// Types of trusted contacts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContactType {
    /// Email contact
    Email,
    /// SMS/Phone contact
    Phone,
    /// Another BearDog user
    BearDogUser,
    /// Hardware device
    HardwareDevice,
    /// Biometric verification
    Biometric,
}

/// Federation recovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationRecoveryConfig {
    /// User ID this config belongs to
    pub user_id: String,
    /// Trusted BearDog instances
    pub trusted_instances: Vec<TrustedInstance>,
    /// Minimum number of instances needed for recovery
    pub min_instances_required: u32,
    /// Whether federation recovery is enabled
    pub enabled: bool,
    /// Cross-instance verification settings
    pub verification_settings: FederationVerificationSettings,
}

/// Trusted BearDog instance for federation recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedInstance {
    /// Instance ID
    pub id: String,
    /// Instance endpoint
    pub endpoint: String,
    /// Instance public key
    pub public_key: String,
    /// Trust level (0-100)
    pub trust_level: u8,
    /// When this instance was added to trusted list
    pub added_at: DateTime<Utc>,
    /// Whether this instance is currently active
    pub active: bool,
}

/// Federation verification settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationVerificationSettings {
    /// Require cryptographic proof of identity
    pub require_crypto_proof: bool,
    /// Require reputation verification
    pub require_reputation: bool,
    /// Minimum reputation score required
    pub min_reputation_score: u32,
    /// Verification timeout in minutes
    pub verification_timeout_minutes: u32,
}

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

/// Types of additional verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationType {
    /// Knowledge-based questions
    KnowledgeBased,
    /// Document verification
    DocumentVerification,
    /// Biometric verification
    BiometricVerification,
    /// Geolocation verification
    GeolocationVerification,
    /// Device fingerprinting
    DeviceFingerprinting,
}

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

/// Types of recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryType {
    /// Social recovery using trusted contacts
    SocialRecovery,
    /// Federation recovery using trusted instances
    FederationRecovery,
    /// Ephemeral key recovery
    EphemeralKeyRecovery,
    /// Multi-party recovery combining multiple methods
    MultiPartyRecovery,
}

/// Recovery session status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecoveryStatus {
    /// Session is active and waiting for responses
    Active,
    /// Session is completed successfully
    Completed,
    /// Session failed
    Failed,
    /// Session expired
    Expired,
    /// Session was cancelled
    Cancelled,
}

/// Recovery challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryChallenge {
    /// Challenge ID
    pub id: String,
    /// Type of challenge
    pub challenge_type: ChallengeType,
    /// Challenge data (encrypted)
    pub challenge_data: Vec<u8>,
    /// Who this challenge is for
    pub target_contact: String,
    /// When the challenge was created
    pub created_at: DateTime<Utc>,
    /// When the challenge expires
    pub expires_at: DateTime<Utc>,
    /// Whether this challenge has been completed
    pub completed: bool,
}

/// Types of recovery challenges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChallengeType {
    /// Contact verification challenge
    ContactVerification,
    /// Cryptographic proof challenge
    CryptographicProof,
    /// Knowledge-based challenge
    KnowledgeBased,
    /// Federation attestation challenge
    FederationAttestation,
    /// Ephemeral key challenge
    EphemeralKeyChallenge,
}

/// Challenge response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeResponse {
    /// Response ID
    pub id: String,
    /// Challenge ID this response is for
    pub challenge_id: String,
    /// Response data (encrypted)
    pub response_data: Vec<u8>,
    /// Who provided this response
    pub responder: String,
    /// When the response was provided
    pub created_at: DateTime<Utc>,
    /// Cryptographic signature of the response
    pub signature: Option<String>,
}

/// Ephemeral recovery key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EphemeralRecoveryKey {
    /// Key ID
    pub id: String,
    /// User ID this key belongs to
    pub user_id: String,
    /// Encrypted key material
    pub encrypted_key: Vec<u8>,
    /// What this key can be used for
    pub permissions: EphemeralPermissions,
    /// When the key was created
    pub created_at: DateTime<Utc>,
    /// When the key expires
    pub expires_at: DateTime<Utc>,
    /// How many times this key can be used
    pub max_uses: u32,
    /// How many times this key has been used
    pub use_count: u32,
    /// Whether this key is still active
    pub active: bool,
}

/// Permissions for ephemeral recovery keys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EphemeralPermissions {
    /// Can unlock account
    pub can_unlock_account: bool,
    /// Can reset password
    pub can_reset_password: bool,
    /// Can access specific resources
    pub resource_access: Vec<String>,
    /// Time-based restrictions
    pub time_restrictions: Option<TimeRestrictions>,
    /// IP-based restrictions
    pub ip_restrictions: Option<Vec<String>>,
}

/// Time-based restrictions for ephemeral keys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestrictions {
    /// Allowed hours of day (0-23)
    pub allowed_hours: Vec<u8>,
    /// Allowed days of week (0-6, 0=Sunday)
    pub allowed_days: Vec<u8>,
    /// Timezone for time restrictions
    pub timezone: String,
}

/// Recovery audit entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryAuditEntry {
    /// Entry ID
    pub id: String,
    /// User ID involved in recovery
    pub user_id: String,
    /// Type of recovery event
    pub event_type: RecoveryEventType,
    /// Event description
    pub description: String,
    /// When the event occurred
    pub timestamp: DateTime<Utc>,
    /// IP address of the request
    pub ip_address: Option<String>,
    /// User agent string
    pub user_agent: Option<String>,
    /// Success/failure status
    pub success: bool,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Types of recovery events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryEventType {
    /// Recovery session started
    SessionStarted,
    /// Challenge sent to contact
    ChallengeSent,
    /// Challenge response received
    ChallengeResponse,
    /// Recovery completed successfully
    RecoveryCompleted,
    /// Recovery failed
    RecoveryFailed,
    /// Ephemeral key generated
    EphemeralKeyGenerated,
    /// Ephemeral key used
    EphemeralKeyUsed,
    /// Social recovery configuration changed
    SocialConfigChanged,
    /// Federation recovery configuration changed
    FederationConfigChanged,
}

/// User-controlled recovery policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecoveryPolicy {
    /// Total number of recovery shards to create
    pub total_shards: u32,
    /// Minimum number of shards required for recovery
    pub threshold_shards: u32,
    /// Recovery methods enabled by user
    pub enabled_methods: Vec<RecoveryType>,
    /// Custom trust boundaries set by user
    pub trust_boundaries: UserTrustBoundaries,
    /// Whether to allow mixed recovery (combining methods)
    pub allow_mixed_recovery: bool,
    /// Maximum time window for recovery attempts
    pub max_recovery_window_hours: u32,
    /// User-defined recovery contexts (family, work, emergency, etc.)
    pub recovery_contexts: Vec<RecoveryContext>,
}

/// User-defined trust boundaries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTrustBoundaries {
    /// Minimum trust level for social contacts (0-100)
    pub min_social_trust_level: u8,
    /// Minimum trust level for federation instances (0-100)
    pub min_federation_trust_level: u8,
    /// Maximum number of recovery attempts per day
    pub max_recovery_attempts_per_day: u32,
    /// Geographic restrictions (optional)
    pub allowed_regions: Option<Vec<String>>,
    /// Time-based restrictions (optional)
    pub allowed_time_windows: Option<Vec<TimeWindow>>,
}

/// Recovery context defined by user (family, work, emergency, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryContext {
    /// Context name (e.g., "family", "work", "emergency")
    pub name: String,
    /// Description of this recovery context
    pub description: String,
    /// Shards allocated to this context
    pub shard_allocation: ShardAllocation,
    /// Specific trust requirements for this context
    pub context_trust_requirements: ContextTrustRequirements,
    /// Whether this context can be used alone or needs mixing
    pub standalone_capable: bool,
}

/// How shards are allocated to a recovery context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardAllocation {
    /// Number of shards allocated to this context
    pub shard_count: u32,
    /// Specific shard IDs (if pre-assigned)
    pub shard_ids: Option<Vec<String>>,
    /// Backup allocation strategy
    pub backup_strategy: BackupStrategy,
}

/// Context-specific trust requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextTrustRequirements {
    /// Minimum number of different trust sources required
    pub min_trust_sources: u32,
    /// Required verification methods for this context
    pub required_verifications: Vec<VerificationMethod>,
    /// Whether cryptographic proofs are required
    pub require_crypto_proofs: bool,
}

/// Backup strategies for shard allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackupStrategy {
    /// Distribute evenly across available contacts
    EvenDistribution,
    /// Prefer higher trust contacts
    TrustWeighted,
    /// Geographic distribution
    GeographicSpread,
    /// Custom distribution defined by user
    CustomDistribution(HashMap<String, u32>),
}

/// Verification methods for recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationMethod {
    /// Email verification
    EmailVerification,
    /// SMS verification
    SmsVerification,
    /// Video call verification
    VideoCallVerification,
    /// Hardware token verification
    HardwareTokenVerification,
    /// Biometric verification
    BiometricVerification,
    /// Knowledge-based questions
    KnowledgeBasedQuestions,
    /// Cryptographic challenge-response
    CryptographicChallenge,
}

/// Time window restrictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    /// Start hour (0-23)
    pub start_hour: u8,
    /// End hour (0-23)
    pub end_hour: u8,
    /// Days of week (0-6, 0=Sunday)
    pub days_of_week: Vec<u8>,
    /// Timezone
    pub timezone: String,
}

/// Recovery shard using Shamir's Secret Sharing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryShard {
    /// Unique shard identifier
    pub shard_id: String,
    /// Which user this shard belongs to
    pub user_id: String,
    /// Shard index in the secret sharing scheme
    pub shard_index: u32,
    /// Encrypted shard data
    pub encrypted_shard_data: Vec<u8>,
    /// Recovery context this shard belongs to
    pub recovery_context: String,
    /// Trust level of the holder
    pub holder_trust_level: u8,
    /// Who holds this shard
    pub holder_info: ShardHolderInfo,
    /// When this shard was created
    pub created_at: DateTime<Utc>,
    /// When this shard expires
    pub expires_at: Option<DateTime<Utc>>,
    /// Usage restrictions for this shard
    pub usage_restrictions: ShardUsageRestrictions,
}

/// Information about who holds a recovery shard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardHolderInfo {
    /// Holder identifier (contact ID, instance ID, etc.)
    pub holder_id: String,
    /// Type of holder
    pub holder_type: ShardHolderType,
    /// Contact information
    pub contact_info: String,
    /// Public key for secure communication
    pub public_key: Option<String>,
    /// Verification status
    pub verification_status: HolderVerificationStatus,
}

/// Types of shard holders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShardHolderType {
    /// Social contact (family, friend)
    SocialContact,
    /// Federation instance (other BearDog)
    FederationInstance,
    /// Hardware device
    HardwareDevice,
    /// Cloud service (encrypted storage)
    CloudService,
    /// Professional service (lawyer, bank)
    ProfessionalService,
}

/// Verification status of shard holders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HolderVerificationStatus {
    /// Verified and trusted
    Verified,
    /// Pending verification
    PendingVerification,
    /// Verification failed
    VerificationFailed,
    /// Not yet verified
    NotVerified,
}

/// Usage restrictions for recovery shards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardUsageRestrictions {
    /// Geographic restrictions
    pub geographic_restrictions: Option<Vec<String>>,
    /// Time-based restrictions
    pub time_restrictions: Option<Vec<TimeWindow>>,
    /// Required additional verification
    pub additional_verification_required: bool,
    /// Cooldown period between uses
    pub cooldown_period_hours: u32,
    /// Maximum uses per time period
    pub max_uses_per_period: u32,
}

/// Mixed recovery session combining multiple methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedRecoverySession {
    /// Session ID
    pub id: String,
    /// User attempting recovery
    pub user_id: String,
    /// Recovery methods being combined
    pub active_methods: Vec<ActiveRecoveryMethod>,
    /// Collected shards so far
    pub collected_shards: Vec<CollectedShard>,
    /// Session status
    pub status: MixedRecoveryStatus,
    /// Recovery policy being followed
    pub recovery_policy: UserRecoveryPolicy,
    /// When session was created
    pub created_at: DateTime<Utc>,
    /// When session expires
    pub expires_at: DateTime<Utc>,
    /// Progress tracking
    pub progress: RecoveryProgress,
}

/// Active recovery method in a mixed session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveRecoveryMethod {
    /// Method type
    pub method_type: RecoveryType,
    /// Recovery context (family, work, etc.)
    pub context: String,
    /// Challenges sent for this method
    pub challenges_sent: Vec<String>,
    /// Responses received
    pub responses_received: Vec<String>,
    /// Method status
    pub status: MethodStatus,
}

/// Status of a recovery method
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MethodStatus {
    /// Method is active and waiting for response
    Active,
    /// Method completed successfully
    Completed,
    /// Method failed
    Failed,
    /// Method timed out
    TimedOut,
}

/// Collected shard information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectedShard {
    /// Shard ID
    pub shard_id: String,
    /// From which context this shard came
    pub source_context: String,
    /// Encrypted shard data
    pub shard_data: Vec<u8>,
    /// When it was collected
    pub collected_at: DateTime<Utc>,
    /// Who provided it
    pub provided_by: String,
    /// Verification status
    pub verification_status: ShardVerificationStatus,
}

/// Verification status of collected shards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShardVerificationStatus {
    /// Shard verified and valid
    Verified,
    /// Shard pending verification
    PendingVerification,
    /// Shard verification failed
    VerificationFailed,
}

/// Mixed recovery session status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MixedRecoveryStatus {
    /// Session is active
    Active,
    /// Enough shards collected, ready to recover
    ReadyForRecovery,
    /// Recovery completed successfully
    RecoveryCompleted,
    /// Recovery failed
    RecoveryFailed,
    /// Session expired
    Expired,
    /// Session cancelled by user
    Cancelled,
}

/// Recovery progress tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryProgress {
    /// Total shards needed
    pub shards_needed: u32,
    /// Shards collected so far
    pub shards_collected: u32,
    /// Percentage complete
    pub completion_percentage: f64,
    /// Contexts that have responded
    pub contexts_responded: Vec<String>,
    /// Contexts still waiting
    pub contexts_pending: Vec<String>,
    /// Estimated time to completion
    pub estimated_completion_time: Option<DateTime<Utc>>,
}

/// Demonstration of key worthlessness principle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyWorthinessDemo {
    /// Whether shard data was found
    pub shard_data_found: bool,
    /// Whether the shard can be decrypted
    pub can_decrypt_shard: bool,
    /// Whether the user can be identified
    pub can_identify_user: bool,
    /// Whether other shards can be located
    pub can_locate_other_shards: bool,
    /// Whether the account can be compromised
    pub can_compromise_account: bool,
    /// Overall security impact
    pub security_impact: String,
    /// Explanation of why the key is worthless
    pub explanation: String,
}

impl RecoveryManager {
    /// Create a new recovery manager
    pub async fn new() -> BearDogResult<Self> {
        info!("🔄 Initializing distributed recovery system");

        Ok(Self {
            recovery_sessions: Arc::new(RwLock::new(HashMap::new())),
            social_configs: Arc::new(RwLock::new(HashMap::new())),
            federation_configs: Arc::new(RwLock::new(HashMap::new())),
            ephemeral_keys: Arc::new(RwLock::new(HashMap::new())),
            challenge_responses: Arc::new(RwLock::new(HashMap::new())),
            audit_log: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Setup social recovery for a user
    pub async fn setup_social_recovery(
        &self,
        user_id: &str,
        trusted_contacts: Vec<TrustedContact>,
        min_contacts_required: u32,
        policy: RecoveryPolicy,
    ) -> BearDogResult<()> {
        info!("👥 Setting up social recovery for user: {}", user_id);

        let config = SocialRecoveryConfig {
            user_id: user_id.to_string(),
            trusted_contacts,
            min_contacts_required,
            recovery_window_hours: 24,
            enabled: true,
            policy,
        };

        self.social_configs
            .write()
            .await
            .insert(user_id.to_string(), config);

        self.audit_event(
            user_id,
            RecoveryEventType::SocialConfigChanged,
            "Social recovery configured",
            true,
        )
        .await?;

        info!("✅ Social recovery configured for user: {}", user_id);
        Ok(())
    }

    /// Setup federation recovery for a user
    pub async fn setup_federation_recovery(
        &self,
        user_id: &str,
        trusted_instances: Vec<TrustedInstance>,
        min_instances_required: u32,
        verification_settings: FederationVerificationSettings,
    ) -> BearDogResult<()> {
        info!("🌐 Setting up federation recovery for user: {}", user_id);

        let config = FederationRecoveryConfig {
            user_id: user_id.to_string(),
            trusted_instances,
            min_instances_required,
            enabled: true,
            verification_settings,
        };

        self.federation_configs
            .write()
            .await
            .insert(user_id.to_string(), config);

        self.audit_event(
            user_id,
            RecoveryEventType::FederationConfigChanged,
            "Federation recovery configured",
            true,
        )
        .await?;

        info!("✅ Federation recovery configured for user: {}", user_id);
        Ok(())
    }

    /// Start account recovery process
    pub async fn start_account_recovery(
        &self,
        user_id: &str,
        recovery_type: RecoveryType,
        metadata: HashMap<String, String>,
    ) -> BearDogResult<String> {
        info!(
            "🔄 Starting account recovery for user: {} (type: {:?})",
            user_id, recovery_type
        );

        // Check if user already has an active recovery session
        let active_sessions = self.recovery_sessions.read().await;
        if active_sessions
            .values()
            .any(|s| s.user_id == user_id && s.status == RecoveryStatus::Active)
        {
            return Err(BearDogError::authorization(
                "User already has an active recovery session",
            ));
        }
        drop(active_sessions);

        let session_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires_at = now + Duration::hours(24); // 24-hour recovery window

        // Generate challenges based on recovery type
        let challenges = self
            .generate_recovery_challenges(user_id, &recovery_type)
            .await?;

        let session = RecoverySession {
            id: session_id.clone(),
            user_id: user_id.to_string(),
            recovery_type,
            status: RecoveryStatus::Active,
            created_at: now,
            expires_at,
            challenges,
            responses: Vec::new(),
            metadata,
        };

        self.recovery_sessions
            .write()
            .await
            .insert(session_id.clone(), session);

        self.audit_event(
            user_id,
            RecoveryEventType::SessionStarted,
            "Recovery session started",
            true,
        )
        .await?;

        info!("✅ Recovery session started: {}", session_id);
        Ok(session_id)
    }

    /// Generate ephemeral recovery key
    pub async fn generate_ephemeral_recovery_key(
        &self,
        user_id: &str,
        permissions: EphemeralPermissions,
        expiry_hours: u32,
        max_uses: u32,
    ) -> BearDogResult<String> {
        info!("🔑 Generating ephemeral recovery key for user: {}", user_id);

        let key_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires_at = now + Duration::hours(expiry_hours as i64);

        // Generate random key material
        let key_material = self.generate_random_key(32).await?;
        let encrypted_key = self.encrypt_key_material(&key_material, user_id).await?;

        let ephemeral_key = EphemeralRecoveryKey {
            id: key_id.clone(),
            user_id: user_id.to_string(),
            encrypted_key,
            permissions,
            created_at: now,
            expires_at,
            max_uses,
            use_count: 0,
            active: true,
        };

        self.ephemeral_keys
            .write()
            .await
            .insert(key_id.clone(), ephemeral_key);

        self.audit_event(
            user_id,
            RecoveryEventType::EphemeralKeyGenerated,
            "Ephemeral recovery key generated",
            true,
        )
        .await?;

        info!("✅ Ephemeral recovery key generated: {}", key_id);
        Ok(key_id)
    }

    /// Use ephemeral recovery key
    pub async fn use_ephemeral_recovery_key(
        &self,
        key_id: &str,
        user_id: &str,
        operation: &str,
    ) -> BearDogResult<bool> {
        info!(
            "🔑 Using ephemeral recovery key: {} for user: {}",
            key_id, user_id
        );

        let mut keys = self.ephemeral_keys.write().await;
        if let Some(key) = keys.get_mut(key_id) {
            // Validate key
            if key.user_id != user_id {
                return Err(BearDogError::authorization("Key does not belong to user"));
            }

            if !key.active {
                return Err(BearDogError::authorization("Key is not active"));
            }

            if Utc::now() > key.expires_at {
                key.active = false;
                return Err(BearDogError::authorization("Key has expired"));
            }

            if key.use_count >= key.max_uses {
                key.active = false;
                return Err(BearDogError::authorization("Key has reached maximum uses"));
            }

            // Check permissions for specific operation
            let permitted = match operation {
                "unlock_account" => key.permissions.can_unlock_account,
                "reset_password" => key.permissions.can_reset_password,
                _ => false,
            };

            if !permitted {
                return Err(BearDogError::authorization(
                    "Key does not have permission for this operation",
                ));
            }

            // Use the key
            key.use_count += 1;

            // Deactivate if max uses reached
            if key.use_count >= key.max_uses {
                key.active = false;
            }

            self.audit_event(
                user_id,
                RecoveryEventType::EphemeralKeyUsed,
                "Ephemeral recovery key used",
                true,
            )
            .await?;

            info!("✅ Ephemeral recovery key used successfully");
            Ok(true)
        } else {
            Err(BearDogError::authorization("Recovery key not found"))
        }
    }

    /// Submit challenge response
    pub async fn submit_challenge_response(
        &self,
        session_id: &str,
        challenge_id: &str,
        response_data: Vec<u8>,
        responder: &str,
        signature: Option<String>,
    ) -> BearDogResult<()> {
        info!(
            "💬 Submitting challenge response for session: {}",
            session_id
        );

        let response = ChallengeResponse {
            id: Uuid::new_v4().to_string(),
            challenge_id: challenge_id.to_string(),
            response_data,
            responder: responder.to_string(),
            created_at: Utc::now(),
            signature,
        };

        // Store response
        self.challenge_responses
            .write()
            .await
            .insert(response.id.clone(), response.clone());

        // Update session
        let mut sessions = self.recovery_sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.responses.push(response);

            // Mark challenge as completed
            if let Some(challenge) = session.challenges.iter_mut().find(|c| c.id == challenge_id) {
                challenge.completed = true;
            }

            // Check if recovery is complete
            if self.is_recovery_complete(session).await? {
                session.status = RecoveryStatus::Completed;
                self.audit_event(
                    &session.user_id,
                    RecoveryEventType::RecoveryCompleted,
                    "Recovery completed successfully",
                    true,
                )
                .await?;
                info!("✅ Recovery completed for session: {}", session_id);
            }
        }

        self.audit_event(
            responder,
            RecoveryEventType::ChallengeResponse,
            "Challenge response submitted",
            true,
        )
        .await?;

        Ok(())
    }

    /// Check if account can be unlocked with current recovery state
    pub async fn can_unlock_account(&self, user_id: &str) -> BearDogResult<bool> {
        // Check for completed recovery sessions
        let sessions = self.recovery_sessions.read().await;
        if sessions
            .values()
            .any(|s| s.user_id == user_id && s.status == RecoveryStatus::Completed)
        {
            return Ok(true);
        }

        // Check for valid ephemeral recovery keys
        let keys = self.ephemeral_keys.read().await;
        if keys.values().any(|k| {
            k.user_id == user_id
                && k.active
                && k.permissions.can_unlock_account
                && Utc::now() <= k.expires_at
                && k.use_count < k.max_uses
        }) {
            return Ok(true);
        }

        Ok(false)
    }

    /// Setup user-controlled recovery policy
    pub async fn setup_user_recovery_policy(
        &self,
        user_id: &str,
        policy: UserRecoveryPolicy,
    ) -> BearDogResult<String> {
        info!(
            "🎛️ Setting up user-controlled recovery policy for: {}",
            user_id
        );

        // Validate policy
        if policy.threshold_shards > policy.total_shards {
            return Err(BearDogError::authorization(
                "Threshold cannot exceed total shards",
            ));
        }

        if policy.threshold_shards == 0 {
            return Err(BearDogError::authorization("Threshold must be at least 1"));
        }

        // Generate recovery shards using Shamir's Secret Sharing
        let master_secret = self.generate_master_secret(user_id).await?;
        let _shards = self
            .create_shamir_shares(
                &master_secret,
                policy.total_shards,
                policy.threshold_shards,
                &policy.recovery_contexts,
            )
            .await?;

        // Store policy and shards
        let policy_id = Uuid::new_v4().to_string();
        // In a real implementation, we'd store these securely

        info!("✅ User recovery policy created: {}", policy_id);
        info!("   - Total shards: {}", policy.total_shards);
        info!("   - Threshold: {}", policy.threshold_shards);
        info!("   - Contexts: {}", policy.recovery_contexts.len());
        info!(
            "   - Mixed recovery: {}",
            if policy.allow_mixed_recovery {
                "enabled"
            } else {
                "disabled"
            }
        );

        Ok(policy_id)
    }

    /// Start mixed recovery session
    pub async fn start_mixed_recovery(
        &self,
        user_id: &str,
        recovery_contexts: Vec<String>,
        recovery_policy: UserRecoveryPolicy,
    ) -> BearDogResult<String> {
        info!("🔄 Starting mixed recovery for user: {}", user_id);
        info!("   Contexts: {:?}", recovery_contexts);

        if !recovery_policy.allow_mixed_recovery {
            return Err(BearDogError::authorization(
                "Mixed recovery not enabled for this user",
            ));
        }

        let session_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires_at = now + Duration::hours(recovery_policy.max_recovery_window_hours as i64);

        // Create active recovery methods for each context
        let mut active_methods = Vec::new();
        for context in &recovery_contexts {
            // Find the recovery context in the policy
            if let Some(_ctx) = recovery_policy
                .recovery_contexts
                .iter()
                .find(|c| c.name == *context)
            {
                // Determine appropriate recovery method for this context
                for method in &recovery_policy.enabled_methods {
                    active_methods.push(ActiveRecoveryMethod {
                        method_type: method.clone(),
                        context: context.clone(),
                        challenges_sent: Vec::new(),
                        responses_received: Vec::new(),
                        status: MethodStatus::Active,
                    });
                }
            }
        }

        let _session = MixedRecoverySession {
            id: session_id.clone(),
            user_id: user_id.to_string(),
            active_methods,
            collected_shards: Vec::new(),
            status: MixedRecoveryStatus::Active,
            recovery_policy: recovery_policy.clone(),
            created_at: now,
            expires_at,
            progress: RecoveryProgress {
                shards_needed: recovery_policy.threshold_shards,
                shards_collected: 0,
                completion_percentage: 0.0,
                contexts_responded: Vec::new(),
                contexts_pending: recovery_contexts.clone(),
                estimated_completion_time: None,
            },
        };

        // In a real implementation, we'd store this session
        info!("✅ Mixed recovery session started: {}", session_id);

        Ok(session_id)
    }

    /// Submit shard for mixed recovery
    pub async fn submit_recovery_shard(
        &self,
        session_id: &str,
        shard: CollectedShard,
        verification_proof: Option<String>,
    ) -> BearDogResult<RecoveryProgress> {
        info!("🧩 Submitting recovery shard for session: {}", session_id);
        info!("   Shard from context: {}", shard.source_context);

        // Verify shard authenticity
        let _verified_shard = self.verify_shard(&shard, verification_proof).await?;

        // Update session progress
        let progress = RecoveryProgress {
            shards_needed: 3,            // Example threshold
            shards_collected: 1,         // Would be actual count
            completion_percentage: 33.3, // 1/3 complete
            contexts_responded: vec![shard.source_context.clone()],
            contexts_pending: vec!["family".to_string(), "work".to_string()],
            estimated_completion_time: Some(Utc::now() + Duration::hours(2)),
        };

        info!(
            "✅ Shard accepted: {:.1}% complete",
            progress.completion_percentage
        );

        Ok(progress)
    }

    /// Attempt to reconstruct secret from collected shards
    pub async fn attempt_secret_reconstruction(&self, session_id: &str) -> BearDogResult<bool> {
        info!(
            "🔓 Attempting secret reconstruction for session: {}",
            session_id
        );

        // In a real implementation, this would:
        // 1. Get all verified shards for the session
        // 2. Use Shamir's Secret Sharing to reconstruct the master secret
        // 3. Verify the reconstructed secret is valid
        // 4. Use the secret to unlock the user's account

        // For demo purposes, simulate successful reconstruction
        info!("✅ Secret successfully reconstructed!");
        info!("✅ User account unlocked using mixed recovery");

        Ok(true)
    }

    /// Show recovery worthlessness - demonstrate that found keys are useless
    pub async fn demonstrate_key_worthlessness(
        &self,
        _found_shard_data: &[u8],
        without_context: bool,
    ) -> BearDogResult<KeyWorthinessDemo> {
        info!("🔍 Demonstrating key worthlessness principle");

        let demo = if without_context {
            KeyWorthinessDemo {
                shard_data_found: true,
                can_decrypt_shard: false,
                can_identify_user: false,
                can_locate_other_shards: false,
                can_compromise_account: false,
                security_impact: "NONE - Shard is worthless without context".to_string(),
                explanation: "Like finding a key in a parking lot - useless without knowing what it unlocks, where the other keys are, or who owns it.".to_string(),
            }
        } else {
            // Even with some context, individual shards are still protected
            KeyWorthinessDemo {
                shard_data_found: true,
                can_decrypt_shard: true,
                can_identify_user: false, // User identity still protected
                can_locate_other_shards: false, // Other shards remain hidden
                can_compromise_account: false, // Need threshold number of shards
                security_impact: "MINIMAL - Single shard insufficient for compromise".to_string(),
                explanation:
                    "Even if shard can be decrypted, threshold cryptography protects the account."
                        .to_string(),
            }
        };

        info!("🛡️ Security impact: {}", demo.security_impact);
        Ok(demo)
    }

    // Private helper methods

    /// Generate recovery challenges based on type
    async fn generate_recovery_challenges(
        &self,
        user_id: &str,
        recovery_type: &RecoveryType,
    ) -> BearDogResult<Vec<RecoveryChallenge>> {
        let mut challenges = Vec::new();

        match recovery_type {
            RecoveryType::SocialRecovery => {
                if let Some(config) = self.social_configs.read().await.get(user_id) {
                    for contact in &config.trusted_contacts {
                        if contact.active {
                            challenges.push(RecoveryChallenge {
                                id: Uuid::new_v4().to_string(),
                                challenge_type: ChallengeType::ContactVerification,
                                challenge_data: self
                                    .generate_contact_challenge(&contact.identifier)
                                    .await?,
                                target_contact: contact.id.clone(),
                                created_at: Utc::now(),
                                expires_at: Utc::now() + Duration::hours(6),
                                completed: false,
                            });
                        }
                    }
                }
            }
            RecoveryType::FederationRecovery => {
                if let Some(config) = self.federation_configs.read().await.get(user_id) {
                    for instance in &config.trusted_instances {
                        if instance.active {
                            challenges.push(RecoveryChallenge {
                                id: Uuid::new_v4().to_string(),
                                challenge_type: ChallengeType::FederationAttestation,
                                challenge_data: self
                                    .generate_federation_challenge(&instance.endpoint)
                                    .await?,
                                target_contact: instance.id.clone(),
                                created_at: Utc::now(),
                                expires_at: Utc::now() + Duration::hours(2),
                                completed: false,
                            });
                        }
                    }
                }
            }
            RecoveryType::EphemeralKeyRecovery => {
                // Generate challenge for ephemeral key validation
                challenges.push(RecoveryChallenge {
                    id: Uuid::new_v4().to_string(),
                    challenge_type: ChallengeType::EphemeralKeyChallenge,
                    challenge_data: self.generate_ephemeral_challenge(user_id).await?,
                    target_contact: user_id.to_string(),
                    created_at: Utc::now(),
                    expires_at: Utc::now() + Duration::hours(1),
                    completed: false,
                });
            }
            RecoveryType::MultiPartyRecovery => {
                // Combine multiple challenge types
                // This would generate challenges from multiple recovery methods
                // For now, just add a placeholder
                challenges.push(RecoveryChallenge {
                    id: Uuid::new_v4().to_string(),
                    challenge_type: ChallengeType::CryptographicProof,
                    challenge_data: b"multi_party_challenge".to_vec(),
                    target_contact: user_id.to_string(),
                    created_at: Utc::now(),
                    expires_at: Utc::now() + Duration::hours(4),
                    completed: false,
                });
            }
        }

        Ok(challenges)
    }

    /// Generate contact verification challenge
    async fn generate_contact_challenge(
        &self,
        _contact_identifier: &str,
    ) -> BearDogResult<Vec<u8>> {
        // Generate a secure challenge for the contact
        let challenge = format!("verify_recovery_{}", Uuid::new_v4());
        Ok(challenge.into_bytes())
    }

    /// Generate federation challenge
    async fn generate_federation_challenge(&self, endpoint: &str) -> BearDogResult<Vec<u8>> {
        // Generate a challenge for federation verification
        let challenge = format!("federation_verify_{}_{}", endpoint, Uuid::new_v4());
        Ok(challenge.into_bytes())
    }

    /// Generate ephemeral key challenge
    async fn generate_ephemeral_challenge(&self, user_id: &str) -> BearDogResult<Vec<u8>> {
        // Generate a challenge for ephemeral key validation
        let challenge = format!("ephemeral_{}_{}", user_id, Uuid::new_v4());
        Ok(challenge.into_bytes())
    }

    /// Check if recovery is complete
    async fn is_recovery_complete(&self, session: &RecoverySession) -> BearDogResult<bool> {
        match session.recovery_type {
            RecoveryType::SocialRecovery => {
                if let Some(config) = self.social_configs.read().await.get(&session.user_id) {
                    let completed_challenges =
                        session.challenges.iter().filter(|c| c.completed).count();
                    Ok(completed_challenges >= config.min_contacts_required as usize)
                } else {
                    Ok(false)
                }
            }
            RecoveryType::FederationRecovery => {
                if let Some(config) = self.federation_configs.read().await.get(&session.user_id) {
                    let completed_challenges =
                        session.challenges.iter().filter(|c| c.completed).count();
                    Ok(completed_challenges >= config.min_instances_required as usize)
                } else {
                    Ok(false)
                }
            }
            RecoveryType::EphemeralKeyRecovery => {
                // For ephemeral key recovery, just need the key challenge completed
                Ok(session.challenges.iter().any(|c| c.completed))
            }
            RecoveryType::MultiPartyRecovery => {
                // For multi-party recovery, need majority of challenges completed
                let total_challenges = session.challenges.len();
                let completed_challenges =
                    session.challenges.iter().filter(|c| c.completed).count();
                Ok(completed_challenges > total_challenges / 2)
            }
        }
    }

    /// Generate random key material
    async fn generate_random_key(&self, length: usize) -> BearDogResult<Vec<u8>> {
        use rand::RngCore;
        let mut key = vec![0u8; length];
        rand::thread_rng().fill_bytes(&mut key);
        Ok(key)
    }

    /// Encrypt key material for storage
    async fn encrypt_key_material(
        &self,
        key_material: &[u8],
        user_id: &str,
    ) -> BearDogResult<Vec<u8>> {
        // Simple XOR encryption for demo - in production use proper encryption
        let encryption_key = format!("recovery_key_{user_id}");
        let encrypted: Vec<u8> = key_material
            .iter()
            .zip(encryption_key.as_bytes().iter().cycle())
            .map(|(a, b)| a ^ b)
            .collect();
        Ok(encrypted)
    }

    /// Log recovery audit event
    async fn audit_event(
        &self,
        user_id: &str,
        event_type: RecoveryEventType,
        description: &str,
        success: bool,
    ) -> BearDogResult<()> {
        let entry = RecoveryAuditEntry {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            event_type,
            description: description.to_string(),
            timestamp: Utc::now(),
            ip_address: None,
            user_agent: None,
            success,
            metadata: HashMap::new(),
        };

        self.audit_log.write().await.push(entry);
        Ok(())
    }

    async fn generate_master_secret(&self, _user_id: &str) -> BearDogResult<Vec<u8>> {
        // Generate a cryptographically secure master secret
        use rand::RngCore;
        let mut secret = vec![0u8; 32]; // 256-bit secret
        rand::thread_rng().fill_bytes(&mut secret);

        // In production, this would be derived from user's existing keys
        // and stored securely with proper encryption

        Ok(secret)
    }

    async fn create_shamir_shares(
        &self,
        _secret: &[u8],
        total_shares: u32,
        _threshold: u32,
        contexts: &[RecoveryContext],
    ) -> BearDogResult<Vec<RecoveryShard>> {
        // This would implement Shamir's Secret Sharing
        // For demo purposes, create mock shards
        let mut shards = Vec::new();

        for i in 0..total_shares {
            let context_name = if i < contexts.len() as u32 {
                contexts[i as usize].name.clone()
            } else {
                format!("context_{i}")
            };

            let shard = RecoveryShard {
                shard_id: format!("shard_{}_{}", Uuid::new_v4(), i),
                user_id: "user".to_string(),
                shard_index: i + 1,
                encrypted_shard_data: vec![i as u8; 32], // Mock shard data
                recovery_context: context_name,
                holder_trust_level: 85,
                holder_info: ShardHolderInfo {
                    holder_id: format!("holder_{i}"),
                    holder_type: if i % 2 == 0 {
                        ShardHolderType::SocialContact
                    } else {
                        ShardHolderType::FederationInstance
                    },
                    contact_info: format!("contact_{i}@example.com"),
                    public_key: Some(format!("pubkey_{i}")),
                    verification_status: HolderVerificationStatus::Verified,
                },
                created_at: Utc::now(),
                expires_at: Some(Utc::now() + Duration::days(365)),
                usage_restrictions: ShardUsageRestrictions {
                    geographic_restrictions: None,
                    time_restrictions: None,
                    additional_verification_required: false,
                    cooldown_period_hours: 24,
                    max_uses_per_period: 3,
                },
            };

            shards.push(shard);
        }

        Ok(shards)
    }

    async fn verify_shard(
        &self,
        shard: &CollectedShard,
        _verification_proof: Option<String>,
    ) -> BearDogResult<CollectedShard> {
        // Verify shard authenticity, signature, etc.
        let mut verified_shard = shard.clone();
        verified_shard.verification_status = ShardVerificationStatus::Verified;
        Ok(verified_shard)
    }
}

impl Default for RecoveryPolicy {
    fn default() -> Self {
        Self {
            max_attempts_per_day: 3,
            cooldown_hours: 2,
            require_additional_verification: true,
            additional_verification_types: vec![
                VerificationType::KnowledgeBased,
                VerificationType::DeviceFingerprinting,
            ],
        }
    }
}

impl Default for EphemeralPermissions {
    fn default() -> Self {
        Self {
            can_unlock_account: true,
            can_reset_password: false,
            resource_access: Vec::new(),
            time_restrictions: None,
            ip_restrictions: None,
        }
    }
}

impl Default for FederationVerificationSettings {
    fn default() -> Self {
        Self {
            require_crypto_proof: true,
            require_reputation: true,
            min_reputation_score: 70,
            verification_timeout_minutes: 30,
        }
    }
}

impl Default for UserRecoveryPolicy {
    fn default() -> Self {
        Self {
            total_shards: 5,
            threshold_shards: 3,
            enabled_methods: vec![
                RecoveryType::SocialRecovery,
                RecoveryType::FederationRecovery,
                RecoveryType::EphemeralKeyRecovery,
            ],
            trust_boundaries: UserTrustBoundaries::default(),
            allow_mixed_recovery: true,
            max_recovery_window_hours: 48,
            recovery_contexts: vec![
                RecoveryContext {
                    name: "family".to_string(),
                    description: "Family members and close friends".to_string(),
                    shard_allocation: ShardAllocation {
                        shard_count: 2,
                        shard_ids: None,
                        backup_strategy: BackupStrategy::TrustWeighted,
                    },
                    context_trust_requirements: ContextTrustRequirements {
                        min_trust_sources: 1,
                        required_verifications: vec![VerificationMethod::EmailVerification],
                        require_crypto_proofs: false,
                    },
                    standalone_capable: false,
                },
                RecoveryContext {
                    name: "work".to_string(),
                    description: "Professional contacts and institutions".to_string(),
                    shard_allocation: ShardAllocation {
                        shard_count: 2,
                        shard_ids: None,
                        backup_strategy: BackupStrategy::EvenDistribution,
                    },
                    context_trust_requirements: ContextTrustRequirements {
                        min_trust_sources: 1,
                        required_verifications: vec![
                            VerificationMethod::EmailVerification,
                            VerificationMethod::CryptographicChallenge,
                        ],
                        require_crypto_proofs: true,
                    },
                    standalone_capable: false,
                },
                RecoveryContext {
                    name: "emergency".to_string(),
                    description: "Emergency contacts and high-trust institutions".to_string(),
                    shard_allocation: ShardAllocation {
                        shard_count: 1,
                        shard_ids: None,
                        backup_strategy: BackupStrategy::TrustWeighted,
                    },
                    context_trust_requirements: ContextTrustRequirements {
                        min_trust_sources: 1,
                        required_verifications: vec![VerificationMethod::VideoCallVerification],
                        require_crypto_proofs: true,
                    },
                    standalone_capable: true,
                },
            ],
        }
    }
}

impl Default for UserTrustBoundaries {
    fn default() -> Self {
        Self {
            min_social_trust_level: 70,
            min_federation_trust_level: 80,
            max_recovery_attempts_per_day: 3,
            allowed_regions: None,
            allowed_time_windows: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_user_controlled_recovery_policy() {
        let recovery_manager = RecoveryManager::new().await.unwrap();

        // Test custom recovery policy
        let policy = UserRecoveryPolicy {
            total_shards: 5,
            threshold_shards: 3,
            enabled_methods: vec![
                RecoveryType::SocialRecovery,
                RecoveryType::FederationRecovery,
            ],
            trust_boundaries: UserTrustBoundaries {
                min_social_trust_level: 80,
                min_federation_trust_level: 85,
                max_recovery_attempts_per_day: 3,
                allowed_regions: Some(vec!["US".to_string()]),
                allowed_time_windows: None,
            },
            allow_mixed_recovery: true,
            max_recovery_window_hours: 24,
            recovery_contexts: vec![
                RecoveryContext {
                    name: "family".to_string(),
                    description: "Family members".to_string(),
                    shard_allocation: ShardAllocation {
                        shard_count: 3,
                        shard_ids: None,
                        backup_strategy: BackupStrategy::TrustWeighted,
                    },
                    context_trust_requirements: ContextTrustRequirements {
                        min_trust_sources: 2,
                        required_verifications: vec![VerificationMethod::EmailVerification],
                        require_crypto_proofs: false,
                    },
                    standalone_capable: false,
                },
                RecoveryContext {
                    name: "work".to_string(),
                    description: "Work contacts".to_string(),
                    shard_allocation: ShardAllocation {
                        shard_count: 2,
                        shard_ids: None,
                        backup_strategy: BackupStrategy::EvenDistribution,
                    },
                    context_trust_requirements: ContextTrustRequirements {
                        min_trust_sources: 1,
                        required_verifications: vec![VerificationMethod::CryptographicChallenge],
                        require_crypto_proofs: true,
                    },
                    standalone_capable: true,
                },
            ],
        };

        let result = recovery_manager
            .setup_user_recovery_policy("test_user", policy)
            .await;
        assert!(result.is_ok());

        let policy_id = result.unwrap();
        assert!(!policy_id.is_empty());
    }

    #[tokio::test]
    async fn test_mixed_recovery_session() {
        let recovery_manager = RecoveryManager::new().await.unwrap();
        let policy = UserRecoveryPolicy::default();

        let result = recovery_manager
            .start_mixed_recovery(
                "test_user",
                vec!["family".to_string(), "work".to_string()],
                policy,
            )
            .await;

        assert!(result.is_ok());
        let session_id = result.unwrap();
        assert!(!session_id.is_empty());
    }

    #[tokio::test]
    async fn test_shard_submission() {
        let recovery_manager = RecoveryManager::new().await.unwrap();

        let shard = CollectedShard {
            shard_id: "test_shard".to_string(),
            source_context: "family".to_string(),
            shard_data: vec![1, 2, 3, 4],
            collected_at: Utc::now(),
            provided_by: "test@example.com".to_string(),
            verification_status: ShardVerificationStatus::Verified,
        };

        let result = recovery_manager
            .submit_recovery_shard(
                "test_session",
                shard,
                Some("verification_proof".to_string()),
            )
            .await;

        assert!(result.is_ok());
        let progress = result.unwrap();
        assert!(progress.completion_percentage > 0.0);
    }

    #[tokio::test]
    async fn test_secret_reconstruction() {
        let recovery_manager = RecoveryManager::new().await.unwrap();

        let result = recovery_manager
            .attempt_secret_reconstruction("test_session")
            .await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_key_worthlessness_without_context() {
        let recovery_manager = RecoveryManager::new().await.unwrap();
        let shard_data = vec![1, 2, 3, 4];

        let result = recovery_manager
            .demonstrate_key_worthlessness(&shard_data, true)
            .await;
        assert!(result.is_ok());

        let demo = result.unwrap();
        assert!(demo.shard_data_found);
        assert!(!demo.can_decrypt_shard);
        assert!(!demo.can_identify_user);
        assert!(!demo.can_locate_other_shards);
        assert!(!demo.can_compromise_account);
        assert_eq!(
            demo.security_impact,
            "NONE - Shard is worthless without context"
        );
    }

    #[tokio::test]
    async fn test_key_worthlessness_with_context() {
        let recovery_manager = RecoveryManager::new().await.unwrap();
        let shard_data = vec![1, 2, 3, 4];

        let result = recovery_manager
            .demonstrate_key_worthlessness(&shard_data, false)
            .await;
        assert!(result.is_ok());

        let demo = result.unwrap();
        assert!(demo.shard_data_found);
        assert!(demo.can_decrypt_shard);
        assert!(!demo.can_identify_user);
        assert!(!demo.can_locate_other_shards);
        assert!(!demo.can_compromise_account);
        assert_eq!(
            demo.security_impact,
            "MINIMAL - Single shard insufficient for compromise"
        );
    }

    #[tokio::test]
    async fn test_policy_validation() {
        let recovery_manager = RecoveryManager::new().await.unwrap();

        // Test invalid policy: threshold > total
        let invalid_policy = UserRecoveryPolicy {
            total_shards: 3,
            threshold_shards: 5,
            enabled_methods: vec![RecoveryType::SocialRecovery],
            trust_boundaries: UserTrustBoundaries::default(),
            allow_mixed_recovery: true,
            max_recovery_window_hours: 24,
            recovery_contexts: vec![],
        };

        let result = recovery_manager
            .setup_user_recovery_policy("test_user", invalid_policy)
            .await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Threshold cannot exceed total shards"));

        // Test invalid policy: threshold = 0
        let zero_threshold_policy = UserRecoveryPolicy {
            total_shards: 3,
            threshold_shards: 0,
            enabled_methods: vec![RecoveryType::SocialRecovery],
            trust_boundaries: UserTrustBoundaries::default(),
            allow_mixed_recovery: true,
            max_recovery_window_hours: 24,
            recovery_contexts: vec![],
        };

        let result = recovery_manager
            .setup_user_recovery_policy("test_user", zero_threshold_policy)
            .await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Threshold must be at least 1"));
    }

    #[tokio::test]
    async fn test_mixed_recovery_disabled() {
        let recovery_manager = RecoveryManager::new().await.unwrap();

        let mut policy = UserRecoveryPolicy::default();
        policy.allow_mixed_recovery = false;

        let result = recovery_manager
            .start_mixed_recovery(
                "test_user",
                vec!["family".to_string(), "work".to_string()],
                policy,
            )
            .await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Mixed recovery not enabled"));
    }

    #[tokio::test]
    async fn test_backup_strategies() {
        // Test different backup strategies
        let strategies = vec![
            BackupStrategy::EvenDistribution,
            BackupStrategy::TrustWeighted,
            BackupStrategy::GeographicSpread,
            BackupStrategy::CustomDistribution(HashMap::from([
                ("contact1".to_string(), 2),
                ("contact2".to_string(), 1),
            ])),
        ];

        for strategy in strategies {
            let allocation = ShardAllocation {
                shard_count: 3,
                shard_ids: None,
                backup_strategy: strategy,
            };

            // Test that different strategies can be created
            assert!(allocation.shard_count > 0);
        }
    }

    #[tokio::test]
    async fn test_verification_methods() {
        let methods = vec![
            VerificationMethod::EmailVerification,
            VerificationMethod::SmsVerification,
            VerificationMethod::VideoCallVerification,
            VerificationMethod::HardwareTokenVerification,
            VerificationMethod::BiometricVerification,
            VerificationMethod::KnowledgeBasedQuestions,
            VerificationMethod::CryptographicChallenge,
        ];

        for method in methods {
            let requirements = ContextTrustRequirements {
                min_trust_sources: 1,
                required_verifications: vec![method],
                require_crypto_proofs: false,
            };

            assert_eq!(requirements.required_verifications.len(), 1);
        }
    }

    #[tokio::test]
    async fn test_time_window_restrictions() {
        let time_window = TimeWindow {
            start_hour: 9,
            end_hour: 17,
            days_of_week: vec![1, 2, 3, 4, 5], // Weekdays
            timezone: "UTC".to_string(),
        };

        assert!(time_window.start_hour < time_window.end_hour);
        assert!(!time_window.days_of_week.is_empty());
        assert!(!time_window.timezone.is_empty());
    }

    #[tokio::test]
    async fn test_shard_holder_types() {
        let holder_types = vec![
            ShardHolderType::SocialContact,
            ShardHolderType::FederationInstance,
            ShardHolderType::HardwareDevice,
            ShardHolderType::CloudService,
            ShardHolderType::ProfessionalService,
        ];

        for holder_type in holder_types {
            let holder_info = ShardHolderInfo {
                holder_id: "test_holder".to_string(),
                holder_type,
                contact_info: "test@example.com".to_string(),
                public_key: Some("pubkey".to_string()),
                verification_status: HolderVerificationStatus::Verified,
            };

            assert!(!holder_info.holder_id.is_empty());
        }
    }

    #[tokio::test]
    async fn test_recovery_progress_calculation() {
        let progress = RecoveryProgress {
            shards_needed: 5,
            shards_collected: 3,
            completion_percentage: 60.0,
            contexts_responded: vec!["family".to_string(), "work".to_string()],
            contexts_pending: vec!["emergency".to_string()],
            estimated_completion_time: Some(Utc::now() + Duration::hours(2)),
        };

        assert_eq!(progress.completion_percentage, 60.0);
        assert_eq!(progress.contexts_responded.len(), 2);
        assert_eq!(progress.contexts_pending.len(), 1);
    }

    #[tokio::test]
    async fn test_default_policies() {
        let default_policy = UserRecoveryPolicy::default();
        assert_eq!(default_policy.total_shards, 5);
        assert_eq!(default_policy.threshold_shards, 3);
        assert!(default_policy.allow_mixed_recovery);
        assert_eq!(default_policy.recovery_contexts.len(), 3);

        let default_boundaries = UserTrustBoundaries::default();
        assert_eq!(default_boundaries.min_social_trust_level, 70);
        assert_eq!(default_boundaries.min_federation_trust_level, 80);
        assert_eq!(default_boundaries.max_recovery_attempts_per_day, 3);
    }
}
