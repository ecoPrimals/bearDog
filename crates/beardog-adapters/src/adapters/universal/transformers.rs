//! Data Transformers for Universal Adapter
//!
//! Format-agnostic data transformation layer

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

impl Default for PassThroughTransformer {
    fn default() -> Self {
        Self::new()
    }
}

impl PassThroughTransformer {
    pub fn new() -> Self {
        Self
    }
}

impl DataTransformer for PassThroughTransformer {
    fn transform(&self, input: serde_json::Value) -> BearDogResult<serde_json::Value> {
        Ok(input)
    }

    fn name(&self) -> &str {
        "passthrough"
    }

    fn supported_inputs(&self) -> Vec<String> {
        vec!["json".to_string()]
    }

    fn supported_outputs(&self) -> Vec<String> {
        vec!["json".to_string()]
    }
}

/// JSON to YAML transformer
pub struct JsonToYamlTransformer;

impl Default for JsonToYamlTransformer {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonToYamlTransformer {
    pub fn new() -> Self {
        Self
    }
}

impl DataTransformer for JsonToYamlTransformer {
    fn transform(&self, input: serde_json::Value) -> BearDogResult<serde_json::Value> {
        let yaml_string =
            serde_yaml::to_string(&input).map_err(|e| BearDogError::DeserializationError {
                message: format!("Failed to serialize to YAML: {e}"),
            })?;

        Ok(serde_json::json!({
            "format": "yaml",
            "data": yaml_string
        }))
    }

    fn name(&self) -> &str {
        "json_to_yaml"
    }

    fn supported_inputs(&self) -> Vec<String> {
        vec!["json".to_string()]
    }

    fn supported_outputs(&self) -> Vec<String> {
        vec!["yaml".to_string()]
    }
}

/// Kubernetes resource transformer
pub struct KubernetesResourceTransformer;

impl Default for KubernetesResourceTransformer {
    fn default() -> Self {
        Self::new()
    }
}

impl KubernetesResourceTransformer {
    pub fn new() -> Self {
        Self
    }
}

impl DataTransformer for KubernetesResourceTransformer {
    fn transform(&self, input: serde_json::Value) -> BearDogResult<serde_json::Value> {
        // Transform BearDog resource request to Kubernetes resource format
        let resource_type = input
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("deployment");
        let name = input
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("beardog-resource");
        let namespace = input
            .get("namespace")
            .and_then(|v| v.as_str())
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
                        }
                    }
                }))
            })),
            "service" => Ok(serde_json::json!({
                "apiVersion": "v1",
                "kind": "Service",
                "metadata": {
                    "name": name,
                    "namespace": namespace,
                    "labels": {
                        "app": name,
                        "managed-by": "beardog"
                    }
                },
                "spec": input.get("spec").cloned().unwrap_or_else(|| serde_json::json!({
                    "selector": {
                        "app": name
                    },
                    "ports": [{
                        "port": 80,
                        "targetPort": 8080
                    }]
                }))
            })),
            _ => Err(BearDogError::InvalidInput {
                message: format!("Unsupported Kubernetes resource type: {resource_type}"),
            }),
        }
    }

    fn name(&self) -> &str {
        "kubernetes_resource"
    }

    fn supported_inputs(&self) -> Vec<String> {
        vec!["beardog_resource".to_string()]
    }

    fn supported_outputs(&self) -> Vec<String> {
        vec!["kubernetes_manifest".to_string()]
    }
}
