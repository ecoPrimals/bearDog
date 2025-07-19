//! biome.yaml Manifest Parser for BearDog
//!
//! Implements parsing and processing of biome.yaml manifests to enable
//! BearDog integration with biomeOS orchestration. Supports manifest-driven
//! deployment, configuration, and security context management.

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;
use tracing::{debug, info, warn};

use crate::universal_primal_provider::{
    PrimalCapability, PrimalService, ServiceEndpoint, ServiceHealth,
};

/// Complete biome.yaml manifest structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeManifest {
    /// Biome metadata and identification
    pub biome: BiomeMetadata,
    /// Primal configurations within this biome
    pub primals: HashMap<String, PrimalConfig>,
    /// Biome-wide security context
    pub security: BiomeSecurityContext,
    /// Resource allocation and limits
    pub resources: BiomeResourceConfig,
    /// Networking configuration
    pub networking: BiomeNetworkConfig,
    /// Environment variables and configuration
    pub environment: HashMap<String, String>,
    /// Deployment strategy
    pub deployment: BiomeDeploymentConfig,
}

/// Biome identification and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeMetadata {
    /// Unique biome identifier
    pub id: String,
    /// Human-readable biome name
    pub name: String,
    /// Biome version (semantic versioning)
    pub version: String,
    /// Biome description
    pub description: Option<String>,
    /// Biome maintainer information
    pub maintainer: Option<String>,
    /// Labels for biome classification
    pub labels: HashMap<String, String>,
    /// Creation timestamp
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// Environment type (development, staging, production)
    pub environment: BiomeEnvironment,
}

/// Environment types for biome deployment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BiomeEnvironment {
    /// Development environment
    #[serde(rename = "development")]
    Development,
    /// Testing/staging environment
    #[serde(rename = "staging")]
    Staging,
    /// Production environment
    #[serde(rename = "production")]
    Production,
    /// Custom environment
    #[serde(rename = "custom")]
    Custom(String),
}

/// Configuration for a specific primal within the biome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalConfig {
    /// Primal type (beardog, songbird, nestgate, etc.)
    pub primal_type: String,
    /// Primal version to deploy
    pub version: String,
    /// Primal-specific configuration
    pub config: serde_json::Value,
    /// Service definitions for this primal
    pub services: Vec<ServiceDefinition>,
    /// Resource requirements
    pub resources: PrimalResourceRequirements,
    /// Security context for this primal
    pub security: PrimalSecurityContext,
    /// Health check configuration
    pub health_check: HealthCheckConfig,
    /// Dependencies on other primals
    pub depends_on: Vec<String>,
    /// Scaling configuration
    pub scaling: ScalingConfig,
}

/// Service definition within a primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDefinition {
    /// Service name
    pub name: String,
    /// Service type/category
    pub service_type: String,
    /// Port configuration
    pub ports: Vec<PortConfig>,
    /// Service endpoints
    pub endpoints: Vec<EndpointConfig>,
    /// Service-specific configuration
    pub config: serde_json::Value,
    /// Required capabilities
    pub capabilities: Vec<String>,
}

/// Port configuration for services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortConfig {
    /// Port number
    pub port: u16,
    /// Target port (if different from port)
    pub target_port: Option<u16>,
    /// Protocol (tcp, udp, http, https, grpc)
    pub protocol: String,
    /// External access configuration
    pub external: bool,
}

/// Endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointConfig {
    /// Endpoint path
    pub path: String,
    /// HTTP methods supported
    pub methods: Vec<String>,
    /// Authentication required
    pub auth_required: bool,
    /// Rate limiting configuration
    pub rate_limit: Option<RateLimitConfig>,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per minute
    pub requests_per_minute: u32,
    /// Burst allowance
    pub burst: Option<u32>,
}

/// Resource requirements for a primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalResourceRequirements {
    /// CPU requirements (cores)
    pub cpu: ResourceSpec,
    /// Memory requirements (MB)
    pub memory: ResourceSpec,
    /// Storage requirements (GB)
    pub storage: Option<ResourceSpec>,
    /// Network bandwidth requirements (Mbps)
    pub network: Option<ResourceSpec>,
}

/// Resource specification with requests and limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSpec {
    /// Minimum required resources
    pub requests: f64,
    /// Maximum allowed resources
    pub limits: Option<f64>,
}

/// Security context for individual primals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalSecurityContext {
    /// Security clearance level (1-10)
    pub clearance_level: u8,
    /// Required security capabilities
    pub capabilities: Vec<String>,
    /// Allowed network policies
    pub network_policies: Vec<NetworkPolicy>,
    /// Encryption requirements
    pub encryption: EncryptionRequirements,
    /// Authentication configuration
    pub authentication: AuthenticationConfig,
}

