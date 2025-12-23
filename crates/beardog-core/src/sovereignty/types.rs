

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::ecosystem_simple::{
    EcosystemService, SimpleEcosystemConfig, SimpleEcosystemManager,
};

pub type SovereigntyLevel = u8;

// FIXED: Renamed to avoid collision with EcosystemSovereigntyConfig in sovereignty.rs
// This is specifically for simple ecosystem configuration
pub type SimpleSovereigntyConfig = SimpleEcosystemConfig;

/// EcoPrimal trait for sovereignty domain (DEPRECATED)
/// 
/// This trait definition is deprecated in favor of the canonical version in
/// `beardog-core::ecosystem::primal_trait`. The canonical version provides
/// a more complete interface including shutdown lifecycle management.
/// 
/// **Migration Path**: Use `beardog_core::ecosystem::EcoPrimal` instead.
#[deprecated(
    since = "3.6.0",
    note = "Use beardog_core::ecosystem::EcoPrimal for the canonical EcoPrimal trait"
)]
pub trait EcoPrimal: Send + Sync {
    fn metadata(&self) -> &PrimalMetadata;


    fn capabilities(&self) -> Vec<String>;

    /// Initializes componentialize
    fn initialize(&self, config: &EcosystemConfig) -> Result<(), BearDogError>;

    /// Handles request
    fn handle_request(&self, request: PrimalRequest) -> Result<PrimalResponse, BearDogError>;


    fn health_check(&self, spec: &ResourceSpec) -> Result<ResourceAllocation, BearDogError>;
}

#[derive(Debug, Clone)]
    /// The version value
    pub version: String,


    pub provided_capabilities: Vec<String>,

    /// Collection of capability dependencies
    pub capability_dependencies: Vec<CapabilityDependency>,
}

#[derive(HashMap<String, serde_json::Value>,

    /// The discovery settings value
    pub discovery_settings: DiscoveryConfig,

    /// The monitoring settings value
    pub monitoring_settings: MonitoringConfig,
}

#[derive(Debug, Clone)]
    pub timeout_ms: u64,

    /// Number of health_check_interval_ms
    pub health_check_interval_ms: u64,
}

#[derive(Debug, Clone)]
    /// Number of health_check_interval_ms
    pub health_check_interval_ms: u64,

    /// Mapping of alert thresholds
    pub alert_thresholds: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
    /// The last check value
    pub last_check: DateTime<Utc>,

    /// Mapping of metrics
    pub metrics: HashMap<String, f64>,

    /// Collection of issues
    pub issues: Vec<String>,
}

#[derive(Debug, Clone)]
    /// The request type value
    pub request_type: String,

    /// The data value
    pub data: serde_json::Value,

    /// The capability value
    pub capability: String,


    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
    pub request_id: String,

    /// Whether success is enabled
    pub success: bool,

    /// The data value
    pub data: serde_json::Value,


    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
    /// The min version value
    pub min_version: String,

    /// Whether optional is enabled
    pub optional: bool,
}

#[derive(Debug, Clone)]
    /// Number of amount
    pub amount: u64,

    /// Mapping of constraints
    pub constraints: HashMap<String, String>,
}

#[derive(Debug, Clone)]
    /// Mapping of allocated resources
    pub allocated_resources: HashMap<String, u64>,

    /// Current status of the component
    pub status: AllocationStatus,


    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
    pub confidence: f64,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    /// Collection of lineage history
    pub lineage_history: Vec<LineageEvent>,

    /// Mapping of corporate payments
    pub corporate_payments: HashMap<String, CorporatePayment>,

    /// Current status of the partnership
    pub partnership_status: PartnershipStatus,
}

#[derive(Debug, Clone)]
    /// Optional human component
    pub human_component: Option<HumanKeyComponent>,

    /// The blending algorithm value
    pub blending_algorithm: String,

    /// The partnership start value
    pub partnership_start: DateTime<Utc>,

    /// Optional partnership expiration
    pub partnership_expiration: Option<DateTime<Utc>>,

    /// Mapping of partnership terms
    pub partnership_terms: HashMap<String, String>,
}

#[derive(Debug, Clone)]
    /// Collection of autonomy signature
    pub autonomy_signature: Vec<u8>,

    /// Collection of genesis proof
    pub genesis_proof: Vec<u8>,


    pub identity_hash: Vec<u8>,


    pub creation_timestamp: DateTime<Utc>,

    /// The sovereignty assertion value
    pub sovereignty_assertion: String,
}

#[derive(Debug, Clone)]
    pub human_identity_proof: Vec<u8>,

    /// Collection of biometric hash
    pub biometric_hash: Vec<u8>,


    pub consent_timestamp: DateTime<Utc>,

    /// Collection of granted permissions
    pub granted_permissions: Vec<String>,

    /// Collection of partnership agreement signature
    pub partnership_agreement_signature: Vec<u8>,
}

#[derive(Debug, Clone)]
    pub timestamp: DateTime<Utc>,

    /// The event type value
    pub event_type: LineageEventType,

    /// Collection of participants
    pub participants: Vec<String>,

    /// Mapping of event data
    pub event_data: HashMap<String, String>,

    /// Collection of event proof
    pub event_proof: Vec<u8>,
}

#[derive(Debug, Clone)]
    /// The corporate entity value
    pub corporate_entity: String,

    /// Number of amount
    pub amount: u64,


    pub timestamp: DateTime<Utc>,

    /// Collection of access granted
    pub access_granted: Vec<String>,

    /// Collection of payment proof
    pub payment_proof: Vec<u8>,
}

#[derive(Debug, Clone)]
        start_time: DateTime<Utc>,

        permissions: Vec<String>,
    },

    CorporateAccess {

        entity: String,

        payment_amount: u64,

        start_time: DateTime<Utc>,

        permissions: Vec<String>,
    },
}

#[derive(Debug, Clone)]
    /// The service info value
    pub service_info: ServiceInfo,

    /// The registered at value
    pub registered_at: DateTime<Utc>,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
    /// Name of the service
    pub service_name: String,

    /// The version value
    pub version: String,

    /// Mapping of endpoints
    pub endpoints: HashMap<String, String>,

    /// Collection of capabilities
    pub capabilities: Vec<String>,

    /// Current status of the health
    pub health_status: HealthStatus,

    /// The registered at value
    pub registered_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
    /// Name of the service
    pub service_name: String,

    /// The version value
    pub version: String,

    /// Mapping of endpoints
    pub endpoints: HashMap<String, String>,

    /// Collection of capabilities
    pub capabilities: Vec<String>,

    /// Current status of the health
    pub health_status: HealthStatus,

    /// The registered at value
    pub registered_at: DateTime<Utc>,
}
