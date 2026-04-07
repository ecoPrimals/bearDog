// SPDX-License-Identifier: AGPL-3.0-or-later

//! Source and target attribution for threat events.

use super::taxonomy::{AssetCriticality, ProtectionLevel, SourceClassification, ThreatSeverity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Source of the threat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatSource {
    /// Source type (IP, domain, process, etc.)
    /// The source type value
    pub source_type: String,
    /// Source identifier
    pub identifier: String,
    /// Unique id for this source record, often distinct from `identifier`.
    pub id: String,
    /// IP address if applicable
    /// Optional ip address
    pub ip_address: Option<String>,
    /// Hostname if applicable
    /// Name of the hostitem
    pub hostname: Option<String>,
    /// User agent if applicable
    /// Optional user agent
    pub user_agent: Option<String>,
    /// Geographic location if available
    /// Optional location
    pub location: Option<String>,
    /// Optional geolocation
    pub geolocation: Option<String>,
    /// Optional threat actor
    pub threat_actor: Option<String>,
    /// Source classification
    /// The classification value
    pub classification: SourceClassification,
    /// Reputation score if available
    /// Optional reputation
    pub reputation: Option<f64>,
    /// Reputation score (alias)
    /// The reputation score value
    pub reputation_score: f64,
    /// Normalized confidence in source attribution, in the same 0.0–1.0 scale as threat confidence.
    pub confidence_score: f64,
    /// First seen timestamp
    /// Optional first seen
    pub first_seen: Option<SystemTime>,
    /// Last seen timestamp
    /// Optional last seen
    pub last_seen: Option<SystemTime>,
    /// Additional metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

impl Default for ThreatSource {
    fn default() -> Self {
        Self {
            source_type: "unknown".to_string(),
            identifier: "unknown".to_string(),
            id: uuid::Uuid::new_v4().to_string(),
            ip_address: None,
            hostname: None,
            user_agent: None,
            location: None,
            geolocation: None,
            threat_actor: None,
            classification: SourceClassification::Unknown,
            reputation: None,
            reputation_score: 0.0,
            confidence_score: 0.0,
            first_seen: None,
            last_seen: None,
            metadata: HashMap::new(),
        }
    }
}

/// Target of the threat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatTarget {
    /// Target type (system, service, data, etc.)
    /// The target type value
    pub target_type: String,
    /// Human-readable target label (service name, host, etc.).
    pub identifier: String,
    /// Unique id for this target in the inventory graph.
    pub id: String,
    /// Logical resource id (database, bucket, queue) when distinct from `id`.
    pub resource_id: String,
    /// Optional cluster or mesh node id when the target spans multiple hosts.
    pub node_id: Option<String>,
    /// User account if applicable
    /// Number of `user_acitems`
    pub user_account: Option<String>,
    /// Asset criticality
    /// The asset criticality value
    pub asset_criticality: AssetCriticality,
    /// Protection level
    /// The protection level value
    pub protection_level: ProtectionLevel,
    /// Service name if applicable
    /// Optional service
    pub service: Option<String>,
    /// Port number if applicable
    /// Optional port
    pub port: Option<u16>,
    /// Protocol if applicable
    /// Optional protocol
    pub protocol: Option<String>,
    /// Additional metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Resource type
    /// The resource type value
    pub resource_type: String,
    /// Criticality level of the target
    /// The criticality value
    pub criticality: ThreatSeverity,
    /// IP address if applicable
    /// Optional ip address
    pub ip_address: Option<String>,
    /// Hostname if applicable
    /// Name of the hostitem
    pub hostname: Option<String>,
}

impl Default for ThreatTarget {
    fn default() -> Self {
        Self {
            target_type: "unknown".to_string(),
            identifier: "unknown".to_string(),
            id: uuid::Uuid::new_v4().to_string(),
            resource_id: "unknown".to_string(),
            node_id: None,
            user_account: None,
            asset_criticality: AssetCriticality::Low,
            protection_level: ProtectionLevel::Basic,
            service: None,
            port: None,
            protocol: None,
            metadata: HashMap::new(),
            resource_type: "unknown".to_string(),
            criticality: ThreatSeverity::Low,
            ip_address: None,
            hostname: None,
        }
    }
}