/// Network policy definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicy {
    /// Policy name
    pub name: String,
    /// Allowed sources
    pub from: Vec<NetworkPolicyRule>,
    /// Allowed destinations  
    pub to: Vec<NetworkPolicyRule>,
    /// Allowed ports
    pub ports: Vec<u16>,
}

/// Network policy rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicyRule {
    /// Rule type (primal, cidr, label)
    pub rule_type: String,
    /// Rule value
    pub value: String,
}

/// Encryption requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionRequirements {
    /// Require TLS for all communications
    pub require_tls: bool,
    /// Minimum TLS version
    pub min_tls_version: Option<String>,
    /// Required cipher suites
    pub cipher_suites: Vec<String>,
    /// Certificate management
    pub certificates: CertificateConfig,
}

/// Certificate configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateConfig {
    /// Certificate authority to use
    pub ca: Option<String>,
    /// Certificate validity period (days)
    pub validity_days: u32,
    /// Auto-renewal enabled
    pub auto_renew: bool,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    /// Authentication method (jwt, oauth, mtls, custom)
    pub method: String,
    /// Authentication provider
    pub provider: Option<String>,
    /// Additional authentication options
    pub options: HashMap<String, String>,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Health check endpoint
    pub endpoint: String,
    /// Check interval in seconds
    pub interval: u32,
    /// Timeout for health checks
    pub timeout: u32,
    /// Number of retries before marking unhealthy
    pub retries: u32,
    /// Initial delay before starting checks
    pub initial_delay: Option<u32>,
}

/// Scaling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingConfig {
    /// Minimum number of instances
    pub min_replicas: u32,
    /// Maximum number of instances
    pub max_replicas: u32,
    /// Target CPU utilization for auto-scaling
    pub target_cpu: Option<f64>,
    /// Target memory utilization for auto-scaling
    pub target_memory: Option<f64>,
    /// Custom scaling metrics
    pub custom_metrics: Vec<CustomMetric>,
}

/// Custom scaling metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMetric {
    /// Metric name
    pub name: String,
    /// Target value
    pub target: f64,
    /// Metric type (gauge, counter, histogram)
    pub metric_type: String,
}

/// Biome-wide security context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeSecurityContext {
    /// Global security policies
    pub policies: Vec<SecurityPolicy>,
    /// Default encryption settings
    pub encryption: GlobalEncryptionConfig,
    /// Network security configuration
    pub network_security: NetworkSecurityConfig,
    /// Audit and compliance settings
    pub audit: AuditConfig,
}

/// Security policy definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    /// Policy name
    pub name: String,
    /// Policy type (network, rbac, security_context)
    pub policy_type: String,
    /// Policy rules
    pub rules: Vec<PolicyRule>,
    /// Policy enforcement mode (enforce, warn, disabled)
    pub enforcement: String,
}

/// Security policy rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    /// Rule name
    pub name: String,
    /// Rule condition
    pub condition: String,
    /// Action to take (allow, deny, log)
    pub action: String,
}

/// Global encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalEncryptionConfig {
    /// Default encryption algorithm
    pub default_algorithm: String,
    /// Key rotation period (days)
    pub key_rotation_days: u32,
    /// Encryption at rest enabled
    pub encrypt_at_rest: bool,
    /// Encryption in transit enabled
    pub encrypt_in_transit: bool,
}

/// Network security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurityConfig {
    /// Default network policies
    pub default_policies: Vec<String>,
    /// Firewall rules
    pub firewall_rules: Vec<FirewallRule>,
    /// VPN configuration
    pub vpn: Option<VpnConfig>,
}

/// Firewall rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRule {
    /// Rule name
    pub name: String,
    /// Source specification
    pub source: String,
    /// Destination specification
    pub destination: String,
    /// Port specification
    pub port: String,
    /// Protocol (tcp, udp, icmp)
    pub protocol: String,
    /// Action (allow, deny)
    pub action: String,
}

/// VPN configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnConfig {
    /// VPN provider
    pub provider: String,
    /// VPN endpoint
    pub endpoint: String,
    /// Authentication configuration
    pub auth: HashMap<String, String>,
}

/// Audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Enable audit logging
    pub enabled: bool,
    /// Audit log level
    pub log_level: String,
    /// Audit destinations
    pub destinations: Vec<AuditDestination>,
    /// Retention policy
    pub retention_days: u32,
}

