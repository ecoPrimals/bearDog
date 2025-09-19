

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use beardog_errors::BearDogError;

#[derive(Debug, Clone)]
        message: String,
    },

    FederationVerification {

        instance_id: String,

        token: String,

    KnowledgeBased {

        question: String,

        answer_hash: String,

    TimeLock {

        unlock_time: DateTime<Utc>,

pub struct RecoveryAuditEntry {


    pub id: String,

    /// The event type value
    pub event_type: RecoveryEventType,


    pub user_id: String,


    pub timestamp: DateTime<Utc>,

    /// Mapping of details
    pub details: HashMap<String, String>,

    /// Optional ip address
    pub ip_address: Option<String>,

    /// Optional user agent
    pub user_agent: Option<String>,


    pub session_id: Option<String>,}

impl RecoveryAuditEntry {

/// New operation.
    /// Creates a new instance
    pub fn new(RecoveryEventType,
        user_id: &str,
        details: HashMap<&str, &str>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: event_type.to_string(),
            user_id,
            timestamp: Utc::now(None,
            user_agent: None,
            session_id: None,
        }
    }

/// With Ip Address operation.
    /// Creates instance with ip address
    pub fn with_ip_address(mut self, ip_address: &str) -> Self {
        self.ip_address = Some(ip_address);
        self

/// With User Agent operation.
    /// Creates instance with user agent
    pub fn with_user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent);

/// With Session Id operation.
    /// Creates instance with session id
    pub fn with_session_id(mut self, session_id: &str) -> Self {
        self.session_id = Some(String,

        code: String,

    Sms {

        phone: String,

    Totp {

    HardwareToken {

        serial: String,

        value: String,
    Biometric {

        data_type: String,

        template_hash: String,

pub enum BackupStrategy {


    /// State indicating distributed
    Distributed,


    /// Represents offline storage variant
    OfflineStorage,


    /// Represents cloud storage variant
    CloudStorage,


    /// Represents physical storage variant
    PhysicalStorage,


    /// Represents hardware module variant
    HardwareModule,
/// Types of shard holder
pub enum ShardHolderType {


    /// Represents individual variant
    Individual,


    /// Represents organization variant
    Organization,


    /// Represents system variant
    System,


    /// Represents hardware variant
    Hardware,


    /// Represents location variant
    Location,

pub enum HolderVerificationStatus {


    /// State indicating verified
    Verified,


    /// Operation in progress
    Pending,


    /// Inactive or disabled state
    Disabled,

pub enum MethodStatus {

pub enum ShardVerificationStatus {


    /// Represents valid variant
    Valid,


    /// Represents invalid variant
    Invalid,

pub enum MixedRecoveryStatus {


    /// Currently initializing
    Initializing,


    /// Currently verifying
    Verifying,

