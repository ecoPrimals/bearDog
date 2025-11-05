// Data Replication Module

use super::types::ReplicationHealth;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Status of data replication across storage nodes
///
/// Tracks replication health, replica locations, lag times, and replication factor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationStatus {
    /// Primary storage location
    pub primary_location: String,
    /// Locations of data replicas
    pub replica_locations: Vec<String>,
    /// Desired number of replicas (replication factor)
    pub replication_factor: u32,
    /// Current number of healthy replicas
    pub healthy_replicas: u32,
    /// Overall replication health status
    pub health: ReplicationHealth,
    /// Last replication health check timestamp
    pub last_check: DateTime<Utc>,
    /// Replication lag per replica in seconds
    pub replication_lag: HashMap<String, f64>,
}

impl Default for ReplicationStatus {
    fn default() -> Self {
        Self {
            primary_location: String::new(),
            replica_locations: Vec::new(),
            replication_factor: 1,
            healthy_replicas: 0,
            health: ReplicationHealth::Offline,
            last_check: Utc::now(),
            replication_lag: HashMap::new(),
        }
    }
}
