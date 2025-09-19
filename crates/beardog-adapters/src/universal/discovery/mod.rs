

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub mod ai_matcher;
/// Configuration management
/// Configuration management
pub mod config;
pub mod engine;
pub mod metrics;
pub mod predictive_scaler;
pub mod protocol_translator;
pub mod quantum_comm;
pub mod service_mesh;
pub mod types;

pub use config::{AIMatcherConfig, NextGenDiscoveryConfig, ServiceMeshConfig};
pub use engine::NextGenDiscoveryEngine;
pub use metrics::{AIInsights, DiscoveryAnalytics, DiscoveryMetrics};
pub use types::{
    CapabilityRequest, DiscoveryResult, EcosystemIntegrationRequest, IntegrationResult,
    ScalingRecommendations, SecureChannel, ServiceEndpoint,
};
