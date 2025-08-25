// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Types for ecosystem integration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// External function request from ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemRequest {
    pub function_name: String,
    pub payload: serde_json::Value,
    pub metadata: HashMap<String, String>,
}
/// External function response to ecosystem
pub struct EcosystemResponse {
    pub success: bool,
    pub processing_time_ms: u64,
/// Configuration for ecosystem integration
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
                // Generate a secure placeholder token or warn about missing config
                tracing::warn!("GRAFANA_API_KEY not set - using development fallback");
                format!("dev_token_{}", chrono::Utc::now().timestamp())
            }),
            aws_kms_region: std::env::var("AWS_REGION").unwrap_or_else(|_| "us-west-2".to_string()),
            vault_endpoint: std::env::var("VAULT_ENDPOINT")
                .unwrap_or_else(|_| "http://vault.ecosystem.internal:8200".to_string()),
            vault_token: std::env::var("VAULT_TOKEN").unwrap_or_else(|_| {
                tracing::warn!("VAULT_TOKEN not set - using development fallback");
                format!("dev_vault_{}", chrono::Utc::now().timestamp())
            service_discovery_interval_ms: 30000,
        }
    }
