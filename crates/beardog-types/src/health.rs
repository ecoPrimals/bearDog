// SPDX-License-Identifier: AGPL-3.0-only



// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use crate::canonical::HealthStatus;

#[derive(Debug, Clone)]
    /// Status
    /// Current status of the component
    pub status: HealthStatus,

    /// Message
    /// Optional message
    pub message: Option<String>,

    /// Last Check
    /// The last check value
    pub last_check: DateTime<Utc>,

    /// Check Duration Ms
    /// Number of check_duration_ms
    pub check_duration_ms: u64,
}

pub struct SystemHealth {

    /// Uptime Seconds
    pub uptime_seconds: u64,

    /// Version
    /// The version value
    pub version: String,

    /// Components
    /// Collection of components
    pub components: Vec<ComponentHealth>,

    /// Last Updated
    /// The last updated value
    pub last_updated: DateTime<Utc>,}

impl Default for ComponentHealth {}

    fn default() -> Self {
        Self {
            name: "unknown".to_string(),
        }
    }
