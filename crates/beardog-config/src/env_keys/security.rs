// SPDX-License-Identifier: AGPL-3.0-or-later

//! Security, crypto, HSM, ACME, vault, and compliance environment variable keys.

// ── Security ─────────────────────────────────────────────────────────

/// Minimum TLS version (e.g. `1.2`, `1.3`).
pub const ENV_MIN_TLS_VERSION: &str = "BEARDOG_MIN_TLS_VERSION";
/// TLS mode for server startup.
pub const ENV_TLS_MODE: &str = "BEARDOG_TLS_MODE";
/// PEM certificate chain path for TLS termination.
pub const ENV_TLS_CERT_PATH: &str = "BEARDOG_TLS_CERT_PATH";
/// PEM private key path for TLS termination.
pub const ENV_TLS_KEY_PATH: &str = "BEARDOG_TLS_KEY_PATH";
/// Enable strict security mode.
pub const ENV_STRICT_MODE: &str = "BEARDOG_STRICT_MODE";
/// Require mutual TLS for connections.
pub const ENV_REQUIRE_MTLS: &str = "BEARDOG_REQUIRE_MTLS";
/// Allow loopback connections without authentication.
pub const ENV_ALLOW_LOCALHOST_BYPASS: &str = "BEARDOG_ALLOW_LOCALHOST_BYPASS";
/// Enable security audit logging.
pub const ENV_ENABLE_AUDIT_LOG: &str = "BEARDOG_ENABLE_AUDIT_LOG";
/// Enable request rate limiting.
pub const ENV_ENABLE_RATE_LIMITING: &str = "BEARDOG_ENABLE_RATE_LIMITING";
/// Per-IP max connections per rate-limit window.
pub const ENV_RATE_LIMIT_MAX_CONN: &str = "BEARDOG_RATE_LIMIT_MAX_CONN";
/// Maximum requests per minute for rate limiting.
pub const ENV_RATE_LIMIT_MAX_REQUESTS_PER_MIN: &str = "BEARDOG_RATE_LIMIT_MAX_REQUESTS_PER_MIN";
/// Burst capacity for rate limiting.
pub const ENV_RATE_LIMIT_BURST_CAPACITY: &str = "BEARDOG_RATE_LIMIT_BURST_CAPACITY";
/// Rate-limit sliding window duration (seconds).
pub const ENV_RATE_LIMIT_WINDOW_SECS: &str = "BEARDOG_RATE_LIMIT_WINDOW_SECS";
/// Global max concurrent connections before rate limiting rejects.
pub const ENV_RATE_LIMIT_MAX_TOTAL: &str = "BEARDOG_RATE_LIMIT_MAX_TOTAL";
/// Automatically block suspicious source IPs.
pub const ENV_AUTO_BLOCK_SUSPICIOUS_IPS: &str = "BEARDOG_AUTO_BLOCK_SUSPICIOUS_IPS";
/// Require authentication for all endpoints.
pub const ENV_REQUIRE_AUTHENTICATION: &str = "BEARDOG_REQUIRE_AUTHENTICATION";
/// `MethodGate` enforcement mode (`permissive` or `enforced`).
pub const ENV_AUTH_MODE: &str = "BEARDOG_AUTH_MODE";
/// BTSP `BirdSong` HSM key label.
pub const ENV_BTSP_BIRDSONG_KEY_LABEL: &str = "BEARDOG_BTSP_BIRDSONG_KEY_LABEL";
/// BTSP lineage root prefix.
pub const ENV_BTSP_LINEAGE_ROOT_PREFIX: &str = "BEARDOG_BTSP_LINEAGE_ROOT_PREFIX";
/// BTSP maximum lineage depth.
pub const ENV_BTSP_LINEAGE_MAX_DEPTH: &str = "BEARDOG_BTSP_LINEAGE_MAX_DEPTH";
/// Enable RBAC authorization.
pub const ENV_AUTHZ_RBAC_ENABLED: &str = "BEARDOG_AUTHZ_RBAC_ENABLED";
/// Enable ABAC authorization.
pub const ENV_AUTHZ_ABAC_ENABLED: &str = "BEARDOG_AUTHZ_ABAC_ENABLED";
/// Default authorization role for new principals.
pub const ENV_AUTHZ_DEFAULT_ROLE: &str = "BEARDOG_AUTHZ_DEFAULT_ROLE";
/// Permission cache TTL (seconds).
pub const ENV_AUTHZ_CACHE_TIMEOUT_SECS: &str = "BEARDOG_AUTHZ_CACHE_TIMEOUT_SECS";
/// Enable capability-based authorization.
pub const ENV_AUTHZ_CAPABILITY_ENABLED: &str = "BEARDOG_AUTHZ_CAPABILITY_ENABLED";
/// Enable legacy authentication integration.
pub const ENV_LEGACY_INTEGRATION_ENABLED: &str = "BEARDOG_LEGACY_INTEGRATION_ENABLED";
/// Legacy integration request timeout (seconds).
pub const ENV_LEGACY_TIMEOUT_SECS: &str = "BEARDOG_LEGACY_TIMEOUT_SECS";
/// Deadline for migrating off legacy auth (ISO date string).
pub const ENV_LEGACY_MIGRATION_DEADLINE: &str = "BEARDOG_LEGACY_MIGRATION_DEADLINE";
/// Enable security health monitoring.
pub const ENV_HEALTH_MONITORING_ENABLED: &str = "BEARDOG_HEALTH_MONITORING_ENABLED";
/// Enable automatic health remediation.
pub const ENV_HEALTH_AUTO_REMEDIATION_ENABLED: &str = "BEARDOG_HEALTH_AUTO_REMEDIATION_ENABLED";
/// Enable security event monitoring.
pub const ENV_SECURITY_MONITORING_ENABLED: &str = "BEARDOG_SECURITY_MONITORING_ENABLED";
/// Security monitoring poll interval (seconds).
pub const ENV_SECURITY_MONITORING_INTERVAL_SECS: &str = "BEARDOG_SECURITY_MONITORING_INTERVAL_SECS";
/// Security alert rate limit (alerts per hour).
pub const ENV_SECURITY_ALERT_RATE_LIMIT_PER_HOUR: &str =
    "BEARDOG_SECURITY_ALERT_RATE_LIMIT_PER_HOUR";
