

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::policies::*;
use super::types::*;
use beardog_errors::BearDogError;

pub struct TrustedContact {

    pub id: String,

    pub name: String,

    pub email: String,

    pub phone: Option<String>,

    pub contact_type: ContactType,

    pub trust_level: u8,

    pub added_at: DateTime<Utc>,

    pub active: bool,

    pub last_verified: Option<DateTime<Utc>>,

    pub metadata: HashMap<String, String>,}

impl TrustedContact {

    pub fn new(id: &str, name: &str, email: &str, contact_type: ContactType) -> Self {
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
            metadata: HashMap::with_capacity(16),
        }
    }

    pub fn update_trust_level(&mut self, level: u8) {
        self.trust_level = level.min(100);

    pub fn mark_verified(&mut self) {
        self.last_verified = Some(Utc::now());
impl Default for SocialRecoveryConfig {}

    fn default() -> Self {
            user_id: String::with_capacity(64),
            trusted_contacts: Vec::new(),
            min_contacts_required: 2,
            recovery_window_hours: 24,
            enabled: false,
            policy: RecoveryPolicy::default(),
