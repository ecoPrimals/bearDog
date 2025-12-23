// Multi-Factor Authentication Configuration
//
// Canonical MFA configuration for TOTP, SMS, and other authentication factors.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// **CANONICAL MFA CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalMfaConfig {
    /// Enable MFA system-wide
    /// Whether feature is enabled
    pub enabled: bool,

    /// Whether required is enabled
    pub required: bool,

    /// TOTP configuration
    /// The totp value
    pub totp: TotpConfig,

    /// SMS configuration
    /// The sms value
    pub sms: SmsConfig,

    /// Backup codes configuration
    /// The backup codes value
    pub backup_codes: BackupCodesConfig,
}

impl Default for CanonicalMfaConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            required: false,
            totp: TotpConfig::default(),
            sms: SmsConfig::default(),
            backup_codes: BackupCodesConfig::default(),
        }
    }
}

impl CanonicalMfaConfig {
    /// Production
    #[must_use]
    pub fn production() -> Self {
        Self {
            required: true, // Require MFA in production
            ..Self::default()
        }
    }

    /// Validate
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.enabled {
            self.totp.validate()?;
            self.sms.validate()?;
        }
        Ok(())
    }
}

/// TOTP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Issuer
    /// The issuer value
    pub issuer: String,
    /// Algorithm
    /// The algorithm value
    pub algorithm: String,
    /// Digits
    /// Number of digits
    pub digits: u32,
    /// Period Seconds
    /// Number of `period_seconds`
    pub period_seconds: u32,
}

impl Default for TotpConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            issuer: "BearDog".to_string(),
            algorithm: "SHA1".to_string(),
            digits: 6,
            period_seconds: 30,
        }
    }
}

impl TotpConfig {
    /// Validate
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.enabled && self.issuer.is_empty() {
            return Err(BearDogError::security(
                "TOTP issuer cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

/// SMS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Provider
    pub provider: String,
    /// From Number
    /// The from number value
    pub from_number: String,
}

impl Default for SmsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "twilio".to_string(),
            from_number: String::new(),
        }
    }
}

impl SmsConfig {
    /// Validate
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.enabled && self.from_number.is_empty() {
            return Err(BearDogError::security(
                "SMS from number must be set when SMS is enabled".to_string(),
            ));
        }
        Ok(())
    }
}

/// Backup codes configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupCodesConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Count
    /// Number of items
    pub count: u32,
    /// Length
    /// Number of length
    pub length: u32,
}

impl Default for BackupCodesConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            count: 10,
            length: 8,
        }
    }
}