/// Audit destination configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditDestination {
    /// Destination type (file, syslog, elasticsearch)
    pub destination_type: String,
    /// Destination endpoint
    pub endpoint: String,
    /// Authentication if required
    pub auth: Option<HashMap<String, String>>,
}

/// Resource configuration for the entire biome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeResourceConfig {
    /// Total CPU allocation (cores)
    pub total_cpu: f64,
    /// Total memory allocation (GB)
    pub total_memory: f64,
    /// Total storage allocation (GB)
    pub total_storage: f64,
    /// Resource quotas by primal
    pub quotas: HashMap<String, ResourceQuota>,
}

/// Resource quota for a specific primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceQuota {
    /// CPU quota (cores)
    pub cpu: f64,
    /// Memory quota (GB)
    pub memory: f64,
    /// Storage quota (GB)
    pub storage: f64,
}

/// Network configuration for the biome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeNetworkConfig {
    /// Network mode (bridge, host, overlay)
    pub mode: String,
    /// DNS configuration
    pub dns: DnsConfig,
    /// Load balancer configuration
    pub load_balancer: LoadBalancerConfig,
    /// Service mesh configuration
    pub service_mesh: ServiceMeshConfig,
}

/// DNS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsConfig {
    /// DNS servers
    pub servers: Vec<String>,
    /// DNS search domains
    pub search_domains: Vec<String>,
    /// DNS options
    pub options: HashMap<String, String>,
}

/// Load balancer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    /// Load balancer type (nginx, haproxy, envoy)
    pub lb_type: String,
    /// Load balancing algorithm
    pub algorithm: String,
    /// Health check configuration
    pub health_check: HealthCheckConfig,
}

/// Service mesh configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMeshConfig {
    /// Enable service mesh
    pub enabled: bool,
    /// Service mesh provider (songbird, istio, linkerd)
    pub provider: String,
    /// Service mesh configuration
    pub config: HashMap<String, serde_json::Value>,
}

/// Deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeDeploymentConfig {
    /// Deployment strategy (rolling, blue_green, canary)
    pub strategy: String,
    /// Rolling update configuration
    pub rolling_update: Option<RollingUpdateConfig>,
    /// Canary deployment configuration
    pub canary: Option<CanaryConfig>,
    /// Deployment hooks
    pub hooks: Vec<DeploymentHook>,
}

/// Rolling update configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollingUpdateConfig {
    /// Maximum unavailable instances
    pub max_unavailable: String,
    /// Maximum surge instances
    pub max_surge: String,
}

/// Canary deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanaryConfig {
    /// Canary traffic percentage
    pub traffic_percent: u32,
    /// Canary duration
    pub duration: u32,
    /// Success criteria
    pub success_criteria: Vec<SuccessCriterion>,
}

/// Success criterion for canary deployments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessCriterion {
    /// Metric name
    pub metric: String,
    /// Threshold value
    pub threshold: f64,
    /// Comparison operator (gt, lt, eq)
    pub operator: String,
}

/// Deployment hook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentHook {
    /// Hook name
    pub name: String,
    /// Hook type (pre_deploy, post_deploy, pre_rollback, post_rollback)
    pub hook_type: String,
    /// Command to execute
    pub command: Vec<String>,
    /// Timeout for hook execution
    pub timeout: u32,
}

/// biome.yaml parser implementation
pub struct BiomeYamlParser;

impl BiomeYamlParser {
    /// Parse biome.yaml from file path
    pub async fn parse_file<P: AsRef<Path>>(path: P) -> BearDogResult<BiomeManifest> {
        let content = fs::read_to_string(path.as_ref()).await.map_err(|e| {
            BearDogError::config(&format!(
                "Failed to read biome.yaml at {}: {}",
                path.as_ref().display(),
                e
            ))
        })?;

        Self::parse_string(&content).await
    }

    /// Parse biome.yaml from string content
    pub async fn parse_string(content: &str) -> BearDogResult<BiomeManifest> {
        let manifest: BiomeManifest = serde_yaml::from_str(content)
            .map_err(|e| BearDogError::config(&format!("Failed to parse biome.yaml: {}", e)))?;

        // Validate the manifest
        Self::validate_manifest(&manifest).await?;

        info!(
            "Successfully parsed biome.yaml manifest: {}",
            manifest.biome.id
        );
        Ok(manifest)
    }

