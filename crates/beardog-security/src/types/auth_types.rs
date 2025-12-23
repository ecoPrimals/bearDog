

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use beardog_errors::BearDogError;

#[derive(Debug, Clone)]
    /// Name of the item
    pub name: String,

    /// The subject type value
    pub subject_type: SubjectType,

    /// Collection of roles
    pub roles: Vec<String>,

    /// Optional clearance level
    pub clearance_level: Option<u32>,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
    /// Whether authorized is enabled
    pub authorized: bool,

    /// The reason value
    pub reason: String,

    /// The risk level value
    pub risk_level: super::RiskLevel,

    /// Collection of additional requirements
    pub additional_requirements: Vec<String>,

    /// Optional expires at
    pub expires_at: Option<DateTime<Utc>>,


    pub audit_id: String,

pub struct AuthenticationResult {

    /// Whether success is enabled
    pub success: bool,

    /// Whether authenticated is enabled
    pub authenticated: bool,


    pub user_id: Option<String>,

    /// Optional user
    pub user: Option<UserInfo>,


    pub session_id: Option<String>,

    /// Optional session token
    pub session_token: Option<SessionToken>,

    /// Whether mfa_required is enabled
    pub mfa_required: bool,

    /// Collection of mfa methods
    pub mfa_methods: Vec<MfaMethod>,

    /// Optional error
    pub error: Option<String>,

pub struct UserInfo {


    pub user_id: String,

    /// Name of the useritem
    pub username: String,

    /// The email value
    pub email: String,

    /// Name of the full
    pub full_name: String,

    /// Collection of permissions
    pub permissions: Vec<String>,

    /// Current status of the component
    pub status: AccountStatus,

    /// Optional last login
    pub last_login: Option<DateTime<Utc>>,

pub enum AccountStatus {


    /// Active or enabled state
    Active,


    /// State indicating suspended
    Suspended,


    /// State indicating locked
    Locked,


    /// Inactive or disabled state
    Disabled,


    /// Operation in progress
    Pending,

pub enum MfaMethod {


    /// Represents totp variant
    Totp,


    /// Represents sms variant
    Sms,


    /// Represents email variant
    Email,

pub struct MfaToken {

    /// The method value
    pub method: MfaMethod,

    /// The token value
    pub token: String,

    /// The expires at value
    pub expires_at: DateTime<Utc>,

    /// Whether used is enabled
    pub used: bool,

pub struct SessionToken {


    pub session_id: String,

    /// The expires at value
    pub expires_at: chrono::DateTime<Utc>,

    /// The token type value
    pub token_type: String,

pub struct Session {

    /// The user info value
    pub user_info: UserInfo,

    /// The created at value
    pub created_at: DateTime<Utc>,

    /// Whether is_active is enabled
    pub is_active: bool,

    /// The last activity value
    pub last_activity: DateTime<Utc>,

pub struct SecuritySession {

    /// Optional client ip
    pub client_ip: Option<String>,

    /// Optional user agent
    pub user_agent: Option<String>,

pub struct MfaTokenEntry {
    /// The created at value
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The expires at value
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// Number of attempts
    pub attempts: u32,
    /// Number of max_attempts
    pub max_attempts: u32,

#[derive(HashMap<String, Session>,

    /// Mapping of mfa tokens
    pub mfa_tokens: HashMap<String, MfaTokenEntry>,}
    pub mfa_tokens: HashMap<String, MfaTokenEntry>,}
    pub mfa_tokens: HashMap<String, MfaTokenEntry>,}

impl SessionStore {}

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default(&str, session: Session) {
        self.sessions.insert(session_id, session);}

/// Get operation.
    /// Gets value
    /// Gets value
    pub fn get(&self, session_id: &str) -> Option<&Session> {
        self.sessions.get(session_id)
/// Get Mut operation.
    /// Gets mut
    /// Returns mutable reference to get
    pub fn get_mut(&mut self, session_id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(session_id)}

/// Remove operation.
    /// Removes item
    /// Removes item
    pub fn remove(&mut self, session_id: &str) -> Option<Session> {
        self.sessions.remove(session_id)
/// Len operation.
    pub fn len(&self) -> usize {
        self.sessions.len()}

/// Is Empty operation.
    /// Checks if empty
    /// Checks if empty
    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
/// Capacity operation.
    pub fn capacity(&self) -> usize {
        self.sessions.capacity()}

/// Shrink To Fit operation.
    pub fn shrink_to_fit(&mut self) {
        self.sessions.shrink_to_fit();
/// Retain operation.
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(Option<DateTime<Utc>>,

    /// Optional method
    pub method: Option<String>,

    /// Optional path
    pub path: Option<String>,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

pub use beardog_types::canonical::configuration::security::{MfaConfig, SessionConfig};
