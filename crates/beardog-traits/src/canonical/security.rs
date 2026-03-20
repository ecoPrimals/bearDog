// SPDX-License-Identifier: AGPL-3.0-only

//! Legacy session and authentication helpers layered on [`BaseProvider`].

use super::base::BaseProvider;
use beardog_errors::BearDogError;

/// Audit row emitted by canonical security providers.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecurityEvent {
    /// The event type value
    pub event_type: String,
    /// When the event was observed (UTC).
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Subject of the event when known.
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

/// Connecting client metadata for risk scoring and session binding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    /// Registered OAuth/client id or device id.
    pub client_id: String,
    /// The ip address value
    pub ip_address: String,
    /// The user agent value
    pub user_agent: String,
    /// OS or runtime label (`linux`, `ios`, `web`, …).
    pub platform: String,
}

/// Short-lived session handle without capability vectors (canonical variant).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureSession {
    /// Opaque session token.
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
    /// Validates credentials and returns an [`AuthenticationResult`].
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

    /// Invalidates a session server-side.
    fn revoke_session(
        session_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Returns whether `session_id` may perform `action` on `resource`.
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

    /// Rotates an access token using refresh semantics.
    fn refresh_token(
        token: &str,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    /// Updates a user password after verifying the previous secret.
    fn change_password(
        user_id: &str,
        old_password: &str,
        new_password: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Enable 2FA operation.
    fn enable_2fa(
        user_id: &str,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    /// Confirms a second factor challenge during login or sensitive operations.
    fn verify_2fa(
        user_id: &str,
        code: &str,
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;
}
