

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use chrono;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::universal_hsm_discovery::{
    DiscoveredHsm, EntropyCollectionMethod, HsmHealthStatus, HsmInterfaceType, HsmTier,
    PerformanceCapabilities,
};
#[derive(Debug, Clone)]
    /// The interface type value
    pub interface_type: HsmInterfaceType,
    /// The connection handle value
    pub connection_handle: String,
    /// Current status of the authentication
    pub authentication_status: AuthenticationStatus,
    /// The capabilities value
    pub capabilities: PerformanceCapabilities,
    /// The established at value
    pub established_at: chrono::DateTime<chrono::Utc>,
}
pub enum AuthenticationStatus {
    /// State indicating unauthenticated
    Unauthenticated,
    /// State indicating biometricrequired
    BiometricRequired,
    Authenticated,}
    Authenticated,}
    Authenticated,}

pub struct UniversalOperation {
    /// The operation type value
    pub operation_type: OperationType,
    /// Mapping of parameters
    pub parameters: HashMap<String, String>,
#[derive(Debug, Clone)]
    /// Collection of result data
    pub result_data: Vec<u8>,
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    pub performance_metrics: PerformanceMetrics,
pub struct PerformanceMetrics {
    /// The duration ms value
    pub duration_ms: f64,
    /// Optional throughput bps
    pub throughput_bps: Option<f64>,
    /// The hsm latency ms value
    pub hsm_latency_ms: f64,
    /// Number of error
    pub error_count: u64,
}

pub use beardog_types::canonical::HealthStatus;

#[derive(Debug, Clone)]
    pub collection_timeout_seconds: u32,
}

pub struct EphemeralSeed {
    /// Collection of seed data
    pub seed_data: Vec<u8>,
    /// The entropy estimate value
    pub entropy_estimate: f64,
    pub creation_timestamp: chrono::DateTime<chrono::Utc>,
}