    /// Validate biome manifest for correctness
    async fn validate_manifest(manifest: &BiomeManifest) -> BearDogResult<()> {
        // Validate biome metadata
        if manifest.biome.id.is_empty() {
            return Err(BearDogError::validation("Biome ID cannot be empty"));
        }

        if manifest.biome.name.is_empty() {
            return Err(BearDogError::validation("Biome name cannot be empty"));
        }

        // Validate semantic versioning
        if !Self::is_valid_semver(&manifest.biome.version) {
            return Err(BearDogError::validation(&format!(
                "Invalid version format: {}. Must follow semantic versioning.",
                manifest.biome.version
            )));
        }

        // Validate primal configurations
        for (primal_name, config) in &manifest.primals {
            Self::validate_primal_config(primal_name, config).await?;
        }

        // Validate resource allocation
        Self::validate_resource_allocation(&manifest.resources, &manifest.primals).await?;

        // Validate security configuration
        Self::validate_security_config(&manifest.security).await?;

        debug!("Biome manifest validation passed");
        Ok(())
    }

    /// Validate individual primal configuration
    async fn validate_primal_config(name: &str, config: &PrimalConfig) -> BearDogResult<()> {
        // Validate primal type
        let valid_types = vec![
            "beardog",
            "songbird",
            "nestgate",
            "toadstool",
            "universal_compute",
        ];
        if !valid_types.contains(&config.primal_type.as_str()) {
            warn!(
                "Unknown primal type: {} for primal: {}",
                config.primal_type, name
            );
        }

        // Validate version
        if !Self::is_valid_semver(&config.version) {
            return Err(BearDogError::validation(&format!(
                "Invalid version for primal {}: {}",
                name, config.version
            )));
        }

        // Validate resource requirements
        if config.resources.cpu.requests <= 0.0 {
            return Err(BearDogError::validation(&format!(
                "CPU requests must be positive for primal: {}",
                name
            )));
        }

        if config.resources.memory.requests <= 0.0 {
            return Err(BearDogError::validation(&format!(
                "Memory requests must be positive for primal: {}",
                name
            )));
        }

        // Validate scaling configuration
        if config.scaling.min_replicas == 0 {
            return Err(BearDogError::validation(&format!(
                "Minimum replicas must be at least 1 for primal: {}",
                name
            )));
        }

        if config.scaling.max_replicas < config.scaling.min_replicas {
            return Err(BearDogError::validation(&format!(
                "Max replicas must be >= min replicas for primal: {}",
                name
            )));
        }

        debug!("Primal configuration validation passed for: {}", name);
        Ok(())
    }

    /// Validate resource allocation across all primals
    async fn validate_resource_allocation(
        resources: &BiomeResourceConfig,
        primals: &HashMap<String, PrimalConfig>,
    ) -> BearDogResult<()> {
        let mut total_cpu_requests = 0.0;
        let mut total_memory_requests = 0.0;

        for (name, config) in primals {
            total_cpu_requests +=
                config.resources.cpu.requests * config.scaling.max_replicas as f64;
            total_memory_requests +=
                config.resources.memory.requests * config.scaling.max_replicas as f64;

            // Check quotas if specified
            if let Some(quota) = resources.quotas.get(name) {
                if config.resources.cpu.requests > quota.cpu {
                    return Err(BearDogError::validation(&format!(
                        "CPU requests exceed quota for primal: {}",
                        name
                    )));
                }
                if config.resources.memory.requests > quota.memory {
                    return Err(BearDogError::validation(&format!(
                        "Memory requests exceed quota for primal: {}",
                        name
                    )));
                }
            }
        }

        // Check total resource allocation
        if total_cpu_requests > resources.total_cpu {
            return Err(BearDogError::validation(&format!(
                "Total CPU requests ({:.2}) exceed biome allocation ({:.2})",
                total_cpu_requests, resources.total_cpu
            )));
        }

        if total_memory_requests > resources.total_memory {
            return Err(BearDogError::validation(&format!(
                "Total memory requests ({:.2}GB) exceed biome allocation ({:.2}GB)",
                total_memory_requests, resources.total_memory
            )));
        }

        debug!("Resource allocation validation passed");
        Ok(())
    }

    /// Validate security configuration
    async fn validate_security_config(security: &BiomeSecurityContext) -> BearDogResult<()> {
        // Validate encryption settings
        if security.encryption.key_rotation_days == 0 {
            return Err(BearDogError::validation(
                "Key rotation period must be positive",
            ));
        }

        // Validate audit configuration
        if security.audit.enabled && security.audit.destinations.is_empty() {
            return Err(BearDogError::validation(
                "Audit destinations required when audit is enabled",
            ));
        }

        debug!("Security configuration validation passed");
        Ok(())
    }

    /// Check if version string follows semantic versioning
    fn is_valid_semver(version: &str) -> bool {
        // Basic semver validation (major.minor.patch)
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return false;
        }

