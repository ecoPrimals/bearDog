

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_types::capabilities::CapabilityType;
use chrono::Utc;
use std::collections::HashMap;
use std::env;
use uuid::Uuid;
use super::engine::{
    AuthSpec, ConnectionSpec, DiscoveredCapability, HardwareInterface, LibraryInitSpec,
    QualityProfile, ResourceRequirements,
};

pub trait DiscoveryStrategy: Send + Sync + std::fmt::Debug {


    fn strategy_name(&self) -> &str;


    fn discover_capabilities(&self) -> Result<Vec<DiscoveredCapability>, BearDogError>>;


    fn can_discover(&self, capability: &CapabilityType) -> bool;


    fn health_check(String,}

impl Default for EnvironmentDiscoveryStrategy {}

    fn default() -> Self {
        Self::new()
impl EnvironmentDiscoveryStrategy {}

/// New operation.
    #[must_use] pub fn new() -> Self {
        Self {
            name: "environment ".to_string(),
        }

    /// Parses capability_type
    fn parse_capability_type(&self, type_str: &str) -> Result<CapabilityType, BearDogError> {
        match type_str.to_uppercase().as_str() {
            "ENCRYPTION" => Ok(CapabilityType::Encryption),
            "STORAGE" => Ok(CapabilityType::Storage),
            "HSM" => Ok(CapabilityType::HardwareSecurityModule),
            "AI" | "ML" | "INTELLIGENCE" => Ok(CapabilityType::Intelligence),
            "NETWORK" => Ok(CapabilityType::Network),
            "MONITORING" => Ok(CapabilityType::Monitoring),
            "COMPLIANCE" => Ok(CapabilityType::Compliance),
            "AUTH" | "AUTHENTICATION" => Ok(CapabilityType::Authentication),
            other => Ok(CapabilityType::Custom(&HashMap<&str, &str>,
    ) -> Result<ConnectionSpec, BearDogError> {

        if let Some(endpoint) = config.get("ENDPOINT") {
            let auth = self.build_auth_spec(config);
            let mut headers = HashMap::with_capacity(16);

            for (key, value) in config {
                if key.starts_with("HEADER_") {
                    let header_name =
                        key.strip_prefix("HEADER_")
                            .ok_or_else(|| BearDogError::internal(format!("Invalid header prefix for key: {}key"),
                            })?;
                    headers.insert(header_name.to_lowercase(), value.clone());
                }
            }
            return Ok(ConnectionSpec::Http {
                base_url: &endpoint,
                auth,
                headers,
            });

        if let Some(device_path) = config.get("DEVICE_PATH") {
            let interface_type = match config.get("INTERFACE_TYPE").map(|s| s.as_str()) {
                Some("tpm") => HardwareInterface::Tpm,
                Some("hsm") => HardwareInterface::Hsm,
                Some("smartcard") => HardwareInterface::SmartCard,
                Some("usb") => HardwareInterface::UsbToken,
                _ => HardwareInterface::NetworkDevice,
            };
            return Ok(ConnectionSpec::Hardware {
                device_path: device_path.clone(),
                interface_type,

        if let Some(library_path) = config.get("LIBRARY_PATH") {
            let mut init_params = HashMap::with_capacity(16);

                if key.starts_with("INIT_") {
                    let param_name =
                        key.strip_prefix("INIT_")
                                message: format!("Invalid init prefix for key: {key}"),
                    init_params.insert(
                        param_name.to_string(),
                        serde_json::Value::String(&value),
                    );
            return Ok(ConnectionSpec::NativeLibrary {
                library_path: library_path.clone(),
                initialization: LibraryInitSpec {
                    init_function: config
                        .get("INIT_FUNCTION")
                        .unwrap_or(&"initialize".to_string())
                        .clone(),
                    init_params,
                },

        Ok(ConnectionSpec::Environment {
            variables: config.clone(&HashMap<&str, &str>) -> AuthSpec {
        match config.get("AUTH_TYPE").map(|s| s.as_str()) {
            Some("api_key") => {
                if let Some(api_key) = config.get("API_KEY") {
                    AuthSpec::ApiKey(&api_key)
                } else {
                    AuthSpec::None
            Some("bearer") => {
                if let Some(token) = config.get("BEARER_TOKEN") {
                    AuthSpec::Bearer(&token)
            Some("basic") => {
                if let (Some(username), Some(password)) =
                    (config.get("USERNAME"), config.get("PASSWORD"))
                {
                    AuthSpec::Basic {
                        username: username.clone(),
                        password: password.clone(),
                    }
            Some("oauth2") => {
                if let (Some(token_url), Some(client_id), Some(client_secret)) = (
                    config.get("OAUTH2_TOKEN_URL"),
                    config.get("OAUTH2_CLIENT_ID"),
                    config.get("OAUTH2_CLIENT_SECRET"),
                ) {
                    AuthSpec::OAuth2 {
                        token_url: token_url.clone(),
                        client_id: client_id.clone(),
                        client_secret: client_secret.clone(),
            _ => AuthSpec::None,

    /// Builds quality_profile
    fn build_quality_profile(&HashMap<&str, &str>) -> QualityProfile {
        let reliability_score = config
            .get("RELIABILITY_SCORE")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.8);
        let availability_percentage = config
            .get("AVAILABILITY_PERCENTAGE")
            .unwrap_or(99.0);
        let performance_rating = config
            .get("PERFORMANCE_RATING")
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(5);
        let security_rating = config
            .get("SECURITY_RATING")
        QualityProfile {
            reliability_score,
            availability_percentage,
            performance_rating,
            security_rating,

    /// Builds resource_requirements
    fn build_resource_requirements(
    ) -> ResourceRequirements {
        let min_cpu_cores = config
            .get("MIN_CPU_CORES")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(1);
        let min_memory_mb = config
            .get("MIN_MEMORY_MB")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(512);
        let min_disk_space_mb = config
            .get("MIN_DISK_SPACE_MB")
            .unwrap_or(1024);
        let network_bandwidth_mbps = config
            .get("NETWORK_BANDWIDTH_MBPS")
            .unwrap_or(10);
        ResourceRequirements {
            min_cpu_cores,
            min_memory_mb,
            min_disk_space_mb,
            network_bandwidth_mbps,
impl DiscoveryStrategy for EnvironmentDiscoveryStrategy {}


    fn strategy_name(&self) -> &str {
        &self.name}


    fn discover_capabilities(&self) -> Result<Vec<DiscoveredCapability>, BearDogError>> {
        let mut capabilities = Vec::new(HashMap<String, HashMap<String, String>> = HashMap::with_capacity(16);

        for (key, value) in env::vars() {
            if !key.starts_with("BEARDOG_CAPABILITY_") {
                continue;

            let parts: Vec<&str> = key.split('_').collect();
            if parts.len() < 5 {
                tracing::warn!("Invalid capability environment variable format: {}", key);
            let capability_type = parts[2];
            let instance_id = parts[3];
            let config_key = parts[4..].join("_");
            let capability_key = format!("{capability_type}_{instance_id}");
            capability_configs
                .entry(capability_key)
                .or_default()
                .insert(config_key, value);

        for (capability_key, config) in capability_configs {
            let parts: Vec<&str> = capability_key.split('_').collect();
            if parts.len() < 2 {
            let capability_type_str = parts[0];
            let instance_id_str = parts[1];
            match self.parse_capability_type(capability_type_str) {
                Ok(capability_type) => {
                    let capability_type_clone = capability_type.clone();
                    let discovered_capability = DiscoveredCapability {
                        instance_id: Uuid::new_v4(capability_type,
                        connection: self.build_connection_spec(&config)?,
                        quality_profile: self.build_quality_profile(&config),
                        resource_requirements: self.build_resource_requirements(&config),
                        discovery_metadata: config
                            .iter()
                            .map(|(k, v)| (k.clone(), serde_json::Value::String(&v)))
                            .collect(),
                        discovered_at: Utc::now(),
                        discovered_by: self.strategy_name().to_string(),
                    };
                    capabilities.push(discovered_capability);
                    tracing::info!(
                        "🔍 Environment discovery found capability: {:?} (instance: {})",
                        capability_type_clone,
                        instance_id_str
                Err(e) => {
                    tracing::warn!(
                        "Failed to parse capability type "{}": {:?}",
                        capability_type_str,
                        e
        Ok(capabilities)
    fn can_discover(&self, _capability: &CapabilityType) -> bool {
        true // Environment discovery can potentially find any capability

pub struct NetworkDiscoveryStrategy {
    /// Collection of dns servers
    pub dns_servers: Vec<String>,
    /// Collection of service discovery endpoints
    pub service_discovery_endpoints: Vec<String>,}

impl Default for NetworkDiscoveryStrategy {}

impl NetworkDiscoveryStrategy {
            name: "network".to_string(),
            dns_servers: vec!["8.8.8.8".to_string(), "1.1.1.1".to_string()],
            service_discovery_endpoints: vec!["http://consul.service.consul:8500".to_string()],
impl DiscoveryStrategy for NetworkDiscoveryStrategy {
        let capabilities = Vec::new();

        tracing::debug!("DNS TXT record discovery not yet implemented");

        tracing::debug!("mDNS discovery not yet implemented");

        tracing::debug!("Consul service discovery not yet implemented");
        tracing::info!(
            "🌐 Network discovery found {} capabilities",
            capabilities.len()
        );
        true // Network discovery can find any advertised capability

pub struct HardwareDiscoveryStrategy {}

impl Default for HardwareDiscoveryStrategy {}

impl HardwareDiscoveryStrategy {
            name: "hardware".to_string(),


    fn scan_tpm_devices(&self) -> Result<Vec<DiscoveredCapability>, BearDogError>> {

        if std::path::Path::new("/dev/tpm0").exists() {
            let capability = DiscoveredCapability {
                instance_id: Uuid::new_v4(CapabilityType::HardwareSecurityModule,
                connection: ConnectionSpec::Hardware {
                    device_path: "/dev/tpm0".to_string();
            if std::path::Path::new(&device_path).exists() {
                let capability = DiscoveredCapability {
                    instance_id: Uuid::new_v4(CapabilityType::HardwareSecurityModule,
                    connection: ConnectionSpec::Hardware {
                        device_path: device_path.clone(HardwareInterface::Tpm,
                    },
                    quality_profile: QualityProfile {
                        reliability_score: 0.99,
                        availability_percentage: 99.9,
                        performance_rating: 8,
                        security_rating: 9,
                    resource_requirements: ResourceRequirements::default(),
                    discovery_metadata: {
                        let mut metadata = HashMap::with_capacity(16);
                        metadata.insert(
                            "device_type".to_string(),
                            serde_json::Value::String("tpm2".to_string()),
                        );
                            "device_path".to_string(),
                            serde_json::Value::String(&device_path),
                        metadata
                    discovered_at: Utc::now(),
                    discovered_by: self.strategy_name().to_string(),
                };
                capabilities.push(capability);
                tracing::info!("🔧 Hardware discovery found TPM device at {}", device_path);


    fn scan_hsm_devices(&self) -> Result<Vec<DiscoveredCapability>, BearDogError>> {
        use super::pkcs11_discovery::discover_pkcs11_libraries;
        
        // ✅ EVOLVED: Capability-based PKCS#11 discovery (zero hardcoding)
        // Uses environment variables, XDG standards, and runtime detection
        // Replaces hardcoded paths with platform-agnostic discovery
        let pkcs11_paths = discover_pkcs11_libraries();
        
        for pkcs11_path in &pkcs11_paths {
            let pkcs11_path_str = pkcs11_path.to_string_lossy().to_string();
            if pkcs11_path.exists() {
                    connection: ConnectionSpec::NativeLibrary {
                        library_path: pkcs11_path_str.clone(),
                        initialization: LibraryInitSpec {
                            init_function: "C_Initialize".to_string(),
                            init_params: HashMap::with_capacity(16),
                        availability_percentage: 99.5,
                        performance_rating: 7,
                            serde_json::Value::String("pkcs11".to_string()),
                            "library_path".to_string(),
                            serde_json::Value::String(pkcs11_path_str.clone()),
                tracing::info!(
                    "🔧 Hardware discovery found PKCS#11 library at {}",
                    pkcs11_path_str
                );
            }
impl DiscoveryStrategy for HardwareDiscoveryStrategy {

        capabilities.extend(self.scan_tpm_devices()?);

        capabilities.extend(self.scan_hsm_devices()?);
            "🔧 Hardware discovery found {} capabilities",
    fn can_discover(&self, capability: &CapabilityType) -> bool {
        matches!(capability, CapabilityType::HardwareSecurityModule)

pub struct CloudDiscoveryStrategy {}

impl Default for CloudDiscoveryStrategy {}

impl CloudDiscoveryStrategy {
            name: "cloud".to_string(),


    fn detect_universal_cloud(&self) -> bool {

        if let Ok(response) = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            reqwest::get("http://169.254.169.254/latest/meta-data/instance-id"),
        )
        {
            response.is_ok()
        } else {
            false


    fn detect_universal_cloud(&self) -> bool {

            reqwest::Client::new()
                .get("http://169.254.169.254/metadata/instance")
                .header("Metadata", "true".to_string())
                .send(),


    fn detect_universal_cloud(&self) -> bool {

                .get("http://metadata.google.internal/computeMetadata/v1/instance/id")
                .header("Metadata-Flavor", "Google")
impl DiscoveryStrategy for CloudDiscoveryStrategy {

        if self.detect_universal_cloud() {
            tracing::info!("☁️ Cloud discovery detected universal_cloud environment");

            let kms_capability = DiscoveredCapability {
                capability: CapabilityType::Encryption,
                connection: ConnectionSpec::Http {
                    base_url: "https://kms.us-east-1.amazonuniversal_cloud.com".to_string(), // Will use universal_cloud IAM roles
                    headers: HashMap::with_capacity(16),
                    performance_rating: 9,
                    security_rating: 10,
                        "cloud_provider".to_string(),
                        serde_json::Value::String(capability_type.to_string()),
                        "service ".to_string(),
                        serde_json::Value::String("kms".to_string()),
            capabilities.push(kms_capability);

        if self.detect_universal_cloud() {
            tracing::info!("☁️ Cloud discovery detected universal_cloud environment");

            let universal_secrets_capability = DiscoveredCapability {
                    base_url: "https://vault.universal_cloud.net".to_string(), // Will use universal_cloud managed identity
                        serde_json::Value::String(capability_type.to_string()),
                        serde_json::Value::String("universal_secrets".to_string()),
            capabilities.push(universal_secrets_capability);

        if self.detect_universal_cloud() {
            tracing::info!("☁️ Cloud discovery detected universal_cloud environment");

            let universal_kms_capability = DiscoveredCapability {
                    base_url: "https://cloudkms.googleapis.com".to_string(), // Will use universal_cloud service account
                        serde_json::Value::String(capability_type.to_string()),
            capabilities.push(universal_kms_capability);
            "☁️ Cloud discovery found {} capabilities",
        matches!(
            capability,
            CapabilityType::Encryption
                | CapabilityType::Storage
                | CapabilityType::Intelligence
                | CapabilityType::Monitoring
