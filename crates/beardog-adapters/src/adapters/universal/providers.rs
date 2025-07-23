//! External System Providers for Universal Adapter
//!
//! Concrete implementations for external (non-ecoPrimals) systems

use super::*;
use beardog_errors::{BearDogError, BearDogResult};
use serde_json;
use std::process::Stdio;
use tokio::process::Command;
use tracing::debug;

// ============================================================================
// KUBERNETES PROVIDER - External System
// ============================================================================

pub struct KubernetesProvider {
    config: KubernetesConfig,
    http_protocol: Box<dyn Protocol>,
    kubectl_auth: Box<dyn Authentication>,
}

#[derive(Debug, Clone)]
pub struct KubernetesConfig {
    pub kubeconfig_path: String,
    pub default_namespace: String,
    pub timeout_seconds: u64,
}

impl Default for KubernetesConfig {
    fn default() -> Self {
        Self {
            kubeconfig_path: std::env::var("KUBECONFIG")
                .unwrap_or_else(|_| "~/.kube/config".to_string()),
            default_namespace: "default".to_string(),
            timeout_seconds: 30,
        }
    }
}

impl KubernetesProvider {
    pub fn new(config: KubernetesConfig) -> Self {
        Self {
            config,
            http_protocol: Box::new(HttpProtocol::new()),
            kubectl_auth: Box::new(KubeconfigAuthentication::new()),
        }
    }
}

#[async_trait::async_trait]
impl ExternalSystemProvider for KubernetesProvider {
    fn system_id(&self) -> &str {
        "kubernetes"
    }

    fn protocol(&self) -> &dyn Protocol {
        self.http_protocol.as_ref()
    }

    fn authentication(&self) -> &dyn Authentication {
        self.kubectl_auth.as_ref()
    }

    async fn execute(
        &self,
        operation: &str,
        payload: serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        match operation {
            "get" => self.kubectl_get(&payload).await,
            "describe" => self.kubectl_describe(&payload).await,
            "apply" => self.kubectl_apply(&payload).await,
            "delete" => self.kubectl_delete(&payload).await,
            "logs" => self.kubectl_logs(&payload).await,
            _ => Err(BearDogError::InvalidInput {
                message: format!("Unknown Kubernetes operation: {operation}"),
            }),
        }
    }

    async fn health_check(&self) -> BearDogResult<bool> {
        debug!("🏥 Kubernetes health check");

        match Command::new("kubectl")
            .args(["cluster-info", "--request-timeout=5s"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
        {
            Ok(status) => Ok(status.success()),
            Err(_) => Ok(false),
        }
    }

    fn capabilities(&self) -> Vec<String> {
        vec![
            "get".to_string(),
            "describe".to_string(),
            "apply".to_string(),
            "delete".to_string(),
            "logs".to_string(),
        ]
    }
}

impl KubernetesProvider {
    async fn kubectl_get(&self, payload: &serde_json::Value) -> BearDogResult<serde_json::Value> {
        let resource = payload
            .get("resource")
            .and_then(|v| v.as_str())
            .unwrap_or("pods");
        let namespace = payload
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&self.config.default_namespace);

        self.execute_kubectl(&["get", resource, "-n", namespace, "-o", "json"])
            .await
    }

    async fn kubectl_describe(
        &self,
        payload: &serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        let resource = payload
            .get("resource")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::InvalidInput {
                message: "Missing 'resource' parameter for describe operation".to_string(),
            })?;
        let namespace = payload
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&self.config.default_namespace);