/// Enable trust score decay over time.
pub const ENV_TRUST_DECAY_ENABLED: &str = "BEARDOG_TRUST_DECAY_ENABLED";
/// Trust decay rate per interval (0.0–1.0).
pub const ENV_TRUST_DECAY_RATE: &str = "BEARDOG_TRUST_DECAY_RATE";
/// Trust decay interval (seconds).
pub const ENV_TRUST_DECAY_INTERVAL_SECS: &str = "BEARDOG_TRUST_DECAY_INTERVAL_SECS";
/// Minimum trust score floor (0.0–1.0).
pub const ENV_TRUST_MINIMUM: &str = "BEARDOG_TRUST_MINIMUM";
/// Trust evaluation algorithm name.
pub const ENV_TRUST_EVAL_ALGORITHM: &str = "BEARDOG_TRUST_EVAL_ALGORITHM";
/// Trust evaluation timeout (seconds).
pub const ENV_TRUST_EVAL_TIMEOUT_SECS: &str = "BEARDOG_TRUST_EVAL_TIMEOUT_SECS";
/// Enable compliance validation checks.
pub const ENV_COMPLIANCE_VALIDATION_ENABLED: &str = "BEARDOG_COMPLIANCE_VALIDATION_ENABLED";
/// Compliance validation frequency (hours).
pub const ENV_COMPLIANCE_VALIDATION_FREQUENCY_HOURS: &str =
    "BEARDOG_COMPLIANCE_VALIDATION_FREQUENCY_HOURS";
