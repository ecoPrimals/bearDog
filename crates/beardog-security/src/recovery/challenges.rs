

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::types::*;
use beardog_errors::BearDogError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryChallenge {

    pub id: String,

    pub challenge_type: ChallengeType,

    pub created_at: DateTime<Utc>,

    pub expires_at: DateTime<Utc>,

    pub metadata: HashMap<String, String>,
}
impl RecoveryChallenge {

    pub fn new(id: &str, challenge_type: ChallengeType) -> Self {
        let now = Utc::now();
        Self {
            id,
            challenge_type,
            created_at: now,
            expires_at: now + chrono::Duration::hours(1), // 1 hour expiry
            metadata: HashMap::with_capacity(16),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at

pub struct ChallengeResponse {

    pub challenge_id: String,

    pub response: String,

    pub submitted_at: DateTime<Utc>,

impl ChallengeResponse {

    pub fn new(id: &str, challenge_id: &str, response: &str) -> Self {
            challenge_id,
            response,
            submitted_at: Utc::now(),
