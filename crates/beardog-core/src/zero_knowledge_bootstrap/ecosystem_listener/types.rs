// SPDX-License-Identifier: AGPL-3.0-or-later

//! Data types for the ecosystem listener (metrics, announcements, events).

use crate::ecosystem::primal_types::{PrimalMetadata, UniversalEndpoint};
use beardog_types::canonical::capabilities::{ServiceCapabilityType, UniversalCapability};
use serde::{Deserialize, Serialize};

/// Placeholder URL for primals with no announced endpoints
pub(super) const UNKNOWN_ENDPOINT_URL: &str = "unknown";

/// Metrics for ecosystem listening operations
#[derive(Clone, Copy, Debug, Default)]
pub struct EcosystemListenerMetrics {
    /// Number of valid announcements received from other primals
    pub announcements_received: u64,
    /// Number of unique primals discovered through listening
    pub primals_discovered: u64,
    /// Number of unique capabilities discovered across all primals
    pub capabilities_discovered: u64,
    /// Number of invalid or malformed announcements rejected
    pub invalid_announcements: u64,
    /// Total time spent listening for announcements (in milliseconds)
    pub listening_duration_ms: u64,
}

/// Primal announcement received from ecosystem
///
/// Represents an announcement from another primal in the ecosystem, containing
/// all information needed to identify and communicate with that primal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalAnnouncement {
    /// Unique identifier for the announcing primal
    pub primal_id: String,
    /// Capabilities offered by this primal
    pub capabilities: Vec<ServiceCapabilityType>,
    /// Communication endpoints for reaching this primal
    pub endpoints: Vec<UniversalEndpoint>,
    /// Additional metadata about the primal
    pub metadata: PrimalMetadata,
    /// Timestamp when this announcement was broadcast
    pub announcement_timestamp: std::time::SystemTime,
    /// Protocol used to discover this announcement (mDNS, HTTP, etc.)
    pub source_protocol: String,
}

/// Ecosystem discovery event
#[derive(Debug, Clone)]
pub enum EcosystemEvent {
    /// State indicating primaldiscovered
    PrimalDiscovered(crate::ecosystem::primal_types::DiscoveredPrimal),
    /// State indicating capabilityannounced
    CapabilityAnnounced(ServiceCapabilityType, UniversalCapability),
    /// State indicating primaldisconnected
    PrimalDisconnected(String),
    /// Represents invalid announcement variant
    InvalidAnnouncement(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecosystem::primal_types::{
        AuthRequirements, EndpointSecurityConfig, PrimalMetadata, UniversalEndpoint,
    };
    use beardog_types::canonical::capabilities::CapabilityType;
    use std::collections::HashMap;

    #[test]
    fn ecosystem_listener_metrics_default_and_debug() {
        let m = EcosystemListenerMetrics::default();
        assert_eq!(m.listening_duration_ms, 0);
        let m2 = EcosystemListenerMetrics {
            announcements_received: 1,
            ..Default::default()
        };
        assert_eq!(m2.announcements_received, 1);
        assert!(!format!("{m2:?}").is_empty());
    }

    #[test]
    fn primal_announcement_serde_roundtrip() {
        let md = PrimalMetadata {
            display_name: None,
            version: "0.1.0".to_string(),
            protocol_versions: vec!["1".to_string()],
            security_attestations: vec![],
            custom_fields: HashMap::new(),
            capabilities: vec![CapabilityType::Monitoring],
            dependencies: vec![],
            supported_protocols: vec!["http".to_string()],
            health_check_endpoint: "/health".to_string(),
            metrics_endpoint: "/metrics".to_string(),
        };
        let ep = UniversalEndpoint {
            url: "http://127.0.0.1:9".to_string(),
            protocols: vec!["http".to_string()],
            auth_requirements: AuthRequirements::default(),
            security_config: EndpointSecurityConfig::default(),
        };
        let ann = PrimalAnnouncement {
            primal_id: "test-primal".to_string(),
            capabilities: vec![CapabilityType::Monitoring],
            endpoints: vec![ep],
            metadata: md,
            announcement_timestamp: std::time::SystemTime::UNIX_EPOCH,
            source_protocol: "unit-test".to_string(),
        };
        let val = serde_json::to_value(&ann).expect("serialize announcement");
        let back: PrimalAnnouncement = serde_json::from_value(val).expect("deserialize");
        assert_eq!(back.primal_id, "test-primal");
        assert_eq!(back.source_protocol, "unit-test");
    }
}