/// Compliance strictness level (`low`, `moderate`, `high`, `strict`).
pub const ENV_COMPLIANCE_STRICTNESS: &str = "BEARDOG_COMPLIANCE_STRICTNESS";
/// Enable automatic compliance remediation.
pub const ENV_COMPLIANCE_AUTO_REMEDIATION: &str = "BEARDOG_COMPLIANCE_AUTO_REMEDIATION";
/// Enable compliance reporting.
pub const ENV_COMPLIANCE_REPORTING_ENABLED: &str = "BEARDOG_COMPLIANCE_REPORTING_ENABLED";
/// Compliance report frequency (days).
pub const ENV_COMPLIANCE_REPORT_FREQUENCY_DAYS: &str = "BEARDOG_COMPLIANCE_REPORT_FREQUENCY_DAYS";
/// Enable data sovereignty validation.
pub const ENV_SOVEREIGNTY_VALIDATION_ENABLED: &str = "BEARDOG_SOVEREIGNTY_VALIDATION_ENABLED";
/// Sovereignty validation frequency (hours).
pub const ENV_SOVEREIGNTY_VALIDATION_FREQUENCY_HOURS: &str =
    "BEARDOG_SOVEREIGNTY_VALIDATION_FREQUENCY_HOURS";
/// Data sovereignty enforcement level.
pub const ENV_SOVEREIGNTY_ENFORCEMENT: &str = "BEARDOG_SOVEREIGNTY_ENFORCEMENT";
/// Cryptographic key rotation interval (seconds).
pub const ENV_KEY_ROTATION_INTERVAL_SECS: &str = "BEARDOG_KEY_ROTATION_INTERVAL_SECS";
/// Genetics integration strength (0.0–1.0).
pub const ENV_GENETICS_INTEGRATION_STRENGTH: &str = "BEARDOG_GENETICS_INTEGRATION_STRENGTH";
/// Genetics validation timeout (seconds).
pub const ENV_GENETICS_VALIDATION_TIMEOUT_SECS: &str = "BEARDOG_GENETICS_VALIDATION_TIMEOUT_SECS";
/// Development rate-limit max requests override.
pub const ENV_RATE_LIMITING_MAX_REQUESTS: &str = "BEARDOG_RATE_LIMITING_MAX_REQUESTS";
/// Development audit log retention (days).
pub const ENV_DEV_AUDIT_RETENTION_DAYS: &str = "BEARDOG_DEV_AUDIT_RETENTION_DAYS";
/// Production audit log retention (days).
pub const ENV_PRODUCTION_AUDIT_RETENTION_DAYS: &str = "BEARDOG_PRODUCTION_AUDIT_RETENTION_DAYS";
/// JWT signing secret.
pub const ENV_JWT_SECRET: &str = "BEARDOG_JWT_SECRET";
/// JWT access token expiration (seconds).
pub const ENV_JWT_EXPIRY_SECS: &str = "BEARDOG_JWT_EXPIRY_SECS";
/// Enable JWT refresh tokens.
pub const ENV_JWT_ENABLE_REFRESH: &str = "BEARDOG_JWT_ENABLE_REFRESH";
/// JWT refresh token expiration (seconds).
pub const ENV_JWT_REFRESH_EXPIRY_SECS: &str = "BEARDOG_JWT_REFRESH_EXPIRY_SECS";
/// `OAuth2` client identifier.
pub const ENV_OAUTH_CLIENT_ID: &str = "BEARDOG_OAUTH_CLIENT_ID";
/// `OAuth2` client secret.
pub const ENV_OAUTH_CLIENT_SECRET: &str = "BEARDOG_OAUTH_CLIENT_SECRET";
/// `OAuth2` redirect URI.
pub const ENV_OAUTH_REDIRECT_URI: &str = "BEARDOG_OAUTH_REDIRECT_URI";
/// OAuth callback URL override.
pub const ENV_AUTH_CALLBACK: &str = "BEARDOG_AUTH_CALLBACK";
/// Authenticated session timeout (seconds).
pub const ENV_AUTH_SESSION_TIMEOUT_SECS: &str = "BEARDOG_AUTH_SESSION_TIMEOUT_SECS";
/// Behavioral verification mode selector.
pub const ENV_BEHAVIORAL_VERIFICATION: &str = "BEARDOG_BEHAVIORAL_VERIFICATION";
/// Genetic evolution activation threshold.
pub const ENV_EVOLUTION_THRESHOLD: &str = "BEARDOG_EVOLUTION_THRESHOLD";
/// Genetic evolution interval (seconds).
pub const ENV_EVOLUTION_INTERVAL_SECS: &str = "BEARDOG_EVOLUTION_INTERVAL_SECS";
/// Genetic renewal mutation rate (0.0–1.0).
pub const ENV_GENETIC_RENEWAL_MUTATION_RATE: &str = "BEARDOG_GENETIC_RENEWAL_MUTATION_RATE";
/// Genetic renewal crossover rate (0.0–1.0).
pub const ENV_GENETIC_RENEWAL_CROSSOVER_RATE: &str = "BEARDOG_GENETIC_RENEWAL_CROSSOVER_RATE";
/// Genetic renewal selection pressure (0.0–1.0).
pub const ENV_GENETIC_RENEWAL_SELECTION_PRESSURE: &str =
    "BEARDOG_GENETIC_RENEWAL_SELECTION_PRESSURE";
