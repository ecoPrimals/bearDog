//! BearDog Security Manager
//!
//! Democratizing enterprise-grade security for everyone

pub use beardog_adapters as adapters;
pub use beardog_api as api;
pub use beardog_auth as auth;
pub use beardog_compliance as compliance;
pub use beardog_config as config;
pub use beardog_core as core;
pub use beardog_errors as errors;
pub use beardog_genetics as genetics;
pub use beardog_monitoring as monitoring;
pub use beardog_node_registry as node_registry;
pub use beardog_production as production;
pub use beardog_security as security;
pub use beardog_threat as threat;
pub use beardog_tunnel as tunnel;
pub use beardog_utils as utils;
pub use beardog_workflows as workflows;

// Re-export commonly used types
pub use beardog_config::BearDogConfig;
pub use beardog_core::BearDogCore;
pub use beardog_errors::{BearDogError, BearDogResult};

/// BearDog version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// BearDog mission statement
pub const MISSION: &str = "Democratizing enterprise-grade security for everyone";
