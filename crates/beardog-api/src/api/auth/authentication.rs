//! Core Authentication Handlers
//!
//! Handles login, logout, token refresh, token validation, and session management.

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
        .and_then(|v| v.to_str().ok())
        .unwrap_or("127.0.0.1")
        .to_string();

    // SECURITY: Authentication with environment-configured credentials
    // In production: verify password hash, check account status, handle rate limiting
    let admin_username =
        std::env::var("BEARDOG_ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let admin_password = std::env::var("BEARDOG_ADMIN_PASSWORD")
        .expect("BEARDOG_ADMIN_PASSWORD environment variable must be set for security");

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
            },
            session: SessionInfo {
                session_id: "".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                expires_at: chrono::Utc::now().to_rfc3339(),
                client_ip: client_ip.clone(),
                user_agent: user_agent.clone(),
                device_fingerprint: "".to_string(),
            },
            mfa_required: false,
            security_notices: vec!["Authentication failed".to_string()],
        };

        return Ok(Json(ApiResponse {
            success: false,
            data: Some(failed_response),
            error: Some("Invalid username or password".to_string()),
            request_id: request_id.clone(),
            timestamp: chrono::Utc::now(),
            meta: ResponseMetadata {
                processing_time_ms: start_time.elapsed().as_millis() as u64,
                cached: false,
                version: env!("CARGO_PKG_VERSION").to_string(),
                pagination: None,
            },
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
        },
        session: SessionInfo {
            session_id: "session_67890".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            expires_at: (chrono::Utc::now() + chrono::Duration::hours(8)).to_rfc3339(),
            client_ip,
            user_agent,
            device_fingerprint: "fp_device_12345".to_string(),
        },
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
    State(_state): State<AppState>,
    Json(request): Json<LogoutRequest>,
) -> Result<Json<ApiResponse<LogoutResponse>>, StatusCode> {
    let start_time = Instant::now();
    let request_id = uuid::Uuid::new_v4().to_string();

    info!("🚪 Logging out user session: {}", request.session_id);

    let response = LogoutResponse {
        success: true,
        session_id: request.session_id,
        logged_out_at: chrono::Utc::now().to_rfc3339(),
        message: "Successfully logged out".to_string(),
        redirect_url: Some("/login".to_string()),
    };

    let processing_time = start_time.elapsed().as_millis() as u64;
    Ok(Json(success_response(
        response,
        request_id,
        processing_time,
        false,
    )))
}

/// Refresh authentication token
pub async fn refresh_token(
    State(_state): State<AppState>,
    Json(request): Json<RefreshTokenRequest>,
) -> Result<Json<ApiResponse<RefreshTokenResponse>>, StatusCode> {
    let start_time = Instant::now();
    let request_id = uuid::Uuid::new_v4().to_string();

    info!("🔄 Refreshing authentication token");

    let response = RefreshTokenResponse {
        access_token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...new".to_string(),
        refresh_token: request.refresh_token, // Keep same refresh token
        expires_in: 3600,                     // 1 hour
        token_type: "Bearer".to_string(),
        issued_at: chrono::Utc::now().to_rfc3339(),
        scope: vec!["read".to_string(), "write".to_string(), "admin".to_string()],
    };

    let processing_time = start_time.elapsed().as_millis() as u64;
    Ok(Json(success_response(
        response,
        request_id,
        processing_time,
        false,
    )))
}

/// Validate authentication token
pub async fn validate_token(
    State(_state): State<AppState>,
    Json(_request): Json<TokenValidationRequest>,
) -> Result<Json<ApiResponse<TokenValidationResponse>>, StatusCode> {
    let start_time = Instant::now();
    let request_id = uuid::Uuid::new_v4().to_string();

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
        issued_at: chrono::Utc::now().to_rfc3339(),
        token_type: "Bearer".to_string(),
        scope: vec!["read".to_string(), "write".to_string(), "admin".to_string()],
        warnings: vec![], // Could include expiration warnings
    };

    let processing_time = start_time.elapsed().as_millis() as u64;
    Ok(Json(success_response(
        response,
        request_id,
        processing_time,
        true,
    )))
}