/// Genetic renewal frequency (hours).
pub const ENV_GENETIC_RENEWAL_FREQUENCY_HOURS: &str = "BEARDOG_GENETIC_RENEWAL_FREQUENCY_HOURS";
/// Production rate-limit max requests per minute.
pub const ENV_PROD_RATE_LIMIT_MAX_REQUESTS_PER_MIN: &str =
    "BEARDOG_PROD_RATE_LIMIT_MAX_REQUESTS_PER_MIN";
/// Production rate-limit burst capacity.
pub const ENV_PROD_RATE_LIMIT_BURST_CAPACITY: &str = "BEARDOG_PROD_RATE_LIMIT_BURST_CAPACITY";
/// Production rate-limit window (seconds).
pub const ENV_PROD_RATE_LIMIT_WINDOW_SECS: &str = "BEARDOG_PROD_RATE_LIMIT_WINDOW_SECS";
/// Development compliance audit retention (days).
pub const ENV_COMPLIANCE_DEV_AUDIT_RETENTION_DAYS: &str =
    "BEARDOG_COMPLIANCE_DEV_AUDIT_RETENTION_DAYS";
/// Development compliance audit frequency (hours).
pub const ENV_COMPLIANCE_DEV_AUDIT_FREQUENCY_HOURS: &str =
    "BEARDOG_COMPLIANCE_DEV_AUDIT_FREQUENCY_HOURS";
// ── Crypto ───────────────────────────────────────────────────────────

/// RSA key size (bits).
pub const ENV_RSA_KEY_SIZE: &str = "BEARDOG_RSA_KEY_SIZE";
/// Elliptic curve name.
pub const ENV_EC_CURVE: &str = "BEARDOG_EC_CURVE";
/// AES key size (bits).
pub const ENV_AES_KEY_SIZE: &str = "BEARDOG_AES_KEY_SIZE";
/// Hash algorithm name.
pub const ENV_HASH_ALGORITHM: &str = "BEARDOG_HASH_ALGORITHM";
/// PBKDF2 iteration count.
pub const ENV_PBKDF2_ITERATIONS: &str = "BEARDOG_PBKDF2_ITERATIONS";

// ── HSM ──────────────────────────────────────────────────────────────

