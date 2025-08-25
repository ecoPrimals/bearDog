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


/// Improved Authentication Handlers with Rich Context
///
/// This module demonstrates the new idiomatic authentication patterns using
/// AuthenticationOutcome instead of traditional Result<(), E> patterns.

use crate::api::*;
use axum::{extract::State, http::HeaderMap, Json};
use beardog_errors::{improved_results::*, success_outcome, BearDogError, BearDogResult};
use std::time::Instant;
use tracing::{info, warn};
// Use canonical SecurityLevel from beardog-errors
use beardog_errors::improved_results::SecurityLevel;
// Define AuthenticationFactor locally
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AuthenticationFactor {
    Totp,
    Biometric,
    Hardware,
}
// Define ClientInfo and SessionInfo locally
pub struct ClientInfo {
    pub client_id: String,
    pub ip_address: String,
    pub user_agent: String,
    pub platform: String,
    pub device_fingerprint: String,
}


pub struct SessionInfo {
    pub session_id: String,
    pub user_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
    pub client_ip: String,
    pub permissions: Vec<String>,
    pub client_info: ClientInfo,
/// Improved authentication using AuthenticationOutcome
pub async fn authenticate_user_improved(
    State(_state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<super::models::LoginRequest>,
) -> BearDogResult<AuthenticationOutcome> {
    let _start_time = Instant::now();
    info!(
        "🔐 Authenticating user with improved patterns: {}",
        request.username
    );
    // Extract client information from headers
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("Unknown")
        .to_string();
    let client_ip = headers
        .get("x-forwarded-for")
        .or_else(|| headers.get("x-real-ip"))
        .unwrap_or("127.0.0.1")
    // Get device fingerprint if available
    let device_fingerprint = headers
        .get("x-device-fingerprint")
        .map(|s| s.to_string());
    // SECURITY: Authentication with environment-configured credentials
    let admin_username =
        std::env::var("BEARDOG_ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let admin_password =
        std::env::var("BEARDOG_ADMIN_PASSWORD").unwrap_or_else(|_| "admin123".to_string()); // Fallback for tests
    let authentication_success =
        request.username == admin_username && request.password == admin_password;
    if !authentication_success {
        warn!("❌ Authentication failed for user: {}", request.username);
        return Err(BearDogError::authentication(format!(
                "Authentication failed for user: {username)",
                username = request.username
            ),
        });
    }
    // Determine security level based on authentication factors
    let security_level = if request.mfa_code.is_some() {
        SecurityLevel::High
    } else if device_fingerprint.is_some() {
        SecurityLevel::Enhanced // Use Enhanced instead of Standard
    } else {
        SecurityLevel::Basic
    };
    // Create session information
    let session_info = SessionInfo {
        session_id: uuid::Uuid::new_v4().to_string(),
        user_id: request.username.clone(),
        permissions: vec![
            "admin".to_string(),
            "read".to_string(),
            "write".to_string(),
            "security".to_string(),
        ],
        created_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(8),
        last_activity: chrono::Utc::now(),
        client_ip: client_ip.clone(),
        user_agent: user_agent.clone(),
        device_fingerprint: device_fingerprint
            .clone()
            .unwrap_or_else(|| "unknown".to_string()),
        client_info: ClientInfo {
            client_id: "default_client".to_string(),
            ip_address: client_ip,
            user_agent: user_agent.clone(),
            platform: "web".to_string(),
            device_fingerprint: device_fingerprint
                .as_ref()
                .unwrap_or(&"unknown".to_string())
                .clone(),
        },
    // Determine additional factors required
    let _additional_factors_required = if security_level == SecurityLevel::Basic {
        vec![AuthenticationFactor::Totp] // Suggest TOTP for higher security
        vec![]
    // Calculate session expiry based on security level
    let _expires_at = match security_level {
        SecurityLevel::Maximum => chrono::Utc::now() + chrono::Duration::hours(1),
        SecurityLevel::High => chrono::Utc::now() + chrono::Duration::hours(4),
        SecurityLevel::Enhanced => chrono::Utc::now() + chrono::Duration::hours(8),
        SecurityLevel::Basic => chrono::Utc::now() + chrono::Duration::hours(2),
        "✅ Authentication successful for user: {} (Security Level: {:?})",
        request.username, security_level
    Ok(AuthenticationOutcome {
        session_id: session_info.session_id.clone(),
        user_info: UserInfo {
            username: request.username.clone(),
            roles: session_info.permissions.clone(),
            attributes: std::collections::HashMap::new(),
        security_level: match security_level {
            SecurityLevel::Basic => beardog_errors::improved_results::SecurityLevel::Basic,
            SecurityLevel::Enhanced => beardog_errors::improved_results::SecurityLevel::Enhanced,
            SecurityLevel::High => beardog_errors::improved_results::SecurityLevel::High,
            SecurityLevel::Maximum => beardog_errors::improved_results::SecurityLevel::Maximum,
        expires_at: session_info.expires_at,
        authentication_method: AuthenticationMethod::Password,
        context: OperationContext::new("beardog-auth"),
        metrics: OperationMetrics::default(),
        warnings: vec![],
    })
/// Improved logout using OperationOutcome
pub async fn logout_user_improved(
    Json(request): Json<super::models::LogoutRequest>,
) -> BearDogOutcome<LogoutResult> {
    info!("🚪 Logging out session: {}", request.session_id);
    // In a real implementation, this would:
    // - Validate the session exists
    // - Revoke the session from storage
    // - Invalidate related tokens
    // - Log the logout event
    // - Handle logout from all devices if requested
    let logout_result = LogoutResult {
        session_id: request.session_id.clone(),
        logged_out_at: chrono::Utc::now(),
        devices_logged_out: if request.logout_all_devices.unwrap_or(false) {
            vec!["current_device".to_string(), "mobile_device".to_string()]
        } else {
            vec!["current_device".to_string()]
        security_tokens_revoked: 2,
        redirect_url: Some("/login".to_string()),
    info!("✅ Logout successful for session: {}", request.session_id);
    success_outcome!(logout_result, "beardog-auth")
/// Improved token refresh using rich context
pub async fn refresh_token_improved(
    Json(request): Json<super::models::RefreshTokenRequest>,
) -> BearDogResult<TokenRefreshOutcome> {
    info!("🔄 Refreshing token: {}", &request.refresh_token[..8]);
    // - Validate the refresh token
    // - Check if it's expired or revoked
    // - Generate new access token
    // - Optionally rotate refresh token
    // - Update session activity
    // Simulate token validation
    if request.refresh_token.is_empty() {
        return Err(BearDogError::authentication("Invalid refresh token".to_string(),
        ));
    let token_uuid = uuid::Uuid::new_v4();
    let new_access_token = format!("access_token_{token_uuid}");
    let refresh_uuid = uuid::Uuid::new_v4();
    let new_refresh_token = format!("refresh_token_{refresh_uuid}");
    let token_refresh_result = TokenRefreshOutcome {
        access_token: new_access_token,
        refresh_token: new_refresh_token,
        expires_in: 3600, // 1 hour
        token_type: "Bearer".to_string(),
        scope: vec!["read".to_string(), "write".to_string(), "admin".to_string()],
        issued_at: chrono::Utc::now(),
        refresh_count: 1,
        previous_token_revoked: true,
    info!("✅ Token refresh successful");
    Ok(token_refresh_result)
/// Improved session validation with detailed context
pub async fn validate_session_improved(
    session_id: String,
) -> BearDogResult<SessionValidationOutcome> {
    info!("🔍 Validating session: {}", session_id);
    // - Look up session in storage
    // - Check expiry
    // - Validate session integrity
    // - Update last activity
    // - Check for security violations
    // Simulate session validation
    if session_id.is_empty() {
        return Err(BearDogError::authentication("Invalid session ID".to_string(),
    let validation_result = SessionValidationOutcome {
        valid: true,
        session_id: session_id.clone(),
        user_id: "admin".to_string(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(4),
        last_activity: chrono::Utc::now() - chrono::Duration::minutes(5),
        security_level: SecurityLevel::Enhanced,
        permissions: vec!["read".to_string(), "write".to_string(), "admin".to_string()],
            client_id: "example_client".to_string(),
            ip_address: "127.0.0.1".to_string(),
            user_agent: "BearDog-Client/1.0".to_string(),
            platform: "mobile".to_string(),
            device_fingerprint: "device123".to_string(),
        security_warnings: vec![], // No warnings for this session
        refresh_recommended: false,
    info!("✅ Session validation successful: {}", session_id);
    Ok(validation_result)
/// Custom result types for authentication operations
pub struct LogoutResult {
    pub logged_out_at: chrono::DateTime<chrono::Utc>,
    pub devices_logged_out: Vec<String>,
    pub security_tokens_revoked: u32,
    pub redirect_url: Option<String>,
}


pub struct TokenRefreshOutcome {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub scope: Vec<String>,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub refresh_count: u32,
    pub previous_token_revoked: bool,
pub struct SessionValidationOutcome {
    pub valid: bool,
    pub security_level: SecurityLevel,
    pub security_warnings: Vec<String>,
    pub refresh_recommended: bool,
#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{
        cache::CacheProviderType, rate_limiting::RateLimiterType, server::ApiServerConfig,
        zero_copy::ZeroCopyHandlerContext, AppState,
    use axum::http::HeaderMap;
    use std::sync::Arc;
    fn create_test_app_state() -> AppState {
        AppState {
            core: Arc::new("test_core".to_string()),
            cache: Arc::new(CacheProviderType::InMemory(
                crate::api::cache::InMemoryCache::new(),
            )),
            rate_limiter: Arc::new(RateLimiterType::new()),
            config: ApiServerConfig::default(),
            zero_copy_context: Arc::new(ZeroCopyHandlerContext::new()),
        }
    #[tokio::test]
    async fn test_improved_authentication_success() {
        std::env::set_var("BEARDOG_ADMIN_PASSWORD", "admin123");
        let state = create_test_app_state();
        let headers = HeaderMap::new();
        let request = Json(super::super::models::LoginRequest {
            username: "admin".to_string(),
            password: "admin123".to_string(),
            mfa_code: None,
            remember_device: Some(false),
        let result = authenticate_user_improved(State(state), headers, request).await;
        assert!(result.is_ok());
        let auth_outcome = result.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        // Check authentication success via session_id and user info
        assert!(!auth_outcome.session_id.is_empty());
        assert_eq!(auth_outcome.user_info.username, "admin");
        assert!(matches!(
            auth_outcome.security_level,
            beardog_errors::improved_results::SecurityLevel::Basic
        assert!(auth_outcome.expires_at > chrono::Utc::now());
    async fn test_improved_authentication_failure() {
            username: "invalid".to_string(),
            password: "wrong".to_string(),
        assert!(result.is_err());}


    async fn test_improved_logout() {
        let request = Json(super::super::models::LogoutRequest {
            session_id: "test_session_123".to_string(),
            logout_all_devices: Some(false),
        let result = logout_user_improved(State(state), request).await;
        let outcome = result.map_err(|e| {
        assert_eq!(outcome.result.session_id, "test_session_123");
        assert_eq!(outcome.result.devices_logged_out.len(), 1);
    async fn test_token_refresh() {
        let request = Json(super::super::models::RefreshTokenRequest {
            refresh_token: "valid_refresh_token".to_string(),
        let result = refresh_token_improved(State(state), request).await;
        let token_outcome = result.map_err(|e| {
        assert!(!token_outcome.access_token.is_empty());
        assert!(!token_outcome.refresh_token.is_empty());
        assert_eq!(token_outcome.token_type, "Bearer");
        assert!(token_outcome.previous_token_revoked);}


    async fn test_session_validation() {
        let result = validate_session_improved(State(state), "valid_session_123".to_string()).await;
        let validation = result.map_err(|e| {
        assert!(validation.valid);
        assert_eq!(validation.session_id, "valid_session_123");
        assert_eq!(validation.user_id, "admin");
        assert!(!validation.refresh_recommended);
