//! Health and status monitoring for AI interface

use serde::{Deserialize, Serialize};

/// AI Health Status Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIHealthStatus {
    pub status: String,
    pub timestamp: String,
    pub components: Vec<AIHealthCheck>,
}

/// AI Health Check Component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIHealthCheck {
    pub name: String,
    pub status: String,
    pub message: String,
}

/// AI Metrics Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMetricsResponse {
    pub system: AISystemMetrics,
    pub security: AISecurityMetrics,
    pub genetics: AIGeneticsMetrics,
    pub tunnel: AITunnelMetrics,
}

/// AI System Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub uptime: u64,
}

/// AI Security Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISecurityMetrics {
    pub encryptions_per_second: f64,
    pub signatures_per_second: f64,
    pub verifications_per_second: f64,
}

/// AI Genetics Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGeneticsMetrics {
    pub active_nodes: u32,
    pub spawns_per_hour: f64,
    pub evolution_rate: f64,
}

/// AI Tunnel Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AITunnelMetrics {
    pub active_connections: u32,
    pub throughput_mbps: f64,
    pub latency_ms: f64,
}

impl Default for AIHealthStatus {
    fn default() -> Self {
        Self {
            status: "healthy".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            components: vec![
                AIHealthCheck {
                    name: "core".to_string(),
                    status: "healthy".to_string(),
                    message: "Core systems operational".to_string(),
                },
                AIHealthCheck {
                    name: "hsm".to_string(),
                    status: "healthy".to_string(),
                    message: "HSM tiers available".to_string(),
                },
            ],
        }
    }
}

impl Default for AIMetricsResponse {
    fn default() -> Self {
        Self {
            system: AISystemMetrics::default(),
            security: AISecurityMetrics::default(),
            genetics: AIGeneticsMetrics::default(),
            tunnel: AITunnelMetrics::default(),
        }
    }
}

impl Default for AISystemMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0,
            uptime: 0,
        }
    }
}

impl Default for AISecurityMetrics {
    fn default() -> Self {
        Self {
            encryptions_per_second: 0.0,
            signatures_per_second: 0.0,
            verifications_per_second: 0.0,
        }
    }
}

impl Default for AIGeneticsMetrics {
    fn default() -> Self {
        Self {
            active_nodes: 0,
            spawns_per_hour: 0.0,
            evolution_rate: 0.0,
        }
    }
}

impl Default for AITunnelMetrics {
    fn default() -> Self {
        Self {
            active_connections: 0,
            throughput_mbps: 0.0,
            latency_ms: 0.0,
        }
    }
}
