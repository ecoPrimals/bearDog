// SPDX-License-Identifier: AGPL-3.0-or-later

//! Primal identity resolution (discovered from environment, never hardcoded).

use serde::{Deserialize, Serialize};

/// Inputs for resolving a primal's identity (pure data, no env reads).
#[derive(Debug, Clone, Default)]
pub struct IdentityInputs {
    /// `PRIMAL_NAME`
    pub primal_name: Option<String>,
    /// `BEARDOG_NAME`
    pub beardog_name: Option<String>,
    /// `HOSTNAME`
    pub hostname: Option<String>,
    /// `HOST`
    pub host: Option<String>,
}

impl IdentityInputs {
    /// Read identity inputs from the process environment.
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            primal_name: std::env::var("PRIMAL_NAME").ok(),
            beardog_name: std::env::var("BEARDOG_NAME").ok(),
            hostname: std::env::var("HOSTNAME").ok(),
            host: std::env::var("HOST").ok(),
        }
    }
}

/// Primal identity (discovered, never hardcoded).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalIdentity {
    /// Primal name (from `PRIMAL_NAME` env or config).
    pub name: String,
    /// Instance ID (unique per process).
    pub instance_id: String,
}

impl PrimalIdentity {
    /// Build identity from explicit inputs (no environment reads).
    #[must_use]
    pub fn from_inputs(inputs: &IdentityInputs) -> Self {
        let name = inputs
            .primal_name
            .clone()
            .or_else(|| inputs.beardog_name.clone())
            .unwrap_or_else(|| {
                inputs
                    .hostname
                    .clone()
                    .or_else(|| inputs.host.clone())
                    .unwrap_or_else(|| "local".to_string())
            });

        let hostname = inputs
            .hostname
            .clone()
            .or_else(|| inputs.host.clone())
            .unwrap_or_else(|| "unknown".to_string());

        let pid = std::process::id();
        let instance_id = format!("{hostname}-{pid}");

        Self { name, instance_id }
    }

    /// Read identity from the process environment.
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_inputs(&IdentityInputs::from_env())
    }
}

/// Version information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    /// Cargo package version.
    pub version: String,
    /// Git commit hash (if available).
    pub git_hash: Option<String>,
    /// Build timestamp (if available).
    pub build_time: Option<String>,
}

impl VersionInfo {
    pub(crate) fn discover() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            git_hash: option_env!("GIT_HASH").map(String::from),
            build_time: option_env!("BUILD_TIME").map(String::from),
        }
    }
}
