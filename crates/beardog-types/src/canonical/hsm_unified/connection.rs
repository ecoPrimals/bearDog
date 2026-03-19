// SPDX-License-Identifier: AGPL-3.0-only

// HSM Connection Configuration

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// HSM connection configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmConnectionConfig {
    /// Connection timeout
    pub timeout: Duration,

    /// Max connections
    /// Number of `max_connections`
    pub max_connections: u32,

    /// Connection retry attempts
    /// Number of `retry_attempts`
    pub retry_attempts: u32,
}
