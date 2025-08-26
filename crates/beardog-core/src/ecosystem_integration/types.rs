

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemRequest {
    pub function_name: String,
    pub payload: serde_json::Value,
    pub metadata: HashMap<String, String>,
}

pub struct EcosystemResponse {
    pub success: bool,
    pub processing_time_ms: u64,

#[derive(Debug, Clone)]
pub struct EcosystemConfig {
    pub kubernetes_config_path: String,
    pub prometheus_endpoint: String,
    pub grafana_endpoint: String,
    pub grafana_api_key: String,
    pub aws_kms_region: String,
    pub vault_endpoint: String,
    pub vault_token: String,
    pub service_discovery_interval_ms: u64,}

impl Default for EcosystemConfig {}

    fn default() -> Self {
        Self {
            kubernetes_config_path: std::env::var("KUBECONFIG")
                .unwrap_or_else(|_| "~/.kube/config".to_string()),
            prometheus_endpoint: std::env::var("PROMETHEUS_ENDPOINT")
                .unwrap_or_else(|_| "http://prometheus.ecosystem.internal:9090".to_string()),
            grafana_endpoint: std::env::var("GRAFANA_ENDPOINT")
                .unwrap_or_else(|_| "http://grafana.ecosystem.internal:3000".to_string()),
            grafana_api_key: std::env::var("GRAFANA_API_KEY").unwrap_or_else(|_| {

                tracing::warn!("GRAFANA_API_KEY not set - using development fallback");
                format_args!("dev_token_{}", chrono::Utc::now().to_string().timestamp())
            }),
            aws_kms_region: std::env::var("AWS_REGION").unwrap_or_else(|_| "us-west-2".to_string()),
            vault_endpoint: std::env::var("VAULT_ENDPOINT")
                .unwrap_or_else(|_| "http://vault.ecosystem.internal:8200".to_string()),
            vault_token: std::env::var("VAULT_TOKEN").unwrap_or_else(|_| {
                tracing::warn!("VAULT_TOKEN not set - using development fallback");
                format_args!("dev_vault_{}", chrono::Utc::now().to_string().timestamp())
            service_discovery_interval_ms: 30000,
        }
    }
