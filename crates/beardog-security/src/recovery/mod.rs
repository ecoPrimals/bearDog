//! Account Recovery System
//!
//! **Distributed Recovery - No Single Point of Failure**
//!
//! This module implements a comprehensive recovery system that follows the principle:
//! "Finding the key doesn't mean owning the house" - recovery is distributed,
//! time-bound, and requires multiple parties without giving full ownership.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use beardog_errors::{BearDogError, BearDogResult};

pub mod challenges;
pub mod ephemeral;
pub mod federation;
pub mod policies;
pub mod sessions;
pub mod shards;
pub mod social;
pub mod types;

pub use challenges::*;
pub use ephemeral::*;
pub use federation::*;
pub use policies::*;
pub use sessions::*;
pub use shards::*;
pub use social::*;
pub use types::*;

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
    #[allow(dead_code)]
    challenge_responses: Arc<RwLock<HashMap<String, ChallengeResponse>>>,
    /// Recovery audit log
    audit_log: Arc<RwLock<Vec<RecoveryAuditEntry>>>,
}

impl RecoveryManager {
    /// Create a new recovery manager
    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            recovery_sessions: Arc::new(RwLock::new(HashMap::new())),
            social_configs: Arc::new(RwLock::new(HashMap::new())),
            federation_configs: Arc::new(RwLock::new(HashMap::new())),
            ephemeral_keys: Arc::new(RwLock::new(HashMap::new())),
            challenge_responses: Arc::new(RwLock::new(HashMap::new())),
            audit_log: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Start a new recovery session
    pub async fn start_recovery_session(
        &self,
        user_id: String,
        recovery_type: RecoveryType,
    ) -> BearDogResult<String> {
        let session_id = Uuid::new_v4().to_string();
        let session = RecoverySession::new(session_id.clone(), user_id, recovery_type);

        let mut sessions = self.recovery_sessions.write().await;
        sessions.insert(session_id.clone(), session);

        Ok(session_id)
    }

    /// Get recovery session status
    pub async fn get_session_status(&self, session_id: &str) -> BearDogResult<RecoveryStatus> {
        let sessions = self.recovery_sessions.read().await;
        let session = sessions
            .get(session_id)
            .ok_or_else(|| BearDogError::not_found("Recovery session not found"))?;

        Ok(session.status.clone())
    }

    /// Configure social recovery for a user
    pub async fn configure_social_recovery(
        &self,
        user_id: String,
        config: SocialRecoveryConfig,
    ) -> BearDogResult<()> {
        let mut configs = self.social_configs.write().await;
        configs.insert(user_id, config);
        Ok(())
    }

    /// Configure federation recovery for a user
    pub async fn configure_federation_recovery(
        &self,
        user_id: String,
        config: FederationRecoveryConfig,
    ) -> BearDogResult<()> {
        let mut configs = self.federation_configs.write().await;
        configs.insert(user_id, config);
        Ok(())
    }

    /// Generate ephemeral recovery key
    pub async fn generate_ephemeral_key(
        &self,
        user_id: String,
        permissions: EphemeralPermissions,
    ) -> BearDogResult<String> {
        let key_id = Uuid::new_v4().to_string();
        let key = EphemeralRecoveryKey::new(key_id.clone(), user_id, permissions);

        let mut keys = self.ephemeral_keys.write().await;
        keys.insert(key_id.clone(), key);

        Ok(key_id)
    }

    /// Validate ephemeral recovery key
    pub async fn validate_ephemeral_key(&self, key_id: &str) -> BearDogResult<bool> {
        let keys = self.ephemeral_keys.read().await;
        let key = keys
            .get(key_id)
            .ok_or_else(|| BearDogError::not_found("Ephemeral key not found"))?;

        Ok(key.is_valid())
    }

    /// Record recovery audit event
    pub async fn record_audit_event(
        &self,
        event_type: RecoveryEventType,
        user_id: String,
        details: HashMap<String, String>,
    ) -> BearDogResult<()> {
        let entry = RecoveryAuditEntry::new(event_type, user_id, details);
        let mut audit_log = self.audit_log.write().await;
        audit_log.push(entry);
        Ok(())
    }

