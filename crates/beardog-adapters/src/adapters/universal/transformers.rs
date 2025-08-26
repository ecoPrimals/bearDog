

use beardog_errors::{BearDogError, BearDogResult};
use serde_json;

pub trait DataTransformer: Send + Sync {

    fn transform(&self, input: serde_json::Value) -> BearDogResult<serde_json::Value>;

    fn name(&self) -> &str;

    fn supported_inputs(&self) -> Vec<String>;

    fn supported_outputs(&self) -> Vec<String>;
}

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

pub struct KubernetesResourceTransformer;
impl Default for KubernetesResourceTransformer {}

impl KubernetesResourceTransformer {
impl DataTransformer for KubernetesResourceTransformer {

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
