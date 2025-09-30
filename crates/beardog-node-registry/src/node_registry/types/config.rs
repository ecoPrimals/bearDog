

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod registry;
pub mod bootstrap;
pub mod federation;
pub mod phonebook;
pub mod p2p;

pub use registry::RegistryConfig;
pub use bootstrap::BootstrapNodeConfig;
pub use federation::FederationConfig;
pub use phonebook::PhonebookConfig;
pub use p2p::P2PConfig; 
