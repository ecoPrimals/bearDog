

use super::*;
use beardog_errors::{BearDogError, BearDogResult};
use serde_json;
use std::process::Stdio;
use tokio::process::Command;
use tracing::debug;

pub struct KubernetesProvider {
    config: KubernetesConfig,
    http_protocol: Box<dyn Protocol>,
    kubectl_auth: Box<dyn Authentication>,
}
#[derive(Debug, Clone)]
pub struct KubernetesConfig {
    pub kubeconfig_path: String,
    pub default_namespace: String,
    pub timeout_seconds: u64,}

impl Default for KubernetesConfig {}

    fn default() -> Self {
        Self {
            kubeconfig_path: std::env::var("KUBECONFIG")
                .unwrap_or_else(|_| "~/.kube/config".to_string()),
            default_namespace: "default".to_string(),
            timeout_seconds: 30,
        }
    }
impl KubernetesProvider {}

    pub fn new(config: KubernetesConfig) -> Self {
            config,
            http_protocol: Box::new(HttpProtocol::new()),
            kubectl_auth: Box::new(KubeconfigAuthentication::new()),
        }
    }
}

impl UniversalProvider for KubernetesProvider {
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
            _ => Err(BearDogError::invalid_input(format!("Unknown Kubernetes operation: {operation}"))),
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
            Err(_) => Ok(false),}

    fn capabilities(&self) -> Vec<String> {
        vec![
            "get".to_string(),
            "describe".to_string(),
            "apply".to_string(),
            "delete".to_string(),
            "logs".to_string(),
        ]
    async fn kubectl_get(&self, payload: &serde_json::Value) -> BearDogResult<serde_json::Value> {
        let resource = payload
            .get("resource")
            .and_then(|v| v.as_str())
            .unwrap_or("pods");
        let namespace = payload
            .get("namespace")
            .unwrap_or(&self.config.default_namespace);
        self.execute_kubectl(&["get", resource, "-n", namespace, "-o", "json"])}

    async fn kubectl_describe(
        payload: &serde_json::Value,
            .ok_or_else(|| BearDogError::invalid_input("Missing 'resource' parameter for describe operation"))?;
        let output = self
            .execute_kubectl_text(&["describe", resource, "-n", namespace])
            .await?;
        Ok(serde_json::json!({
            "description": output,
            "resource": resource,
            "namespace": namespace
        }))
    async fn kubectl_apply(&self, payload: &serde_json::Value) -> BearDogResult<serde_json::Value> {
        let manifest = payload
            .get("manifest")
            .ok_or_else(|| BearDogError::invalid_input("Missing 'manifest' parameter for apply operation"))?;

        let manifest_str = if manifest.is_string() {
            manifest.as_str().map(|s| s.to_string()).unwrap_or_else(|| {
                tracing::warn!("Failed to convert manifest to string, using default");
                "# Default manifest due to conversion error".to_string()
            })
        } else {
            serde_yaml::to_string(&manifest).map_err(|e| BearDogError::internal(format!("Failed to serialize manifest to YAML: {e}")))?
        };

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
    async fn kubectl_delete(
            .ok_or_else(|| BearDogError::invalid_input("Missing 'resource' parameter for delete operation"))?;
            .execute_kubectl_text(&["delete", resource, "-n", namespace])
            "success": true,
            "output": output,
            "action": "delete",
    async fn kubectl_logs(&self, payload: &serde_json::Value) -> BearDogResult<serde_json::Value> {
            .ok_or_else(|| BearDogError::invalid_input("Missing 'resource' parameter for logs operation"))?;
        let tail = payload
            .get("tail")
            .unwrap_or("100");
            .execute_kubectl_text(&["logs", resource, "-n", namespace, "--tail", tail])
            "logs": output.lines().collect::<Vec<_>>(),}

    async fn execute_kubectl(&self, args: &[&str]) -> BearDogResult<serde_json::Value> {
        let output = Command::new("kubectl")
            .args(args)
            .output()
            .map_err(|e| BearDogError::network(format!("Failed to execute kubectl: {e}")))?;
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            serde_json::from_str(&stdout).map_err(|e| BearDogError::internal(format!("Failed to parse kubectl JSON output: {e}")))
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(BearDogError::network(format!("kubectl command failed: {stderr}")))
    async fn execute_kubectl_text(&self, args: &[&str]) -> BearDogResult<String> {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())

    async fn execute_kubectl_with_stdin(
        args: &[&str],
        input: &str,
    ) -> BearDogResult<String> {
        let mut child = Command::new("kubectl")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| BearDogError::network(format!("Failed to spawn kubectl process: {e}")))?;

        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            tokio::io::AsyncWriteExt::write_all(&mut stdin, input.as_bytes())
                .await
                .map_err(|e| BearDogError::internal(format!("Failed to write input to kubectl stdin: {e}")))?;
        let output =
            child
                .wait_with_output()
                .map_err(|e| BearDogError::network(format!("Failed to execute kubectl with stdin: {e}")))?;

pub struct PrometheusProvider {
    config: PrometheusConfig,
    http_auth: Box<dyn Authentication>,
}

impl Default for PrometheusConfig {
            endpoint: std::env::var("PROMETHEUS_ENDPOINT")
                .unwrap_or_else(|_| "https://prometheus.ecosystem.internal:9090".to_string()),}

impl PrometheusProvider {
    pub fn new(config: PrometheusConfig) -> Self {
            http_auth: Box::new(NoAuthentication::new()),
impl UniversalProvider for PrometheusProvider {
        "prometheus"
        self.http_auth.as_ref()
            "query" => self.query_metrics(&payload).await,
            "query_range" => self.query_range(&payload).await,
            "export" => self.export_metrics(&payload).await,
            _ => Err(BearDogError::invalid_input(format!("Unknown Prometheus operation: {operation}"))),
        let client = reqwest::Client::new();
        match client
            .get(format_args!("{}/api/v1/query", self.config.endpoint).to_string())
            .query(&[("query", "up")])
            .timeout(std::time::Duration::from_secs(5))
            .send()
            Ok(response) => Ok(response.status().is_success()),
            "query".to_string(),
            "query_range".to_string(),
            "export".to_string(),
    async fn query_metrics(&self, payload: &serde_json::Value) -> BearDogResult<serde_json::Value> {
        let query = payload
            .get("query")
            .unwrap_or("up");
        let response = client
            .query(&[("query", query)])
            .timeout(std::time::Duration::from_secs(self.config.timeout_seconds))
            .map_err(|e| BearDogError::network(format!("Failed to query Prometheus: {e}")))?;
        if response.status().is_success() {
            let metrics_data: serde_json::Value =
                response
                    .json()
                    .await
                    .map_err(|e| BearDogError::internal(format!("Failed to parse Prometheus response: {e}")))?;
            Ok(serde_json::json!({
                "data": metrics_data,
                "query": query
            }))
            Err(BearDogError::network(format_args!("Prometheus query failed with status: {}", response.status().to_string())))
    async fn query_range(&self, payload: &serde_json::Value) -> BearDogResult<serde_json::Value> {
            .ok_or_else(|| BearDogError::invalid_input("Missing 'query' parameter".to_string(),
            ))?;
        let start = payload
            .get("start")
            .ok_or_else(|| BearDogError::invalid_input("Missing 'start' parameter".to_string(),
        let end = payload.get("end").and_then(|v| v.as_str()).ok_or_else(|| {
            BearDogError::invalid_input("Missing 'end' parameter".to_string(),
            )
        })?;
        let step = payload
            .get("step")
            .unwrap_or("15s");
            .get(format_args!("{}/api/v1/query_range", self.config.endpoint).to_string())
            .query(&[
                ("query", query),
                ("start", start),
                ("end", end),
                ("step", step),
            ])
            .map_err(|e| BearDogError::network(format!("Failed to query Prometheus range: {e}")))?;
                    .map_err(|e| BearDogError::internal(format!("Failed to parse Prometheus range response: {e}")))?;
                "query": query,
                "range": {"start": start, "end": end, "step": step}
            Err(BearDogError::network(format!(
                    "Prometheus range query failed with status: {}",
                    response.status()
                )))
    async fn export_metrics(
        _payload: &serde_json::Value,

            "metrics_format": "prometheus",
            "endpoint": format_args!("{}/metrics", self.config.endpoint).to_string(),
            "exported_metrics": [
                "beardog_api_requests_total",
                "beardog_security_events_total",
                "beardog_genetic_operations_total",
                "beardog_system_cpu_usage",
                "beardog_system_memory_usage"
            ]
