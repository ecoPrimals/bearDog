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


/// Authentication and Authorization Types
///
/// This module contains all types related to user authentication, authorization,
/// multi-factor authentication, and session management.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use beardog_errors::{BearDogError, BearDogResult};
/// Security subject (user or system) performing actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subject {
    /// Unique identifier for the subject
    pub id: String,
    /// Display name or username
    pub name: String,
    /// Type of subject (User, Service, etc.)
    pub subject_type: SubjectType,
    /// Subject roles
    pub roles: Vec<String>,
    /// Security clearance level
    pub clearance_level: Option<u32>,
    /// Additional metadata about the subject
    pub metadata: HashMap<String, String>,
}
/// Types of security subjects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SubjectType {
    /// Human user
    User,
    /// Service account or system
    Service,
    /// System account
    System,
    /// Device account
    Device,
    /// Administrative account
    Admin,
/// Authorization result containing decision and context
#[derive(Debug, Clone, Serialize, Deserialize)]}


pub struct AuthorizationResult {
    /// Whether the authorization was granted
    pub permitted: bool,
    /// Whether user is authorized (for compatibility)
    pub authorized: bool,
    /// Reason for the authorization decision
    pub reason: String,
    /// Risk level assessed for this authorization
    pub risk_level: super::RiskLevel,
    /// Additional requirements that must be met
    pub additional_requirements: Vec<String>,
    /// When this authorization expires
    pub expires_at: Option<DateTime<Utc>>,
    /// Unique identifier for audit trail
    pub audit_id: String,
/// Authentication result from security provider
pub struct AuthenticationResult {
    /// Whether authentication was successful
    pub success: bool,
    /// Whether user is authenticated (for compatibility)
    pub authenticated: bool,
    /// Authenticated user ID
    pub user_id: Option<String>,
    /// User information (for compatibility)
    pub user: Option<UserInfo>,
    /// Session ID if authentication successful
    pub session_id: Option<String>,
    /// Session token if created
    pub session_token: Option<SessionToken>,
    /// Whether MFA is required
    pub mfa_required: bool,
    /// Available MFA methods
    pub mfa_methods: Vec<MfaMethod>,
    /// Reason for authentication result
    /// Error message if authentication failed
    pub error: Option<String>,
    /// When the authentication expires
/// User information structure (updated with needed fields)
pub struct UserInfo {
    /// Unique user identifier
    /// Unique user identifier (for compatibility)
    pub user_id: String,
    /// Username
    pub username: String,
    /// User's email address
    pub email: String,
    /// User's full name
    pub full_name: String,
    /// User roles
    /// User permissions
    pub permissions: Vec<String>,
    /// Account status
    pub status: AccountStatus,
    /// Last login timestamp
    pub last_login: Option<DateTime<Utc>>,
/// Account status for user accounts
pub enum AccountStatus {
    /// Account is active and can be used
    Active,
    /// Account is suspended temporarily
    Suspended,
    /// Account is locked due to security concerns
    Locked,
    /// Account has been disabled
    Disabled,
    /// Account is pending activation
    Pending,
/// Multi-factor authentication method types}


pub enum MfaMethod {
    /// Time-based One-Time Password (TOTP)
    Totp,
    /// SMS-based token
    Sms,
    /// Email-based token
    Email,
/// MFA token information
pub struct MfaToken {
    /// Token identifier
    /// MFA method used
    pub method: MfaMethod,
    /// Token value (encrypted)
    pub token: String,
    /// When the token expires
    pub expires_at: DateTime<Utc>,
    /// Whether the token has been used
    pub used: bool,
    /// User ID associated with this token
/// Session token returned after session creation
pub struct SessionToken {
    /// Unique session identifier
    pub session_id: String,
    /// Token value (could be JWT or random string)
    /// Token expiration time
    pub expires_at: chrono::DateTime<Utc>,
    /// Token type (e.g., "Bearer")
    pub token_type: String,
/// User session information
pub struct Session {
    /// Session identifier
    /// Associated user ID
    /// User information
    pub user_info: UserInfo,
    /// Session creation time
    pub created_at: DateTime<Utc>,
    /// Session expiration time
    /// Whether the session is active
    pub is_active: bool,
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    /// User permissions for this session
    /// Additional session metadata
/// Security session (compatibility with existing interfaces)
pub struct SecuritySession {
    /// User identifier
    /// Whether session is active
    /// Client IP address
    pub client_ip: Option<String>,
    /// User agent string
    pub user_agent: Option<String>,
/// MFA token entry for internal storage
pub struct MfaTokenEntry {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub attempts: u32,
    pub max_attempts: u32,
/// Session storage for managing active sessions
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SessionStore {
    /// Active sessions
    sessions: HashMap<String, Session>,
    /// MFA tokens
    pub mfa_tokens: HashMap<String, MfaTokenEntry>,}


impl SessionStore {}


    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, session_id: String, session: Session) {
        self.sessions.insert(session_id, session);}


    pub fn get(&self, session_id: &str) -> Option<&Session> {
        self.sessions.get(session_id)
    pub fn get_mut(&mut self, session_id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(session_id)}


    pub fn remove(&mut self, session_id: &str) -> Option<Session> {
        self.sessions.remove(session_id)
    pub fn len(&self) -> usize {
        self.sessions.len()}


    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    pub fn capacity(&self) -> usize {
        self.sessions.capacity()}


    pub fn shrink_to_fit(&mut self) {
        self.sessions.shrink_to_fit();
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&String, &mut Session) -> bool,
    {
        self.sessions.retain(f);}


    pub fn iter(&self) -> impl Iterator<Item = (&String, &Session)> {
        self.sessions.iter()}


    pub fn values(&self) -> impl Iterator<Item = &Session> {
        self.sessions.values()
/// Security context for authorization decisions}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityContext {
    /// Current timestamp
    pub timestamp: Option<DateTime<Utc>>,
    /// User agent
    /// Request method
    pub method: Option<String>,
    /// Request path
    pub path: Option<String>,
    /// Additional context metadata
    pub metadata: HashMap<String, String>,
}

// MFA and Session configurations - USE CANONICAL VERSIONS
// Re-export from canonical security configuration
pub use beardog_types::canonical::configuration::security::{MfaConfig, SessionConfig};
