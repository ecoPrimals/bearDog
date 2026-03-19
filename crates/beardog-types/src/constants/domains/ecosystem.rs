// SPDX-License-Identifier: AGPL-3.0-only

//! Ecosystem Domain Constants
//!
//! Constants related to ecosystem services, primal identifiers,
//! and inter-service communication.

/// Core primal identifier for BearDog
///
/// Used throughout the ecosystem for service identification
pub const BEARDOG_ID: &str = "beardog";

/// Service type identifiers used in federation and node registry
pub mod service_types {
    /// Security service identifier
    pub const SECURITY: &str = "security";

    /// Phonebook service identifier  
    pub const PHONEBOOK: &str = "phonebook";

    /// Federation service identifier
    pub const FEDERATION: &str = "federation";

    /// Compute service identifier
    pub const COMPUTE: &str = "compute";

    /// Storage service identifier
    pub const STORAGE: &str = "storage";

    /// Relay service identifier
    pub const RELAY: &str = "relay";

    /// Backup service identifier
    pub const BACKUP: &str = "backup";

    /// Monitoring service identifier
    pub const MONITORING: &str = "monitoring";

    /// Analytics service identifier
    pub const ANALYTICS: &str = "analytics";

    /// Gateway service identifier
    pub const GATEWAY: &str = "gateway";
}

/// Version and build information
pub mod version {
    /// BearDog version from Cargo.toml
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");

    /// Software HSM version
    pub const SOFTWARE_HSM_VERSION: &str = "1.0.0";

    /// Workflow system version
    pub const WORKFLOW_SYSTEM_VERSION: &str = "3.1.0";

    /// BearDog mission statement
    pub const MISSION: &str = "Democratizing enterprise-grade security for everyone";
}
