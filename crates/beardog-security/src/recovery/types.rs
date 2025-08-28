

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use beardog_errors::BearDogError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryType {

    SocialRecovery,

    FederationRecovery,

    EphemeralKeyRecovery,

    MultiPartyRecovery,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecoveryStatus {

    Active,

    PendingVerification,

    Completed,

    Failed,

    Cancelled,

    Expired,

pub enum RecoveryEventType {

    SessionStarted,

    SessionCompleted,

    SessionFailed,

    SessionCancelled,

    ConfigurationUpdated,

    EphemeralKeyGenerated,

    EphemeralKeyUsed,

    EphemeralKeyExpired,

    ChallengeIssued,

    ChallengeResponseReceived,

    RateLimited,

    SuspiciousActivity,

    AuditRequested,

pub enum ContactType {

    Family,

    Friend,

    Professional,

    Emergency,

    Legal,

    Medical,

    Financial,

    Other,

pub enum VerificationType {

    KnowledgeBased,

    DocumentVerification,

    BiometricVerification,

    GeolocationVerification,

    DeviceFingerprinting,

pub enum ChallengeType {

    SocialVerification {

        contact_id: String,

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

    pub event_type: RecoveryEventType,

    pub user_id: String,

    pub timestamp: DateTime<Utc>,

    pub details: HashMap<String, String>,

    pub ip_address: Option<String>,

    pub user_agent: Option<String>,

    pub session_id: Option<String>,}

impl RecoveryAuditEntry {

    pub fn new(
        event_type: RecoveryEventType,
        user_id: &str,
        details: HashMap<&str, &str>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            event_type,
            user_id,
            timestamp: Utc::now(),
            details,
            ip_address: None,
            user_agent: None,
            session_id: None,
        }
    }

    pub fn with_ip_address(mut self, ip_address: &str) -> Self {
        self.ip_address = Some(ip_address);
        self

    pub fn with_user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent);

    pub fn with_session_id(mut self, session_id: &str) -> Self {
        self.session_id = Some(session_id);

pub enum VerificationMethod {

    Email {

        email: String,

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

    Distributed,

    OfflineStorage,

    CloudStorage,

    PhysicalStorage,

    HardwareModule,

pub enum ShardHolderType {

    Individual,

    Organization,

    System,

    Hardware,

    Location,

pub enum HolderVerificationStatus {

    Verified,

    Pending,

    Disabled,

pub enum MethodStatus {

pub enum ShardVerificationStatus {

    Valid,

    Invalid,

pub enum MixedRecoveryStatus {

    Initializing,

    Verifying,

