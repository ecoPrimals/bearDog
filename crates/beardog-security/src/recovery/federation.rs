

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use beardog_errors::BearDogError;

// FederationRecoveryConfig removed - use UnifiedRecoveryConfig from beardog-types instead

pub struct TrustedInstance {

    pub id: String,

    pub endpoint: String,

    pub public_key: String,

    pub trust_level: u8,

    pub added_at: DateTime<Utc>,

    pub active: bool,

pub struct FederationVerificationSettings {

    pub require_crypto_proof: bool,

    pub require_reputation: bool,

    pub min_reputation_score: u32,

    pub verification_timeout_minutes: u32,}

impl Default for FederationRecoveryConfig {}

    fn default() -> Self {
        Self {
            user_id: String::with_capacity(64),
            trusted_instances: Vec::new(),
            min_instances_required: 2,
            enabled: false,
            verification_settings: FederationVerificationSettings::default(),
        }
    }
impl Default for FederationVerificationSettings {
            require_crypto_proof: true,
            require_reputation: false,
            min_reputation_score: 70,
            verification_timeout_minutes: 30,
