
 /// Configuration management
 /// Configuration management

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub mod config;
pub mod connection;
pub mod manager;
pub mod metrics;
// REMOVED: pub mod providers; - Deprecated, use universal capability discovery

pub use config::{CloudIntegrationConfig, CloudSecurityConfig, RetryConfig};
pub use connection::{CloudConnection, CloudConnectionPool, ConnectionStatus};
pub use manager::EnterpriseCloudManager;
pub use metrics::CloudMetrics;
// REMOVED: pub use providers::{CloudIntegration, CloudProvider}; - Deprecated
