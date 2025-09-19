// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub mod bridge;
pub mod hardware;
pub mod metrics;
pub mod software;
pub mod types;

// Re-export main types and structs
pub use bridge::{SecurityProviderBridge, VendorHsmIntegrationImpl};
pub use hardware::HardwareVendorHsmIntegration;
pub use metrics::SecurityMetricsCollector;
pub use software::SoftwareVendorHsmIntegration;
pub use types::*;

pub use beardog_types::canonical::configuration::adapters::BridgeConfig;
