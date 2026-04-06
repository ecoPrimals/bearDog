// SPDX-License-Identifier: AGPL-3.0-or-later

// HSM Security Configuration

use serde::{Deserialize, Serialize};

/// HSM security configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmSecurityConfig {
    /// Authentication required
    /// Whether `authentication_required` is enabled
    pub authentication_required: bool,

    /// Access control enabled
    /// Whether `access_control` is enabled
    pub access_control_enabled: bool,

    /// Audit logging enabled
    /// Whether `audit_logging` is enabled
    pub audit_logging_enabled: bool,
}
