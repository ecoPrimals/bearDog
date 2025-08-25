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


/// Prometheus Integration Handler
///
/// Provides licensed access to Prometheus metrics operations

use super::ExternalFunctionHandler;
use crate::licensing::LicenseManager;
// Removed async_trait - using native async fn for zero-cost abstractions
use beardog_errors::{BearDogError, BearDogResult};
use beardog_errors::idiomatic::SystemResult;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
/// **PROMETHEUS METRICS EXPORT HANDLER** - Zero-cost implementation
#[derive(Clone)]
pub struct PrometheusExport;
// Uses native async fn from trait definition - no async_trait needed
impl ExternalFunctionHandler for PrometheusExport {}


    fn function_name(&self) -> &str {
        "prometheus_export"
    }
    /// Execute Prometheus operation with licensing check
    async fn execute(
        &self,
        license_manager: &LicenseManager,
        _operation: &str,
        payload: serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        // Check licensing with autonomous decision making
        if !license_manager
            .is_function_available(self.function_name())
            .await?
        {
            return Err(BearDogError::configuration(format!(
                    "🔒 Prometheus integration '}' requires licensing or individual/small-team classification.\n\n\
                    🏠 Individual developers: Automatically granted access\n\
                    👥 Small teams: Automatically granted access\n\
                    🏢 Corporate usage: External adapters locked - acquire unlock certificate",
                    self.function_name()
                ),
            });
        }
        let default_endpoint = format!(
            "http://{}:{}",
            beardog_types::config::constants::network::get_default_host(),
            beardog_types::config::constants::network::DEFAULT_GRPC_PORT
        );
        let endpoint = payload
            .get("endpoint")
            .and_then(|v| v.as_str())
            .unwrap_or(&default_endpoint);
        let metrics = payload
            .get("metrics")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        tracing::info!(
            "📊 Prometheus {} operation on endpoint: {}",
            _operation,
            endpoint
        match _operation {
            "query" => {
                let query = payload
                    .get("query")
                    .and_then(|v| v.as_str())
                    .unwrap_or("up");
                match self.prometheus_query(endpoint, query).await {
                    Ok(result) => Ok(serde_json::json!({
                        "operation": _operation,
                        "endpoint": endpoint,
                        "query": query,
                        "status": "success",
                        "data": result
                    })),
                    Err(e) => Ok(serde_json::json!({
                        "status": "error",
                        "error": e.to_string()
                }
            }
            "export_metrics" => match self.export_beardog_metrics(endpoint, &metrics).await {
                Ok(exported_count) => Ok(serde_json::json!({
                    "operation": _operation,
                    "endpoint": endpoint,
                    "status": "success",
                    "exported_metrics": exported_count
                })),
                Err(e) => Ok(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
            },
            "health_check" => match self.prometheus_health_check(endpoint).await {
                Ok(health_status) => Ok(serde_json::json!({
                    "health": health_status
            _ => Ok(serde_json::json!({
                "operation": _operation,
                "status": "error",
                "error": format!("Unknown Prometheus operation: {}", _operation),
                "available_operations": ["query", "export_metrics", "health_check"]
            })),
}
impl PrometheusExport {
    /// Execute Prometheus query
    async fn prometheus_query(&self, endpoint: &str, query: &str) -> BearDogResult<Value> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| BearDogError::configuration(format!("HTTP client error: {e}"),
            })?;
        let encoded_query = query.replace(" ", "%20").replace("=", "%3D");
        let url = format!("{endpoint}/api/v1/query?query={encoded_query}");
        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| BearDogError::configuration(format!("Prometheus query request failed: {e}"),
        if response.status().is_success() {
            let result: Value = response
                .json()
                .await
                .map_err(|e| BearDogError::configuration(format!("Failed to parse Prometheus response: {e}"),
                })?;
            Ok(result)
        } else {
            Err(BearDogError::configuration(format!("Prometheus query failed with status: }", response.status()),
            })
    /// Export BearDog metrics to Prometheus
    async fn export_beardog_metrics(
        endpoint: &str,
        metrics: &[Value],
    ) -> BearDogResult<usize> {
        // Convert metrics to Prometheus format
        let prometheus_metrics = self.convert_to_prometheus_format(metrics);
            .post(format!("{endpoint}/metrics"))
            .header("Content-Type", "text/plain")
            .body(prometheus_metrics.clone())
            .map_err(|e| BearDogError::configuration(format!("Prometheus export request failed: {e}"),
            tracing::info!("✅ Exported {} metrics to Prometheus", metrics.len());
            Ok(metrics.len())
            Err(BearDogError::configuration(format!(
                    "Prometheus export failed with status: }",
                    response.status()
    /// Check Prometheus health
    async fn prometheus_health_check(&self, endpoint: &str) -> Result<Value, SystemError> {
            .timeout(Duration::from_secs(10))
        let health_url = format!("{endpoint}/api/v1/status/runtimeinfo");
        match client.get(&health_url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Value>().await {
                        Ok(health_data) => Ok(serde_json::json!({
                            "status": "healthy",
                            "endpoint": endpoint,
                            "runtime_info": health_data
                        })),
                        Err(_) => Ok(serde_json::json!({
                            "message": "Prometheus is responding but runtime info unavailable"
                    }
                } else {
                    Ok(serde_json::json!({
                        "status": "unhealthy",
                        "error": format!("HTTP status: {}", response.status())
                    }))
            Err(e) => Err(BearDogError::configuration(format!("Prometheus health check failed: {e}"),
            }),
    /// Convert BearDog metrics to Prometheus format
    fn convert_to_prometheus_format(&self, metrics: &[Value]) -> String {
        let mut prometheus_output = String::new();
        for metric in metrics {
            if let Some(name) = metric.get("name").and_then(|v| v.as_str()) {
                let value = metric.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let help = metric.get("help").and_then(|v| v.as_str()).unwrap_or("");
                let metric_type = metric
                    .get("type")
                    .unwrap_or("gauge");
                // Add help and type annotations
                prometheus_output.push_str(&format!("# HELP {name} {help}\n"));
                prometheus_output.push_str(&format!("# TYPE {name} {metric_type}\n"));
                // Add labels if present
                if let Some(labels) = metric.get("labels").and_then(|v| v.as_object()) {
                    let label_string: Vec<String> = labels
                        .iter()
                        .map(|(k, v)| format!("{}=\"{}\"", k, v.as_str().unwrap_or("")))
                        .collect();
                    prometheus_output.push_str(&format!(
                        "{}{{{}}} {}\n",
                        name,
                        label_string.join(","),
                        value
                    ));
                    prometheus_output.push_str(&format!("{name} {value}\n"));
        prometheus_output
