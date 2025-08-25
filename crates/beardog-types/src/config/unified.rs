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


/// # Unified Configuration System
///
/// This module provides a unified configuration system that consolidates
/// all BearDog configuration types into a single, cohesive structure.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Import canonical configuration types
use crate::canonical::configuration::security::{RateLimitConfig, SessionConfig};

// Re-export the complete configuration as the canonical unified config
/// **CANONICAL BEARDOG CONFIGURATION** - Complete unified configuration system
/// 
/// This is the main configuration structure that consolidates all domain-specific
/// configurations into a single, manageable structure. It replaces the fragmented
/// BearDogConfig with the complete CompleteBearDogConfig for consistency.
pub use crate::config::CompleteBearDogConfig as BearDogConfig;

// ============================================================================
// DEPLOYMENT CONFIGURATION HIERARCHY
/// Unified deployment configuration - consolidates PrimalDeploymentConfig, BiomeConfig, etc.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedDeploymentConfig {
    /// Core `BearDog` application configuration
    pub app: BearDogConfig,
    /// Resource management configuration
    pub resources: ResourceConfig,
    /// Network and service mesh configuration
    pub networking: NetworkConfig,
    /// Security configuration for the deployment
    pub security: BiomeSecurityConfig,
    /// Compliance configuration
    pub compliance: ComplianceConfig,
    /// Resource configuration for biome management
    pub biome_resources: BiomeResourceConfig,