/// Enable HSM auto-detection at startup.
pub const ENV_HSM_AUTO_DETECT: &str = "BEARDOG_HSM_AUTO_DETECT";
/// Prefer hardware HSM over software fallback.
pub const ENV_HSM_PREFER_HARDWARE: &str = "BEARDOG_HSM_PREFER_HARDWARE";
/// Enable `SoftHSM` backend.
pub const ENV_HSM_ENABLE_SOFTHSM: &str = "BEARDOG_HSM_ENABLE_SOFTHSM";
/// Enable `YubiHSM` backend.
pub const ENV_HSM_ENABLE_YUBIHSM: &str = "BEARDOG_HSM_ENABLE_YUBIHSM";
/// Enable TPM backend.
pub const ENV_HSM_ENABLE_TPM: &str = "BEARDOG_HSM_ENABLE_TPM";
/// Enable Android `StrongBox` backend.
pub const ENV_HSM_ENABLE_STRONGBOX: &str = "BEARDOG_HSM_ENABLE_STRONGBOX";
/// `YubiHSM` connector URL.
pub const ENV_YUBIHSM_CONNECTOR: &str = "BEARDOG_YUBIHSM_CONNECTOR";
/// TPM device path (e.g. `/dev/tpm0`).
pub const ENV_TPM_DEVICE: &str = "BEARDOG_TPM_DEVICE";
/// Software HSM storage directory.
pub const ENV_HSM_STORAGE: &str = "BEARDOG_HSM_STORAGE";
/// Enable hardware HSM at runtime.
pub const ENV_ENABLE_HARDWARE_HSM: &str = "BEARDOG_ENABLE_HARDWARE_HSM";
/// Maximum HSM key slots.
pub const ENV_HSM_MAX_KEYS: &str = "BEARDOG_HSM_MAX_KEYS";
/// Maximum HSM operations per second.
pub const ENV_HSM_MAX_OPS_PER_SEC: &str = "BEARDOG_HSM_MAX_OPS_PER_SEC";
/// HSM connection pool size.
pub const ENV_HSM_POOL_SIZE: &str = "BEARDOG_HSM_POOL_SIZE";
/// HSM batch operation size.
pub const ENV_HSM_BATCH_SIZE: &str = "BEARDOG_HSM_BATCH_SIZE";
/// HSM operation cache size.
pub const ENV_HSM_CACHE_SIZE: &str = "BEARDOG_HSM_CACHE_SIZE";
/// Hardware HSM connection timeout (seconds).
pub const ENV_HSM_HARDWARE_CONNECTION_TIMEOUT_SECS: &str =
    "BEARDOG_HSM_HARDWARE_CONNECTION_TIMEOUT_SECS";
/// Hardware HSM connection max retries.
pub const ENV_HSM_HARDWARE_MAX_RETRIES: &str = "BEARDOG_HSM_HARDWARE_MAX_RETRIES";
/// Comma-separated CIDR ranges for network HSM discovery (e.g. `"10.0.0.0/8,192.168.1.0/24"`).
pub const ENV_HSM_NETWORK_SCAN_RANGES: &str = "BEARDOG_HSM_NETWORK_SCAN_RANGES";
/// Network HSM scan timeout in milliseconds.
pub const ENV_HSM_NETWORK_SCAN_TIMEOUT_MS: &str = "BEARDOG_HSM_NETWORK_SCAN_TIMEOUT_MS";
/// Maximum parallel network HSM scan threads.
pub const ENV_HSM_NETWORK_SCAN_PARALLEL: &str = "BEARDOG_HSM_NETWORK_SCAN_PARALLEL";

// ── ACME ─────────────────────────────────────────────────────────────

/// ACME directory URL.
pub const ENV_ACME_DIRECTORY: &str = "BEARDOG_ACME_DIRECTORY";
/// ACME email contact.
pub const ENV_ACME_EMAIL: &str = "BEARDOG_ACME_EMAIL";
/// ACME domains (comma-separated).
pub const ENV_ACME_DOMAINS: &str = "BEARDOG_ACME_DOMAINS";
/// ACME HTTP-01 challenge bind port.
pub const ENV_ACME_CHALLENGE_PORT: &str = "BEARDOG_ACME_CHALLENGE_PORT";
/// Days before certificate expiry to trigger ACME renewal.
pub const ENV_ACME_RENEWAL_DAYS: &str = "BEARDOG_ACME_RENEWAL_DAYS";
// ── HSM (extended) ───────────────────────────────────────────────────

