// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::types::*;
use beardog_threat::threat::ThreatSeverity;
use serde::{Deserialize, Serialize};

// Define missing types locally for now
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkThreat {
    /// The threat type value
    pub threat_type: String,
    /// The description value
    pub description: String,
    /// The severity value
    pub severity: ThreatLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityState {
    /// Represents normal variant
    Normal,
    /// State indicating elevated
    Elevated,
    /// Represents high variant
    High,
    /// Represents critical variant
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustIndicator {
    /// The metric value
    pub metric: String,
    /// The value value
    pub value: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkEvent {
    /// State indicating peerconnected
    PeerConnected {
        peer_id: String,
        peer_capabilities: Box<PeerCapabilities>,
        trust_indicators: Vec<TrustIndicator>,
    },

    PeerDisconnected {
        reason: DisconnectReason,
        was_planned: bool,
    },

    NetworkConditionChanged {
        latency_ms: u64,
        packet_loss_percent: f64,
        bandwidth_mbps: u64,
        jitter_ms: u64,
    },

    ThreatDetected {
        threat: Box<NetworkThreat>,
        severity: ThreatSeverity,
    },

    SecurityStateChanged {
        new_state: SecurityState,
        previous_state: SecurityState,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerCapabilities {
    /// Collection of supported protocols
    pub supported_protocols: Vec<String>,
    pub max_bandwidth: u64,
    /// Number of security_level
    pub security_level: u8,
}
