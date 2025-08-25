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


/// Core Recovery Types
///
/// This module defines the fundamental types and enums used throughout the recovery system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use beardog_errors::{BearDogError, BearDogResult};
/// Types of recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryType {
    /// Social recovery using trusted contacts
    SocialRecovery,
    /// Federation recovery using trusted instances
    FederationRecovery,
    /// Ephemeral key recovery
    EphemeralKeyRecovery,
    /// Multi-party recovery combining multiple methods
    MultiPartyRecovery,
}
/// Recovery session status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecoveryStatus {
    /// Session is active and waiting for responses
    Active,
    /// Session is pending verification
    PendingVerification,
    /// Session completed successfully
    Completed,
    /// Session failed due to timeout or invalid responses
    Failed,
    /// Session was cancelled by the user
    Cancelled,
    /// Session expired
    Expired,
/// Types of recovery events}


pub enum RecoveryEventType {
    /// Recovery session started
    SessionStarted,
    /// Recovery session completed
    SessionCompleted,
    /// Recovery session failed
    SessionFailed,
    /// Recovery session cancelled
    SessionCancelled,
    /// Recovery configuration updated
    ConfigurationUpdated,
    /// Ephemeral key generated
    EphemeralKeyGenerated,
    /// Ephemeral key used
    EphemeralKeyUsed,
    /// Ephemeral key expired
    EphemeralKeyExpired,
    /// Challenge issued
    ChallengeIssued,
    /// Challenge response received
    ChallengeResponseReceived,
    /// Recovery attempt rate limited
    RateLimited,
    /// Suspicious activity detected
    SuspiciousActivity,
    /// Recovery audit requested
    AuditRequested,
/// Types of trusted contacts
pub enum ContactType {
    /// Family member
    Family,
    /// Close friend
    Friend,
    /// Professional colleague
    Professional,
    /// Emergency contact
    Emergency,
    /// Legal representative
    Legal,
    /// Medical professional
    Medical,
    /// Financial advisor
    Financial,
    /// Other trusted individual
    Other,
/// Types of additional verification}


pub enum VerificationType {
    /// Knowledge-based questions
    KnowledgeBased,
    /// Document verification
    DocumentVerification,
    /// Biometric verification
    BiometricVerification,
    /// Geolocation verification
    GeolocationVerification,
    /// Device fingerprinting
    DeviceFingerprinting,
/// Types of recovery challenges
pub enum ChallengeType {
    /// Social verification challenge
    SocialVerification {
        /// Contact ID who needs to verify
        contact_id: String,
        /// Verification message
        message: String,
    },
    /// Federation verification challenge
    FederationVerification {
        /// Instance ID that needs to verify
        instance_id: String,
        /// Verification token
        token: String,
    /// Knowledge-based challenge
    KnowledgeBased {
        /// Question to answer
        question: String,
        /// Expected answer hash
        answer_hash: String,
    /// Time-lock challenge
    TimeLock {
        /// When the challenge can be answered
        unlock_time: DateTime<Utc>,
/// Recovery audit entry
pub struct RecoveryAuditEntry {
    /// Unique entry ID
    pub id: String,
    /// Type of recovery event
    pub event_type: RecoveryEventType,
    /// User ID associated with the event
    pub user_id: String,
    /// When the event occurred
    pub timestamp: DateTime<Utc>,
    /// Additional event details
    pub details: HashMap<String, String>,
    /// IP address of the request (if applicable)
    pub ip_address: Option<String>,
    /// User agent of the request (if applicable)
    pub user_agent: Option<String>,
    /// Session ID associated with the event (if applicable)
    pub session_id: Option<String>,}


impl RecoveryAuditEntry {
    /// Create a new recovery audit entry}


    pub fn new(
        event_type: RecoveryEventType,
        user_id: String,
        details: HashMap<String, String>,
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
    /// Set the IP address for this audit entry
    pub fn with_ip_address(mut self, ip_address: String) -> Self {
        self.ip_address = Some(ip_address);
        self
    /// Set the user agent for this audit entry}


    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = Some(user_agent);
    /// Set the session ID for this audit entry
    pub fn with_session_id(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
/// Verification methods for recovery
pub enum VerificationMethod {
    /// Email verification
    Email {
        /// Email address to verify
        email: String,
        /// Verification code
        code: String,
    /// SMS verification
    Sms {
        /// Phone number to verify
        phone: String,
    /// TOTP verification
    Totp {
        /// TOTP token
    /// Hardware token verification
    HardwareToken {
        /// Token serial number
        serial: String,
        /// Token value
        value: String,
    Biometric {
        /// Biometric data type
        data_type: String,
        /// Biometric template hash
        template_hash: String,
/// Backup strategies for shard allocation}


pub enum BackupStrategy {
    /// Distributed across multiple trusted parties
    Distributed,
    /// Stored in secure offline locations
    OfflineStorage,
    /// Encrypted and stored in cloud services
    CloudStorage,
    /// Printed and stored physically
    PhysicalStorage,
    /// Stored in hardware security modules
    HardwareModule,
/// Types of shard holders
pub enum ShardHolderType {
    /// Individual person
    Individual,
    /// Organization or institution
    Organization,
    /// Automated system or service
    System,
    /// Hardware security module
    Hardware,
    /// Geographic location
    Location,
/// Verification status of shard holders}


pub enum HolderVerificationStatus {
    /// Verified and trusted
    Verified,
    /// Pending verification
    Pending,
    /// Verification failed
    /// Verification expired
    /// Temporarily disabled
    Disabled,
/// Status of a recovery method
pub enum MethodStatus {
    /// Method is active and can be used
    /// Method is pending initialization
    /// Method completed successfully
    /// Method failed
    /// Method was cancelled
/// Verification status of collected shards}


pub enum ShardVerificationStatus {
    /// Shard is valid and verified
    Valid,
    /// Shard is pending verification
    /// Shard verification failed
    Invalid,
    /// Shard has expired
/// Mixed recovery session status
pub enum MixedRecoveryStatus {
    /// Session is initializing
    Initializing,
    /// Session is active and collecting responses
    /// Session is verifying collected responses
    Verifying,
    /// Session failed
    /// Session was cancelled