/// HSM connection timeout (milliseconds).
pub const ENV_HSM_CONNECTION_TIMEOUT_MS: &str = "BEARDOG_HSM_CONNECTION_TIMEOUT_MS";
/// HSM connection max retries.
pub const ENV_HSM_CONNECTION_MAX_RETRIES: &str = "BEARDOG_HSM_CONNECTION_MAX_RETRIES";
/// HSM connection retry delay (milliseconds).
pub const ENV_HSM_CONNECTION_RETRY_DELAY_MS: &str = "BEARDOG_HSM_CONNECTION_RETRY_DELAY_MS";
/// HSM keep-alive interval (seconds).
pub const ENV_HSM_KEEP_ALIVE_SECS: &str = "BEARDOG_HSM_KEEP_ALIVE_SECS";
/// HSM session timeout (seconds).
pub const ENV_HSM_SESSION_TIMEOUT_SECS: &str = "BEARDOG_HSM_SESSION_TIMEOUT_SECS";
/// HSM storage path override.
pub const ENV_HSM_STORAGE_PATH: &str = "BEARDOG_HSM_STORAGE_PATH";
/// HSM server address.
pub const ENV_HSM_SERVER: &str = "BEARDOG_HSM_SERVER";
/// HSM server port.
pub const ENV_HSM_PORT: &str = "BEARDOG_HSM_PORT";
/// HSM tier evaluation interval (seconds).
pub const ENV_HSM_TIER_EVALUATION_INTERVAL_SECS: &str = "BEARDOG_HSM_TIER_EVALUATION_INTERVAL_SECS";
/// HSM health check interval (seconds).
pub const ENV_HSM_HEALTH_CHECK_INTERVAL_SECS: &str = "BEARDOG_HSM_HEALTH_CHECK_INTERVAL_SECS";
/// HSM health check timeout (seconds).
pub const ENV_HSM_HEALTH_CHECK_TIMEOUT_SECS: &str = "BEARDOG_HSM_HEALTH_CHECK_TIMEOUT_SECS";
/// HSM health failure threshold.
pub const ENV_HSM_HEALTH_FAILURE_THRESHOLD: &str = "BEARDOG_HSM_HEALTH_FAILURE_THRESHOLD";
/// HSM health recovery threshold.
pub const ENV_HSM_HEALTH_RECOVERY_THRESHOLD: &str = "BEARDOG_HSM_HEALTH_RECOVERY_THRESHOLD";
/// HSM failover timeout (seconds).
pub const ENV_HSM_FAILOVER_TIMEOUT_SECS: &str = "BEARDOG_HSM_FAILOVER_TIMEOUT_SECS";
/// HSM failback delay (seconds).
pub const ENV_HSM_FAILBACK_DELAY_SECS: &str = "BEARDOG_HSM_FAILBACK_DELAY_SECS";
/// HSM max concurrent sessions.
pub const ENV_HSM_MAX_CONCURRENT_SESSIONS: &str = "BEARDOG_HSM_MAX_CONCURRENT_SESSIONS";
/// HSM backup interval (seconds).
pub const ENV_HSM_BACKUP_INTERVAL_SECS: &str = "BEARDOG_HSM_BACKUP_INTERVAL_SECS";
/// HSM backup retention (seconds).
pub const ENV_HSM_BACKUP_RETENTION_SECS: &str = "BEARDOG_HSM_BACKUP_RETENTION_SECS";

// ── HSM config (runtime tuning) ────────────────────────────────────────

/// Default HSM operation timeout (seconds).
pub const ENV_HSM_DEFAULT_TIMEOUT_SECS: &str = "BEARDOG_HSM_DEFAULT_TIMEOUT_SECS";
/// HSM connection pool size.
pub const ENV_HSM_CONNECTION_POOL_SIZE: &str = "BEARDOG_HSM_CONNECTION_POOL_SIZE";
/// HSM max retry attempts.
pub const ENV_HSM_MAX_RETRIES: &str = "BEARDOG_HSM_MAX_RETRIES";
/// HSM retry initial delay (milliseconds).
pub const ENV_HSM_RETRY_INITIAL_DELAY_MS: &str = "BEARDOG_HSM_RETRY_INITIAL_DELAY_MS";
/// HSM retry maximum delay (seconds).
pub const ENV_HSM_RETRY_MAX_DELAY_SECS: &str = "BEARDOG_HSM_RETRY_MAX_DELAY_SECS";
/// HSM operational mode.
pub const ENV_HSM_MODE: &str = "BEARDOG_HSM_MODE";
/// HSM auto-initialization flag.
pub const ENV_HSM_AUTO_INIT: &str = "BEARDOG_HSM_AUTO_INIT";
/// Colon-separated HSM library search paths.
pub const ENV_HSM_LIBRARY_PATHS: &str = "BEARDOG_HSM_LIBRARY_PATHS";
/// HSM audit log directory.
pub const ENV_AUDIT_DIR: &str = "BEARDOG_AUDIT_DIR";
/// `SoftHSM2` configuration file path.
pub const ENV_SOFTHSM2_CONF: &str = "SOFTHSM2_CONF";
/// `YubiHSM` connector URL (unprefixed alias).
pub const ENV_YUBIHSM_CONNECTOR_URL: &str = "YUBIHSM_CONNECTOR_URL";

