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


/// # Security Types Module
///
/// **CANONICAL SECURITY TYPES**
/// This module consolidates security types from beardog-security/src/types/mod.rs
/// and other scattered security type definitions into a single authoritative location.
/// ## Consolidated Types From:
/// - `beardog-security/src/types/mod.rs` (842 lines) - Primary security types
/// - `beardog-security/src/types/auth_types.rs` - Authentication types  
/// - `beardog-security/src/types/audit_types.rs` - Audit and monitoring types
/// - `beardog-security/src/types/crypto_types.rs` - Cryptographic types
/// - Various scattered security type definitions

use crate::canonical::MfaConfig;
use chrono::{DateTime, Utc};
// Import canonical types (these are re-exported through the module system to avoid conflicts)
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Re-export canonical types directly to avoid conflicts
// (Removed config::security re-exports that conflict with canonical types)
pub use crate::crypto::{AesMode, CryptoAlgorithm, EcCurve};
pub use beardog_traits::canonical::SecurityProvider;
// ================================================================================
// AUTHENTICATION TYPES
/// Authentication configuration
pub use crate::canonical::AuthenticationConfig;
// Default implementation removed - using canonical implementation from crate::canonical
/// User session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub session_id: String,};


    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub ip_address: String,
    pub user_agent: String,
}
/// Authentication result
pub struct AuthenticationResult {
    pub success: bool,
    pub user_id: Option<String>,
    pub session_token: Option<String>,
    pub error_message: Option<String>,
    pub requires_mfa: bool,
// ENCRYPTION TYPES
/// Encryption configuration
pub use crate::canonical::configuration::security::EncryptionConfig;
// RATE LIMITING TYPES
/// Rate limiting configuration
pub use crate::canonical::configuration::security::RateLimitConfig;
// TOKEN TYPES
/// Token configuration
pub struct TokenConfig {
    /// JWT secret key
    pub jwt_secret: String,
    /// Token expiration time in seconds
    pub token_expiration_seconds: u64,
    /// Enable token refresh
    pub enable_refresh: bool,
    /// Refresh token expiration in seconds
    pub refresh_expiration_seconds: u64,}


impl Default for TokenConfig {}


    fn default() -> Self {
        Self {
            jwt_secret: "change-me-in-production".to_string(),
            token_expiration_seconds: 3600, // 1 hour
            enable_refresh: true,
            refresh_expiration_seconds: 86400, // 24 hours
        }
    }
// SESSION TYPES
/// Session configuration
pub use crate::canonical::configuration::security::SessionConfig;
// AUDIT TYPES
/// Security audit event
pub use crate::canonical::SecurityAuditEvent;
/// Types of security events
pub enum SecurityEventType {
    Login,
    Logout,
    AuthenticationFailure,
    PasswordChange,
    MfaSetup,
    MfaVerification,
    TokenGeneration,
    TokenRevocation,
    PermissionGranted,
    PermissionDenied,
    DataAccess,
    DataModification,
    ConfigurationChange,
// PROVIDER CONFIGURATION
/// **MIGRATION COMPLETE** ✅
/// SecurityProviderConfig has been consolidated with the canonical ProviderConfig.
/// All security provider configuration should now use the canonical types.
/// 
/// ## Migration Path:
/// ```rust
/// // OLD:
/// use beardog_types::security::SecurityProviderConfig;
/// 
/// // NEW:
/// use beardog_types::canonical::providers::ProviderConfig;
/// ```

// Re-export the canonical ProviderConfig for backward compatibility
pub use beardog_types::canonical::providers::ProviderConfig as SecurityProviderConfig;
