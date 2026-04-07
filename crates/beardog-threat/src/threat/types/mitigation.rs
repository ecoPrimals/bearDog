// SPDX-License-Identifier: AGPL-3.0-or-later

//! Mitigation timeline steps attached to threat events.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// One recorded action taken while containing or remediating a threat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationStep {
    /// Step identifier
    pub id: String,
    /// Action taken
    /// The action value
    pub action: String,
    /// When the mitigation action started or completed (implementation-defined).
    pub timestamp: SystemTime,
    /// Result of the action
    /// The result value
    pub result: String,
    /// Success status
    /// Whether success is enabled
    pub success: bool,
}

impl MitigationStep {
    /// Create a new mitigation step
    /// Creates a new instance
    #[must_use]
    pub fn new(id: String, action: String, result: String, success: bool) -> Self {
        Self {
            id,
            action,
            timestamp: SystemTime::now(),
            result,
            success,
        }
    }
}