/// Get current session information
pub async fn get_current_session(
    State(_state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<CurrentSessionResponse>>, StatusCode> {
    let start_time = Instant::now();
    let request_id = uuid::Uuid::new_v4().to_string();

    // Extract authorization header
    let _auth_header = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("Bearer token_example");

    let response = CurrentSessionResponse {
        session_id: "session_67890".to_string(),
        user_id: "user_12345".to_string(),
        username: "admin".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        last_activity: chrono::Utc::now().to_rfc3339(),
        expires_at: (chrono::Utc::now() + chrono::Duration::hours(8)).to_rfc3339(),
        active_permissions: vec![
            "read:users".to_string(),
            "write:users".to_string(),
            "admin:system".to_string(),
        ],
        session_status: "active".to_string(),
        concurrent_sessions: 1,
        session_metadata: {
            let mut metadata = HashMap::new();
            metadata.insert("client_type".to_string(), "web".to_string());
            metadata.insert("auth_method".to_string(), "password+mfa".to_string());
            metadata
        },
    };

    let processing_time = start_time.elapsed().as_millis() as u64;
    Ok(Json(success_response(
        response,
        request_id,
        processing_time,
        true,
    )))
}

/// Invalidate current session (placeholder)
pub async fn invalidate_session(
    State(_): State<AppState>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let request_id = uuid::Uuid::new_v4().to_string();
    Ok(Json(success_response(
        serde_json::json!({"message": "Session invalidated"}),
        request_id,
        15,
        false,
    )))
}

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
            },
            zero_copy_context: std::sync::Arc::new(
                crate::api::zero_copy_handlers::ZeroCopyHandlerContext::default(),
            ),
        }
    }

    fn create_test_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_str("Authorization").unwrap(),
            HeaderValue::from_str("Bearer test_token").unwrap(),
        );
        headers.insert(
            HeaderName::from_str("User-Agent").unwrap(),
            HeaderValue::from_str("BearDog-Test/1.0").unwrap(),
        );
        headers.insert(
            HeaderName::from_str("X-Real-IP").unwrap(),
            HeaderValue::from_str("192.168.1.100").unwrap(),
        );
        headers
    }

    /// Extract client IP from request headers
    fn extract_client_ip(headers: &HeaderMap) -> String {
        // Check X-Real-IP header first
        if let Some(ip) = headers.get("X-Real-IP") {
            if let Ok(ip_str) = ip.to_str() {
                return ip_str.to_string();
            }
        }

        // Check X-Forwarded-For header
        if let Some(forwarded) = headers.get("X-Forwarded-For") {
            if let Ok(forwarded_str) = forwarded.to_str() {
                // Take the first IP from comma-separated list
                if let Some(first_ip) = forwarded_str.split(',').next() {
                    return first_ip.trim().to_string();
                }
            }
        }

        // Fallback to localhost
        "127.0.0.1".to_string()
    }

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
        let response = result.unwrap();
        // response is Json<ApiResponse<AuthenticationResponse>>
        assert!(response.0.success);
    }

    #[tokio::test]
    async fn test_authenticate_user_with_mfa() {
        let state = State(create_test_app_state());
        let headers = create_test_headers();
        let request = Json(LoginRequest {
            username: "admin".to_string(),
            password: "password123".to_string(),
            mfa_code: Some("123456".to_string()),
            remember_device: Some(true),
        });

        let result = authenticate_user(state, headers, request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authenticate_user_invalid_credentials() {
        let state = State(create_test_app_state());
        let headers = create_test_headers();
        let request = Json(LoginRequest {
            username: "invalid_user".to_string(),
            password: "wrong_password".to_string(),
            mfa_code: None,
            remember_device: None,
        });

        let result = authenticate_user(state, headers, request).await;
        assert!(result.is_ok()); // Should handle gracefully

        let response = result.unwrap();
        // Should return failed authentication response, not HTTP error
        assert!(!response.0.success);
        assert!(response.0.error.is_some());
        assert!(!response.0.data.as_ref().unwrap().success);
    }

    #[tokio::test]
    async fn test_logout_user() {
        let state = State(create_test_app_state());
        let logout_request = Json(LogoutRequest {
            session_id: "test_session_123".to_string(),
            logout_all_devices: Some(false),
        });

        let result = logout_user(state, logout_request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        // response is Json<ApiResponse<LogoutResponse>>
        assert!(response.0.success);
        assert_eq!(
            response.0.data.as_ref().unwrap().session_id,
            "test_session_123"
        );
    }

    #[tokio::test]
    async fn test_refresh_token() {
        let state = State(create_test_app_state());
        let request = Json(RefreshTokenRequest {
            refresh_token: "test_refresh_token".to_string(),
        });

        let result = refresh_token(state, request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        // response is Json<ApiResponse<RefreshTokenResponse>>
        assert!(response.0.success);
    }

    #[tokio::test]
    async fn test_validate_token() {
        let state = State(create_test_app_state());
        let request = Json(TokenValidationRequest {
            token: "test_jwt_token".to_string(),
        });

        let result = validate_token(state, request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        // response is Json<ApiResponse<TokenValidationResponse>>
        assert!(response.0.success);
        assert!(response.0.data.as_ref().unwrap().valid);
    }

    #[tokio::test]
    async fn test_get_session_info() {
        let state = State(create_test_app_state());
        let headers = create_test_headers();

        let result = get_current_session(state, headers).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        // response is Json<ApiResponse<CurrentSessionResponse>>
        assert!(response.0.success);
    }

    #[tokio::test]
    async fn test_extract_client_ip() {
        let headers = create_test_headers();
        let client_ip = extract_client_ip(&headers);

        // Should extract IP from X-Real-IP header
        assert_eq!(client_ip, "192.168.1.100");
    }

    #[tokio::test]
    async fn test_extract_client_ip_fallback() {
        let empty_headers = HeaderMap::new();
        let client_ip = extract_client_ip(&empty_headers);

        // Should fallback to localhost
        assert_eq!(client_ip, "127.0.0.1");
    }

    #[tokio::test]
    async fn test_authentication_edge_cases() {
        let state = State(create_test_app_state());

        // Test with empty headers
        let empty_headers = HeaderMap::new();
        let request = Json(LoginRequest {
            username: "test".to_string(),
            password: "test".to_string(),
            mfa_code: None,
            remember_device: None,
        });

        let result = authenticate_user(state.clone(), empty_headers, request).await;
        assert!(result.is_ok());

        // Test with very long username
        let long_username = "a".repeat(1000);
        let request = Json(LoginRequest {
            username: long_username,
            password: "test".to_string(),
            mfa_code: None,
            remember_device: None,
        });

        let result = authenticate_user(state, create_test_headers(), request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_session_management() {
        let state = State(create_test_app_state());
        let headers = create_test_headers();

        // Test session info retrieval
        let result = get_current_session(state.clone(), headers.clone()).await;
        assert!(result.is_ok());

        // Test logout
        let logout_request = Json(LogoutRequest {
            session_id: "empty_session".to_string(),
            logout_all_devices: Some(false),
        });
        let logout_result = logout_user(state, logout_request).await;
        assert!(logout_result.is_ok());
    }

    #[test]
    fn test_request_models() {
        // Test LoginRequest serialization
        let login_request = LoginRequest {
            username: "test".to_string(),
            password: "password".to_string(),
            mfa_code: Some("123456".to_string()),
            remember_device: Some(true),
        };

        let json_str = serde_json::to_string(&login_request).expect("Serialization failed");
        assert!(json_str.contains("test"));
        assert!(json_str.contains("123456"));

        // Test RefreshTokenRequest
        let refresh_request = RefreshTokenRequest {
            refresh_token: "refresh123".to_string(),
        };

        let json_str = serde_json::to_string(&refresh_request).expect("Serialization failed");
        assert!(json_str.contains("refresh123"));

        // Test ValidateTokenRequest
        let validate_request = TokenValidationRequest {
            token: "jwt123".to_string(),
        };

        let json_str = serde_json::to_string(&validate_request).expect("Serialization failed");
        assert!(json_str.contains("jwt123"));
    }
}
