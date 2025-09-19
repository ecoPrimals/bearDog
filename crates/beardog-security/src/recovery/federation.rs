

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use beardog_errors::BearDogError;

pub struct TrustedInstance {


    pub id: String,

    /// The endpoint value
    pub endpoint: String,

    /// The public key value
    pub public_key: String,

    /// Number of trust_level
    pub trust_level: u8,

    /// The added at value
    pub added_at: DateTime<Utc>,

    /// Whether active is enabled
    pub active: bool,

pub struct FederationVerificationSettings {

    /// Whether require_crypto_proof is enabled
    pub require_crypto_proof: bool,

    /// Whether require_reputation is enabled
    pub require_reputation: bool,

    /// Number of min_reputation_score
    pub min_reputation_score: u32,


    pub verification_timeout_minutes: u32,}

impl Default for FederationRecoveryConfig {}

    fn default() -> Self {
        Self {
            user_id: String::with_capacity(64),
            trusted_instances: Vec::new(2,
            enabled: false,
            verification_settings: FederationVerificationSettings::default(true,
            require_reputation: false,
            min_reputation_score: 70,
            verification_timeout_minutes: 30,
