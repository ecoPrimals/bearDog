

use std::time::Duration;

pub mod core {
    use super::Duration;

    pub const MAX_SESSIONS: usize = 10_000;

    pub const SESSION_TIMEOUT: Duration = Duration::from_secs(3600); // 1 hour

    pub const MAX_AUTH_ATTEMPTS: u32 = 5;

    pub const LOCKOUT_DURATION: Duration = Duration::from_secs(900); // 15 minutes

    pub const STANDARD_KEY_SIZE: usize = 32; // 256 bits

    pub const MAX_OPERATION_ATTEMPTS: u32 = 30;

    pub const CRYPTO_CHARSET: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    pub const BASE58_ALPHABET: &[u8] =
        b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    pub const OTP_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
}

pub mod system_security {

    pub const DEFAULT_SESSION_TIMEOUT: Duration = Duration::from_secs(3600);

    pub const DEFAULT_AUTH_TIMEOUT: Duration = Duration::from_secs(300);

    pub const DEFAULT_MAX_FAILED_ATTEMPTS: u32 = 5;

    pub const DEFAULT_LOCKOUT_DURATION: Duration = Duration::from_secs(1800);

    pub const DEFAULT_TOKEN_EXPIRY: Duration = Duration::from_secs(3600);

    pub const DEFAULT_KEY_ROTATION_INTERVAL: Duration = Duration::from_secs(86400);

    pub const DEFAULT_ENCRYPTION_KEY_SIZE: usize = 32;

    pub const DEFAULT_NONCE_SIZE: usize = 12;

    pub const DEFAULT_SIGNATURE_SIZE: usize = 64;

    pub const DEFAULT_PUBLIC_KEY_SIZE: usize = 32;

    pub const DEFAULT_PRIVATE_KEY_SIZE: usize = 32;

    pub const DEFAULT_HASH_SIZE: usize = 32;

pub mod pkcs11 {

    pub const PKCS11_LIBRARY_PATHS: &[&str] = &[

        "/usr/lib/pkcs11/libsofthsm2.so",
        "/usr/lib/x86_64-linux-gnu/pkcs11/libsofthsm2.so",
        "/usr/local/lib/softhsm/libsofthsm2.so",

        "/opt/homebrew/lib/softhsm/libsofthsm2.so",

        "softhsm2.dll",
        "C:\\SoftHSM2\\lib\\softhsm2.dll",
    ];

    pub const DEFAULT_SLOT_ID: u64 = 0;

    pub const DEFAULT_USER_PIN: &str = "1234";

    pub const DEFAULT_SO_PIN: &str = "5678";

pub mod extended {

    pub const MIN_ENTROPY_POOL_SIZE: usize = 4096;

    pub const MAX_ENTROPY_POOL_SIZE: usize = 65536;

    pub const ENTROPY_REFRESH_INTERVAL: Duration = Duration::from_secs(3600);

    pub const KEY_ROTATION_INTERVAL: Duration = Duration::from_secs(86400 * 7); // 1 week

    pub const AUDIT_LOG_RETENTION: Duration = Duration::from_secs(86400 * 365); // 1 year

    pub const MAX_ADVANCED_AUTH_ATTEMPTS: u32 = 3;

    pub const SECURITY_TOKEN_LIFETIME: Duration = Duration::from_secs(300); // 5 minutes

    pub const BIOMETRIC_TEMPLATE_MAX_AGE: Duration = Duration::from_secs(86400 * 30);

pub mod secret_keys {

    pub const DATABASE_PASSWORD: &str = "database_password";

    pub const ENCRYPTION_KEY: &str = "encryption_key";

    pub const API_KEY: &str = "api_key";

pub mod env_vars {

    pub const BEARDOG_DATABASE_PASSWORD: &str = "BEARDOG_DATABASE_PASSWORD";

    pub const BEARDOG_ENCRYPTION_KEY: &str = "BEARDOG_ENCRYPTION_KEY";

    pub const BEARDOG_API_KEY: &str = "BEARDOG_API_KEY";

pub mod crypto {