// ── Android HSM ──────────────────────────────────────────────────────

/// Android device manufacturer.
pub const ENV_ANDROID_MANUFACTURER: &str = "ANDROID_MANUFACTURER";
/// Android device model.
pub const ENV_ANDROID_MODEL: &str = "ANDROID_MODEL";
/// Android device codename.
pub const ENV_ANDROID_DEVICE: &str = "ANDROID_DEVICE";
/// Android hardware platform.
pub const ENV_ANDROID_HARDWARE: &str = "ANDROID_HARDWARE";
/// Android board name.
pub const ENV_ANDROID_BOARD: &str = "ANDROID_BOARD";
/// Android brand name.
pub const ENV_ANDROID_BRAND: &str = "ANDROID_BRAND";
/// Android OS version string.
pub const ENV_ANDROID_VERSION: &str = "ANDROID_VERSION";
/// Android API level.
pub const ENV_ANDROID_API_LEVEL: &str = "ANDROID_API_LEVEL";
/// Android security patch level.
pub const ENV_ANDROID_SECURITY_PATCH: &str = "ANDROID_SECURITY_PATCH";
/// Android `StrongBox` version string.
pub const ENV_ANDROID_STRONGBOX_VERSION: &str = "ANDROID_STRONGBOX_VERSION";
/// Android Titan M version string.
pub const ENV_ANDROID_TITAN_M_VERSION: &str = "ANDROID_TITAN_M_VERSION";
/// Android `StrongBox` availability flag.
pub const ENV_ANDROID_STRONGBOX_AVAILABLE: &str = "ANDROID_STRONGBOX_AVAILABLE";
/// Android Titan M availability flag.
pub const ENV_ANDROID_TITAN_M_AVAILABLE: &str = "ANDROID_TITAN_M_AVAILABLE";
/// `StrongBox` mock availability flag (test/dev).
pub const ENV_STRONGBOX_MOCK_AVAILABLE: &str = "STRONGBOX_MOCK_AVAILABLE";
/// Keystore backend selection (`memory`, `keymaster`).
pub const ENV_KEYSTORE_BACKEND: &str = "BEARDOG_KEYSTORE_BACKEND";

// ── iOS HSM ───────────────────────────────────────────────────────────

/// iOS version string.
pub const ENV_IOS_VERSION: &str = "IOS_VERSION";
/// iOS device model identifier.
pub const ENV_IOS_DEVICE_MODEL: &str = "IOS_DEVICE_MODEL";
/// Touch ID availability flag.
pub const ENV_HAS_TOUCH_ID: &str = "HAS_TOUCH_ID";
/// Face ID availability flag.
pub const ENV_HAS_FACE_ID: &str = "HAS_FACE_ID";
/// T2 security chip availability flag.
pub const ENV_HAS_T2_CHIP: &str = "HAS_T2_CHIP";
/// Apple Silicon availability flag.
pub const ENV_HAS_APPLE_SILICON: &str = "HAS_APPLE_SILICON";

// ── iOS HSM (runtime detection) ──────────────────────────────────────────

/// Touch ID availability flag.
pub const ENV_IOS_TOUCH_ID_AVAILABLE: &str = "IOS_TOUCH_ID_AVAILABLE";
/// Face ID availability flag.
pub const ENV_IOS_FACE_ID_AVAILABLE: &str = "IOS_FACE_ID_AVAILABLE";
/// Secure Enclave availability flag.
pub const ENV_IOS_SECURE_ENCLAVE_AVAILABLE: &str = "IOS_SECURE_ENCLAVE_AVAILABLE";
/// Biometric authentication availability flag.
pub const ENV_IOS_BIOMETRIC_AVAILABLE: &str = "IOS_BIOMETRIC_AVAILABLE";
/// iOS device type identifier.
pub const ENV_IOS_DEVICE_TYPE: &str = "IOS_DEVICE_TYPE";
/// Generic device type identifier.
pub const ENV_DEVICE_TYPE: &str = "DEVICE_TYPE";
/// Android `StrongBox` feature flag (short form).
pub const ENV_ANDROID_STRONGBOX: &str = "ANDROID_STRONGBOX";
/// `StrongBox` availability flag (generic).
pub const ENV_STRONGBOX_AVAILABLE: &str = "STRONGBOX_AVAILABLE";
/// Android package name for deployment.
pub const ENV_PACKAGE_NAME: &str = "BEARDOG_PACKAGE_NAME";
/// Logcat follow timeout (seconds) for Android deployment.
pub const ENV_LOGCAT_FOLLOW_SECS: &str = "BEARDOG_LOGCAT_FOLLOW_SECS";

