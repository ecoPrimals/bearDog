// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use beardog_types::canonical::configuration::security::{
    BackupCodesConfig, MfaConfig, PasswordPolicyConfig, RateLimitConfig,
    SecurityConfig as UnifiedSecurityConfig, SessionStorage, TotpConfig, UnifiedAuthConfig,
};