    /// Get recovery audit log for a user
    pub async fn get_audit_log(&self, user_id: &str) -> BearDogResult<Vec<RecoveryAuditEntry>> {
        let audit_log = self.audit_log.read().await;
        let user_events = audit_log
            .iter()
            .filter(|entry| entry.user_id == user_id)
            .cloned()
            .collect();

        Ok(user_events)
    }

    /// Clean up expired sessions and keys
    pub async fn cleanup_expired(&self) -> BearDogResult<()> {
        // Clean up expired sessions
        let mut sessions = self.recovery_sessions.write().await;
        sessions.retain(|_, session| !session.is_expired());

        // Clean up expired ephemeral keys
        let mut keys = self.ephemeral_keys.write().await;
        keys.retain(|_, key| key.is_valid());

        Ok(())
    }

    /// Get recovery statistics
    pub async fn get_statistics(&self) -> BearDogResult<RecoveryStatistics> {
        let sessions = self.recovery_sessions.read().await;
        let social_configs = self.social_configs.read().await;
        let federation_configs = self.federation_configs.read().await;
        let ephemeral_keys = self.ephemeral_keys.read().await;
        let audit_log = self.audit_log.read().await;

        Ok(RecoveryStatistics {
            active_sessions: sessions.len(),
            social_configs: social_configs.len(),
            federation_configs: federation_configs.len(),
            ephemeral_keys: ephemeral_keys.len(),
            audit_entries: audit_log.len(),
        })
    }

    /// Check if an account can be unlocked for recovery
    pub async fn can_unlock_account(&self, username: &str) -> BearDogResult<bool> {
        let social_configs = self.social_configs.read().await;
        let federation_configs = self.federation_configs.read().await;

        // Check if user has any recovery methods configured
        Ok(social_configs.contains_key(username) || federation_configs.contains_key(username))
    }

    /// Setup social recovery for a user (legacy method)
    pub async fn setup_social_recovery(
        &self,
        user_id: &str,
        trusted_contacts: Vec<TrustedContact>,
        min_contacts_required: u32,
        policy: RecoveryPolicy,
    ) -> BearDogResult<()> {
        let config = SocialRecoveryConfig {
            user_id: user_id.to_string(),
            trusted_contacts,
            min_contacts_required,
            recovery_window_hours: 24,
            enabled: true,
            policy,
        };

        self.configure_social_recovery(user_id.to_string(), config)
            .await
    }

    /// Setup federation recovery for a user (legacy method)
    pub async fn setup_federation_recovery(
        &self,
        user_id: &str,
        trusted_instances: Vec<TrustedInstance>,
        min_instances_required: u32,
        verification_settings: FederationVerificationSettings,
    ) -> BearDogResult<()> {
        let config = FederationRecoveryConfig {
            user_id: user_id.to_string(),
            trusted_instances,
            min_instances_required,
            enabled: true,
            verification_settings,
        };

        self.configure_federation_recovery(user_id.to_string(), config)
            .await
    }

    /// Start account recovery (legacy method)
    pub async fn start_account_recovery(
        &self,
        user_id: &str,
        recovery_type: RecoveryType,
        metadata: HashMap<String, String>,
    ) -> BearDogResult<String> {
        let session_id = self
            .start_recovery_session(user_id.to_string(), recovery_type)
            .await?;

        // Add metadata to the session
        let mut sessions = self.recovery_sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            for (key, value) in metadata {
                session.add_metadata(key, value);
            }
        }

        Ok(session_id)
    }

    /// Generate ephemeral recovery key (legacy method)
    pub async fn generate_ephemeral_recovery_key(
        &self,
        user_id: &str,
        permissions: EphemeralPermissions,
        _expiry_hours: u32,
        _max_uses: u32,
    ) -> BearDogResult<String> {
        self.generate_ephemeral_key(user_id.to_string(), permissions)
            .await
    }
}

/// Recovery system statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryStatistics {
    /// Number of active recovery sessions
    pub active_sessions: usize,
    /// Number of social recovery configurations
    pub social_configs: usize,
    /// Number of federation recovery configurations
    pub federation_configs: usize,
    /// Number of active ephemeral keys
    pub ephemeral_keys: usize,
    /// Number of audit log entries
    pub audit_entries: usize,
}
