

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use beardog_errors::BearDogError;

pub use beardog_types::canonical::configuration::security::{
    SecurityConfig as UnifiedSecurityConfig,
    MfaConfig,
    TotpConfig,
    BackupCodesConfig,
    PasswordPolicyConfig,
    RateLimitConfig,
    UnifiedAuthConfig,
    SessionStorage,
};
