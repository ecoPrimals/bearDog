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


/// Security Types Module
///
/// **CANONICAL SECURITY TYPES** - Unified security types for all BearDog operations
/// This module provides the core security types that form the foundation of
/// BearDog's security system, including configurations, providers, and data structures.

use beardog_errors::{BearDogError, BearDogResult, SecurityError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
// Import sub-modules
pub mod audit_types;
pub mod auth_types;
pub mod config_types;
pub mod crypto_types;
// Re-export types from sub-modules - avoid duplicates
pub use audit_types::*;
pub use crypto_types::*;
// Only use specific items from auth_types and config_types to avoid conflicts
pub use auth_types::{UserInfo};
pub use config_types::{UnifiedSecurityConfig, BasicSecurityConfig, CryptoOptimizationConfig, AuthenticationConfig, AuthorizationConfig, HsmSettingsConfig, RateLimitConfig as ConfigRateLimitConfig, EncryptionConfig};
// Core security types
// SecurityProviderConfig moved to beardog-types::security - use that instead
pub use beardog_types::security::SecurityProviderConfig;


    fn default() -> Self {
        Self {
            max_failed_attempts: 5,
            lockout_duration_minutes: 30,
            session_timeout_minutes: 60,
            enable_audit_logging: true,
            require_mfa: false,
        }
    }
// Rate limiting configuration - USE CANONICAL VERSION
// Re-export from canonical security configuration
pub use beardog_types::canonical::configuration::security::RateLimitConfig;


// Default implementation now provided by canonical type
// Security provider implementation


#[derive(Debug, Clone)]
pub struct BearDogSecurityProvider {
    pub config: SecurityProviderConfig,
    pub rate_limiter: RateLimiter,
    pub security_rules: SecurityRules,
    pub locked_accounts: Arc<RwLock<HashMap<String, DateTime<Utc>>>>,
    pub failed_attempts: Arc<RwLock<HashMap<String, u32>>>,
    pub metrics: SecurityProviderMetrics,
    pub session_store: SessionStore,
    pub audit_manager: AuditManager,
}


pub struct RateLimiter {
    pub config: RateLimitConfig,
    pub state: HashMap<String, RateLimiterState>,
pub struct RateLimiterState {
    pub requests: u32,
    pub last_reset: DateTime<Utc>,
}


pub struct SecurityRules {
    pub rules: Vec<SecurityRule>,
    pub default_policy: PolicyDecision,
    pub context: SecurityContext,
pub struct SecurityRule {
    pub id: String,
    pub condition: String,
    pub action: PolicyDecision,
}


pub enum PolicyDecision {
    Allow,
    Deny,
    RequireAdditionalAuth,}


pub struct SecurityContext {
    pub user_id: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub timestamp: DateTime<Utc>,}


impl Default for SecurityContext {
            user_id: None,
            ip_address: None,
            user_agent: None,
            timestamp: Utc::now(),}


#[derive(Debug, Clone, Default)]
pub struct SecurityProviderMetrics {
    pub successful_authentications: u64,
    pub failed_authentications: u64,
    pub active_sessions: u64,
    pub blocked_requests: u64,
    pub total_authentications: u64,
    pub successful_authorizations: u64,
    pub failed_authorizations: u64,
    pub auth_success_rate: f64,
    pub authz_success_rate: f64,
    pub avg_response_time_ms: f64,
    pub requests_per_second: f64,
    pub error_rate: f64,
    pub total_sessions_created: u64,
    pub mfa_tokens_generated: u64,
    pub mfa_verifications_successful: u64,
    pub mfa_verifications_failed: u64,
    pub uptime_seconds: u64,
    pub low_risk_operations: u64,
    pub medium_risk_operations: u64,
    pub high_risk_operations: u64,
    pub critical_risk_operations: u64,
    pub collected_at: DateTime<Utc>,
}


pub struct SessionStore {
    pub sessions: Arc<RwLock<HashMap<String, SecuritySession>>>,}


impl SessionStore {}


    pub fn new() -> Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),}


    pub async fn len(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.len()
impl Default for SessionStore {
        Self::new()}


pub struct AuditManager {
    pub events: Arc<RwLock<Vec<SecurityAuditEvent>>>,}


impl AuditManager {
            events: Arc::new(RwLock::new(Vec::new())),}


impl Default for AuditManager {
pub struct SecuritySession {
    pub session_id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
}


pub struct SecurityAuditEvent {
    pub event_id: String,
    pub event_type: String,
    pub details: HashMap<String, String>,
pub struct AuthenticationResult {
    pub success: bool,
    pub session_id: Option<String>,
    pub message: String,
}


pub struct AuthorizationResult {
    pub authorized: bool,
    pub permissions: Vec<String>,
pub struct SecurityProviderHealth {
    pub status: String,
    pub last_check: DateTime<Utc>,}


impl Default for BearDogSecurityProvider {
            config: SecurityProviderConfig::default(),
            rate_limiter: RateLimiter::default(),
            security_rules: SecurityRules::default(),
            locked_accounts: Arc::new(RwLock::new(HashMap::new())),
            failed_attempts: Arc::new(RwLock::new(HashMap::new())),
            metrics: SecurityProviderMetrics::default(),
            session_store: SessionStore::default(),
            audit_manager: AuditManager::default(),}


impl Default for RateLimiter {
            config: RateLimitConfig::default(),
            state: HashMap::new(),
impl Default for SecurityRules {
            rules: Vec::new(),
            default_policy: PolicyDecision::Deny,
            context: SecurityContext::default(),
// MIGRATION COMPLETE: Removed conversion from BearDogError to SecurityError
// All functions now use BearDogError directly for unified error handling
