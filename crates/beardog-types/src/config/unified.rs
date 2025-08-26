

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::canonical::configuration::security::{RateLimitConfig, SessionConfig};

pub use crate::config::CompleteBearDogConfig as BearDogConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedDeploymentConfig {

    pub app: BearDogConfig,

    pub resources: ResourceConfig,

    pub networking: NetworkConfig,

    pub security: BiomeSecurityConfig,

    pub compliance: ComplianceConfig,

    pub biome_resources: BiomeResourceConfig,

pub struct BiomeConfig {

    pub id: String,

    pub name: String,

    pub version: String,

    pub description: String,

    pub environment: String,

    pub resource_limits: BiomeResourceConfig,

    pub service_discovery: ServiceDiscoveryConfig,

pub struct PrimalConfig {

    pub primal_type: String,

    pub capabilities: Vec<String>,

    pub resource_requirements: ResourceRequirements,

    pub services: Vec<ServiceConfig>,

    pub integration: IntegrationConfig,

    pub environment: HashMap<String, String>,

pub struct ResourceConfig {

    pub cpu: CpuConfig,

    pub memory: MemoryConfig,

    pub storage: StorageConfig,

    pub network: NetworkResourceConfig,

pub struct CpuConfig {

    pub min_cores: f32,

    pub max_cores: f32,

    pub limit_percent: u8,

    pub request_percent: u8,

pub struct MemoryConfig {

    pub min_mb: u64,

    pub max_mb: u64,

    pub limit_mb: u64,

    pub request_mb: u64,

pub struct StorageConfig {

    pub storage_type: String,

    pub size_gb: u64,

    pub path: String,

    pub backup: BackupConfig,

pub struct NetworkResourceConfig {

    pub bandwidth_mbps: u32,

    pub max_connections: u32,

    pub port_ranges: Vec<PortRange>,

pub struct ScalingConfig {

    pub horizontal: HorizontalScalingConfig,

    pub vertical: VerticalScalingConfig,

    pub auto_scaling: AutoScalingConfig,

    pub load_balancing: LoadBalancingConfig,

pub struct HorizontalScalingConfig {

    pub min_instances: u32,

    pub max_instances: u32,

    pub target_instances: u32,

    pub policies: Vec<ScalingPolicy>,

pub struct VerticalScalingConfig {

    pub enabled: bool,

    pub cpu_limits: (f32, f32), // (min, max)

    pub memory_limits: (u64, u64), // (min_mb, max_mb)

pub struct AutoScalingConfig {

    pub cpu_scale_up_threshold: f32,

    pub cpu_scale_down_threshold: f32,

    pub memory_scale_up_threshold: f32,

    pub memory_scale_down_threshold: f32,

    pub cooldown_seconds: u64,

pub struct NetworkConfig {

    pub service_mesh: ServiceMeshConfig,

    pub dns: DnsConfig,

    pub tls: TlsConfig,

    pub vpn: Option<VpnConfig>,

pub struct ServiceMeshConfig {

    pub mesh_type: String,

    pub config: HashMap<String, String>,

pub struct LoadBalancingConfig {

    pub algorithm: String, // round_robin, least_connections, ip_hash

    pub health_check: HealthCheckConfig,

    pub session_affinity: Option<SessionAffinityConfig>,

pub struct DnsConfig {

    pub servers: Vec<String>,

    pub search_domains: Vec<String>,

    pub timeout_seconds: u32,

pub struct TlsConfig {

    pub certificates: CertificateConfig,

    pub cipher_suites: Vec<String>,

pub struct VpnConfig {

    pub vpn_type: String,

    pub server: String,

    pub credentials: HashMap<String, String>,

pub struct ServiceConfig {

    pub service_type: String,

    pub ports: Vec<PortConfig>,

    pub endpoints: Vec<EndpointConfig>,
    pub security: ServiceSecurityConfig,

pub struct PortConfig {

    pub port: u16,

    pub target_port: Option<u16>,

    pub protocol: String,

    pub external: bool,

pub struct EndpointConfig {

    pub methods: Vec<String>,

    pub auth_required: bool,

    pub rate_limit: Option<RateLimitConfig>,

pub struct HealthCheckConfig {

    pub interval_seconds: u32,

    pub failure_threshold: u32,

    pub success_threshold: u32,

pub struct BiomeSecurityConfig {

    pub encryption: EncryptionConfig,

    pub access_control: AccessControlConfig,

    pub audit: AuditConfig,

pub struct ServiceSecurityConfig {

    pub authentication: crate::canonical::configuration::security::AuthenticationConfig,

    pub authorization: AuthorizationConfig,

    pub rate_limiting: RateLimitConfig,

    pub session: UnifiedAuthConfig,

pub use crate::canonical::configuration::security::EncryptionConfig;

pub struct AccessControlConfig {

    pub model: String,

    pub default_deny: bool,

    pub roles: HashMap<String, Vec<String>>,

pub struct AuthorizationConfig {

    pub policies: HashMap<String, String>,

pub struct AuditConfig {

    pub log_path: String,

    pub retention_days: u32,

pub struct ComplianceConfig {

    pub frameworks: Vec<String>,

pub struct ResourceRequirements {

    pub storage: Option<StorageConfig>,

pub struct IntegrationConfig {

    pub external_services: HashMap<String, ExternalServiceConfig>,

    pub apis: HashMap<String, ApiConfig>,

pub struct ExternalServiceConfig {

    pub url: String,
    pub auth: Option<crate::config::security_unified::AuthenticationConfig>,

pub struct ApiConfig {

    pub base_path: String,

pub struct ServiceDiscoveryConfig {

    pub method: String,

pub struct BiomeResourceConfig {

    pub cpu_limits: CpuConfig,

    pub memory_limits: MemoryConfig,

    pub storage_limits: StorageConfig,

pub struct BackupConfig {

    pub interval_hours: u32,

    pub location: String,

pub struct CertificateConfig {

    pub cert_path: String,

    pub key_path: String,

    pub ca_path: Option<String>,

pub struct SessionAffinityConfig {

    pub affinity_type: String,

    pub cookie: Option<CookieConfig>,

pub struct CookieConfig {

    pub ttl_seconds: u32,

pub struct PortRange {

    pub start: u16,

    pub end: u16,

pub struct ScalingPolicy {

    pub metric: String,

    pub threshold: f64,

    pub action: String,

pub struct TokenConfig {

    pub token_type: String,

    pub ttl_seconds: u64,

    pub signing_key: String,

pub use crate::canonical::configuration::security::TotpConfig;

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
            config: HashMap::with_capacity(16),
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

impl Default for AccessControlConfig {}

            model: "RBAC".to_string(),
            default_deny: true,
            roles: HashMap::with_capacity(16),
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
