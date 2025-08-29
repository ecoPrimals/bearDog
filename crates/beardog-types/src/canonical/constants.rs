//! Canonical constants for the BearDog ecosystem
//!
//! This module re-exports all constants from the unified constants module.
//! All constants are now consolidated in `crates/beardog-types/src/constants/unified.rs`

// Re-export key unified constants (specific imports for better maintainability)
pub use crate::constants::unified::{default_timeout_ms, UnifiedConstantRegistry};

// Provide convenient module-level access
pub mod api {
    pub use crate::constants::unified::api::{
        MAX_REQUEST_SIZE, MISSION, PROJECT_NAME, PROJECT_VERSION, VERSION, VERSION_HEADER,
    };
}

pub mod network {
    pub use crate::constants::unified::network::{addresses, endpoints, limits, ports, timeouts};
}

pub mod security {
    // Re-export security constants from unified module
    pub use crate::constants::unified::security;
}

pub mod performance {
    // Re-export performance constants from unified module
    pub use crate::constants::unified::performance;
}

pub mod hsm {
    // Re-export HSM constants from unified module
    pub use crate::constants::unified::hsm;
}

pub mod cache {
    // Re-export cache constants from unified module
    pub use crate::constants::unified::cache;
}

pub mod nodes {
    // Re-export node constants from unified module
    pub use crate::constants::unified::nodes;
}

pub mod compliance {
    pub use crate::constants::unified::compliance::{gdpr, hipaa, pci_dss, violations};
}

pub mod system {
    pub use crate::constants::unified::system::*;

    // Re-export system sub-modules for convenience
    pub mod environment {
        pub use crate::constants::unified::system::environment::*;
    }

    pub mod features {
        pub use crate::constants::unified::system::features::*;
    }

    pub mod health {
        pub use crate::constants::unified::system::health::*;
    }

    pub mod monitoring {
        pub use crate::constants::unified::system::monitoring::*;
    }
}

// Registry alias for compatibility
pub use crate::constants::unified::UnifiedConstantRegistry as CanonicalConstantRegistry;
