

use beardog_errors::{BearDogError, BearDogResult};
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

    async fn discover_capabilities(&self) -> BearDogResult<Vec<DiscoveredCapability>>;

    async fn can_discover(&self, capability: &CapabilityType) -> bool;

    async fn health_check(&self) -> BearDogResult<bool> {
        Ok(true) // Default implementation
    }
}

#[derive(Debug)]
pub struct EnvironmentDiscoveryStrategy {
    pub name: String,}

impl Default for EnvironmentDiscoveryStrategy {}

    fn default() -> Self {
        Self::new()
impl EnvironmentDiscoveryStrategy {}

    #[must_use] pub fn new() -> Self {
        Self {
            name: "environment".to_string(),
        }

    fn parse_capability_type(&self, type_str: &str) -> BearDogResult<CapabilityType> {
        match type_str.to_uppercase().as_str() {
            "ENCRYPTION" => Ok(CapabilityType::Encryption),
            "STORAGE" => Ok(CapabilityType::Storage),
            "HSM" => Ok(CapabilityType::HardwareSecurityModule),
            "AI" | "ML" | "INTELLIGENCE" => Ok(CapabilityType::Intelligence),
            "NETWORK" => Ok(CapabilityType::Network),
            "MONITORING" => Ok(CapabilityType::Monitoring),
            "COMPLIANCE" => Ok(CapabilityType::Compliance),
            "AUTH" | "AUTHENTICATION" => Ok(CapabilityType::Authentication),
            other => Ok(CapabilityType::Custom(other.to_lowercase())),

    fn build_connection_spec(
        &self,
        config: &HashMap<&str, &str>,
    ) -> BearDogResult<ConnectionSpec> {

        if let Some(endpoint) = config.get("ENDPOINT") {
            let auth = self.build_auth_spec(config);
            let mut headers = HashMap::with_capacity(16);

            for (key, value) in config {
                if key.starts_with("HEADER_") {
                    let header_name =
                        key.strip_prefix("HEADER_")
                            .ok_or_else(|| BearDogError::internal(format!("Invalid header prefix for key: {key)"),
                            })?;
                    headers.insert(header_name.to_lowercase(), value.clone());
                }
            }
            return Ok(ConnectionSpec::Http {
                base_url: endpoint.clone(),
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
                        serde_json::Value::String(value.clone()),
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
            variables: config.clone(),
        })

    fn build_auth_spec(&self, config: &HashMap<&str, &str>) -> AuthSpec {
        match config.get("AUTH_TYPE").map(|s| s.as_str()) {
            Some("api_key") => {
                if let Some(api_key) = config.get("API_KEY") {
                    AuthSpec::ApiKey(api_key.clone())
                } else {
                    AuthSpec::None
            Some("bearer") => {
                if let Some(token) = config.get("BEARER_TOKEN") {
                    AuthSpec::Bearer(token.clone())
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

    fn build_quality_profile(&self, config: &HashMap<&str, &str>) -> QualityProfile {
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

    async fn discover_capabilities(&self) -> BearDogResult<Vec<DiscoveredCapability>> {
        let mut capabilities = Vec::new();
        let mut capability_configs: HashMap<String, HashMap<String, String>> = HashMap::with_capacity(16);

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
                        instance_id: Uuid::new_v4(),
                        capability: capability_type,
                        connection: self.build_connection_spec(&config)?,
                        quality_profile: self.build_quality_profile(&config),
                        resource_requirements: self.build_resource_requirements(&config),
                        discovery_metadata: config
                            .iter()
                            .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
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
                        "Failed to parse capability type '{}': {:?}",
                        capability_type_str,
                        e
        Ok(capabilities)
    async fn can_discover(&self, _capability: &CapabilityType) -> bool {
        true // Environment discovery can potentially find any capability

pub struct NetworkDiscoveryStrategy {
    pub dns_servers: Vec<String>,
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

    async fn scan_tpm_devices(&self) -> BearDogResult<Vec<DiscoveredCapability>> {

        if std::path::Path::new("/dev/tpm0").exists() {
            let capability = DiscoveredCapability {
                instance_id: Uuid::new_v4(),
                capability: CapabilityType::HardwareSecurityModule,
                connection: ConnectionSpec::Hardware {
                    device_path: "/dev/tpm0".to_string(),
                    interface_type: HardwareInterface::Tpm,
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
                        "device_path".to_string(),
                        serde_json::Value::String("/dev/tpm0".to_string()),
                    metadata
                discovered_at: Utc::now(),
                discovered_by: self.strategy_name().to_string(),
            capabilities.push(capability);
            tracing::info!("🔧 Hardware discovery found TPM 2.0 device at /dev/tpm0");

        for i in 1..=4 {
            let device_path = format!("/dev/tpm{i}");
            if std::path::Path::new(&device_path).exists() {
                let capability = DiscoveredCapability {
                    instance_id: Uuid::new_v4(),
                    capability: CapabilityType::HardwareSecurityModule,
                    connection: ConnectionSpec::Hardware {
                        device_path: device_path.clone(),
                        interface_type: HardwareInterface::Tpm,
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
                            serde_json::Value::String(device_path.clone()),
                        metadata
                    discovered_at: Utc::now(),
                    discovered_by: self.strategy_name().to_string(),
                };
                capabilities.push(capability);
                tracing::info!("🔧 Hardware discovery found TPM device at {}", device_path);

    async fn scan_hsm_devices(&self) -> BearDogResult<Vec<DiscoveredCapability>> {

        let pkcs11_paths = vec![
            "/usr/lib/softhsm/libsofthsm2.so",
            "/usr/lib/x86_64-linux-gnu/softhsm/libsofthsm2.so",
            "/usr/local/lib/softhsm/libsofthsm2.so",
            "/opt/nfast/toolkits/pkcs11/libcknfast.so",
            "/usr/lib/libCryptoki2_64.so",
        ];
        for pkcs11_path in pkcs11_paths {
            if std::path::Path::new(pkcs11_path).exists() {
                    connection: ConnectionSpec::NativeLibrary {
                        library_path: pkcs11_path.to_string(),
                        initialization: LibraryInitSpec {
                            init_function: "C_Initialize".to_string(),
                            init_params: HashMap::with_capacity(16),
                        },
                        reliability_score: 0.95,
                        availability_percentage: 99.5,
                        performance_rating: 7,
                            serde_json::Value::String("pkcs11".to_string()),
                            "library_path".to_string(),
                            serde_json::Value::String(pkcs11_path.to_string()),
                tracing::info!(
                    "🔧 Hardware discovery found PKCS#11 library at {}",
                    pkcs11_path
                );
impl DiscoveryStrategy for HardwareDiscoveryStrategy {

        capabilities.extend(self.scan_tpm_devices().await?);

        capabilities.extend(self.scan_hsm_devices().await?);
            "🔧 Hardware discovery found {} capabilities",
    async fn can_discover(&self, capability: &CapabilityType) -> bool {
        matches!(capability, CapabilityType::HardwareSecurityModule)

pub struct CloudDiscoveryStrategy {}

impl Default for CloudDiscoveryStrategy {}

impl CloudDiscoveryStrategy {
            name: "cloud".to_string(),

    async fn detect_aws(&self) -> bool {

        if let Ok(response) = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            reqwest::get("http://169.254.169.254/latest/meta-data/instance-id"),
        )
        .await
        {
            response.is_ok()
        } else {
            false

    async fn detect_azure(&self) -> bool {

            reqwest::Client::new()
                .get("http://169.254.169.254/metadata/instance")
                .header("Metadata", "true")
                .send(),

    async fn detect_gcp(&self) -> bool {

                .get("http://metadata.google.internal/computeMetadata/v1/instance/id")
                .header("Metadata-Flavor", "Google")
impl DiscoveryStrategy for CloudDiscoveryStrategy {

        if self.detect_aws().await {
            tracing::info!("☁️ Cloud discovery detected AWS environment");

            let kms_capability = DiscoveredCapability {
                capability: CapabilityType::Encryption,
                connection: ConnectionSpec::Http {
                    base_url: "https://kms.us-east-1.amazonaws.com".to_string(),
                    auth: AuthSpec::None, // Will use AWS IAM roles
                    headers: HashMap::with_capacity(16),
                    reliability_score: 0.999,
                    performance_rating: 9,
                    security_rating: 10,
                        "cloud_provider".to_string(),
                        serde_json::Value::String("aws".to_string()),
                        "service".to_string(),
                        serde_json::Value::String("kms".to_string()),
            capabilities.push(kms_capability);

        if self.detect_azure().await {
            tracing::info!("☁️ Cloud discovery detected Azure environment");

            let keyvault_capability = DiscoveredCapability {
                    base_url: "https://vault.azure.net".to_string(),
                    auth: AuthSpec::None, // Will use Azure managed identity
                        serde_json::Value::String("azure".to_string()),
                        serde_json::Value::String("keyvault".to_string()),
            capabilities.push(keyvault_capability);

        if self.detect_gcp().await {
            tracing::info!("☁️ Cloud discovery detected GCP environment");

            let gcp_kms_capability = DiscoveredCapability {
                    base_url: "https://cloudkms.googleapis.com".to_string(),
                    auth: AuthSpec::None, // Will use GCP service account
                        serde_json::Value::String("gcp".to_string()),
            capabilities.push(gcp_kms_capability);
            "☁️ Cloud discovery found {} capabilities",
        matches!(
            capability,
            CapabilityType::Encryption
                | CapabilityType::Storage
                | CapabilityType::Intelligence
                | CapabilityType::Monitoring