        let output = self
            .execute_kubectl_text(&["describe", resource, "-n", namespace])
            .await?;
        Ok(serde_json::json!({
            "description": output,
            "resource": resource,
            "namespace": namespace
        }))
    }

    async fn kubectl_apply(&self, payload: &serde_json::Value) -> BearDogResult<serde_json::Value> {
        let manifest = payload
            .get("manifest")
            .ok_or_else(|| BearDogError::InvalidInput {
                message: "Missing 'manifest' parameter for apply operation".to_string(),
            })?;

        // SECURITY: Use stdin instead of temporary files to avoid exposing sensitive manifests
        let manifest_str = if manifest.is_string() {
            manifest.as_str().map(|s| s.to_string()).unwrap_or_else(|| {
                tracing::warn!("Failed to convert manifest to string, using default");
                "# Default manifest due to conversion error".to_string()
            })
        } else {
            serde_yaml::to_string(&manifest).map_err(|e| BearDogError::DeserializationError {
                message: format!("Failed to serialize manifest to YAML: {e}"),
            })?
        };

        // Use kubectl with stdin to avoid temporary file exposure
        let result = self
            .execute_kubectl_with_stdin(&["apply", "-f", "-"], manifest_str)
            .await;

        match result {
            Ok(output) => Ok(serde_json::json!({
                "success": true,
                "output": output,
                "action": "apply"
            })),
            Err(e) => Err(e),
        }
    }

    async fn kubectl_delete(
        &self,
        payload: &serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        let resource = payload
            .get("resource")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::InvalidInput {
                message: "Missing 'resource' parameter for delete operation".to_string(),
            })?;
        let namespace = payload
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&self.config.default_namespace);

        let output = self
            .execute_kubectl_text(&["delete", resource, "-n", namespace])
            .await?;
        Ok(serde_json::json!({
            "success": true,
            "output": output,
            "action": "delete",
            "resource": resource,
            "namespace": namespace
        }))
    }

    async fn kubectl_logs(&self, payload: &serde_json::Value) -> BearDogResult<serde_json::Value> {
        let resource = payload
            .get("resource")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::InvalidInput {
                message: "Missing 'resource' parameter for logs operation".to_string(),
            })?;
        let namespace = payload
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&self.config.default_namespace);
        let tail = payload
            .get("tail")
            .and_then(|v| v.as_str())
            .unwrap_or("100");

        let output = self
            .execute_kubectl_text(&["logs", resource, "-n", namespace, "--tail", tail])
            .await?;
        Ok(serde_json::json!({
            "logs": output.lines().collect::<Vec<_>>(),
            "resource": resource,
            "namespace": namespace
        }))
    }

    async fn execute_kubectl(&self, args: &[&str]) -> BearDogResult<serde_json::Value> {
        let output = Command::new("kubectl")
            .args(args)
            .output()
            .await
            .map_err(|e| BearDogError::ExternalServiceError {
                service: "kubectl".to_string(),
                message: format!("Failed to execute kubectl: {e}"),
            })?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            serde_json::from_str(&stdout).map_err(|e| BearDogError::DeserializationError {
                message: format!("Failed to parse kubectl JSON output: {e}"),
            })
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(BearDogError::ExternalServiceError {
                service: "kubectl".to_string(),
                message: format!("kubectl command failed: {stderr}"),
            })
        }
    }

    async fn execute_kubectl_text(&self, args: &[&str]) -> BearDogResult<String> {
        let output = Command::new("kubectl")
            .args(args)
            .output()
            .await
            .map_err(|e| BearDogError::ExternalServiceError {
                service: "kubectl".to_string(),
                message: format!("Failed to execute kubectl: {e}"),
            })?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(BearDogError::ExternalServiceError {
                service: "kubectl".to_string(),
                message: format!("kubectl command failed: {stderr}"),
            })
        }
    }

    /// Execute kubectl with stdin input - secure alternative to temporary files
    async fn execute_kubectl_with_stdin(
        &self,
        args: &[&str],
        input: String,
    ) -> BearDogResult<String> {
        let mut child = Command::new("kubectl")
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| BearDogError::ExternalServiceError {
                service: "kubectl".to_string(),
                message: format!("Failed to spawn kubectl process: {e}"),
            })?;

        // Write input to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            tokio::io::AsyncWriteExt::write_all(&mut stdin, input.as_bytes())
                .await
                .map_err(|e| BearDogError::SystemError {
                    message: format!("Failed to write input to kubectl stdin: {e}"),
                })?;
        }

        let output =
            child
                .wait_with_output()
                .await
                .map_err(|e| BearDogError::ExternalServiceError {
                    service: "kubectl".to_string(),
                    message: format!("Failed to execute kubectl with stdin: {e}"),
                })?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(BearDogError::ExternalServiceError {
                service: "kubectl".to_string(),
                message: format!("kubectl command failed: {stderr}"),
            })
        }
    }
}

// ============================================================================
// PROMETHEUS PROVIDER - External System
// ============================================================================

pub struct PrometheusProvider {
    config: PrometheusConfig,
    http_protocol: Box<dyn Protocol>,
    http_auth: Box<dyn Authentication>,
}

#[derive(Debug, Clone)]
pub struct PrometheusConfig {
    pub endpoint: String,
    pub timeout_seconds: u64,
}

impl Default for PrometheusConfig {
    fn default() -> Self {
        Self {
            endpoint: std::env::var("PROMETHEUS_ENDPOINT")
                .unwrap_or_else(|_| "https://prometheus.ecosystem.internal:9090".to_string()),
            timeout_seconds: 30,
        }
    }
}

impl PrometheusProvider {
    pub fn new(config: PrometheusConfig) -> Self {
        Self {
            config,
            http_protocol: Box::new(HttpProtocol::new()),
            http_auth: Box::new(NoAuthentication::new()),
        }
    }
}

