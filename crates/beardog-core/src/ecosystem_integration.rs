//! Ecosystem Integration for BearDog
//!
//! Provides integration with external systems through licensed external functions.
//! This is the core revenue-generating capability of BearDog.

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use tracing::{debug, error, info, warn};

use std::sync::Arc;
use tokio::process::Command as TokioCommand;

/// External function request from ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemRequest {
    pub function_name: String,
    pub payload: serde_json::Value,
    pub metadata: HashMap<String, String>,
}

/// External function response to ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemResponse {
    pub success: bool,
    pub payload: serde_json::Value,
    pub metadata: HashMap<String, String>,
    pub processing_time_ms: u64,
}

/// Ecosystem integration engine
pub struct EcosystemIntegration {
    license_manager: Arc<LicenseManager>,
    config: EcosystemConfig,
}

/// Configuration for ecosystem integration
#[derive(Debug, Clone)]
pub struct EcosystemConfig {
    pub kubernetes_config_path: String,
    pub prometheus_endpoint: String,
    pub grafana_endpoint: String,
    pub grafana_api_key: String,
    pub aws_region: String,
    pub azure_tenant_id: String,
    pub splunk_endpoint: String,
    pub splunk_token: String,
    pub timeout_seconds: u64,
}

impl Default for EcosystemConfig {
    fn default() -> Self {
        Self {
            kubernetes_config_path: std::env::var("KUBECONFIG")
                .unwrap_or_else(|_| "~/.kube/config".to_string()),
            prometheus_endpoint: std::env::var("PROMETHEUS_ENDPOINT")
                .unwrap_or_else(|_| "http://prometheus.ecosystem.internal:9090".to_string()),
            grafana_endpoint: std::env::var("GRAFANA_ENDPOINT")
                .unwrap_or_else(|_| "http://grafana.ecosystem.internal:3000".to_string()),
            grafana_api_key: std::env::var("GRAFANA_API_KEY")
                .unwrap_or_else(|_| "admin".to_string()),
            aws_region: std::env::var("AWS_REGION").unwrap_or_else(|_| "us-west-2".to_string()),
            azure_tenant_id: std::env::var("AZURE_TENANT_ID").unwrap_or_else(|_| "".to_string()),
            splunk_endpoint: std::env::var("SPLUNK_HEC_ENDPOINT")
                .unwrap_or_else(|_| "https://splunk.ecosystem.internal:8088".to_string()),
            splunk_token: std::env::var("SPLUNK_HEC_TOKEN").unwrap_or_else(|_| "".to_string()),
            timeout_seconds: 30,
        }
    }
}

/// License manager for external functions
pub struct LicenseManager {
    // License validation logic would go here
}

impl Default for LicenseManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LicenseManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn verify_external_function_access(&self, function_name: &str) -> BearDogResult<bool> {
        debug!("Verifying access to external function: {}", function_name);

        // SECURITY: For now, we fail securely by rejecting license verification
        // until proper Ed25519 signature verification is implemented
        // This prevents any potential authentication bypass

        // Future implementation will use these variables for real signature verification:
        let _license_public_key = [
            0x1a, 0x2b, 0x3c, 0x4d, 0x5e, 0x6f, 0x70, 0x81, 0x92, 0xa3, 0xb4, 0xc5, 0xd6, 0xe7,
            0xf8, 0x09, 0x0a, 0x1b, 0x2c, 0x3d, 0x4e, 0x5f, 0x60, 0x71, 0x82, 0x93, 0xa4, 0xb5,
            0xc6, 0xd7, 0xe8, 0xf9,
        ];

        // Message format for future signature verification
        let _license_message = format!("beardog-license:{function_name}:external-function");

        tracing::error!(
            "SECURITY: License verification not implemented - rejecting for safety. \
             Function '{}' requires proper license signature verification.",
            function_name
        );

