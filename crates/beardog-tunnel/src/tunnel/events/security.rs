

use serde::{Deserialize, Serialize};
use super::compliance::*;
use super::threat::ThreatMitigationAction;
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityNetworkEvent {
    SessionEstablished {
        session_id: String,
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
pub enum SecurityResponse {
    SessionCreated {
        session_id: String,
        peer_id: String,
    },

    SessionTerminated {
        session_id: String,
        reason: String,
    },

    SecurityAdapted {
        reason: String,
        source_peer: String,
    },
}

