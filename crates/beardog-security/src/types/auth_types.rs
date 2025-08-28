

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use beardog_errors::BearDogError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subject {

    pub id: String,

    pub name: String,

    pub subject_type: SubjectType,

    pub roles: Vec<String>,

    pub clearance_level: Option<u32>,

    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SubjectType {

    User,

    Service,

    System,

    Device,

    Admin,

#[derive(Debug, Clone, Serialize, Deserialize)]}

pub struct AuthorizationResult {

    pub permitted: bool,

    pub authorized: bool,

    pub reason: String,

    pub risk_level: super::RiskLevel,

    pub additional_requirements: Vec<String>,

    pub expires_at: Option<DateTime<Utc>>,

    pub audit_id: String,

pub struct AuthenticationResult {

    pub success: bool,

    pub authenticated: bool,

    pub user_id: Option<String>,

    pub user: Option<UserInfo>,

    pub session_id: Option<String>,

    pub session_token: Option<SessionToken>,

    pub mfa_required: bool,

    pub mfa_methods: Vec<MfaMethod>,

    pub error: Option<String>,

pub struct UserInfo {

    pub user_id: String,

    pub username: String,

    pub email: String,

    pub full_name: String,

    pub permissions: Vec<String>,

    pub status: AccountStatus,

    pub last_login: Option<DateTime<Utc>>,

pub enum AccountStatus {

    Active,

    Suspended,

    Locked,

    Disabled,

    Pending,

pub enum MfaMethod {

    Totp,

    Sms,

    Email,

pub struct MfaToken {

    pub method: MfaMethod,

    pub token: String,

    pub expires_at: DateTime<Utc>,

    pub used: bool,

pub struct SessionToken {

    pub session_id: String,

    pub expires_at: chrono::DateTime<Utc>,

    pub token_type: String,

pub struct Session {

    pub user_info: UserInfo,

    pub created_at: DateTime<Utc>,

    pub is_active: bool,

    pub last_activity: DateTime<Utc>,

pub struct SecuritySession {

    pub client_ip: Option<String>,

    pub user_agent: Option<String>,

pub struct MfaTokenEntry {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub attempts: u32,
    pub max_attempts: u32,

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SessionStore {

    sessions: HashMap<String, Session>,

    pub mfa_tokens: HashMap<String, MfaTokenEntry>,}

impl SessionStore {}

    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, session_id: &str, session: Session) {
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityContext {

    pub timestamp: Option<DateTime<Utc>>,

    pub method: Option<String>,

    pub path: Option<String>,

    pub metadata: HashMap<String, String>,
}

pub use beardog_types::canonical::configuration::security::{MfaConfig, SessionConfig};