        Err(BearDogError::Authentication {
            message: format!(
                "License verification not implemented - access denied for function: {function_name}. \
                 Contact support to implement proper licensing."
            ),
        })
    }
}

impl Default for EcosystemIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl EcosystemIntegration {
    pub fn new() -> Self {
        Self {
            license_manager: Arc::new(LicenseManager::new()),
            config: EcosystemConfig::default(),
        }
    }

    pub fn with_config(config: EcosystemConfig) -> Self {
        Self {
            license_manager: Arc::new(LicenseManager::new()),
            config,
        }
    }

    /// Handle ecosystem request and route to appropriate external function
    pub async fn handle_ecosystem_request(
        &self,
        request: EcosystemRequest,
    ) -> BearDogResult<EcosystemResponse> {
        let start_time = std::time::Instant::now();

        info!(
            "🌐 Handling ecosystem request for function: {}",
            request.function_name
        );

        // Verify license access
        self.license_manager
            .verify_external_function_access(&request.function_name)?;

        let payload = match request.function_name.as_str() {
            // HIGH PRIORITY IMPLEMENTATIONS (Revenue Critical)
            "kubernetes_integration" => self.handle_kubernetes_request(request.payload).await?,
            "prometheus_export" => self.handle_prometheus_export(request.payload).await?,
            "grafana_dashboards" => self.handle_grafana_dashboard(request.payload).await?,

            // MEDIUM PRIORITY IMPLEMENTATIONS
            "aws_kms_integration" => self.handle_aws_kms(request.payload).await?,
            "azure_key_vault" => self.handle_azure_keyvault(request.payload).await?,
            "splunk_integration" => self.handle_splunk_integration(request.payload).await?,

            // FUTURE IMPLEMENTATIONS
            "active_directory" => self.handle_active_directory(request.payload).await?,
            "ldap_integration" => self.handle_ldap_integration(request.payload).await?,
            "okta_sso" => self.handle_okta_sso(request.payload).await?,
            "hashicorp_vault" => self.handle_hashicorp_vault(request.payload).await?,

            _ => {
                warn!("Unknown external function: {}", request.function_name);
                return Err(BearDogError::InvalidInput {
                    message: format!("Unknown external function: {}", request.function_name),
                });
            }
        };

        let processing_time = start_time.elapsed().as_millis() as u64;

        Ok(EcosystemResponse {
            success: true,
            payload,
            metadata: HashMap::new(),
            processing_time_ms: processing_time,
        })
    }

    // ============================================================================
    // HIGH PRIORITY EXTERNAL FUNCTIONS (Revenue Critical)
    // ============================================================================

    /// Real Kubernetes integration using kubectl API
    async fn handle_kubernetes_request(
        &self,
        payload: serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        info!("🐳 Executing Kubernetes integration");

        let action = payload
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("get");
        let resource = payload
            .get("resource")
            .and_then(|v| v.as_str())
            .unwrap_or("pods");
        let namespace = payload
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or("default");

        match action {
            "get" => self.kubectl_get(resource, namespace).await,
            "describe" => self.kubectl_describe(resource, namespace).await,
            "logs" => self.kubectl_logs(resource, namespace).await,
            "apply" => self.kubectl_apply(&payload).await,
            "delete" => self.kubectl_delete(resource, namespace).await,
            _ => Err(BearDogError::InvalidInput {
                message: format!("Unknown Kubernetes action: {action}"),
            }),
        }
    }

