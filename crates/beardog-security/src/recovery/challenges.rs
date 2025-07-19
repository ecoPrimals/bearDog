//! Recovery Challenges
//!
//! This module handles recovery challenges and responses.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types::*;

/// Recovery challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryChallenge {
    /// Challenge ID
    pub id: String,
    /// Type of challenge
    pub challenge_type: ChallengeType,
    /// When the challenge was created
    pub created_at: DateTime<Utc>,
    /// When the challenge expires
    pub expires_at: DateTime<Utc>,
    /// Challenge metadata
    pub metadata: HashMap<String, String>,
}

impl RecoveryChallenge {
    /// Create a new recovery challenge
    pub fn new(id: String, challenge_type: ChallengeType) -> Self {
        let now = Utc::now();
        Self {
            id,
            challenge_type,
            created_at: now,
            expires_at: now + chrono::Duration::hours(1), // 1 hour expiry
            metadata: HashMap::new(),
        }
    }

    /// Check if the challenge has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

/// Challenge response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeResponse {
    /// Response ID
    pub id: String,
    /// Challenge ID this response is for
    pub challenge_id: String,
    /// Response data
    pub response: String,
    /// When the response was submitted
    pub submitted_at: DateTime<Utc>,
    /// Response metadata
    pub metadata: HashMap<String, String>,
}

impl ChallengeResponse {
    /// Create a new challenge response
    pub fn new(id: String, challenge_id: String, response: String) -> Self {
        Self {
            id,
            challenge_id,
            response,
            submitted_at: Utc::now(),
            metadata: HashMap::new(),
        }
    }
}
