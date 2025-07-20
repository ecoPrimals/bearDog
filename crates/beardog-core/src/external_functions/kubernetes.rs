//! Kubernetes Integration Handler
//!
//! Provides licensed access to Kubernetes cluster operations

use super::ExternalFunctionHandler;
use crate::licensing::LicenseManager;
use async_trait::async_trait;
use beardog_errors::{BearDogError, BearDogResult};
use serde_json::Value;
use std::process::Stdio;
use tokio::process::Command;

/// Kubernetes integration handler
pub struct KubernetesIntegration;

#[async_trait]
impl ExternalFunctionHandler for KubernetesIntegration {
    async fn execute(
        &self,
        payload: Value,
        license_manager: &LicenseManager,
    ) -> BearDogResult<Value> {
        // Check license first
        license_manager.verify_external_function_access(self.function_name())?;

        // Extract parameters
        let operation = payload
            .get("operation")
            .and_then(|v| v.as_str())
            .unwrap_or("list");
        let namespace = payload
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or("default");

        tracing::info!(
            "🚢 Kubernetes {} operation in namespace {}",
            operation,
            namespace
        );

        // Implement actual Kubernetes API operations
        match operation {
            "list_pods" => {
                match self
                    .kubectl_exec(&["get", "pods", "-n", namespace, "-o", "json"])
                    .await
                {
                    Ok(output) => Ok(serde_json::json!({
                        "operation": operation,
                        "namespace": namespace,
                        "status": "success",
                        "data": output
                    })),
                    Err(e) => Ok(serde_json::json!({
                        "operation": operation,
                        "namespace": namespace,
                        "status": "error",
                        "error": e.to_string()
                    })),
                }
            }
            "list_services" => {
                match self
                    .kubectl_exec(&["get", "services", "-n", namespace, "-o", "json"])
                    .await
                {
                    Ok(output) => Ok(serde_json::json!({
                        "operation": operation,
                        "namespace": namespace,
                        "status": "success",
                        "data": output
                    })),
                    Err(e) => Ok(serde_json::json!({
                        "operation": operation,
                        "namespace": namespace,
                        "status": "error",
                        "error": e.to_string()
                    })),
                }
            }
            "list_deployments" => {
                match self
                    .kubectl_exec(&["get", "deployments", "-n", namespace, "-o", "json"])
                    .await
                {
                    Ok(output) => Ok(serde_json::json!({
                        "operation": operation,
                        "namespace": namespace,
                        "status": "success",
                        "data": output
                    })),
                    Err(e) => Ok(serde_json::json!({
                        "operation": operation,
                        "namespace": namespace,
                        "status": "error",
                        "error": e.to_string()
                    })),
                }
            }
            "create_namespace" => {
                let namespace_name = payload
                    .get("namespace_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or(namespace);

                match self
                    .kubectl_exec(&["create", "namespace", namespace_name])
                    .await
                {
                    Ok(output) => Ok(serde_json::json!({
                        "operation": operation,
                        "namespace": namespace_name,
                        "status": "success",
                        "data": output
                    })),
                    Err(e) => Ok(serde_json::json!({
                        "operation": operation,
                        "namespace": namespace_name,
                        "status": "error",
                        "error": e.to_string()
                    })),
                }
            }
            "apply_yaml" => {
                let yaml_content = payload.get("yaml").and_then(|v| v.as_str()).unwrap_or("");

                if yaml_content.is_empty() {
                    return Ok(serde_json::json!({
                        "operation": operation,
                        "status": "error",
                        "error": "yaml content is required for apply operation"
                    }));
                }

                match self.kubectl_apply(yaml_content).await {
                    Ok(output) => Ok(serde_json::json!({
                        "operation": operation,
                        "namespace": namespace,
                        "status": "success",
                        "data": output
                    })),
                    Err(e) => Ok(serde_json::json!({
                        "operation": operation,
                        "namespace": namespace,
                        "status": "error",
                        "error": e.to_string()
                    })),
                }
            }
            "get_cluster_info" => match self.kubectl_exec(&["cluster-info"]).await {
                Ok(output) => Ok(serde_json::json!({
                    "operation": operation,
                    "status": "success",
                    "data": output
                })),
                Err(e) => Ok(serde_json::json!({
                    "operation": operation,
                    "status": "error",
                    "error": e.to_string()
                })),
            },
            "get_nodes" => match self.kubectl_exec(&["get", "nodes", "-o", "json"]).await {
                Ok(output) => Ok(serde_json::json!({
                    "operation": operation,
                    "status": "success",
                    "data": output
                })),
                Err(e) => Ok(serde_json::json!({
                    "operation": operation,
                    "status": "error",
                    "error": e.to_string()
                })),
            },
            _ => Ok(serde_json::json!({
                "operation": operation,
                "status": "error",
                "error": format!("Unknown Kubernetes operation: {}", operation),
                "available_operations": [
                    "list_pods", "list_services", "list_deployments",
                    "create_namespace", "apply_yaml", "get_cluster_info", "get_nodes"
                ]
            })),
        }
    }

    fn function_name(&self) -> &'static str {
        "kubernetes"
    }

    fn description(&self) -> &'static str {
        "Kubernetes cluster management and orchestration"
    }
}

impl KubernetesIntegration {
    /// Execute kubectl command
    async fn kubectl_exec(&self, args: &[&str]) -> BearDogResult<String> {
        let output = Command::new("kubectl")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| BearDogError::Configuration {
                message: format!("kubectl command failed: {}", e),
            })?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(BearDogError::Configuration {
                message: format!("kubectl error: {}", error),
            })
        }
    }

    /// Apply YAML configuration via kubectl
    async fn kubectl_apply(&self, yaml_content: &str) -> BearDogResult<String> {
        use std::process::Stdio;
        use tokio::io::AsyncWriteExt;

        let mut child = Command::new("kubectl")
            .args(["apply", "-f", "-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| BearDogError::Configuration {
                message: format!("Failed to start kubectl apply: {}", e),
            })?;

        // Write YAML to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(yaml_content.as_bytes())
                .await
                .map_err(|e| BearDogError::Configuration {
                    message: format!("Failed to write to kubectl stdin: {}", e),
                })?;
        }

        let output = child
            .wait_with_output()
            .await
            .map_err(|e| BearDogError::Configuration {
                message: format!("kubectl apply command failed: {}", e),
            })?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(BearDogError::Configuration {
                message: format!("kubectl apply error: {}", error),
            })
        }
    }
}