    async fn kubectl_get(
        &self,
        resource: &str,
        namespace: &str,
    ) -> BearDogResult<serde_json::Value> {
        debug!("kubectl get {} -n {}", resource, namespace);

        let output = TokioCommand::new("kubectl")
            .args(["get", resource, "-n", namespace, "-o", "json"])
            .output()
            .await
            .map_err(|e| BearDogError::ExternalServiceError {
                service: "kubectl".to_string(),
                message: format!("Failed to execute kubectl: {e}"),
            })?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            serde_json::from_str(&stdout).map_err(|e| BearDogError::DeserializationError {
                message: format!("Failed to parse kubectl output: {e}"),
            })
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(BearDogError::ExternalServiceError {
                service: "kubectl".to_string(),
                message: format!("kubectl command failed: {stderr}"),
            })
        }
    }

    async fn kubectl_describe(
        &self,
        resource: &str,
        namespace: &str,
    ) -> BearDogResult<serde_json::Value> {
        debug!("kubectl describe {} -n {}", resource, namespace);

        let output = TokioCommand::new("kubectl")
            .args(["describe", resource, "-n", namespace])
            .output()
            .await
            .map_err(|e| BearDogError::ExternalServiceError {
                service: "kubectl".to_string(),
                message: format!("Failed to execute kubectl describe: {e}"),
            })?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(serde_json::json!({
                "description": stdout,
                "resource": resource,
                "namespace": namespace
            }))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(BearDogError::ExternalServiceError {
                service: "kubectl".to_string(),
                message: format!("kubectl describe failed: {stderr}"),
            })
        }
    }

    async fn kubectl_logs(
        &self,
        resource: &str,
        namespace: &str,
    ) -> BearDogResult<serde_json::Value> {
        debug!("kubectl logs {} -n {}", resource, namespace);

        let output = TokioCommand::new("kubectl")
            .args(["logs", resource, "-n", namespace, "--tail=100"])
            .output()
            .await
            .map_err(|e| BearDogError::ExternalServiceError {
                service: "kubectl".to_string(),
                message: format!("Failed to execute kubectl logs: {e}"),
            })?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(serde_json::json!({
                "logs": stdout.lines().collect::<Vec<_>>(),
                "resource": resource,
                "namespace": namespace
            }))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(BearDogError::ExternalServiceError {
                service: "kubectl".to_string(),
                message: format!("kubectl logs failed: {stderr}"),
            })
        }
    }

    async fn kubectl_apply(&self, payload: &serde_json::Value) -> BearDogResult<serde_json::Value> {
        let manifest = payload
            .get("manifest")
            .ok_or_else(|| BearDogError::InvalidInput {
                message: "Missing manifest for kubectl apply".to_string(),
            })?;

        debug!("kubectl apply with manifest");

        // SECURITY: Use stdin instead of temporary files to avoid exposing sensitive manifests
        let manifest_str = serde_json::to_string_pretty(manifest).map_err(|e| {
            BearDogError::DeserializationError {
                message: format!("Failed to serialize manifest: {e}"),
            }
        })?;

        // Use kubectl apply -f - to read from stdin instead of temporary file
        let mut child = TokioCommand::new("kubectl")
            .args(["apply", "-f", "-"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| BearDogError::ExternalServiceError {
                service: "kubectl".to_string(),
                message: format!("Failed to spawn kubectl process: {e}"),
            })?;

        // Write manifest directly to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            tokio::io::AsyncWriteExt::write_all(&mut stdin, manifest_str.as_bytes())
                .await
                .map_err(|e| BearDogError::SystemError {
                    message: format!("Failed to write manifest to kubectl stdin: {e}"),
                })?;
        }

        let output =
            child
                .wait_with_output()
                .await
                .map_err(|e| BearDogError::ExternalServiceError {
                    service: "kubectl".to_string(),
                    message: format!("Failed to execute kubectl apply: {e}"),
                })?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(serde_json::json!({
                "success": true,
                "output": stdout,
                "action": "apply"
            }))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(BearDogError::ExternalServiceError {
                service: "kubectl".to_string(),
                message: format!("kubectl apply failed: {stderr}"),
            })
        }
    }

    async fn kubectl_delete(
        &self,
        resource: &str,
        namespace: &str,
    ) -> BearDogResult<serde_json::Value> {
        debug!("kubectl delete {} -n {}", resource, namespace);

        let output = TokioCommand::new("kubectl")
            .args(["delete", resource, "-n", namespace])
            .output()
            .await
            .map_err(|e| BearDogError::ExternalServiceError {
                service: "kubectl".to_string(),
                message: format!("Failed to execute kubectl delete: {e}"),
            })?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(serde_json::json!({
                "success": true,
                "output": stdout,
                "action": "delete",
                "resource": resource,
                "namespace": namespace
            }))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(BearDogError::ExternalServiceError {
                service: "kubectl".to_string(),
                message: format!("kubectl delete failed: {stderr}"),
            })
        }
    }

    /// Real Prometheus metrics export
    async fn handle_prometheus_export(
        &self,
        payload: serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        info!("📊 Exporting Prometheus metrics");

        let client = reqwest::Client::new();
        let metrics_endpoint = format!("{}/api/v1/query", self.config.prometheus_endpoint);

        let query = payload
            .get("query")
            .and_then(|v| v.as_str())
            .unwrap_or("up");

        let response = client
            .get(&metrics_endpoint)
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
                "metrics": metrics_data,
                "query": query,
                "endpoint": metrics_endpoint
            }))
        } else {
            Err(BearDogError::ExternalServiceError {
                service: "prometheus".to_string(),
                message: format!("Prometheus query failed with status: {}", response.status()),
            })
        }
    }

    /// Real Grafana dashboard creation
    async fn handle_grafana_dashboard(
        &self,
        payload: serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        info!("📈 Creating Grafana dashboard");

        let client = reqwest::Client::new();
        let dashboard_endpoint = format!("{}/api/dashboards/db", self.config.grafana_endpoint);

        let dashboard_title = payload
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("BearDog Dashboard");

        let dashboard_config = serde_json::json!({
            "dashboard": {
                "id": null,
                "title": dashboard_title,
                "tags": ["beardog", "security", "monitoring"],
                "timezone": "browser",
                "panels": [
                    {
                        "id": 1,
                        "title": "System CPU Usage",
                        "type": "stat",
                        "targets": [{
                            "expr": "100 - (avg(irate(node_cpu_seconds_total{mode=\"idle\"}[5m])) * 100)",
                            "legendFormat": "CPU Usage %"
                        }],
                        "gridPos": {"h": 8, "w": 12, "x": 0, "y": 0}
                    },
                    {
                        "id": 2,
                        "title": "Memory Usage",
                        "type": "stat",
                        "targets": [{
                            "expr": "(1 - (node_memory_MemAvailable_bytes / node_memory_MemTotal_bytes)) * 100",
                            "legendFormat": "Memory Usage %"
                        }],
                        "gridPos": {"h": 8, "w": 12, "x": 12, "y": 0}
                    },
                    {
                        "id": 3,
                        "title": "BearDog Security Events",
                        "type": "graph",
                        "targets": [{
                            "expr": "rate(beardog_security_events_total[5m])",
                            "legendFormat": "Security Events/sec"
                        }],
                        "gridPos": {"h": 8, "w": 24, "x": 0, "y": 8}
                    }
                ],
                "time": {"from": "now-1h", "to": "now"},
                "refresh": "30s"
            },
            "overwrite": true
        });

        let response = client
            .post(&dashboard_endpoint)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.grafana_api_key),
            )
            .header("Content-Type", "application/json")
            .json(&dashboard_config)
            .timeout(std::time::Duration::from_secs(self.config.timeout_seconds))
            .send()
            .await
            .map_err(|e| BearDogError::ExternalServiceError {
                service: "grafana".to_string(),
                message: format!("Failed to create Grafana dashboard: {e}"),
            })?;

        if response.status().is_success() {
            let dashboard_response: serde_json::Value =
                response
                    .json()
                    .await
                    .map_err(|e| BearDogError::DeserializationError {
                        message: format!("Failed to parse Grafana response: {e}"),
                    })?;

            Ok(serde_json::json!({
                "success": true,
                "dashboard": dashboard_response,
                "title": dashboard_title,
                "endpoint": dashboard_endpoint
            }))
        } else {
            let error_text = response
                .text()
                .await
                .map_err(|e| {
                    warn!("Failed to read Grafana error response: {}", e);
                    e
                })
                .unwrap_or_else(|_| "Failed to read Grafana error response".to_string());

            Err(BearDogError::ExternalServiceError {
                service: "grafana".to_string(),
                message: format!("Grafana dashboard creation failed: {error_text}"),
            })
        }
    }

    // ============================================================================
    // MEDIUM PRIORITY EXTERNAL FUNCTIONS
    // ============================================================================

    async fn handle_aws_kms(
        &self,
        _payload: serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        // SECURITY: Never return success for unimplemented cryptographic operations
        error!("SECURITY VIOLATION: AWS KMS integration attempted but not implemented");
        Err(BearDogError::Configuration {
            message: "AWS KMS integration not available - cannot perform cryptographic operations"
                .to_string(),
        })
    }

    async fn handle_azure_keyvault(
        &self,
        _payload: serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        // SECURITY: Never return success for unimplemented cryptographic operations
        error!("SECURITY VIOLATION: Azure Key Vault integration attempted but not implemented");
        Err(BearDogError::Configuration {
            message: "Azure Key Vault integration not available - cannot perform cryptographic operations".to_string(),
        })
    }

    async fn handle_splunk_integration(
        &self,
        payload: serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        info!("📊 Forwarding logs to Splunk");

        if self.config.splunk_token.is_empty() {
            return Err(BearDogError::ConfigurationError {
                message: "Splunk HEC token not configured".to_string(),
            });
        }

        let client = reqwest::Client::new();
        let hec_endpoint = format!("{}/services/collector/event", self.config.splunk_endpoint);

        let event_data = serde_json::json!({
            "time": chrono::Utc::now().timestamp(),
            "host": "beardog-node",
            "source": "beardog",
            "sourcetype": "_json",
            "event": payload
        });

        let response = client
            .post(&hec_endpoint)
            .header(
                "Authorization",
                format!("Splunk {}", self.config.splunk_token),
            )
            .header("Content-Type", "application/json")
            .json(&event_data)
            .timeout(std::time::Duration::from_secs(self.config.timeout_seconds))
            .send()
            .await
            .map_err(|e| BearDogError::ExternalServiceError {
                service: "splunk".to_string(),
                message: format!("Failed to send data to Splunk: {e}"),
            })?;

        if response.status().is_success() {
            let response_data: serde_json::Value = response
                .json()
                .await
                .unwrap_or_else(|_| serde_json::json!({"status": "success"}));

            Ok(serde_json::json!({
                "success": true,
                "splunk_response": response_data,
                "endpoint": hec_endpoint
            }))
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(BearDogError::ExternalServiceError {
                service: "splunk".to_string(),
                message: format!("Splunk HEC failed: {error_text}"),
            })
        }
    }

    // ============================================================================
    // FUTURE IMPLEMENTATIONS (Placeholder)
    // ============================================================================

    async fn handle_active_directory(
        &self,
        _payload: serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        warn!("Active Directory integration not yet implemented");
        Ok(serde_json::json!({"active_directory": "placeholder", "status": "not_implemented"}))
    }

    async fn handle_ldap_integration(
        &self,
        _payload: serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        warn!("LDAP integration not yet implemented");
        Ok(serde_json::json!({"ldap": "placeholder", "status": "not_implemented"}))
    }

    async fn handle_okta_sso(
        &self,
        _payload: serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        warn!("Okta SSO integration not yet implemented");
        Ok(serde_json::json!({"okta_sso": "placeholder", "status": "not_implemented"}))
    }

    async fn handle_hashicorp_vault(
        &self,
        _payload: serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        warn!("HashiCorp Vault integration not yet implemented");
        Ok(serde_json::json!({"hashicorp_vault": "placeholder", "status": "not_implemented"}))
    }
}