        parts.iter().all(|part| part.parse::<u32>().is_ok())
    }

    /// Extract BearDog specific configuration from manifest
    pub async fn extract_beardog_config(
        manifest: &BiomeManifest,
    ) -> BearDogResult<Option<PrimalConfig>> {
        for (name, config) in &manifest.primals {
            if config.primal_type == "beardog" {
                debug!("Found BearDog configuration in manifest: {}", name);
                return Ok(Some(config.clone()));
            }
        }

        debug!("No BearDog configuration found in manifest");
        Ok(None)
    }

    /// Convert manifest service definitions to PrimalService format
    pub async fn convert_to_primal_services(
        services: &[ServiceDefinition],
    ) -> BearDogResult<Vec<PrimalService>> {
        let mut primal_services = Vec::new();

        for service_def in services {
            let service = PrimalService {
                id: service_def.name.clone(),
                name: service_def.name.clone(),
                description: format!("{} service", service_def.service_type),
                endpoint: Self::convert_to_service_endpoint(service_def).await?,
                capabilities: service_def
                    .capabilities
                    .iter()
                    .filter_map(|cap| Self::convert_capability_string(cap))
                    .collect(),
                health: ServiceHealth::Healthy, // Default to healthy
            };

            primal_services.push(service);
        }

        Ok(primal_services)
    }

    /// Convert service definition to service endpoint
    async fn convert_to_service_endpoint(
        service_def: &ServiceDefinition,
    ) -> BearDogResult<ServiceEndpoint> {
        let primary_port = service_def
            .ports
            .first()
            .ok_or_else(|| BearDogError::validation("Service must have at least one port"))?;

        let protocol = if primary_port.protocol.contains("https") {
            "https"
        } else if primary_port.protocol.contains("http") {
            "http"
        } else {
            &primary_port.protocol
        };

        let path = service_def
            .endpoints
            .first()
            .map(|ep| ep.path.clone())
            .unwrap_or_else(|| "/".to_string());

        Ok(ServiceEndpoint {
            protocol: protocol.to_string(),
            host: "localhost".to_string(), // Will be updated during deployment
            port: primary_port.port,
            path,
            security: crate::universal_primal_provider::EndpointSecurity {
                require_tls: protocol == "https",
                require_client_cert: false,
                require_api_key: true,
                custom_auth: vec!["beardog-auth".to_string()],
            },
        })
    }

    /// Convert capability string to PrimalCapability enum
    fn convert_capability_string(capability: &str) -> Option<PrimalCapability> {
        match capability {
            "security" | "security.encryption" | "security.authentication" => {
                Some(PrimalCapability::Security)
            }
            "ai" | "ai.inference" | "ai.ml" => Some(PrimalCapability::AI),
            "monitoring" | "metrics" | "observability" => Some(PrimalCapability::Monitoring),
            "storage" | "storage.distributed" => Some(PrimalCapability::Storage),
            "networking" | "network.routing" => Some(PrimalCapability::Networking),
            "compliance" | "audit" => Some(PrimalCapability::Compliance),
            "workflow" | "orchestration" => Some(PrimalCapability::Workflow),
            "threat_detection" | "security.threat" => Some(PrimalCapability::ThreatDetection),
            "key_management" | "security.keys" => Some(PrimalCapability::KeyManagement),
            _ => Some(PrimalCapability::Custom(capability.to_string())),
        }
    }
}

impl Default for BiomeEnvironment {
    fn default() -> Self {
        Self::Development
    }
}

impl Default for ResourceSpec {
    fn default() -> Self {
        Self {
            requests: 1.0,
            limits: Some(2.0),
        }
    }
}

impl Default for PrimalResourceRequirements {
    fn default() -> Self {
        Self {
            cpu: ResourceSpec::default(),
            memory: ResourceSpec {
                requests: 512.0,
                limits: Some(1024.0),
            },
            storage: Some(ResourceSpec {
                requests: 10.0,
                limits: Some(100.0),
            }),
            network: Some(ResourceSpec {
                requests: 100.0,
                limits: Some(1000.0),
            }),
        }
    }
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            endpoint: "/health".to_string(),
            interval: 30,
            timeout: 10,
            retries: 3,
            initial_delay: Some(30),
        }
    }
}

impl Default for ScalingConfig {
    fn default() -> Self {
        Self {
            min_replicas: 1,
            max_replicas: 3,
            target_cpu: Some(70.0),
            target_memory: Some(80.0),
            custom_metrics: Vec::new(),
        }
    }
}
