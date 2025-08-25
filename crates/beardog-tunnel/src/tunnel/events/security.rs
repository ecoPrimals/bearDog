// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Security event definitions
///
/// Contains events that BearDog security layer sends to Songbird network layer.

use serde::{Deserialize, Serialize};
use super::compliance::*;
use super::threat::ThreatMitigationAction;
use super::types::*;
/// Events that BearDog security layer sends to Songbird network layer
/// These events allow BearDog to direct network behavior based on security requirements,
/// ensuring that routing and communication decisions support forest protection goals.
/// # Security-Driven Network Control
/// BearDog can influence network behavior to:
/// - Route traffic away from compromised peers
/// - Enforce compliance requirements for different geographic regions
/// - Adapt network performance to security posture changes
/// - Coordinate threat response across the protective network
/// # Example: Protecting Academic Collaborations
/// When researchers from different institutions collaborate, BearDog ensures their
/// communications are routed through secure channels and comply with institutional
/// security policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityNetworkEvent {
    /// Security session established - network can now route protected traffic
    ///
    /// Notifies the network layer that BearDog has established a secure communication
    /// channel with a peer, enabling safe data exchange for forest inhabitants.
    SessionEstablished {
        /// Unique identifier for the established security session
        session_id: String,
        /// Peer that the session was established with
        peer_id: String,
        /// Level of security protection provided by this session
        security_level: SecurityLevel,
        /// Optional bandwidth limit to prevent resource exhaustion attacks
        bandwidth_limit: Option<u64>,
    },
    /// Security threat detected - network may need to adjust routing
    /// Alerts the network layer about threats that may require routing changes
    /// to protect vulnerable forest explorers from malicious actors.
    ThreatDetected {
        /// Severity of the detected threat
        threat_level: ThreatLevel,
        /// Peer that appears to be affected by or source of the threat
        affected_peer: String,
        /// BearDog's recommendation for how the network should respond
        recommended_action: ThreatMitigationAction,
    /// Security upgrade applied - network performance characteristics may change
    /// Informs the network that BearDog has upgraded security protections,
    /// which may affect throughput, latency, or routing preferences.
    SecurityUpgraded {
        /// Session identifier
        /// Previous security level before upgrade
        old_security_level: SecurityLevel,
        /// New enhanced security level after upgrade
        new_security_level: SecurityLevel,
        /// Expected impact on network performance
        performance_impact: PerformanceImpact,
    /// Compliance requirement imposed - routing behavior must be adjusted
    /// Notifies the network about regulatory or policy requirements that
    /// restrict how traffic can be routed to protect academic integrity
    /// and comply with institutional policies.
    ComplianceRequirement {
        /// Type of compliance requirement (GDPR, HIPAA, institutional policy, etc.)
        requirement_type: ComplianceType,
        /// Geographic regions affected by this requirement
        affected_regions: Vec<GeographicRegion>,
        /// Specific routing restrictions that must be enforced
        routing_restrictions: Vec<RoutingRestriction>,
}
/// Security response events
pub enum SecurityResponse {
    /// Security session created successfully
    /// Confirms that BearDog has established secure communication with a peer
    /// and the network can begin routing protected traffic.
    SessionCreated {
        /// Unique identifier for the new security session
        /// Peer that the session was created with
        /// Level of security protection established
    /// Security session terminated
    /// Notifies that BearDog has ended secure communication with a peer,
    /// requiring the network to stop routing traffic to that destination.
    SessionTerminated {
        /// Peer whose session was terminated
    /// Security level automatically adapted to changing conditions
    /// Indicates that BearDog has adjusted protection levels in response
    /// to changing threat conditions or performance requirements.
    SecurityAdapted {
        /// Explanation of why security adaptation was necessary
        reason: String,
        /// New security level after adaptation
    /// Threat detected requiring immediate protective action
    /// Urgent notification that BearDog has identified a threat requiring
    /// immediate network-level response to protect forest inhabitants.
        /// Peer that appears to be the source of the threat
        source_peer: String,
        /// Recommended immediate action for the network layer
