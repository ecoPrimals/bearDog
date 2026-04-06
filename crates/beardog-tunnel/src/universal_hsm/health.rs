// SPDX-License-Identifier: AGPL-3.0-or-later

//! Universal HSM Health Module
//!
//! Health monitoring for universal HSM providers

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Whether healthy
    pub is_healthy: bool,
    /// Status message
    pub message: String,
}

/// Health check
pub async fn check_health() -> HealthStatus {
    HealthStatus {
        is_healthy: true,
        message: "OK".to_string(),
    }
}

