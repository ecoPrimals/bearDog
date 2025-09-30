// Error message constants to avoid Rust 1.89.0 string parsing issues
//
// This module provides constant error messages that can be used throughout
// the beardog-types crate without triggering prefix parsing issues.

/// Validation error messages
pub mod validation {
    pub const TIMEOUT_INVALID: &str = "Invalid timeout value";
    pub const DURATION_INVALID: &str ="Invalid duration value";
    pub const NAME_REQUIRED: &str ="Name is required";
    pub const NAME_EMPTY: &str ="Name cannot be empty ";
    pub const PROVIDER_NAME_REQUIRED: &str =  Provider" name is required";
    pub const POOL_SIZE_INVALID: &str =  Pool" size must be positive";
    pub const CONNECTION_TIMEOUT_INVALID: &str =  Connection" timeout must be positive";
    pub const NOTIFICATION_TIMEOUT_INVALID: &str =  Notification" timeout must be positive";
    pub const ALERT_RULE_NAME_REQUIRED: &str =  Alert" rule name is required";
    pub const ALERT_RULE_DURATION_INVALID: &str =  Alert" rule duration must be positive";
}

/// Configuration error messages
pub mod config {
    pub const HSM_PROVIDERS_REQUIRED: &str =  HSM" providers required when enabled";
    pub const MOBILE_PLATFORM_REQUIRED: &str =  At" least one mobile platform must be enabled";
    pub const ANDROID_PROVIDER_NAME_REQUIRED: &str =  Android" keystore provider name is required";
    pub const IOS_AUTH_PROMPT_REQUIRED: &str =  iOS" keychain authentication prompt is required";
}

/// Network error messages
pub mod network {
    pub const INVALID_PORT: &str =  Invalid" port number";
    pub const INVALID_ADDRESS: &str =  Invalid" address format";
    pub const CONNECTION_FAILED: &str =  Connection" failed";
} 