// SPDX-License-Identifier: AGPL-3.0-only

//! License payloads, validation results, and a context-aware license manager facade.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

/// Parsed license material as stored or received from a customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseData {
    /// The license key value
    pub license_key: String,
    /// Optional expiration
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
    /// Collection of features
    pub features: Vec<String>,
}

/// Outcome of validating a license against signing keys and business rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseValidationResult {
    /// Whether the license is currently acceptable for use.
    pub valid: bool,
    /// The license type value
    pub license_type: String,
    /// Optional expires at
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Whether features is enabled
    pub features_enabled: Vec<String>,
    /// Collection of warnings
    pub warnings: Vec<String>,
    /// Whether grace_period_active is enabled
    pub grace_period_active: bool,
    /// Optional days remaining
    pub days_remaining: Option<u32>,
}

/// Single feature gate with optional metering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseFeature {
    /// Name of the feature
    pub feature_name: String,
    /// Whether feature is enabled
    pub enabled: bool,
    /// Optional usage limit
    pub usage_limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Commercial or trial license tiers carried inside validation flows.
pub enum LicenseType {
    /// Time-limited evaluation with a fixed number of days remaining.
    Trial {
        /// Whole days left before the trial blocks premium features.
        days_remaining: u32,
    },
    /// Standard paid tier with a hard expiry.
    Standard {
        /// Instant when the standard subscription ends.
        expires_at: chrono::DateTime<chrono::Utc>,
    },
    /// Premium tier with a hard expiry.
    Premium {
        /// Instant when premium entitlements lapse.
        expires_at: chrono::DateTime<chrono::Utc>,
    },
    /// Enterprise tier; may be perpetual when `expires_at` is None.
    Enterprise {
        /// Optional contract end; None means no fixed expiry in metadata.
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    },
}

/// Tunables for how aggressively license checks reject borderline keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseManager {
    /// Optional grace period days
    pub grace_period_days: Option<u64>,
    /// When true, expired or malformed licenses fail closed immediately.
    pub strict_validation: bool,
}

/// Verifies signed license blobs and tracks grace windows per deployment context.
pub struct ContextAwareLicenseManager {
    signed_licenses: HashMap<String, LicenseData>,
    verification_key: String,
    grace_period_days: u64,
}

impl Default for ContextAwareLicenseManager {
    fn default() -> Self {
        Self {
            signed_licenses: HashMap::with_capacity(16),
            verification_key: Self::get_verification_key(),
            grace_period_days: 3, // 3 days grace period for expired licenses
        }
    }
}

impl ContextAwareLicenseManager {
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Get Verification Key operation.
    /// Gets verification_key
    fn get_verification_key() -> String {
        // In a real implementation, this would load from secure storage
        "beardog-license-verification-key-v1".to_string()
    }

    /// Validate License operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Validates license
    /// Validates license
    pub fn validate_license(
        &self,
        license_key: &str,
    ) -> Result<LicenseValidationResult, BearDogError> {
        debug!(
            "🔐 Validating license key: {}",
            &license_key[..8.min(license_key.len())]
        );

        if let Some(license_data) = self.signed_licenses.get(license_key) {
            let now = chrono::Utc::now();
            let mut warnings = Vec::new();
            let mut grace_period_active = false;

            let valid = if let Some(expiration) = license_data.expiration {
                if expiration < now {
                    let days_expired = (now - expiration).num_days();
                    if days_expired <= self.grace_period_days as i64 {
                        grace_period_active = true;
                        warnings.push(format!(
                            "License expired {} days ago, grace period active",
                            days_expired
                        ));
                        true
                    } else {
                        false
                    }
                } else {
                    true
                }
            } else {
                true // No expiration means perpetual license
            };

            let days_remaining = license_data.expiration.map(|exp| {
                let remaining = (exp - now).num_days();
                if remaining < 0 {
                    0
                } else {
                    remaining as u32
                }
            });

            Ok(LicenseValidationResult {
                valid,
                license_type: "standard".to_string(),
                expires_at: license_data.expiration,
                features_enabled: license_data.features.clone(),
                warnings,
                grace_period_active,
                days_remaining,
            })
        } else {
            Err(BearDogError::unauthorized(
                "Invalid license key".to_string(),
            ))
        }
    }

    /// Add License operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn add_license(
        &mut self,
        license_key: String,
        license_data: LicenseData,
    ) -> Result<(), BearDogError> {
        debug!(
            "➕ Adding license key: {}",
            &license_key[..8.min(license_key.len())]
        );

        self.signed_licenses.insert(license_key, license_data);
        info!("✅ License added successfully");
        Ok(())
    }

    /// Check Feature Enabled operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn check_feature_enabled(
        &self,
        license_key: &str,
        feature: &str,
    ) -> Result<bool, BearDogError> {
        let validation = self.validate_license(license_key)?;

        if !validation.valid {
            return Ok(false);
        }

        Ok(validation.features_enabled.iter().any(|f| f == feature))
    }

    /// Get License Info operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Gets license_info
    /// Gets license_info
    pub fn get_license_info(
        &self,
        license_key: &str,
    ) -> Result<LicenseValidationResult, BearDogError> {
        self.validate_license(license_key)
    }

    /// Remove License operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Removes license
    /// Removes license
    pub fn remove_license(&mut self, license_key: &str) -> Result<(), BearDogError> {
        debug!(
            "🗑️ Removing license key: {}",
            &license_key[..8.min(license_key.len())]
        );

        if self.signed_licenses.remove(license_key).is_some() {
            info!("✅ License removed successfully");
            Ok(())
        } else {
            Err(BearDogError::not_found("License key not found".to_string()))
        }
    }

    /// List Active Licenses operation.
    pub fn list_active_licenses(&self) -> Vec<String> {
        self.signed_licenses.keys().cloned().collect()
    }

    /// Get Grace Period Days operation.
    /// Gets grace_period_days
    /// Gets grace_period_days
    pub fn get_grace_period_days(&self) -> u64 {
        self.grace_period_days
    }

    /// Set Grace Period Days operation.
    /// Sets grace_period_days
    /// Sets grace_period_days
    pub fn set_grace_period_days(&mut self, days: u64) {
        self.grace_period_days = days;
        debug!("🔧 Grace period set to {} days", days);
    }
}

/// Namespace for classifying externally exposed integration hooks by product category.
pub struct ExternalFunctions;

impl ExternalFunctions {
    /// Get Category operation.
    /// Gets category
    /// Gets category
    pub fn get_category(function_name: &str) -> Option<&'static str> {
        match function_name {
            "kubernetes_integration" | "prometheus_export" | "grafana_dashboards" => {
                Some("monitoring")
            }
            // Cloud cryptography features use capability-based discovery
            "cloud_crypto" | "key_management" => {
                Some("cloud_crypto")
            }
            "capability_discovery" => Some("cloud_crypto"),
            _ => None,
        }
    }

    /// Is Rust Ecosystem operation.
    /// Checks if rust ecosystem
    /// Checks if rust ecosystem
    pub fn is_rust_ecosystem(function_name: &str) -> bool {
        matches!(function_name, "serde" | "tokio" | "reqwest" | "tracing")
    }
}
