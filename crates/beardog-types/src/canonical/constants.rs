//! Canonical constants for the BearDog ecosystem
//!
//! This module re-exports all constants from the unified constants module.
//! All constants are now consolidated in `crates/beardog-types/src/constants/unified.rs`

// Re-export all unified constants
pub use crate::constants::unified::*;

// Provide convenient module-level access
pub mod api {
    pub use crate::constants::unified::api::*;
}

pub mod network {
    pub use crate::constants::unified::network::*;
}

pub mod security {
    pub use crate::constants::unified::security::*;
}

pub mod performance {
    pub use crate::constants::unified::performance::*;
}

pub mod hsm {
    pub use crate::constants::unified::hsm::*;
}

pub mod cache {
    pub use crate::constants::unified::cache::*;
}

pub mod nodes {
    pub use crate::constants::unified::nodes::*;
}

pub mod compliance {
    pub use crate::constants::unified::compliance::*;
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

// Legacy aliases for backward compatibility
pub use crate::constants::unified::{
    default_api_bind_address, default_api_host, default_api_port, default_health_check_interval_ms,
    default_key_rotation_timeout_ms, default_timeout_ms,
};

// Registry alias for compatibility
pub use crate::constants::unified::UnifiedConstantRegistry as CanonicalConstantRegistry;
