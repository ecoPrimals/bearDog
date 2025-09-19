

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::types::*;
use beardog_errors::BearDogError;

#[derive(Debug, Clone)]
    /// The challenge type value
    pub challenge_type: ChallengeType,

    /// The created at value
    pub created_at: DateTime<Utc>,

    /// The expires at value
    pub expires_at: DateTime<Utc>,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}
impl RecoveryChallenge {

/// New operation.
    /// Creates a new instance
    pub fn new(&str, challenge_type: ChallengeType) -> Self {
        let now = Utc::now();
        Self {
            id: id.to_string(),
            expires_at: now + chrono::Duration::hours(1), // 1 hour expiry
            metadata: HashMap::with_capacity(16),
        }
    }

/// Is Expired operation.
    /// Checks if expired
    /// Checks if expired
    pub fn is_expired(&self) -> bool {
        Utc::now(String,

    /// The response value
    pub response: String,

    /// The submitted at value
    pub submitted_at: DateTime<Utc>,

impl ChallengeResponse {

/// New operation.
    /// Creates a new instance
    pub fn new(&str, challenge_id: &str, response: &str) -> Self {
            challenge_id,
            response,
            submitted_at: Utc::now(),
