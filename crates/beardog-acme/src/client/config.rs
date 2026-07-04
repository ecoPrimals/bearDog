// SPDX-License-Identifier: AGPL-3.0-or-later

//! ACME configuration and HTTP client bootstrap.

use beardog_config::env_keys;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

use crate::error::AcmeError;
use crate::rustls_provider;

pub(super) fn build_http_client() -> Result<Client, AcmeError> {
    rustls_provider::assert_installed();
    Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| AcmeError::Config(format!("HTTP client: {e}")))
}

/// ACME client configuration.
#[derive(Debug, Clone)]
pub struct AcmeConfig {
    /// ACME directory URL (e.g., Let's Encrypt production or staging).
    pub directory_url: String,

    /// Domain names to request certificates for.
    pub domains: Vec<String>,

    /// Contact email addresses for account registration.
    pub contacts: Vec<String>,

    /// Port for the HTTP-01 challenge server (default: 80).
    pub challenge_port: u16,

    /// Renewal threshold in days before expiry (default: 30).
    pub renewal_days_before_expiry: u32,

    /// Renewal check interval (default: 12 hours).
    pub check_interval: Duration,
}

impl AcmeConfig {
    /// Load configuration from environment variables.
    ///
    /// # Errors
    ///
    /// Returns an error if required variables are missing.
    pub fn from_env() -> Result<Self, AcmeError> {
        let directory_url = std::env::var(env_keys::ENV_ACME_DIRECTORY)
            .unwrap_or_else(|_| crate::directories::LETS_ENCRYPT_PRODUCTION.to_string());

        let domains: Vec<String> = std::env::var(env_keys::ENV_ACME_DOMAINS)
            .map_err(|_| AcmeError::Config(format!("{} not set", env_keys::ENV_ACME_DOMAINS)))?
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if domains.is_empty() {
            return Err(AcmeError::Config(format!(
                "{} must contain at least one domain",
                env_keys::ENV_ACME_DOMAINS
            )));
        }

        let contacts: Vec<String> = std::env::var(env_keys::ENV_ACME_EMAIL)
            .map(|e| vec![format!("mailto:{e}")])
            .unwrap_or_default();

        let challenge_port = std::env::var(env_keys::ENV_ACME_CHALLENGE_PORT)
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(80);

        let renewal_days = std::env::var(env_keys::ENV_ACME_RENEWAL_DAYS)
            .ok()
            .and_then(|d| d.parse().ok())
            .unwrap_or(30);

        Ok(Self {
            directory_url,
            domains,
            contacts,
            challenge_port,
            renewal_days_before_expiry: renewal_days,
            check_interval: Duration::from_secs(12 * 3600),
        })
    }
}

/// ACME directory endpoints (RFC 8555 §7.1.1).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Directory {
    pub(super) new_nonce: String,
    pub(super) new_account: String,
    pub(super) new_order: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acme_config_defaults() {
        let config = AcmeConfig {
            directory_url: crate::directories::LETS_ENCRYPT_STAGING.to_string(),
            domains: vec!["test.example.com".to_string()],
            contacts: vec!["mailto:test@example.com".to_string()],
            challenge_port: 8080,
            renewal_days_before_expiry: 30,
            check_interval: Duration::from_secs(3600),
        };
        assert_eq!(config.domains.len(), 1);
        assert_eq!(config.challenge_port, 8080);
    }
}
