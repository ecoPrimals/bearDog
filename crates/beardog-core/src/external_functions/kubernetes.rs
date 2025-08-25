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


/// Kubernetes Integration Handler
///
/// Provides licensed access to Kubernetes cluster operations

use super::ExternalFunctionHandler;
use crate::licensing::LicenseManager;
// Removed async_trait - using native async fn for zero-cost abstractions
use beardog_errors::{BearDogError, BearDogResult};
use beardog_errors::idiomatic::SecurityResult;
use serde_json;
use std::process::Stdio;
use tokio::process::Command;
/// **KUBERNETES INTEGRATION HANDLER** - Zero-cost implementation
#[derive(Clone)]
pub struct KubernetesIntegration;
// Uses native async fn from trait definition - no async_trait needed
impl ExternalFunctionHandler for KubernetesIntegration {}


    fn function_name(&self) -> &str {
        "kubernetes_integration"
    }
    /// Execute Kubernetes operation with licensing check
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
                    "🔒 Kubernetes integration '}' requires licensing or individual/small-team classification.\n\n\
                    🏠 Individual developers: Automatically granted access\n\
                    👥 Small teams: Automatically granted access\n\
                    🏢 Corporate usage: External adapters locked - acquire unlock certificate",
                    self.function_name()
                ),
            });
        }
        // Extract parameters
        let namespace = payload
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or("default");
        tracing::info!(
            "🚢 Kubernetes {} operation in namespace {}",
            _operation,
            namespace
        );
        // Implement actual Kubernetes API operations
        match _operation {
            "list_pods" => {
                match self
                    .kubectl_exec(&["get", "pods", "-n", namespace, "-o", "json"])
                    .await
                {
                    Ok(output) => Ok(serde_json::json!({
                        "operation": _operation,
                        "namespace": namespace,
                        "status": "success",
                        "data": output
                    })),
                    Err(e) => Ok(serde_json::json!({
                        "status": "error",
                        "error": e.to_string()
                }
            }
            "list_services" => {
                    .kubectl_exec(&["get", "services", "-n", namespace, "-o", "json"])
            "list_deployments" => {
                    .kubectl_exec(&["get", "deployments", "-n", namespace, "-o", "json"])
            "create_namespace" => {
                let namespace_name = payload
                    .get("namespace_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or(namespace);
                    .kubectl_exec(&["create", "namespace", namespace_name])
                        "namespace": namespace_name,
            "apply_yaml" => {
                let yaml_content = payload.get("yaml").and_then(|v| v.as_str()).unwrap_or("");
                if yaml_content.is_empty() {
                    return Ok(serde_json::json!({
                        "error": "yaml content is required for apply operation"
                    }));
                match self.kubectl_apply(yaml_content).await {
            "get_cluster_info" => match self.kubectl_exec(&["cluster-info"]).await {
                Ok(output) => Ok(serde_json::json!({
                    "operation": _operation,
                    "status": "success",
                    "data": output
                })),
                Err(e) => Ok(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
            },
            "get_nodes" => match self.kubectl_exec(&["get", "nodes", "-o", "json"]).await {
            _ => Ok(serde_json::json!({
                "operation": _operation,
                "status": "error",
                "error": format!("Unknown Kubernetes operation: {}", _operation),
                "available_operations": [
                    "list_pods", "list_services", "list_deployments",
                    "create_namespace", "apply_yaml", "get_cluster_info", "get_nodes"
                ]
            })),
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
            .map_err(|e| BearDogError::configuration(format!("kubectl command failed: {e}"),
            })?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(BearDogError::configuration(format!("kubectl error: {error)"},
            })
    /// Apply YAML configuration via kubectl
    async fn kubectl_apply(&self, yaml_content: &str) -> Result<String, SecurityError> {
        use std::process::Stdio;
        use tokio::io::AsyncWriteExt;
        let mut child = Command::new("kubectl")
            .args(["apply", "-f", "-"])
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| BearDogError::configuration(format!("Failed to start kubectl apply: {e}"),
        // Write YAML to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(yaml_content.as_bytes())
                .await
                .map_err(|e| BearDogError::configuration(format!("Failed to write to kubectl stdin: {e}"),
                })?;
        let output = child
            .wait_with_output()
            .map_err(|e| BearDogError::configuration(format!("kubectl apply command failed: {e}"),
            Err(BearDogError::configuration(format!("kubectl apply error: {error}"),