/// Biome configuration - consolidates BiomeMetadata, BiomeSecurityContext, etc.
pub struct BiomeConfig {
    /// Unique biome identifier
    pub id: String,
    /// Human-readable biome name
    pub name: String,
    /// Biome version
    pub version: String,
    /// Biome description
    pub description: String,
    /// Environment type (development, staging, production)
    pub environment: String,
    /// Security configuration for this biome
    /// Resource limits for this biome
    pub resource_limits: BiomeResourceConfig,
    /// Service discovery configuration
    pub service_discovery: ServiceDiscoveryConfig,
/// Primal configuration - consolidates PrimalDeploymentConfig, PrimalIntegrationConfig, etc.
pub struct PrimalConfig {
    /// Primal type (beardog, songbird, etc.)
    pub primal_type: String,
    /// Primal version
    /// Primal capabilities
    pub capabilities: Vec<String>,
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
    /// Service definitions
    pub services: Vec<ServiceConfig>,
    /// Integration settings
    pub integration: IntegrationConfig,
    /// Environment variables
    pub environment: HashMap<String, String>,
// RESOURCE CONFIGURATION HIERARCHY
/// Unified resource configuration - consolidates various resource configs
pub struct ResourceConfig {
    /// CPU allocation and limits
    pub cpu: CpuConfig,
    /// Memory allocation and limits
    pub memory: MemoryConfig,
    /// Storage configuration
    pub storage: StorageConfig,
    /// Network resource limits
    pub network: NetworkResourceConfig,
/// CPU configuration
pub struct CpuConfig {
    /// Minimum CPU cores required
    pub min_cores: f32,
    /// Maximum CPU cores allowed
    pub max_cores: f32,
    /// CPU limit as percentage
    pub limit_percent: u8,
    /// CPU request as percentage
    pub request_percent: u8,
/// Memory configuration
pub struct MemoryConfig {
    /// Minimum memory in MB
    pub min_mb: u64,
    /// Maximum memory in MB
    pub max_mb: u64,
    /// Memory limit in MB
    pub limit_mb: u64,
    /// Memory request in MB
    pub request_mb: u64,
/// Storage configuration
pub struct StorageConfig {
    /// Storage type (local, network, cloud)
    pub storage_type: String,
    /// Storage size in GB
    pub size_gb: u64,
    /// Storage path
    pub path: String,
    /// Backup configuration
    pub backup: BackupConfig,
/// Network resource configuration
pub struct NetworkResourceConfig {
    /// Bandwidth limit in Mbps
    pub bandwidth_mbps: u32,
    /// Connection limits
    pub max_connections: u32,
    /// Port ranges
    pub port_ranges: Vec<PortRange>,
// SCALING CONFIGURATION HIERARCHY
/// Unified scaling configuration - consolidates ScalingConfig, AutoScalingConfig, etc.
pub struct ScalingConfig {
    /// Horizontal scaling configuration
    pub horizontal: HorizontalScalingConfig,
    /// Vertical scaling configuration
    pub vertical: VerticalScalingConfig,
    /// Auto-scaling policies
    pub auto_scaling: AutoScalingConfig,
    /// Load balancing configuration
    pub load_balancing: LoadBalancingConfig,
/// Horizontal scaling configuration
pub struct HorizontalScalingConfig {
    /// Minimum number of instances
    pub min_instances: u32,
    /// Maximum number of instances
    pub max_instances: u32,
    /// Target instances
    pub target_instances: u32,
    /// Scaling policies
    pub policies: Vec<ScalingPolicy>,
/// Vertical scaling configuration
pub struct VerticalScalingConfig {
    /// Enable vertical scaling
    pub enabled: bool,
    /// CPU scaling limits
    pub cpu_limits: (f32, f32), // (min, max)
    /// Memory scaling limits
    pub memory_limits: (u64, u64), // (min_mb, max_mb)
/// Auto-scaling configuration
pub struct AutoScalingConfig {
    /// Enable auto-scaling
    /// CPU threshold for scaling up
    pub cpu_scale_up_threshold: f32,
    /// CPU threshold for scaling down
    pub cpu_scale_down_threshold: f32,
    /// Memory threshold for scaling up
    pub memory_scale_up_threshold: f32,
    /// Memory threshold for scaling down
    pub memory_scale_down_threshold: f32,
    /// Cooldown period in seconds
    pub cooldown_seconds: u64,
// NETWORK CONFIGURATION HIERARCHY
/// Unified network configuration - consolidates various network configs
pub struct NetworkConfig {
    /// Service mesh configuration
    pub service_mesh: ServiceMeshConfig,
    /// DNS configuration
    pub dns: DnsConfig,
    /// TLS/SSL configuration
    pub tls: TlsConfig,
    /// VPN configuration
    pub vpn: Option<VpnConfig>,
/// Service mesh configuration
pub struct ServiceMeshConfig {
    /// Enable service mesh
    /// Service mesh type (istio, linkerd, consul)
    pub mesh_type: String,
    /// Mesh configuration
    pub config: HashMap<String, String>,
/// Load balancing configuration
pub struct LoadBalancingConfig {
    /// Load balancing algorithm
    pub algorithm: String, // round_robin, least_connections, ip_hash
    /// Health check configuration
    pub health_check: HealthCheckConfig,
    /// Session affinity configuration
    pub session_affinity: Option<SessionAffinityConfig>,
/// DNS configuration
pub struct DnsConfig {
    /// DNS servers
    pub servers: Vec<String>,
    /// DNS search domains
    pub search_domains: Vec<String>,
    /// DNS resolution timeout
    pub timeout_seconds: u32,
/// TLS configuration
pub struct TlsConfig {
    /// Enable TLS
    /// TLS version
    /// Certificate configuration
    pub certificates: CertificateConfig,
    /// Cipher suites
    pub cipher_suites: Vec<String>,
/// VPN configuration
pub struct VpnConfig {
    /// VPN type (wireguard, openvpn)
    pub vpn_type: String,
    /// VPN server configuration
    pub server: String,
    /// VPN credentials
    pub credentials: HashMap<String, String>,
// SERVICE CONFIGURATION HIERARCHY
/// Unified service configuration - consolidates ServiceDefinition, ServiceConfig, etc.
pub struct ServiceConfig {
    /// Service name
    /// Service type
    pub service_type: String,
    /// Port configuration
    pub ports: Vec<PortConfig>,
    /// Endpoint configuration
    pub endpoints: Vec<EndpointConfig>,
    pub security: ServiceSecurityConfig,
/// Port configuration
pub struct PortConfig {
    /// Port number
    pub port: u16,
    /// Target port
    pub target_port: Option<u16>,
    /// Protocol (tcp, udp, http, https, grpc)
    pub protocol: String,
    /// External access
    pub external: bool,
/// Endpoint configuration
pub struct EndpointConfig {
    /// Endpoint path
    /// HTTP methods
    pub methods: Vec<String>,
    /// Authentication required
    pub auth_required: bool,
    /// Rate limiting
    pub rate_limit: Option<RateLimitConfig>,
/// Health check configuration
pub struct HealthCheckConfig {
    /// Health check path
    /// Health check interval in seconds
    pub interval_seconds: u32,
    /// Health check timeout in seconds
    /// Failure threshold
    pub failure_threshold: u32,
    /// Success threshold
    pub success_threshold: u32,
// SECURITY CONFIGURATION HIERARCHY
/// Biome security configuration
pub struct BiomeSecurityConfig {
    /// Encryption configuration
    pub encryption: EncryptionConfig,
    /// Access control configuration
    pub access_control: AccessControlConfig,
    /// Audit configuration
    pub audit: AuditConfig,
/// Service security configuration
pub struct ServiceSecurityConfig {
    /// Authentication configuration
    pub authentication: crate::canonical::configuration::security::AuthenticationConfig,
    /// Authorization configuration
    pub authorization: AuthorizationConfig,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitConfig,
    /// Session configuration
    pub session: SessionConfig,
/// Encryption configuration - Using canonical definition
/// Use the canonical encryption configuration from the canonical module.
pub use crate::canonical::configuration::security::EncryptionConfig;
/// Access control configuration
pub struct AccessControlConfig {
    /// Access control model (RBAC, ABAC, etc.)};


