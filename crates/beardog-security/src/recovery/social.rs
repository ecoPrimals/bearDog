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


/// Social Recovery
///
/// This module handles social recovery configurations and trusted contacts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::policies::*;
use super::types::*;
use beardog_errors::{BearDogError, BearDogResult};
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
pub struct TrustedContact {
    /// Contact ID
    pub id: String,
    /// Contact name
    pub name: String,
    /// Contact email
    pub email: String,
    /// Contact phone (optional)
    pub phone: Option<String>,
    /// Type of contact
    pub contact_type: ContactType,
    /// Trust level (0-100)
    pub trust_level: u8,
    /// When this contact was added
    pub added_at: DateTime<Utc>,
    /// Whether this contact is active
    pub active: bool,
    /// Last verification timestamp
    pub last_verified: Option<DateTime<Utc>>,
    /// Contact metadata
    pub metadata: HashMap<String, String>,}


impl TrustedContact {
    /// Create a new trusted contact}


    pub fn new(id: String, name: String, email: String, contact_type: ContactType) -> Self {
        Self {
            id,
            name,
            email,
            phone: None,
            contact_type,
            trust_level: 50,
            added_at: Utc::now(),
            active: true,
            last_verified: None,
            metadata: HashMap::new(),
        }
    }
    /// Update the trust level
    pub fn update_trust_level(&mut self, level: u8) {
        self.trust_level = level.min(100);
    /// Mark contact as verified}


    pub fn mark_verified(&mut self) {
        self.last_verified = Some(Utc::now());
impl Default for SocialRecoveryConfig {}


    fn default() -> Self {
            user_id: String::new(),
            trusted_contacts: Vec::new(),
            min_contacts_required: 2,
            recovery_window_hours: 24,
            enabled: false,
            policy: RecoveryPolicy::default(),
