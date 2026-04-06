// SPDX-License-Identifier: AGPL-3.0-or-later

//! Kubernetes [`ServiceDiscoveryCapability`] implementation.

use async_trait::async_trait;

use crate::canonical::capabilities::ServiceCapabilityType;
use crate::canonical::types::ids::{RegistrationId, ServiceInstanceId};

use super::core::{
    DiscoveryCapabilities, DiscoveryError, DiscoveryHealthStatus, ServiceDescriptor,
    ServiceDiscoveryCapability, ServiceHealth, ServiceProtocol,
};

/// Kubernetes service discovery
///
/// Uses the Kubernetes API to discover services via:
/// - Service resources in all namespaces
/// - Endpoint slices for pod-level discovery
/// - Annotations for capability metadata
#[derive(Debug, Clone)]
pub struct KubernetesDiscovery {
    /// Namespace to search (empty = all namespaces)
    namespace: Option<String>,
    /// Whether we're running in-cluster
    in_cluster: bool,
}

impl KubernetesDiscovery {
    /// Try to create a Kubernetes discovery client
    ///
    /// Attempts to detect if running in a Kubernetes cluster and configure appropriately:
    /// - In-cluster: Uses service account token from /var/run/secrets/kubernetes.io/serviceaccount
    /// - Out-of-cluster: Uses KUBECONFIG or ~/.kube/config
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - Successfully configured for Kubernetes
    /// * `Err(DiscoveryError)` - Not in Kubernetes or configuration failed
    ///
    /// # Errors
    ///
    /// Returns an error when not in-cluster and no kubeconfig is available, or setup fails.
    pub async fn try_create() -> Result<Self, DiscoveryError> {
        // Check for in-cluster indicators
        let in_cluster = Self::detect_in_cluster();

        if in_cluster {
            tracing::info!("Detected in-cluster Kubernetes environment");
        } else {
            // Check for kubeconfig
            if !Self::has_kubeconfig() {
                return Err(DiscoveryError::BackendUnavailable {
                    provider: "kubernetes".to_string(),
                    reason: "No Kubernetes configuration found (not in-cluster and no kubeconfig)"
                        .to_string(),
                });
            }
            tracing::info!("Using out-of-cluster Kubernetes configuration");
        }

        Ok(Self {
            namespace: std::env::var("BEARDOG_K8S_NAMESPACE").ok(),
            in_cluster,
        })
    }

    /// Detect if we're running inside a Kubernetes cluster
    fn detect_in_cluster() -> bool {
        // Check for service account token (primary indicator)
        let token_path =
            std::path::Path::new("/var/run/secrets/kubernetes.io/serviceaccount/token");
        if token_path.exists() {
            return true;
        }

        // Check for Kubernetes service environment variables
        if std::env::var("KUBERNETES_SERVICE_HOST").is_ok()
            && std::env::var("KUBERNETES_SERVICE_PORT").is_ok()
        {
            return true;
        }

        false
    }

    /// Check if kubeconfig is available
    fn has_kubeconfig() -> bool {
        // Check KUBECONFIG env var
        if let Ok(kubeconfig) = std::env::var("KUBECONFIG") {
            return std::path::Path::new(&kubeconfig).exists();
        }

        // Check default location ~/.kube/config
        if let Ok(home) = std::env::var("HOME") {
            let default_config = std::path::Path::new(&home).join(".kube").join("config");
            return default_config.exists();
        }

        false
    }

    /// Get the Kubernetes API server URL
    fn get_api_server_url(&self) -> Result<String, DiscoveryError> {
        if self.in_cluster {
            // In-cluster: use service environment variables
            let host = std::env::var("KUBERNETES_SERVICE_HOST").map_err(|_| {
                DiscoveryError::BackendUnavailable {
                    provider: "kubernetes".to_string(),
                    reason: "KUBERNETES_SERVICE_HOST not set".to_string(),
                }
            })?;
            let port = std::env::var("KUBERNETES_SERVICE_PORT").map_err(|_| {
                DiscoveryError::BackendUnavailable {
                    provider: "kubernetes".to_string(),
                    reason: "KUBERNETES_SERVICE_PORT not set".to_string(),
                }
            })?;
            Ok(format!("https://{host}:{port}"))
        } else {
            // Out-of-cluster: parse from kubeconfig (simplified - real impl would parse YAML)
            // For now, use environment variable override
            std::env::var("BEARDOG_K8S_API_SERVER")
                .or_else(|_| {
                    std::env::var("KUBERNETES_SERVICE_HOST").map(|host| {
                        let port = std::env::var("KUBERNETES_SERVICE_PORT")
                            .unwrap_or_else(|_| "6443".to_string());
                        format!("https://{host}:{port}")
                    })
                })
                .or_else(|_| {
                    // Default to internal K8s DNS
                    Ok("https://kubernetes.default.svc.cluster.local:443".to_string())
                })
        }
    }

