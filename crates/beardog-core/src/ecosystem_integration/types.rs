// Fixed types.rs - Ecosystem integration types
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemNode {
    pub node_id: String,
    pub node_type: String,
    pub endpoint: String,
    pub capabilities: Vec<String>,
    pub health_status: HealthStatus,
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMeshConfig {
    pub mesh_id: String,
    pub nodes: Vec<EcosystemNode>,
    pub routing_rules: HashMap<String, String>,
    pub load_balancing_strategy: String,
    pub circuit_breaker_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationMetrics {
    pub total_nodes: usize,
    pub healthy_nodes: usize,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub average_latency_ms: f64,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemEvent {
    pub event_id: String,
    pub event_type: String,
    pub source_node: String,
    pub target_node: Option<String>,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub severity: String,
}

impl EcosystemNode {
    pub fn new(node_id: String, node_type: String, endpoint: String) -> Self {
        Self {
            node_id,
            node_type,
            endpoint,
            capabilities: Vec::new(),
            health_status: HealthStatus::Healthy,
            last_seen: Utc::now(),
        }
    }

    pub fn is_healthy(&self) -> bool {
        matches!(self.health_status, HealthStatus::Healthy)
    }

    pub fn update_health(&mut self, status: HealthStatus) {
        self.health_status = status;
        self.last_seen = Utc::now();
    }
}

impl ServiceMeshConfig {
    pub fn new(mesh_id: String) -> Self {
        Self {
            mesh_id,
            nodes: Vec::new(),
            routing_rules: HashMap::new(),
            load_balancing_strategy: "round_robin".to_string(),
            circuit_breaker_enabled: true,
        }
    }

    pub fn add_node(&mut self, node: EcosystemNode) {
        self.nodes.push(node);
    }

    pub fn get_healthy_nodes(&self) -> Vec<&EcosystemNode> {
        self.nodes.iter().filter(|node| node.is_healthy()).collect()
    }
}

impl IntegrationMetrics {
    pub fn new() -> Self {
        Self {
            total_nodes: 0,
            healthy_nodes: 0,
            total_requests: 0,
            successful_requests: 0,
            average_latency_ms: 0.0,
            last_updated: Utc::now(),
        }
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            (self.successful_requests as f64 / self.total_requests as f64) * 100.0
        }
    }

    pub fn health_ratio(&self) -> f64 {
        if self.total_nodes == 0 {
            0.0
        } else {
            (self.healthy_nodes as f64 / self.total_nodes as f64) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecosystem_node_creation() {
        let node = EcosystemNode::new(
            "test-node-001".to_string(),
            "beardog".to_string(),
            "http://localhost:8080".to_string(),
        );

        assert_eq!(node.node_id, "test-node-001");
        assert_eq!(node.node_type, "beardog");
        assert!(node.is_healthy());
    }

    #[test]
    fn test_service_mesh_config() {
        let mut mesh = ServiceMeshConfig::new("test-mesh".to_string());
        let node = EcosystemNode::new(
            "node-1".to_string(),
            "test".to_string(),
            "http://test:8080".to_string(),
        );

        mesh.add_node(node);
        assert_eq!(mesh.nodes.len(), 1);
        assert_eq!(mesh.get_healthy_nodes().len(), 1);
    }

    #[test]
    fn test_integration_metrics() {
        let mut metrics = IntegrationMetrics::new();
        metrics.total_requests = 100;
        metrics.successful_requests = 95;
        
        assert_eq!(metrics.success_rate(), 95.0);
    }
}
