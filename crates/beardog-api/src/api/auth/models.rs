//! Authentication API Models
//!
//! All request and response models for authentication, authorization,
//! multi-factor authentication, and user management APIs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ====== AUTHENTICATION MODELS ======

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub mfa_code: Option<String>,
    pub remember_device: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResponse {
    pub success: bool,
    pub user: AuthenticatedUser,
    pub tokens: AuthTokens,
    pub session: SessionInfo,
    pub mfa_required: bool,
    pub security_notices: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub user_id: String,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub mfa_enabled: bool,
    pub last_login: String,
    pub account_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub token_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub created_at: String,
    pub expires_at: String,
    pub client_ip: String,
    pub user_agent: String,
    pub device_fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutRequest {
    pub session_id: String,
    pub logout_all_devices: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutResponse {
    pub success: bool,
    pub session_id: String,
    pub logged_out_at: String,
    pub message: String,
    pub redirect_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub issued_at: String,
    pub scope: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenValidationRequest {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenValidationResponse {
    pub valid: bool,
    pub user_id: String,
    pub username: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub expires_at: String,
    pub issued_at: String,
    pub token_type: String,
    pub scope: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentSessionResponse {
    pub session_id: String,
    pub user_id: String,
    pub username: String,
    pub created_at: String,
    pub last_activity: String,
    pub expires_at: String,
    pub active_permissions: Vec<String>,
    pub session_status: String,
    pub concurrent_sessions: u32,
    pub session_metadata: HashMap<String, String>,
}

// ====== MFA MODELS ======

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupMfaRequest {
    pub user_id: String,
    pub mfa_method: String, // "totp", "sms", "email"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupMfaResponse {
    pub mfa_secret: String,
    pub qr_code_url: String,
    pub backup_codes: Vec<String>,
    pub setup_instructions: String,
    pub supported_methods: Vec<String>,
    pub verification_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyMfaRequest {
    pub user_id: String,
    pub mfa_method: String,
    pub mfa_code: String,
    pub trust_device: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyMfaResponse {
    pub verified: bool,
    pub mfa_method: String,
    pub verification_timestamp: String,
    pub remaining_backup_codes: Option<u32>,
    pub trust_device_token: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaStatusResponse {
    pub user_id: String,
    pub mfa_enabled: bool,
    pub enabled_methods: Vec<String>,
    pub primary_method: String,
    pub backup_codes_remaining: u32,
    pub trusted_devices: u32,
    pub last_mfa_verification: String,
    pub enforcement_policy: String,
}

// ====== USER MANAGEMENT MODELS ======

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<String>,
    pub role: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListResponse {
    pub users: Vec<UserSummary>,
    pub total_count: u32,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
    pub has_more: bool,
    pub filters_applied: UserListFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListFilters {
    pub status: Option<String>,
    pub role: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummary {
    pub user_id: String,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub status: String,
    pub roles: Vec<String>,
    pub created_at: String,
    pub last_login: Option<String>,
    pub mfa_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub temporary_password: Option<String>,
    pub roles: Option<Vec<String>>,
    pub send_welcome_email: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub status: String,
    pub created_at: String,
    pub verification_required: bool,
    pub verification_email_sent: bool,
    pub default_roles: Vec<String>,
    pub password_reset_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDetailsResponse {
    pub user_id: String,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub status: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub profile: UserProfile,
    pub security: UserSecurityInfo,
    pub activity: UserActivity,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub department: Option<String>,
    pub title: Option<String>,
    pub timezone: Option<String>,
    pub locale: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSecurityInfo {
    pub mfa_enabled: bool,
    pub password_last_changed: String,
    pub failed_login_attempts: u32,
    pub account_locked: bool,
    pub lock_reason: Option<String>,
    pub trusted_devices: u32,
    pub active_sessions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActivity {
    pub created_at: String,
    pub last_login: Option<String>,
    pub last_password_change: String,
    pub login_count: u32,
    pub password_change_count: u32,
}

// ====== AUTHORIZATION MODELS ======

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleListResponse {
    pub roles: Vec<RoleSummary>,
    pub total_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleSummary {
    pub role_id: String,
    pub name: String,
    pub description: String,
    pub permission_count: u32,
    pub user_count: u32,
    pub system_role: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionListQuery {
    pub category: Option<String>,
    pub resource: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionListResponse {
    pub permissions: Vec<PermissionSummary>,
    pub total_count: u32,
    pub categories: Vec<String>,
    pub filters_applied: PermissionListFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionListFilters {
    pub category: Option<String>,
    pub resource: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionSummary {
    pub permission_id: String,
    pub name: String,
    pub description: String,
    pub resource: String,
    pub action: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionCheckRequest {
    pub user_id: String,
    pub permission: String,
    pub context: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionCheckResponse {
    pub user_id: String,
    pub permission: String,
    pub granted: bool,
    pub reason: String,
    pub source: Option<String>,
    pub expires_at: Option<String>,
    pub context_restrictions: HashMap<String, String>,
}

// ====== API KEY MODELS ======

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyListResponse {
    pub api_keys: Vec<ApiKeySummary>,
    pub total_count: u32,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeySummary {
    pub key_id: String,
    pub name: String,
    pub prefix: String,
    pub scopes: Vec<String>,
    pub created_at: String,
    pub last_used: Option<String>,
    pub expires_at: Option<String>,
    pub status: String,
    pub usage_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub scopes: Vec<String>,
    pub expires_at: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyResponse {
    pub key_id: String,
    pub api_key: String,
    pub name: String,
    pub scopes: Vec<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub prefix: String,
    pub warning: String,
    pub usage_instructions: String,
}
