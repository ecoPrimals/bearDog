// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Recovery Challenges
///
/// This module handles recovery challenges and responses.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::types::*;
use beardog_errors::{BearDogError, BearDogResult};
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
    /// Create a new recovery challenge}


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
/// Challenge response
pub struct ChallengeResponse {
    /// Response ID
    /// Challenge ID this response is for
    pub challenge_id: String,
    /// Response data
    pub response: String,
    /// When the response was submitted
    pub submitted_at: DateTime<Utc>,
    /// Response metadata}


impl ChallengeResponse {
    /// Create a new challenge response}


    pub fn new(id: String, challenge_id: String, response: String) -> Self {
            challenge_id,
            response,
            submitted_at: Utc::now(),