// ── Crypto service ───────────────────────────────────────────────────────

/// Crypto service display name.
pub const ENV_CRYPTO_SERVICE_NAME: &str = "BEARDOG_CRYPTO_SERVICE_NAME";
/// Enable HSM integration in crypto service.
pub const ENV_CRYPTO_HSM_ENABLED: &str = "BEARDOG_CRYPTO_HSM_ENABLED";
/// Enable genetics integration in crypto service.
pub const ENV_CRYPTO_GENETIC_ENABLED: &str = "BEARDOG_CRYPTO_GENETIC_ENABLED";
/// Crypto service max data size (bytes).
pub const ENV_CRYPTO_MAX_DATA_SIZE: &str = "BEARDOG_CRYPTO_MAX_DATA_SIZE";
/// Enable crypto service audit logging.
pub const ENV_CRYPTO_AUDIT_ENABLED: &str = "BEARDOG_CRYPTO_AUDIT_ENABLED";
/// RSA key generation mode.
pub const ENV_RSA_KEY_MODE: &str = "BEARDOG_RSA_KEY_MODE";
/// Application master encryption key.
pub const ENV_MASTER_KEY: &str = "BEARDOG_MASTER_KEY";
/// HSM master encryption key.
pub const ENV_HSM_MASTER_KEY: &str = "BEARDOG_HSM_MASTER_KEY";

// ── Vault / secrets ──────────────────────────────────────────────────────

/// `HashiCorp` Vault endpoint URL.
pub const ENV_VAULT_ENDPOINT: &str = "VAULT_ENDPOINT";
/// `HashiCorp` Vault authentication token.
pub const ENV_VAULT_TOKEN: &str = "VAULT_TOKEN";

// ── External cloud auth detection ────────────────────────────────────────

/// AWS access key ID (unprefixed).
pub const ENV_AWS_ACCESS_KEY_ID: &str = "AWS_ACCESS_KEY_ID";
/// AWS deployment region (unprefixed).
pub const ENV_AWS_REGION: &str = "AWS_REGION";
/// Google OAuth client secret (unprefixed).
pub const ENV_GOOGLE_CLIENT_SECRET: &str = "GOOGLE_CLIENT_SECRET";
/// GitHub OAuth client secret (unprefixed).
pub const ENV_GITHUB_CLIENT_SECRET: &str = "GITHUB_CLIENT_SECRET";
/// Azure client ID (unprefixed).
pub const ENV_AZURE_CLIENT_ID: &str = "AZURE_CLIENT_ID";
/// Google application credentials path (unprefixed).
pub const ENV_GOOGLE_APPLICATION_CREDENTIALS: &str = "GOOGLE_APPLICATION_CREDENTIALS";

/// Registry socket path fallback override.
pub const ENV_REGISTRY_SOCKET_FALLBACK: &str = "BEARDOG_REGISTRY_SOCKET_FALLBACK";

// ── Compliance ──────────────────────────────────────────────────────────

/// Audit log retention period in days.
pub const ENV_COMPLIANCE_AUDIT_RETENTION_DAYS: &str = "BEARDOG_COMPLIANCE_AUDIT_RETENTION_DAYS";
/// Audit frequency in hours.
pub const ENV_COMPLIANCE_AUDIT_FREQUENCY_HOURS: &str = "BEARDOG_COMPLIANCE_AUDIT_FREQUENCY_HOURS";

// ── iOS device (extended) ────────────────────────────────────────────────

/// iOS device model identifier (unprefixed).
pub const ENV_IOS_MODEL: &str = "IOS_MODEL";
