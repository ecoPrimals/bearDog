use super::compliance::{ComplianceType, GeographicRegion, RoutingRestriction};
use super::threat::ThreatMitigationAction;
use super::types::{PerformanceImpact, SecurityLevel, ThreatLevel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEvent {
    /// State indicating securitylevelchanged
    SecurityLevelChanged {
        peer_id: String,
        security_level: SecurityLevel,
        bandwidth_limit: Option<u64>,
    },

    ThreatDetected {
        threat_level: ThreatLevel,
        affected_peer: String,
        recommended_action: ThreatMitigationAction,
    },

    SecurityUpgraded {
        old_security_level: SecurityLevel,
        new_security_level: SecurityLevel,
        performance_impact: PerformanceImpact,
    },

    ComplianceRequirement {
        requirement_type: ComplianceType,
        affected_regions: Vec<GeographicRegion>,
        routing_restrictions: Vec<RoutingRestriction>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionEvent {
    /// Session successfully established
    SessionEstablished { session_id: String, peer_id: String },
    /// Session terminated
    SessionTerminated { session_id: String, reason: String },
    /// Security configuration adapted
    SecurityAdapted { reason: String, source_peer: String },
}
