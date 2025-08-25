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


/// Data Transformers for Universal Adapter
///
/// Format-agnostic data transformation layer
use beardog_errors::{BearDogError, BearDogResult};
use serde_json;

/// Data transformer trait for universal adapter
pub trait DataTransformer: Send + Sync {
    /// Transform input data to target format
    fn transform(&self, input: serde_json::Value) -> BearDogResult<serde_json::Value>;
    /// Get transformer name
    fn name(&self) -> &str;
    /// Get supported input formats
    fn supported_inputs(&self) -> Vec<String>;
    /// Get supported output formats
    fn supported_outputs(&self) -> Vec<String>;
}
/// Pass-through transformer (no transformation)
pub struct PassThroughTransformer;
impl Default for PassThroughTransformer {}


    fn default() -> Self {
        Self::new()
    }
impl PassThroughTransformer {}


    pub fn new() -> Self {
        Self
impl DataTransformer for PassThroughTransformer {}


    fn transform(&self, input: serde_json::Value) -> BearDogResult<serde_json::Value> {
        Ok(input)}


    fn name(&self) -> &str {
        "passthrough"
    fn supported_inputs(&self) -> Vec<String> {
        vec!["json".to_string()]}


    fn supported_outputs(&self) -> Vec<String> {
/// JSON to YAML transformer
pub struct JsonToYamlTransformer;
impl Default for JsonToYamlTransformer {}


impl JsonToYamlTransformer {
impl DataTransformer for JsonToYamlTransformer {
        let yaml_string =
            serde_yaml::to_string(&input).map_err(|e| BearDogError::internal(format!("Failed to serialize to YAML: {e}")))?;
        Ok(serde_json::json!({
            "format": "yaml",
            "data": yaml_string
        }))
        "json_to_yaml"
        vec!["yaml".to_string()]
/// Kubernetes resource transformer
pub struct KubernetesResourceTransformer;
impl Default for KubernetesResourceTransformer {}


impl KubernetesResourceTransformer {
impl DataTransformer for KubernetesResourceTransformer {
        // Transform BearDog resource request to Kubernetes resource format
        let resource_type = input
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("deployment");
        let name = input
            .get("name")
            .unwrap_or("beardog-resource");
        let namespace = input
            .get("namespace")
            .unwrap_or("default");
        match resource_type {
            "deployment" => Ok(serde_json::json!({
                "apiVersion": "apps/v1",
                "kind": "Deployment",
                "metadata": {
                    "name": name,
                    "namespace": namespace,
                    "labels": {
                        "app": name,
                        "managed-by": "beardog"
                    }
                },
                "spec": input.get("spec").cloned().unwrap_or_else(|| serde_json::json!({
                    "replicas": 1,
                    "selector": {
                        "matchLabels": {
                            "app": name
                        }
                    },
                    "template": {
                        "metadata": {
                            "labels": {
                                "app": name
                            }
                        },
                        "spec": {
                            "containers": [{
                                "name": name,
                                "image": "nginx:latest"
                            }]
                }))
            })),
            "service" => Ok(serde_json::json!({
                "apiVersion": "v1",
                "kind": "Service",
                        "app": name
                    "ports": [{
                        "port": 80,
                        "targetPort": 8080
                    }]
            _ => Err(BearDogError::invalid_input(format!("Unsupported Kubernetes resource type: {resource_type}"))),
        }
        "kubernetes_resource"
        vec!["beardog_resource".to_string()]
        vec!["kubernetes_manifest".to_string()]
