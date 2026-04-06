// SPDX-License-Identifier: AGPL-3.0-or-later

//! Security Events Module
//!
//! Provides event types for security-related occurrences in the tunnel system.

use super::compliance::{ComplianceType, GeographicRegion, RoutingRestriction};
use super::threat::ThreatMitigationAction;
use super::types::{PerformanceImpact, SecurityLevel, ThreatLevel};
use serde::{Deserialize, Serialize};

/// Security-related events
///
/// Events that affect the security posture of tunnel connections.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEvent {
    /// Security level of a peer has changed
    SecurityLevelChanged {
        /// ID of the affected peer
        peer_id: String,
        /// New security level
        security_level: SecurityLevel,
        /// Optional bandwidth limit applied at this security level
        bandwidth_limit: Option<u64>,
    },
    /// A security threat has been detected
    ThreatDetected {
        /// Severity level of the threat
        threat_level: ThreatLevel,
        /// ID of the peer involved
        affected_peer: String,
        /// Recommended mitigation action
        recommended_action: ThreatMitigationAction,
    },
    /// Security level has been upgraded
    SecurityUpgraded {
        /// Previous security level
        old_security_level: SecurityLevel,
        /// New security level
        new_security_level: SecurityLevel,
        /// Expected performance impact of the upgrade
        performance_impact: PerformanceImpact,
    },
    /// Compliance requirement detected
    ComplianceRequirement {
        /// Type of compliance requirement
        requirement_type: ComplianceType,
        /// Geographic regions affected
        affected_regions: Vec<GeographicRegion>,
        /// Routing restrictions to apply
        routing_restrictions: Vec<RoutingRestriction>,
    },
}

/// Session lifecycle events
///
/// Events related to tunnel session management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionEvent {
    /// Session successfully established
    SessionEstablished {
        /// Unique session identifier
        session_id: String,
        /// ID of the remote peer
        peer_id: String,
    },
    /// Session terminated
    SessionTerminated {
        /// Session that was terminated
        session_id: String,
        /// Reason for termination
        reason: String,
    },
    /// Security configuration adapted
    SecurityAdapted {
        /// Reason for adaptation
        reason: String,
        /// Peer that triggered the adaptation
        source_peer: String,
    },
}
