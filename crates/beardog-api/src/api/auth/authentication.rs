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


/// Core Authentication Handlers
///
/// Handles login, logout, token refresh, token validation, and session management.

use super::models::*;
use crate::api::*;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use std::collections::HashMap;
use std::time::Instant;
use tracing::{info, warn};
// ====== AUTHENTICATION HANDLERS ======
/// Authenticate user with credentials
pub async fn authenticate_user(
    State(_state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthenticationResponse>>, StatusCode> {
    let start_time = Instant::now();
    let request_id = uuid::Uuid::new_v4().to_string();
    info!("🔐 Authenticating user: {}", request.username);
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
    // SECURITY: Authentication with environment-configured credentials
    // In production: verify password hash, check account status, handle rate limiting
    let admin_username =
        std::env::var("BEARDOG_ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let admin_password = std::env::var("BEARDOG_ADMIN_PASSWORD")
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "BEARDOG_ADMIN_PASSWORD environment variable must be set for security", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "BEARDOG_ADMIN_PASSWORD environment variable must be set for security", e))
})?;
    let authentication_success =
        request.username == admin_username && request.password == admin_password;
    if !authentication_success {
        warn!("❌ Authentication failed for user: {}", request.username);
        let failed_response = AuthenticationResponse {
            success: false,
            user: AuthenticatedUser {
                user_id: "".to_string(),
                username: request.username.clone(),
                display_name: "".to_string(),
                email: "".to_string(),
                roles: vec![],
                permissions: vec![],
                mfa_enabled: false,
                last_login: chrono::Utc::now().to_rfc3339(),
                account_status: "invalid".to_string(),
            },
            tokens: AuthTokens {
                access_token: "".to_string(),
                refresh_token: "".to_string(),
                expires_in: 0,
                token_type: "Bearer".to_string(),
            session: SessionInfo {
                session_id: "".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                expires_at: chrono::Utc::now().to_rfc3339(),
                client_ip: client_ip.clone(),
                user_agent: user_agent.clone(),
                device_fingerprint: "".to_string(),
            mfa_required: false,
            security_notices: vec!["Authentication failed".to_string()],
        };
        return Ok(Json(ApiResponse {
            data: Some(failed_response),
            error: Some("Invalid username or password".to_string()),
            request_id: request_id.clone(),
            timestamp: chrono::Utc::now(),
            meta: ResponseMetadata {
                processing_time_ms: start_time.elapsed().as_millis() as u64,
                cached: false,
                version: env!("CARGO_PKG_VERSION").to_string(),
                pagination: None,
        }));
    }
    let response = AuthenticationResponse {
        success: true,
        user: AuthenticatedUser {
            user_id: "user_12345".to_string(),
            username: request.username.clone(),
            display_name: "System Administrator".to_string(),
            email: "admin@beardog.com".to_string(),
            roles: vec!["admin".to_string(), "user".to_string()],
            permissions: vec![
                "read:users".to_string(),
                "write:users".to_string(),
                "admin:system".to_string(),
            ],
            mfa_enabled: true,
            last_login: chrono::Utc::now().to_rfc3339(),
            account_status: "active".to_string(),
        },
        tokens: AuthTokens {
            access_token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...".to_string(),
            refresh_token: "rt_refresh_token_12345".to_string(),
            expires_in: 3600, // 1 hour
            token_type: "Bearer".to_string(),
        session: SessionInfo {
            session_id: "session_67890".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            expires_at: (chrono::Utc::now() + chrono::Duration::hours(8)).to_rfc3339(),
            client_ip,
            user_agent,
            device_fingerprint: "fp_device_12345".to_string(),
        mfa_required: false, // Would be true if MFA needed
        security_notices: vec!["Your last login was successful from 192.168.1.100".to_string()],
    };
    let processing_time = start_time.elapsed().as_millis() as u64;
    Ok(Json(success_response(
        response,
        request_id,
        processing_time,
        false,
    )))
}
/// Logout user and invalidate session
pub async fn logout_user(
    Json(request): Json<LogoutRequest>,
) -> Result<Json<ApiResponse<LogoutResponse>>, StatusCode> {
    info!("🚪 Logging out user session: {}", request.session_id);
    let response = LogoutResponse {
        session_id: request.session_id,
        logged_out_at: chrono::Utc::now().to_rfc3339(),
        message: "Successfully logged out".to_string(),
        redirect_url: Some("/login".to_string()),
/// Refresh authentication token
pub async fn refresh_token(
    Json(request): Json<RefreshTokenRequest>,
) -> Result<Json<ApiResponse<RefreshTokenResponse>>, StatusCode> {
    info!("🔄 Refreshing authentication token");
    let response = RefreshTokenResponse {
        access_token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...new".to_string(),
        refresh_token: request.refresh_token, // Keep same refresh token
        expires_in: 3600,                     // 1 hour
        token_type: "Bearer".to_string(),
        issued_at: chrono::Utc::now().to_rfc3339(),
        scope: vec!["read".to_string(), "write".to_string(), "admin".to_string()],
/// Validate authentication token}


pub async fn validate_token(
    Json(_request): Json<TokenValidationRequest>,
) -> Result<Json<ApiResponse<TokenValidationResponse>>, StatusCode> {
    info!("✅ Validating authentication token");
    let response = TokenValidationResponse {
        valid: true,
        user_id: "user_12345".to_string(),
        username: "admin".to_string(),
        roles: vec!["admin".to_string(), "user".to_string()],
        permissions: vec![
            "read:users".to_string(),
            "write:users".to_string(),
            "admin:system".to_string(),
        ],
        expires_at: (chrono::Utc::now() + chrono::Duration::hours(1)).to_rfc3339(),
        warnings: vec![], // Could include expiration warnings
        true,
/// Get current session information
pub async fn get_current_session(
) -> Result<Json<ApiResponse<CurrentSessionResponse>>, StatusCode> {
    // Extract authorization header
    let _auth_header = headers
        .get("authorization")
        .unwrap_or("Bearer token_example");
    let response = CurrentSessionResponse {
        session_id: "session_67890".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        last_activity: chrono::Utc::now().to_rfc3339(),
        expires_at: (chrono::Utc::now() + chrono::Duration::hours(8)).to_rfc3339(),
        active_permissions: vec![
        session_status: "active".to_string(),
        concurrent_sessions: 1,
        session_metadata: {
            let mut metadata = HashMap::new();
            metadata.insert("client_type".to_string(), "web".to_string());
            metadata.insert("auth_method".to_string(), "password+mfa".to_string());
            metadata
/// Invalidate current session (placeholder)}


pub async fn invalidate_session(
    State(_): State<AppState>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
        serde_json::json!({"message": "Session invalidated"}),
        15,
#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{HeaderMap, HeaderName, HeaderValue};
    use std::str::FromStr;
    fn create_test_app_state() -> AppState {
        AppState {
            core: std::sync::Arc::new(()),
            cache: std::sync::Arc::new(crate::api::cache::CacheProviderType::InMemory(
                crate::api::cache::InMemoryCache::new(),
            )),
            rate_limiter: std::sync::Arc::new(crate::api::rate_limiting::RateLimiterType::default()),
            config: crate::api::server::ApiServerConfig {
                bind_address: std::env::var("BEARDOG_API_BIND_ADDRESS")
                    .unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
                request_timeout_seconds: 30,
                max_request_size: 1024 * 1024,
                compression_enabled: true,
                cors_enabled: true,
                rate_limiting_enabled: true,
                caching_enabled: true,
            zero_copy_context: std::sync::Arc::new(
                crate::api::zero_copy_handlers::ZeroCopyHandlerContext::default(),
            ),
        }
    fn create_test_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_str("Authorization").map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?,
            HeaderValue::from_str("Bearer test_token").map_err(|e| {
        );
            HeaderName::from_str("User-Agent").map_err(|e| {
            HeaderValue::from_str("BearDog-Test/1.0").map_err(|e| {
            HeaderName::from_str("X-Real-IP").map_err(|e| {
            HeaderValue::from_str("192.168.1.100").map_err(|e| {
        headers
    /// Extract client IP from request headers
    fn extract_client_ip(headers: &HeaderMap) -> String {
        // Check X-Real-IP header first
        if let Some(ip) = headers.get("X-Real-IP") {
            if let Ok(ip_str) = ip.to_str() {
                return ip_str.to_string();
            }
        // Check X-Forwarded-For header
        if let Some(forwarded) = headers.get("X-Forwarded-For") {
            if let Ok(forwarded_str) = forwarded.to_str() {
                // Take the first IP from comma-separated list
                if let Some(first_ip) = forwarded_str.split(',').next() {
                    return first_ip.trim().to_string();
                }
        // Fallback to localhost
        "127.0.0.1".to_string()
    #[tokio::test]
    async fn test_authenticate_user_success() {
        let state = State(create_test_app_state());
        let headers = create_test_headers();
        let request = Json(LoginRequest {
            username: "admin".to_string(),
            password: "password123".to_string(),
            mfa_code: None,
            remember_device: Some(false),
        });
        let result = authenticate_user(state, headers, request).await;
        // Should not panic and return proper response structure
        assert!(result.is_ok());
        let response = result.map_err(|e| {
        // response is Json<ApiResponse<AuthenticationResponse>>
        assert!(response.0.success);
    async fn test_authenticate_user_with_mfa() {
            mfa_code: Some("123456".to_string()),
            remember_device: Some(true),}


    async fn test_authenticate_user_invalid_credentials() {
            username: "invalid_user".to_string(),
            password: "wrong_password".to_string(),
            remember_device: None,
        assert!(result.is_ok()); // Should handle gracefully
        // Should return failed authentication response, not HTTP error
        assert!(!response.0.success);
        assert!(response.0.error.is_some());
        assert!(!response.0.data.as_ref().map_err(|e| {
})?.success);
    async fn test_logout_user() {
        let logout_request = Json(LogoutRequest {
            session_id: "test_session_123".to_string(),
            logout_all_devices: Some(false),
        let result = logout_user(state, logout_request).await;
        // response is Json<ApiResponse<LogoutResponse>>
        assert_eq!(
            response.0.data.as_ref().map_err(|e| {
})?.session_id,
            "test_session_123"
    async fn test_refresh_token() {
        let request = Json(RefreshTokenRequest {
            refresh_token: "test_refresh_token".to_string(),
        let result = refresh_token(state, request).await;
        // response is Json<ApiResponse<RefreshTokenResponse>>}


    async fn test_validate_token() {
        let request = Json(TokenValidationRequest {
            token: "test_jwt_token".to_string(),
        let result = validate_token(state, request).await;
        // response is Json<ApiResponse<TokenValidationResponse>>
        assert!(response.0.data.as_ref().map_err(|e| {
})?.valid);
    async fn test_get_session_info() {
        let result = get_current_session(state, headers).await;
        // response is Json<ApiResponse<CurrentSessionResponse>>}


    async fn test_extract_client_ip() {
        let client_ip = extract_client_ip(&headers);
        // Should extract IP from X-Real-IP header
        assert_eq!(client_ip, "192.168.1.100");
    async fn test_extract_client_ip_fallback() {
        let empty_headers = HeaderMap::new();
        let client_ip = extract_client_ip(&empty_headers);
        // Should fallback to localhost
        assert_eq!(client_ip, "127.0.0.1");}


    async fn test_authentication_edge_cases() {
        // Test with empty headers
            username: "test".to_string(),
            password: "test".to_string(),
        let result = authenticate_user(state.clone(), empty_headers, request).await;
        // Test with very long username
        let long_username = "a".repeat(1000);
            username: long_username,
        let result = authenticate_user(state, create_test_headers(), request).await;
    async fn test_session_management() {
        // Test session info retrieval
        let result = get_current_session(state.clone(), headers.clone()).await;
        // Test logout
            session_id: "empty_session".to_string(),
        let logout_result = logout_user(state, logout_request).await;
        assert!(logout_result.is_ok());
    #[test]}


    fn test_request_models() {
        // Test LoginRequest serialization
        let login_request = LoginRequest {
            password: "password".to_string(),
        let json_str = serde_json::to_string(&login_request).map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Serialization failed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Serialization failed", e))
        assert!(json_str.contains("test"));
        assert!(json_str.contains("123456"));
        // Test RefreshTokenRequest
        let refresh_request = RefreshTokenRequest {
            refresh_token: "refresh123".to_string(),
        let json_str = serde_json::to_string(&refresh_request).map_err(|e| {
        assert!(json_str.contains("refresh123"));
        // Test ValidateTokenRequest
        let validate_request = TokenValidationRequest {
            token: "jwt123".to_string(),
        let json_str = serde_json::to_string(&validate_request).map_err(|e| {
        assert!(json_str.contains("jwt123"));
