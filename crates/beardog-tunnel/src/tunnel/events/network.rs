// SPDX-License-Identifier: AGPL-3.0-only

//! Network Events Module
//!
//! Provides event types for network-related occurrences in the tunnel system.

use super::types::{DisconnectReason, ThreatLevel};
use beardog_threat::threat::ThreatSeverity;
use serde::{Deserialize, Serialize};

/// Network threat information
///
/// Represents a detected network-level security threat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkThreat {
    /// Type of threat (e.g., "mitm", "replay", "dos")
    pub threat_type: String,
    /// Human-readable description of the threat
    pub description: String,
    /// Severity level of the threat
    pub severity: ThreatLevel,
}

/// Network security state
///
/// Represents the current security posture of the network connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityState {
    /// Normal operating conditions
    Normal,
    /// Elevated threat level detected
    Elevated,
    /// High risk conditions
    High,
    /// Critical security issue
    Critical,
}

/// Trust indicator metric
///
/// A single trust metric for evaluating peer trustworthiness.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustIndicator {
    /// Name of the trust metric
    pub metric: String,
    /// Current value (0.0 - 1.0 typically)
    pub value: f64,
    /// Confidence level of this measurement (0.0 - 1.0)
    pub confidence: f64,
}

/// Network events
///
/// Events that occur during network tunnel operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkEvent {
    /// A new peer has connected
    PeerConnected {
        /// Unique identifier of the connected peer
        peer_id: String,
        /// Capabilities reported by the peer
        peer_capabilities: Box<PeerCapabilities>,
        /// Trust indicators for the peer
        trust_indicators: Vec<TrustIndicator>,
    },
    /// A peer has disconnected
    PeerDisconnected {
        /// Reason for disconnection
        reason: DisconnectReason,
        /// Whether the disconnect was graceful/planned
        was_planned: bool,
    },
    /// Network conditions have changed
    NetworkConditionChanged {
        /// Round-trip latency in milliseconds
        latency_ms: u64,
        /// Packet loss percentage (0-100)
        packet_loss_percent: f64,
        /// Available bandwidth in Mbps
        bandwidth_mbps: u64,
        /// Jitter (latency variance) in milliseconds
        jitter_ms: u64,
    },
    /// A network threat has been detected
    ThreatDetected {
        /// Details about the detected threat
        threat: Box<NetworkThreat>,
        /// Severity classification
        severity: ThreatSeverity,
    },
    /// Security state has changed
    SecurityStateChanged {
        /// New security state
        new_state: SecurityState,
        /// Previous security state
        previous_state: SecurityState,
    },
}

/// Peer capabilities
///
/// Describes the capabilities of a connected peer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerCapabilities {
    /// List of supported protocol versions
    pub supported_protocols: Vec<String>,
    /// Maximum bandwidth in bytes per second
    pub max_bandwidth: u64,
    /// Security level (0-255, higher is more secure)
    pub security_level: u8,
}
