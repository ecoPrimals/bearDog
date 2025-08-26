

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIHealthStatus {
    pub status: String,
    pub timestamp: String,
    pub components: Vec<AIHealthCheck>,
}

pub struct AIHealthCheck {
    pub name: String,
    pub message: String,

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AIMetricsResponse {
    pub system: AISystemMetrics,
    pub security: AISecurityMetrics,
    pub genetics: AIGeneticsMetrics,
    pub tunnel: AITunnelMetrics,

pub struct AISystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub uptime: u64,

pub struct AISecurityMetrics {
    pub encryptions_per_second: f64,
    pub signatures_per_second: f64,
    pub verifications_per_second: f64,

pub struct AIGeneticsMetrics {
    pub active_nodes: u32,
    pub spawns_per_hour: f64,
    pub evolution_rate: f64,

pub struct AITunnelMetrics {
    pub active_connections: u32,
    pub throughput_mbps: f64,
    pub latency_ms: f64,}

impl Default for AIHealthStatus {}

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
                    name: "hsm".to_string(),
                    message: "HSM tiers available".to_string(),
            ],
        }
    }
impl Default for AISystemMetrics {
            cpu_usage: 0.0,
            memory_usage: 0,
            uptime: 0,}

impl Default for AISecurityMetrics {
            encryptions_per_second: 0.0,
            signatures_per_second: 0.0,
            verifications_per_second: 0.0,
impl Default for AIGeneticsMetrics {
            active_nodes: 0,
            spawns_per_hour: 0.0,
            evolution_rate: 0.0,}

impl Default for AITunnelMetrics {
            active_connections: 0,
            throughput_mbps: 0.0,
            latency_ms: 0.0,
