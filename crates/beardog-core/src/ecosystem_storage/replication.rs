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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replication_status_default() {
        let status = ReplicationStatus::default();
        assert!(status.primary_location.is_empty());
        assert!(status.replica_locations.is_empty());
        assert_eq!(status.replication_factor, 1);
        assert_eq!(status.healthy_replicas, 0);
        assert_eq!(status.health, ReplicationHealth::Offline);
    }

    #[test]
    fn test_replication_status_clone() {
        let mut status = ReplicationStatus::default();
        status.primary_location = "primary-node".to_string();
        status.replica_locations = vec!["replica-1".to_string(), "replica-2".to_string()];
        status.replication_factor = 3;
        status.healthy_replicas = 2;
        status.health = ReplicationHealth::Degraded;

        let cloned = status.clone();
        assert_eq!(status.primary_location, cloned.primary_location);
        assert_eq!(status.replica_locations, cloned.replica_locations);
        assert_eq!(status.replication_factor, cloned.replication_factor);
    }

    #[test]
    fn test_replication_status_serialization() {
        let mut status = ReplicationStatus::default();
        status.primary_location = "main-storage".to_string();
        status.replication_factor = 5;
        status.health = ReplicationHealth::Healthy;

        let serialized = serde_json::to_string(&status).expect("serialize");
        assert!(serialized.contains("main-storage"));

        let deserialized: ReplicationStatus =
            serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(status.primary_location, deserialized.primary_location);
    }

    #[test]
    fn test_replication_status_with_lag() {
        let mut status = ReplicationStatus::default();
        status.replication_lag.insert("replica-1".to_string(), 0.5);
        status.replication_lag.insert("replica-2".to_string(), 1.2);

        assert_eq!(status.replication_lag.len(), 2);
        assert_eq!(status.replication_lag.get("replica-1"), Some(&0.5));
    }

    #[test]
    fn test_replication_status_healthy_config() {
        let status = ReplicationStatus {
            primary_location: "primary".to_string(),
            replica_locations: vec![
                "replica-1".to_string(),
                "replica-2".to_string(),
                "replica-3".to_string(),
            ],
            replication_factor: 3,
            healthy_replicas: 3,
            health: ReplicationHealth::Healthy,
            last_check: Utc::now(),
            replication_lag: HashMap::new(),
        };

        assert_eq!(status.healthy_replicas, status.replication_factor);
        assert_eq!(status.health, ReplicationHealth::Healthy);
    }
}
