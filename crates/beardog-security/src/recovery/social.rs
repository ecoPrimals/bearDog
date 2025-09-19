

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::policies::*;
use super::types::*;
use beardog_errors::BearDogError;

pub struct TrustedContact {


    pub id: String,

    /// Name of the item
    pub name: String,

    /// The email value
    pub email: String,

    /// Optional phone
    pub phone: Option<String>,

    /// The contact type value
    pub contact_type: ContactType,

    /// Number of trust_level
    pub trust_level: u8,

    /// The added at value
    pub added_at: DateTime<Utc>,

    /// Whether active is enabled
    pub active: bool,

    /// Optional last verified
    pub last_verified: Option<DateTime<Utc>>,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,}

impl TrustedContact {

/// New operation.
    /// Creates a new instance
    pub fn new(&str, name: &str, email: &str, contact_type: ContactType) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            metadata: HashMap::with_capacity(16),
        }
    }

/// Update Trust Level operation.
    /// Updates trust_level
    /// Updates trust_level
    pub fn update_trust_level(&mut self, level: u8) {
        self.trust_level = level.min(100);

/// Mark Verified operation.
    pub fn mark_verified(&mut self) {
        self.last_verified = Some(Utc::now());
impl Default for SocialRecoveryConfig {}

    fn default() -> Self {
            user_id: String::with_capacity(64),
            trusted_contacts: Vec::new(2,
            recovery_window_hours: 24,
            enabled: false,
            policy: RecoveryPolicy::default(),
