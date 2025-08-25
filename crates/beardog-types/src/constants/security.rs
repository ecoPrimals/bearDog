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


/// # Security Constants
///
/// **CANONICAL SECURITY-RELATED CONSTANTS**
/// Cryptographic parameters, authentication limits, and security configuration constants.

use std::time::Duration;
/// **CANONICAL SECURITY CONSTANTS** - Consolidates security-related constants
pub mod core {
    use super::Duration;
    /// Maximum security sessions allowed
    /// Consolidates: MAX_SESSIONS from security modules
    pub const MAX_SESSIONS: usize = 10_000;
    /// Session timeout duration
    /// Consolidates: SESSION_TIMEOUT_SECS from security modules
    pub const SESSION_TIMEOUT: Duration = Duration::from_secs(3600); // 1 hour
    /// Maximum authentication attempts before lockout
    /// Consolidates: MAX_AUTH_ATTEMPTS from security modules
    pub const MAX_AUTH_ATTEMPTS: u32 = 5;
    /// Account lockout duration after failed attempts
    /// Consolidates: LOCKOUT_DURATION_SECS from security modules
    pub const LOCKOUT_DURATION: Duration = Duration::from_secs(900); // 15 minutes
    /// Standard cryptographic key size
    pub const STANDARD_KEY_SIZE: usize = 32; // 256 bits
    /// Maximum retry attempts for operations
    /// Consolidates: MAX_ATTEMPTS from workflow modules
    pub const MAX_OPERATION_ATTEMPTS: u32 = 30;
    /// Standard charset for cryptographic operations
    /// Consolidates: CHARSET from multiple crypto modules
    pub const CRYPTO_CHARSET: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    /// Base58 alphabet for address encoding
    /// Consolidates: ALPHABET from address management
    pub const BASE58_ALPHABET: &[u8] =
        b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    /// Character set for TOTP/OTP generation
    /// Consolidates: CHARSET from auth modules
    pub const OTP_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
}
/// **CANONICAL SYSTEM SECURITY CONSTANTS** - Security configuration defaults
/// **MIGRATED FROM**: `beardog-config/src/constants.rs::security`
/// These constants define security defaults for authentication, encryption, and access control
pub mod system_security {
    /// Default session timeout (1 hour)
    pub const DEFAULT_SESSION_TIMEOUT: Duration = Duration::from_secs(3600);
    /// Default auth timeout (5 minutes)
    pub const DEFAULT_AUTH_TIMEOUT: Duration = Duration::from_secs(300);
    /// Default max failed attempts
    pub const DEFAULT_MAX_FAILED_ATTEMPTS: u32 = 5;
    /// Default lockout duration (30 minutes)
    pub const DEFAULT_LOCKOUT_DURATION: Duration = Duration::from_secs(1800);
    /// Default token expiry (1 hour)
    pub const DEFAULT_TOKEN_EXPIRY: Duration = Duration::from_secs(3600);
    /// Default key rotation interval (24 hours)
    pub const DEFAULT_KEY_ROTATION_INTERVAL: Duration = Duration::from_secs(86400);
    /// Default encryption key size (256 bits)
    pub const DEFAULT_ENCRYPTION_KEY_SIZE: usize = 32;
    /// Default nonce size (96 bits for AES-GCM)
    pub const DEFAULT_NONCE_SIZE: usize = 12;
    /// Default signature size (Ed25519 signature size)
    pub const DEFAULT_SIGNATURE_SIZE: usize = 64;
    /// Default public key size (Ed25519 public key size)
    pub const DEFAULT_PUBLIC_KEY_SIZE: usize = 32;
    /// Default private key size (Ed25519 private key size)
    pub const DEFAULT_PRIVATE_KEY_SIZE: usize = 32;
    /// Default hash size (SHA-256 hash size)
    pub const DEFAULT_HASH_SIZE: usize = 32;
/// **CANONICAL PKCS#11 CONSTANTS** - Hardware security module standards
pub mod pkcs11 {
    /// PKCS#11 library search paths for different platforms
    pub const PKCS11_LIBRARY_PATHS: &[&str] = &[
        // Linux paths
        "/usr/lib/pkcs11/libsofthsm2.so",
        "/usr/lib/x86_64-linux-gnu/pkcs11/libsofthsm2.so",
        "/usr/local/lib/softhsm/libsofthsm2.so",
        // macOS paths
        "/opt/homebrew/lib/softhsm/libsofthsm2.so",
        // Windows paths (DLL)
        "softhsm2.dll",
        "C:\\SoftHSM2\\lib\\softhsm2.dll",
    ];
    /// Default PKCS#11 slot ID
    pub const DEFAULT_SLOT_ID: u64 = 0;
    /// Default PKCS#11 user PIN for testing
    pub const DEFAULT_USER_PIN: &str = "1234";
    /// Default PKCS#11 SO PIN for testing
    pub const DEFAULT_SO_PIN: &str = "5678";
/// **CANONICAL EXTENDED SECURITY CONSTANTS** - Advanced security features
pub mod extended {
    /// Entropy pool minimum size
    pub const MIN_ENTROPY_POOL_SIZE: usize = 4096;
    /// Maximum entropy pool size
    pub const MAX_ENTROPY_POOL_SIZE: usize = 65536;
    /// Entropy refresh interval
    pub const ENTROPY_REFRESH_INTERVAL: Duration = Duration::from_secs(3600);
    /// Key rotation interval
    pub const KEY_ROTATION_INTERVAL: Duration = Duration::from_secs(86400 * 7); // 1 week
    /// Audit log retention period
    pub const AUDIT_LOG_RETENTION: Duration = Duration::from_secs(86400 * 365); // 1 year
    /// Maximum failed attempts for advanced authentication
    pub const MAX_ADVANCED_AUTH_ATTEMPTS: u32 = 3;
    /// Security token lifetime
    pub const SECURITY_TOKEN_LIFETIME: Duration = Duration::from_secs(300); // 5 minutes
    /// Biometric template maximum age
    pub const BIOMETRIC_TEMPLATE_MAX_AGE: Duration = Duration::from_secs(86400 * 30);
    // 30 days
/// **CANONICAL SECRET KEY CONSTANTS** - Environment variable names for secrets
/// **MIGRATED FROM**: `beardog-config/src/secrets.rs::secret_keys`
pub mod secret_keys {
    /// Database password secret key identifier
    pub const DATABASE_PASSWORD: &str = "database_password";
    /// Encryption key for data at rest
    pub const ENCRYPTION_KEY: &str = "encryption_key";
    /// API key for external services
    pub const API_KEY: &str = "api_key";
/// **CANONICAL ENVIRONMENT VARIABLE CONSTANTS** - Security-related environment variables
/// **MIGRATED FROM**: `beardog-config/src/secrets.rs::env_vars`
pub mod env_vars {
    /// Database password environment variable
    pub const BEARDOG_DATABASE_PASSWORD: &str = "BEARDOG_DATABASE_PASSWORD";
    /// Encryption key environment variable
    pub const BEARDOG_ENCRYPTION_KEY: &str = "BEARDOG_ENCRYPTION_KEY";
    /// API key environment variable
    pub const BEARDOG_API_KEY: &str = "BEARDOG_API_KEY";
/// **CANONICAL CRYPTOGRAPHIC CONSTANTS** - Encoding and cryptographic parameters
/// **MIGRATED FROM**: `beardog-security/src/address_management.rs`
pub mod crypto {
