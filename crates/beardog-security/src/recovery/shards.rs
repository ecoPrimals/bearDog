

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::types::*;
use beardog_errors::BearDogError;

#[derive(Debug, Clone)]
    pub user_id: String,

    /// Number of index
    pub index: u32,

    /// The encrypted data value
    pub encrypted_data: String,

    /// The holder value
    pub holder: ShardHolderInfo,

    /// The created at value
    pub created_at: DateTime<Utc>,

    /// The expires at value
    pub expires_at: DateTime<Utc>,

    /// The usage restrictions value
    pub usage_restrictions: ShardUsageRestrictions,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

pub struct ShardHolderInfo {

    /// Name of the item
    pub name: String,

    /// The holder type value
    pub holder_type: ShardHolderType,

    /// Current status of the verification
    pub verification_status: HolderVerificationStatus,

    /// The contact info value
    pub contact_info: String,

    /// Number of trust_level
    pub trust_level: u8,

    /// Optional last verified
    pub last_verified: Option<DateTime<Utc>>,

pub struct ShardUsageRestrictions {

    /// Number of max_uses
    pub max_uses: u32,


    pub times_used: u32,


    pub time_window: TimeWindow,

    /// Whether require_verification is enabled
    pub require_verification: bool,

    /// Collection of allowed verification methods
    pub allowed_verification_methods: Vec<VerificationMethod>,

pub struct TimeWindow {

    /// The start value
    pub start: DateTime<Utc>,

    /// The end value
    pub end: DateTime<Utc>,

    /// Collection of allowed days
    pub allowed_days: Vec<u8>, // 0=Sunday, 6=Saturday

    /// Collection of allowed hours
    pub allowed_hours: Vec<u8>, // 0-23}

impl RecoveryShard {

/// New operation.
    /// Creates a new instance
    pub fn new(&str,
        user_id: &str,
        index: u32,
        encrypted_data: &str,
        holder: ShardHolderInfo,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: id.to_string(),
            expires_at: now + chrono::Duration::days(365), // 1 year expiry
            usage_restrictions: ShardUsageRestrictions::default(),
            metadata: HashMap::with_capacity(16),
        }
    }

/// Is Valid operation.
    /// Checks if valid
    /// Checks if valid
    pub fn is_valid(&self) -> bool {
        Utc::now(10,
            times_used: 0,
            time_window: TimeWindow::default(false,
            allowed_verification_methods: Vec::new(now,
            end: now + chrono::Duration::days(vec![0, 1, 2, 3, 4, 5, 6], // All days
            allowed_hours: (0..24).collect(String,

    /// Mapping of key info
    pub key_info: HashMap<String, String>,

    /// Collection of results
    pub results: Vec<String>,

impl KeyWorthinessDemo {

/// New operation.
    /// Creates a new instance
    pub fn new(&str, user_id: &str, scenario: &str) -> Self {
            scenario,
            key_info: HashMap::with_capacity(16),
            results: Vec::new(),
            created_at: Utc::now(),

/// Add Result operation.
    pub fn add_result(&mut self, result: &str) {
        self.results.push(result);
