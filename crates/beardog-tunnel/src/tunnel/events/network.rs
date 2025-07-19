//! Network event definitions
//!
//! Contains events that the Songbird network layer sends to BearDog security layer.

use serde::{Deserialize, Serialize};

use super::capabilities::*;
use super::threat::*;
use super::types::*;

/// Events that Songbird network layer sends to BearDog security layer
///
/// These events inform BearDog's protective systems about network conditions, peer behavior,
/// and potential threats. This enables real-time security adaptation to protect scientists
/// and forest explorers from digital threats.
///
/// # Event Categories
///
/// - **Peer Events**: Discovery and disconnection of network peers
/// - **Performance Events**: Network condition changes that may affect security
/// - **Threat Events**: Suspicious activities requiring immediate security response
/// - **Optimization Events**: Network improvements that may enable enhanced security
///
/// # Forest Protection Use Cases
///
/// - **Scientist Safety**: Detect peers trying to access restricted research data
/// - **Newcomer Protection**: Identify potential threats to naive forest explorers  
/// - **Collective Intelligence**: Share threat information across the protective network
/// - **Academic Integrity**: Protect sacred knowledge from unauthorized access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkSecurityEvent {
    /// New peer discovered in the forest - requires security verification
    ///
    /// When a new node joins the network, BearDog must evaluate whether it's safe
    /// for scientists and researchers to interact with this peer.
    PeerDiscovered {
        /// Unique identifier for the discovered peer
        peer_id: String,
        /// Security and performance capabilities of the peer
        peer_capabilities: Box<PeerCapabilities>,
        /// Trust indicators that help assess peer safety
        trust_indicators: Vec<TrustIndicator>,
    },

    /// Peer disconnected from the forest - security cleanup needed
    ///
    /// When peers leave, BearDog must clean up security state and assess
    /// whether the disconnection indicates a potential threat.
    PeerDisconnected {
        /// Unique identifier for the disconnected peer
        peer_id: String,
        /// Why the peer disconnected (helps detect attacks)
        reason: DisconnectReason,
        /// Whether this was an expected disconnection
        was_planned: bool,
    },

    /// Network performance change - may affect security capabilities
    ///
    /// Performance changes can impact security effectiveness. BearDog adapts
    /// protection levels based on available network resources.
    NetworkConditionChanged {
        /// Current network latency in milliseconds
        latency_ms: u64,
        /// Percentage of packets being lost
        packet_loss_percent: f64,
        /// Available bandwidth in megabits per second
        bandwidth_mbps: u64,
        /// Network timing variance in milliseconds
        jitter_ms: u64,
    },

    /// Suspicious network activity detected - immediate protection needed
    ///
    /// The most critical event type - indicates potential threats to forest
    /// inhabitants requiring immediate defensive action.
    SuspiciousActivity {
        /// Peer that appears to be the source of suspicious activity
        source_peer: String,
        /// Type of suspicious behavior detected
        activity_type: SuspiciousActivityType,
        /// How severe the threat appears to be
        severity: NetworkThreatLevel,
        /// Evidence supporting the threat assessment
        evidence: Vec<NetworkEvidence>,
    },

    /// Network route optimization completed - may enable better security
    ///
    /// Route improvements can provide opportunities for enhanced security
    /// through better performance or reduced attack surface.
    RouteOptimized {
        /// Identifier for the optimized tunnel
        tunnel_id: String,
        /// Previous latency before optimization
        old_latency_ms: u64,
        /// New improved latency after optimization
        new_latency_ms: u64,
        /// Type of optimization that was performed
        optimization_type: OptimizationType,
    },
}
