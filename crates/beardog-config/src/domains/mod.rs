//! Domain-specific configuration modules

pub mod capacity;
pub mod crypto;
pub mod hsm;
pub mod limits;
pub mod monitoring;
pub mod network;
pub mod network_addresses;
pub mod network_hosts;
pub mod network_ports;
pub mod paths;
pub mod security;

// Migrated to modern timeouts_new module (Week 3 modernization complete)
pub mod timeouts_new;

// Re-export timeouts_new as timeouts for ergonomics
pub use timeouts_new as timeouts;
