//! Configuration types for node registry
//!
//! This module contains all configuration-related types for the node registry,
//! organized into focused sub-modules for better maintainability.
//!
//! # Sub-modules
//!
//! - `registry`: Main registry configuration (`RegistryConfig`)
//! - `bootstrap`: Bootstrap node configuration (`BootstrapNodeConfig`)
//! - `federation`: Federation configuration (`FederationConfig`)
//! - `phonebook`: Phonebook service configuration (`PhonebookConfig`)
//! - `p2p`: Peer-to-peer networking configuration (`P2PConfig`)
//!
//! # Example
//!
//! ```rust
//! use beardog::node_registry::types::config::{RegistryConfig, FederationConfig};
//!
//! let registry_config = RegistryConfig::new(
//!     "my-registry".to_string(),
//!     "My Registry".to_string()
//! )
//! .with_port(9090)
//! .with_max_nodes(5000);
//!
//! let federation_config = FederationConfig::new()
//!     .with_enabled(true)
//!     .with_heartbeat_interval(30);
//! ```

pub mod registry;
pub mod bootstrap;
pub mod federation;
pub mod phonebook;
pub mod p2p;

// Re-export all configuration types for backward compatibility
pub use registry::RegistryConfig;
pub use bootstrap::BootstrapNodeConfig;
pub use federation::FederationConfig;
pub use phonebook::PhonebookConfig;
pub use p2p::P2PConfig; 