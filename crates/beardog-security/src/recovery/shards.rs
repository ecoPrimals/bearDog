

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::types::*;
use beardog_errors::{BearDogError, BearDogResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryShard {

    pub id: String,

    pub user_id: String,

    pub index: u32,

    pub encrypted_data: String,

    pub holder: ShardHolderInfo,

    pub created_at: DateTime<Utc>,

    pub expires_at: DateTime<Utc>,

    pub usage_restrictions: ShardUsageRestrictions,

    pub metadata: HashMap<String, String>,
}

pub struct ShardHolderInfo {

    pub name: String,

    pub holder_type: ShardHolderType,

    pub verification_status: HolderVerificationStatus,

    pub contact_info: String,

    pub trust_level: u8,

    pub last_verified: Option<DateTime<Utc>>,

pub struct ShardUsageRestrictions {

    pub max_uses: u32,

    pub times_used: u32,

    pub time_window: TimeWindow,

    pub require_verification: bool,

    pub allowed_verification_methods: Vec<VerificationMethod>,

pub struct TimeWindow {

    pub start: DateTime<Utc>,

    pub end: DateTime<Utc>,

    pub allowed_days: Vec<u8>, // 0=Sunday, 6=Saturday

    pub allowed_hours: Vec<u8>, // 0-23}

impl RecoveryShard {

    pub fn new(
        id: &str,
        user_id: &str,
        index: u32,
        encrypted_data: &str,
        holder: ShardHolderInfo,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            user_id,
            index,
            encrypted_data,
            holder,
            created_at: now,
            expires_at: now + chrono::Duration::days(365), // 1 year expiry
            usage_restrictions: ShardUsageRestrictions::default(),
            metadata: HashMap::with_capacity(16),
        }
    }

    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at

    pub fn can_use(&self) -> bool {
        self.is_valid() && self.usage_restrictions.times_used < self.usage_restrictions.max_uses

    pub fn mark_used(&mut self) {
        self.usage_restrictions.times_used += 1;
impl Default for ShardUsageRestrictions {}

    fn default() -> Self {
            max_uses: 10,
            times_used: 0,
            time_window: TimeWindow::default(),
            require_verification: false,
            allowed_verification_methods: Vec::new(),
impl Default for TimeWindow {
            start: now,
            end: now + chrono::Duration::days(365),
            allowed_days: vec![0, 1, 2, 3, 4, 5, 6], // All days
            allowed_hours: (0..24).collect(),        // All hours

pub struct KeyWorthinessDemo {

    pub scenario: String,

    pub key_info: HashMap<String, String>,

    pub results: Vec<String>,

impl KeyWorthinessDemo {

    pub fn new(id: &str, user_id: &str, scenario: &str) -> Self {
            scenario,
            key_info: HashMap::with_capacity(16),
            results: Vec::new(),
            created_at: Utc::now(),

    pub fn add_result(&mut self, result: &str) {
        self.results.push(result);
