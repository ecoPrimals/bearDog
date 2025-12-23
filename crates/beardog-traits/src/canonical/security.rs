// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::base::BaseProvider;
use beardog_errors::BearDogError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecurityEvent {
    /// The event type value
    pub event_type: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_id: Option<String>,
    /// The severity value
    pub severity: String,
    /// The metadata value
    pub metadata: std::collections::HashMap<String, String>,
}

// Use unified provider migration types
use beardog_types::canonical::providers_unified::migration::{
    AuthenticationCredentials, AuthenticationResult,
};

// Define missing types locally for compatibility
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub client_id: String,
    /// The ip address value
    pub ip_address: String,
    /// The user agent value
    pub user_agent: String,
    pub platform: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureSession {
    pub session_id: String,
    /// The created at value
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The expires at value
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

///
/// **UNIFICATION NOTE**: This trait exists alongside `unified::SecurityProvider`.
/// Future versions should consolidate to a single `SecurityProvider` interface
/// that combines the best of both approaches.
#[allow(clippy::type_complexity)]
pub trait SecurityProvider: BaseProvider {
    fn authenticate(
        credentials: AuthenticationCredentials,
    ) -> impl std::future::Future<Output = Result<AuthenticationResult, BearDogError>> + Send;

    /// Creates session
    fn create_session(
        user_id: &str,
    ) -> impl std::future::Future<Output = Result<SecureSession, BearDogError>> + Send;

    /// Validates session
    fn validate_session(
        session_id: &str,
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;

    fn revoke_session(
        session_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    fn authorize(
        session_id: &str,
        resource: &str,
        action: &str,
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;

    /// Log Security Event operation.
    fn log_security_event(
        event: SecurityEvent,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Gets `security_requirements`
    fn get_security_requirements(
        resource_type: &str,
    ) -> impl std::future::Future<Output = Result<Vec<String>, BearDogError>> + Send;

    /// Validates client
    fn validate_client(
        client_info: &ClientInfo,
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;

    fn refresh_token(
        token: &str,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    fn change_password(
        user_id: &str,
        old_password: &str,
        new_password: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Enable 2FA operation.
    fn enable_2fa(
        user_id: &str,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    fn verify_2fa(
        user_id: &str,
        code: &str,
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;
}