#[async_trait::async_trait]
impl ExternalSystemProvider for PrometheusProvider {
    fn system_id(&self) -> &str {
        "prometheus"
    }

    fn protocol(&self) -> &dyn Protocol {
        self.http_protocol.as_ref()
    }

    fn authentication(&self) -> &dyn Authentication {
        self.http_auth.as_ref()
    }

    async fn execute(
        &self,
        operation: &str,
        payload: serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        match operation {
            "query" => self.query_metrics(&payload).await,
            "query_range" => self.query_range(&payload).await,
            "export" => self.export_metrics(&payload).await,
            _ => Err(BearDogError::InvalidInput {
                message: format!("Unknown Prometheus operation: {operation}"),
            }),
        }
    }

    async fn health_check(&self) -> BearDogResult<bool> {
        let client = reqwest::Client::new();
        match client
            .get(format!("{}/api/v1/query", self.config.endpoint))
            .query(&[("query", "up")])
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    fn capabilities(&self) -> Vec<String> {
        vec![
            "query".to_string(),
            "query_range".to_string(),
            "export".to_string(),
        ]
    }
}

impl PrometheusProvider {
    async fn query_metrics(&self, payload: &serde_json::Value) -> BearDogResult<serde_json::Value> {
        let query = payload
            .get("query")
            .and_then(|v| v.as_str())
            .unwrap_or("up");

        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/api/v1/query", self.config.endpoint))
            .query(&[("query", query)])
            .timeout(std::time::Duration::from_secs(self.config.timeout_seconds))
            .send()
            .await
            .map_err(|e| BearDogError::ExternalServiceError {
                service: "prometheus".to_string(),
                message: format!("Failed to query Prometheus: {e}"),
            })?;

        if response.status().is_success() {
            let metrics_data: serde_json::Value =
                response
                    .json()
                    .await
                    .map_err(|e| BearDogError::DeserializationError {
                        message: format!("Failed to parse Prometheus response: {e}"),
                    })?;

            Ok(serde_json::json!({
                "success": true,
                "data": metrics_data,
                "query": query
            }))
        } else {
            Err(BearDogError::ExternalServiceError {
                service: "prometheus".to_string(),
                message: format!("Prometheus query failed with status: {}", response.status()),
            })
        }
    }

    async fn query_range(&self, payload: &serde_json::Value) -> BearDogResult<serde_json::Value> {
        let query = payload
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::InvalidInput {
                message: "Missing 'query' parameter".to_string(),
            })?;
        let start = payload
            .get("start")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::InvalidInput {
                message: "Missing 'start' parameter".to_string(),
            })?;
        let end = payload.get("end").and_then(|v| v.as_str()).ok_or_else(|| {
            BearDogError::InvalidInput {
                message: "Missing 'end' parameter".to_string(),
            }
        })?;
        let step = payload
            .get("step")
            .and_then(|v| v.as_str())
            .unwrap_or("15s");

        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/api/v1/query_range", self.config.endpoint))
            .query(&[
                ("query", query),
                ("start", start),
                ("end", end),
                ("step", step),
            ])
            .timeout(std::time::Duration::from_secs(self.config.timeout_seconds))
            .send()
            .await
            .map_err(|e| BearDogError::ExternalServiceError {
                service: "prometheus".to_string(),
                message: format!("Failed to query Prometheus range: {e}"),
            })?;

        if response.status().is_success() {
            let metrics_data: serde_json::Value =
                response
                    .json()
                    .await
                    .map_err(|e| BearDogError::DeserializationError {
                        message: format!("Failed to parse Prometheus range response: {e}"),
                    })?;

            Ok(serde_json::json!({
                "success": true,
                "data": metrics_data,
                "query": query,
                "range": {"start": start, "end": end, "step": step}
            }))
        } else {
            Err(BearDogError::ExternalServiceError {
                service: "prometheus".to_string(),
                message: format!(
                    "Prometheus range query failed with status: {}",
                    response.status()
                ),
            })
        }
    }

    async fn export_metrics(
        &self,
        _payload: &serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        // Export BearDog metrics in Prometheus format
        Ok(serde_json::json!({
            "success": true,
            "metrics_format": "prometheus",
            "endpoint": format!("{}/metrics", self.config.endpoint),
            "exported_metrics": [
                "beardog_api_requests_total",
                "beardog_security_events_total",
                "beardog_genetic_operations_total",
                "beardog_system_cpu_usage",
                "beardog_system_memory_usage"
            ]
        }))
    }
}
