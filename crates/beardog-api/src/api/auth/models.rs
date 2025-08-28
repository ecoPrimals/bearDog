

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub logout_all_devices: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutResponse {
    pub logged_out_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenValidationRequest {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentSessionResponse {
    pub session: SessionInfo,
    pub user: AuthenticatedUser,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordChangeRequest {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetRequest {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetConfirmRequest {
    pub token: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistrationRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub terms_accepted: bool,
    pub newsletter_subscription: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistrationResponse {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub verification_required: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailVerificationRequest {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendVerificationRequest {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityQuestion {
    pub question_id: String,
    pub question_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityQuestionAnswer {
    pub question_id: String,
    pub answer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityQuestionsSetupRequest {
    pub questions_and_answers: Vec<SecurityQuestionAnswer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLockoutInfo {
    pub is_locked: bool,
    pub lockout_until: Option<String>,
    pub failed_attempts: u32,
    pub max_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
    pub last_seen: String,
    pub is_trusted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedDevicesResponse {
    pub devices: Vec<DeviceInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeDeviceRequest {
    pub device_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginHistoryEntry {
    pub login_time: String,
    pub ip_address: String,
    pub user_agent: String,
    pub location: Option<String>,
    pub success: bool,
    pub failure_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginHistoryResponse {
    pub entries: Vec<LoginHistoryEntry>,
    pub total_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileUpdateRequest {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub notification_preferences: Option<HashMap<String, bool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: String,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub timezone: String,
    pub language: String,
    pub created_at: String,
    pub last_updated: String,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub notification_preferences: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyCreateRequest {
    pub name: String,
    pub scopes: Vec<String>,
    pub expires_at: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyResponse {
    pub api_key: String,
    pub warning: String,
    pub usage_instructions: String,
}