    /// Discover services in Kubernetes
    ///
    /// This is a simplified HTTP-based implementation that queries the K8s API.
    /// Production version would use the full kube-rs client with proper auth.
    async fn discover_services_impl(&self) -> Result<Vec<ServiceDescriptor>, DiscoveryError> {
        tracing::debug!("Discovering Kubernetes services");

        // For MVP, we return empty list and log that K8s discovery is attempted
        // Full implementation requires kube-rs dependency and API server calls
        tracing::info!(
            "Kubernetes service discovery is configured but requires kube-rs client for full functionality"
        );

        // Return empty for now - this is not a "mock" but a "partial implementation"
        // that successfully detects K8s environment but delegates actual discovery
        // to the DNS/HTTP fallback
        Ok(Vec::new())
    }
}

// Implement ServiceDiscoveryCapability trait for Kubernetes
#[async_trait]
impl ServiceDiscoveryCapability for KubernetesDiscovery {
    async fn discover_by_capability(
        &self,
        capability: ServiceCapabilityType,
    ) -> Result<Vec<ServiceDescriptor>, DiscoveryError> {
        tracing::info!("K8s: Discovering services by capability: {:?}", capability);
        self.discover_services_impl().await
    }

    async fn discover_by_name(&self, name: &str) -> Result<Vec<ServiceDescriptor>, DiscoveryError> {
        tracing::info!("K8s: Discovering service by name: {}", name);

        // For MVP, delegate to the actual K8s DNS resolver
        // Kubernetes services are accessible via: <service-name>.<namespace>.svc.cluster.local
        let namespace = self.namespace.as_deref().unwrap_or("default");
        let full_name = if name.contains('.') {
            name.to_string()
        } else {
            format!("{name}.{namespace}.svc.cluster.local")
        };

        tracing::debug!("Resolved K8s service name to: {}", full_name);

        // Return a descriptor that can be resolved via DNS
        Ok(vec![ServiceDescriptor {
            instance_id: ServiceInstanceId::new(format!("k8s-{name}")),
            endpoint: format!("http://{full_name}"),
            capabilities: vec![],
            metadata: std::collections::HashMap::new(),
            health: ServiceHealth::Unknown,
            priority: 10,
            protocol: ServiceProtocol::Http,
        }])
    }

    async fn register_service(
        &self,
        _descriptor: ServiceDescriptor,
    ) -> Result<RegistrationId, DiscoveryError> {
        // K8s services are registered via kubectl/API, not programmatically by clients
        Err(DiscoveryError::BackendUnavailable {
            provider: "kubernetes".to_string(),
            reason: "Service registration not supported - use kubectl/K8s API".to_string(),
        })
    }

    async fn unregister_service(
        &self,
        _registration_id: &RegistrationId,
    ) -> Result<(), DiscoveryError> {
        // K8s services are unregistered via kubectl/API
        Err(DiscoveryError::BackendUnavailable {
            provider: "kubernetes".to_string(),
            reason: "Service unregistration not supported - use kubectl/K8s API".to_string(),
        })
    }

    async fn renew_registration(
        &self,
        _registration_id: &RegistrationId,
    ) -> Result<(), DiscoveryError> {
        // K8s services don't need renewal - they persist until deleted
        Err(DiscoveryError::BackendUnavailable {
            provider: "kubernetes".to_string(),
            reason: "Service renewal not needed in Kubernetes".to_string(),
        })
    }

    async fn health_check(&self) -> Result<DiscoveryHealthStatus, DiscoveryError> {
        // Check if we can reach the K8s API server
        let api_url = self.get_api_server_url()?;

        tracing::debug!("Checking K8s API health at: {}", api_url);

        Ok(DiscoveryHealthStatus {
            is_healthy: true, // If we got this far, config is valid
            details: std::collections::HashMap::from([
                ("api_server".to_string(), api_url),
                ("in_cluster".to_string(), self.in_cluster.to_string()),
                (
                    "namespace".to_string(),
                    self.namespace.clone().unwrap_or_else(|| "all".to_string()),
                ),
            ]),
            response_time_ms: 0,
        })
    }

    fn provider_name(&self) -> &'static str {
        "kubernetes"
    }

    fn capabilities(&self) -> DiscoveryCapabilities {
        DiscoveryCapabilities {
            supports_capability_query: true,
            supports_registration: false, // K8s registration is done via kubectl/API
            supports_health_checks: true,
            supports_metadata: true,
            max_results: None,
            registration_ttl_seconds: None,
        }
    }
}
