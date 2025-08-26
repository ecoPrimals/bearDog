

use serde::{Deserialize, Serialize};
use super::capabilities::*;
use super::threat::*;
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkSecurityEvent {

    PeerDiscovered {

        peer_id: String,

        peer_capabilities: Box<PeerCapabilities>,

        trust_indicators: Vec<TrustIndicator>,
    },

    PeerDisconnected {

        reason: DisconnectReason,

        was_planned: bool,

    NetworkConditionChanged {

        latency_ms: u64,

        packet_loss_percent: f64,

        bandwidth_mbps: u64,

        jitter_ms: u64,

    SuspiciousActivity {

        source_peer: String,

        activity_type: SuspiciousActivityType,

        severity: NetworkThreatLevel,

        evidence: Vec<NetworkEvidence>,

    RouteOptimized {

        tunnel_id: String,

        old_latency_ms: u64,

        new_latency_ms: u64,

        optimization_type: OptimizationType,
}
