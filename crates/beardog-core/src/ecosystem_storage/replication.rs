// Data Replication Module

use super::types::ReplicationHealth;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationStatus {
    /// Primary location
    /// The primary location value
    pub primary_location: String,
    /// Replica locations
    /// Collection of replica locations
    pub replica_locations: Vec<String>,
    /// Replication factor (desired number of replicas)
    /// Number of replication_factor
    pub replication_factor: u32,
    /// Current number of healthy replicas
    /// Number of healthy_replicas
    pub healthy_replicas: u32,
    /// Overall replication health
    /// The health value
    pub health: ReplicationHealth,
    /// Last replication check
    /// The last check value
    pub last_check: DateTime<Utc>,
    /// Replication lag per replica (in seconds)
    /// Mapping of replication lag
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
