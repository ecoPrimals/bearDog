// SPDX-License-Identifier: AGPL-3.0-only

//! Data types for the ecosystem listener (metrics, announcements, events).

use crate::ecosystem::primal_types::{PrimalMetadata, UniversalEndpoint};
use beardog_types::canonical::capabilities::{ServiceCapabilityType, UniversalCapability};
use serde::{Deserialize, Serialize};

/// Placeholder URL for primals with no announced endpoints
pub(crate) const UNKNOWN_ENDPOINT_URL: &str = "unknown";

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