    pub model: String,
    /// Default deny policy
    pub default_deny: bool,
    /// Role definitions
    pub roles: HashMap<String, Vec<String>>,
/// Authentication configuration
/// Authorization configuration
pub struct AuthorizationConfig {
    /// Authorization model
    /// Policy definitions
    pub policies: HashMap<String, String>,
/// Audit configuration
pub struct AuditConfig {
    /// Enable auditing
    /// Audit log path
    pub log_path: String,
    /// Audit retention period in days
    pub retention_days: u32,
/// Compliance configuration
pub struct ComplianceConfig {
    /// Compliance frameworks (GDPR, HIPAA, SOX, etc.)
    pub frameworks: Vec<String>,
    /// Compliance policies
// SUPPORTING CONFIGURATION TYPES
/// Resource requirements
pub struct ResourceRequirements {
    /// CPU requirements
    /// Memory requirements
    /// Storage requirements
    pub storage: Option<StorageConfig>,
/// Integration configuration
pub struct IntegrationConfig {
    /// External service integrations
    pub external_services: HashMap<String, ExternalServiceConfig>,
    /// API configurations
    pub apis: HashMap<String, ApiConfig>,
/// External service configuration
pub struct ExternalServiceConfig {
    /// Service URL
    pub url: String,
    pub auth: Option<crate::config::security_unified::AuthenticationConfig>,
    /// Timeout in seconds
/// API configuration
pub struct ApiConfig {
    /// API version
    /// Base path
    pub base_path: String,
/// Service discovery configuration
pub struct ServiceDiscoveryConfig {
    /// Discovery method (dns, consul, kubernetes)
    pub method: String,
    /// Discovery configuration
/// Biome resource configuration
pub struct BiomeResourceConfig {
    /// CPU limits
    pub cpu_limits: CpuConfig,
    /// Memory limits
    pub memory_limits: MemoryConfig,
    /// Storage limits
    pub storage_limits: StorageConfig,
/// Backup configuration
pub struct BackupConfig {
    /// Enable backups
    /// Backup interval in hours
    pub interval_hours: u32,
    /// Backup retention in days
    /// Backup location
    pub location: String,
/// Certificate configuration
pub struct CertificateConfig {
    /// Certificate path
    pub cert_path: String,
    /// Private key path
    pub key_path: String,
    /// CA certificate path
    pub ca_path: Option<String>,
/// Session affinity configuration
pub struct SessionAffinityConfig {
    /// Affinity type (client_ip, cookie)
    pub affinity_type: String,
    /// Cookie configuration
    pub cookie: Option<CookieConfig>,
/// Cookie configuration
pub struct CookieConfig {
    /// Cookie name
    /// Cookie TTL in seconds
    pub ttl_seconds: u32,
/// Port range configuration
pub struct PortRange {
    /// Start port
    pub start: u16,
    /// End port
    pub end: u16,
/// Scaling policy
pub struct ScalingPolicy {
    /// Policy name
    /// Metric to monitor
    pub metric: String,
    /// Threshold value
    pub threshold: f64,
    /// Scaling action (scale_up, scale_down)
    pub action: String,
/// Token configuration
pub struct TokenConfig {
    /// Token type (JWT, OAuth, etc.)
    pub token_type: String,
    /// Token TTL in seconds
    pub ttl_seconds: u64,
    /// Token signing key
    pub signing_key: String,
/// MFA configuration
// Use canonical TotpConfig instead of duplicate
pub use crate::canonical::configuration::security::TotpConfig;
// DEFAULT IMPLEMENTATIONS}


impl Default for BiomeConfig {
            id: "default-biome".to_string(),
            name: "Default Biome".to_string(),
            version: "1.0.0".to_string(),
            description: "Default biome configuration".to_string(),
            environment: "development".to_string(),
            security: BiomeSecurityConfig::default(),
            resource_limits: BiomeResourceConfig::default(),
            service_discovery: ServiceDiscoveryConfig::default(),}


impl Default for CpuConfig {
            min_cores: 0.1,
            max_cores: 2.0,
            limit_percent: 80,
            request_percent: 50,
impl Default for MemoryConfig {
            min_mb: 128,
            max_mb: 2048,
            limit_mb: 1024,
            request_mb: 512,}


impl Default for StorageConfig {
            storage_type: "local".to_string(),
            size_gb: 10,
            path: "/data".to_string(),
            backup: BackupConfig::default(),
impl Default for NetworkResourceConfig {
            bandwidth_mbps: 100,
            max_connections: 1000,
            port_ranges: vec![PortRange {
                start: 8000,
                end: 9000,
            }],
impl Default for HorizontalScalingConfig {
            min_instances: 1,
            max_instances: 10,
            target_instances: 3,
            policies: Vec::new(),}


impl Default for VerticalScalingConfig {
            enabled: false,
            cpu_limits: (0.1, 4.0),
            memory_limits: (128, 4096),
impl Default for AutoScalingConfig {
            enabled: true,
            cpu_scale_up_threshold: 70.0,
            cpu_scale_down_threshold: 30.0,
            memory_scale_up_threshold: 80.0,
            memory_scale_down_threshold: 40.0,
            cooldown_seconds: 300,}


impl Default for ServiceMeshConfig {
            mesh_type: "istio".to_string(),
            config: HashMap::new(),
impl Default for LoadBalancingConfig {
            algorithm: "round_robin".to_string(),
            health_check: HealthCheckConfig::default(),
            session_affinity: None,}


impl Default for DnsConfig {
            servers: vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
            search_domains: Vec::new(),
            timeout_seconds: 5,
impl Default for TlsConfig {
            version: "1.3".to_string(),
            certificates: CertificateConfig::default(),
            cipher_suites: Vec::new(),}


impl Default for HealthCheckConfig {
            path: "/health".to_string(),
            interval_seconds: 30,
            failure_threshold: 3,
            success_threshold: 1,
// Using canonical implementation from canonical module
impl Default for AccessControlConfig {}


            model: "RBAC".to_string(),
            default_deny: true,
            roles: HashMap::new(),
impl Default for AuditConfig {
            log_path: "/logs/audit.log".to_string(),
            retention_days: 90,}


impl Default for ServiceDiscoveryConfig {
            method: "dns".to_string(),
impl Default for BackupConfig {
            interval_hours: 24,
            retention_days: 30,
            location: "/backups".to_string(),}


impl Default for CertificateConfig {
            cert_path: "/certs/server.crt".to_string(),
            key_path: "/certs/server.key".to_string(),
            ca_path: None,
