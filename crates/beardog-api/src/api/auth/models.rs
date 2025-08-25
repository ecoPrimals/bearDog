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


/// Authentication API Models
///
/// All request and response models for authentication, authorization,
/// multi-factor authentication, and user management APIs.

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
pub struct AuthenticationResponse {
    pub success: bool,
    pub user: AuthenticatedUser,
    pub tokens: AuthTokens,
    pub session: SessionInfo,
    pub mfa_required: bool,
    pub security_notices: Vec<String>,
}


pub struct AuthenticatedUser {
    pub user_id: String,
    pub display_name: String,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub mfa_enabled: bool,
    pub last_login: String,
    pub account_status: String,
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub token_type: String,
}


pub struct SessionInfo {
    pub session_id: String,
    pub created_at: String,
    pub expires_at: String,
    pub client_ip: String,
    pub user_agent: String,
    pub device_fingerprint: String,
pub struct LogoutRequest {
    pub logout_all_devices: Option<bool>,
}


pub struct LogoutResponse {
    pub logged_out_at: String,
    pub message: String,
    pub redirect_url: Option<String>,
pub struct RefreshTokenRequest {
pub struct RefreshTokenResponse {
    pub issued_at: String,
    pub scope: Vec<String>,
}


pub struct TokenValidationRequest {
    pub token: String,
pub struct TokenValidationResponse {
    pub valid: bool,
    pub warnings: Vec<String>,
}


pub struct CurrentSessionResponse {
    pub last_activity: String,
    pub active_permissions: Vec<String>,
    pub session_status: String,
    pub concurrent_sessions: u32,
    pub session_metadata: HashMap<String, String>,
// ====== MFA MODELS ======
pub struct SetupMfaRequest {
    pub mfa_method: String, // "totp", "sms", "email"
pub struct SetupMfaResponse {
    pub mfa_secret: String,
    pub qr_code_url: String,
    pub backup_codes: Vec<String>,
    pub setup_instructions: String,
    pub supported_methods: Vec<String>,
    pub verification_required: bool,
}


pub struct VerifyMfaRequest {
    pub mfa_method: String,
    pub mfa_code: String,
    pub trust_device: Option<bool>,
pub struct VerifyMfaResponse {
    pub verified: bool,
    pub verification_timestamp: String,
    pub remaining_backup_codes: Option<u32>,
    pub trust_device_token: Option<String>,
}


pub struct MfaStatusResponse {
    pub enabled_methods: Vec<String>,
    pub primary_method: String,
    pub backup_codes_remaining: u32,
    pub trusted_devices: u32,
    pub last_mfa_verification: String,
    pub enforcement_policy: String,
// ====== USER MANAGEMENT MODELS ======
pub struct UserListQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<String>,
    pub role: Option<String>,
    pub search: Option<String>,
}


pub struct UserListResponse {
    pub users: Vec<UserSummary>,
    pub total_count: u32,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
    pub has_more: bool,
    pub filters_applied: UserListFilters,
pub struct UserListFilters {
pub struct UserSummary {
    pub status: String,
    pub last_login: Option<String>,
}


pub struct CreateUserRequest {
    pub temporary_password: Option<String>,
    pub roles: Option<Vec<String>>,
    pub send_welcome_email: Option<bool>,
pub struct CreateUserResponse {
    pub verification_email_sent: bool,
    pub default_roles: Vec<String>,
    pub password_reset_required: bool,
}


pub struct UserDetailsResponse {
    pub profile: UserProfile,
    pub security: UserSecurityInfo,
    pub activity: UserActivity,
    pub metadata: HashMap<String, String>,
pub struct UserProfile {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub department: Option<String>,
    pub title: Option<String>,
    pub timezone: Option<String>,
    pub locale: Option<String>,
}


pub struct UserSecurityInfo {
    pub password_last_changed: String,
    pub failed_login_attempts: u32,
    pub account_locked: bool,
    pub lock_reason: Option<String>,
    pub active_sessions: u32,
pub struct UserActivity {
    pub last_password_change: String,
    pub login_count: u32,
    pub password_change_count: u32,
// ====== AUTHORIZATION MODELS ======
pub struct RoleListResponse {
    pub roles: Vec<RoleSummary>,
}


pub struct RoleSummary {
    pub role_id: String,
    pub name: String,
    pub description: String,
    pub permission_count: u32,
    pub user_count: u32,
    pub system_role: bool,
pub struct PermissionListQuery {
    pub category: Option<String>,
    pub resource: Option<String>,
}


pub struct PermissionListResponse {
    pub permissions: Vec<PermissionSummary>,
    pub categories: Vec<String>,
    pub filters_applied: PermissionListFilters,
pub struct PermissionListFilters {
pub struct PermissionSummary {
    pub permission_id: String,
    pub resource: String,
    pub action: String,
    pub category: String,
}


pub struct PermissionCheckRequest {
    pub permission: String,
    pub context: Option<HashMap<String, String>>,
pub struct PermissionCheckResponse {
    pub granted: bool,
    pub reason: String,
    pub source: Option<String>,
    pub expires_at: Option<String>,
    pub context_restrictions: HashMap<String, String>,
// ====== API KEY MODELS ======
pub struct ApiKeyListResponse {
    pub api_keys: Vec<ApiKeySummary>,
}


pub struct ApiKeySummary {
    pub key_id: String,
    pub prefix: String,
    pub scopes: Vec<String>,
    pub last_used: Option<String>,
    pub usage_count: u32,
pub struct CreateApiKeyRequest {
    pub description: Option<String>,
}


pub struct CreateApiKeyResponse {
    pub api_key: String,
    pub warning: String,
    pub usage_instructions: String,
