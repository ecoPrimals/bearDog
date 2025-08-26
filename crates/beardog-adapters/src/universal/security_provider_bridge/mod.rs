

use beardog_errors::{BearDogError, BearDogResult};
use beardog_traits::canonical::SecurityProvider;
use beardog_types::canonical::configuration::security::SecurityConfig;
use beardog_types::canonical::services::UniversalResponse;
use serde_json::json;
use tracing::warn;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub struct SecurityProviderBridge<P: SecurityProvider + Send + Sync + 'static> {

    security_provider: P,

    config: BridgeConfig,

    sessions: Arc<RwLock<HashMap<String, SecuritySession>>>,

    vendor_integrations: Arc<RwLock<HashMap<String, Box<dyn VendorHsmIntegration>>>>,

    metrics_collector: Arc<SecurityMetricsCollector>,
}

#[allow(async_fn_in_trait)]
pub trait VendorHsmIntegration: Send + Sync + std::fmt::Debug {

    fn vendor_name(&self) -> &str;

    async fn initialize(&self, config: &VendorHsmConfig) -> BearDogResult<()>;

    async fn generate_vendor_key(&self, key_spec: &VendorKeySpec)
        -> BearDogResult<VendorKeyHandle>;

    async fn vendor_sign(
        &self,
        key_handle: &VendorKeyHandle,
        data: &[u8],
    ) -> BearDogResult<Vec<u8>>;

    async fn vendor_verify(
        signature: &[u8],
    ) -> BearDogResult<bool>;

    async fn get_vendor_capabilities(&self) -> BearDogResult<VendorCapabilities>;

    async fn health_check(&self) -> BearDogResult<VendorHealthStatus>;

#[derive(Debug, Clone)]
pub struct VendorHsmConfig {
    pub vendor: String,
}

    pub model: String,
    pub connection_string: String,
    pub authentication: VendorAuthConfig,
    pub features: HashMap<String, String>,

pub enum VendorAuthConfig {
    Pin { pin: String },
    Certificate { cert_path: String, key_path: String },
    SmartCard { card_id: String, pin: String },
    NetworkAuth { username: String, password: String },
    Custom { auth_data: HashMap<String, String> },

pub struct VendorKeySpec {
    pub key_id: String,
    pub key_type: VendorKeyType,
    pub usage: Vec<VendorKeyUsage>,
    pub security_level: VendorSecurityLevel,

pub enum VendorKeyType {
    Rsa {
        size: u32,
        public_exponent: Option<u32>,
    },
    EccP256,
    EccP384,
    EccP521,
    Aes {
    Custom {
        algorithm: String,
        parameters: HashMap<String, String>,

pub enum VendorKeyUsage {
    Sign,
    Verify,
    Encrypt,
    Decrypt,
    KeyWrap,
    KeyUnwrap,
    Derive,

pub enum VendorSecurityLevel {
    Software,
    HardwareBacked,
    Fips140Level2,
    Fips140Level3,
    CommonCriteriaEal4Plus,

pub struct VendorKeyHandle {
    pub handle: String,
    pub capabilities: VendorKeyCapabilities,

pub struct VendorKeyCapabilities {
    pub extractable: bool,
    pub hardware_backed: bool,
    pub attestation_supported: bool,
    pub user_presence_required: bool,

pub struct VendorCapabilities {
    pub supported_algorithms: Vec<String>,
    pub max_key_size: HashMap<String, u32>,
    pub performance_metrics: VendorPerformanceMetrics,
    pub security_features: Vec<String>,

pub struct VendorPerformanceMetrics {
    pub operations_per_second: HashMap<String, f64>,
    pub average_latency_ms: HashMap<String, f64>,
    pub max_concurrent_operations: u32,

pub struct VendorHealthStatus {
    pub healthy: bool,
    pub response_time_ms: f64,
    pub error_count: u64,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub details: HashMap<String, String>,

#[derive(Debug)]
pub struct SecurityMetricsCollector {
    operation_counters: Arc<RwLock<HashMap<String, u64>>>,
    latency_metrics: Arc<RwLock<HashMap<String, Vec<f64>>>>,
    error_counters: Arc<RwLock<HashMap<String, u64>>>,}

impl SecurityMetricsCollector {}

    pub fn new() -> Self {
        Self {
            operation_counters: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            latency_metrics: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            error_counters: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        }
    }
    pub async fn record_operation(&self, operation: &str, duration_ms: f64, success: bool) {

        {
            let mut counters = self.operation_counters.write().await;
            *counters.entry(operation.to_string()).or_insert(0) += 1;

            let mut latencies = self.latency_metrics.write().await;
            latencies
                .entry(operation.to_string())
                .or_insert_with(Vec::new)
                .push(duration_ms);

        if !success {
            let mut errors = self.error_counters.write().await;
            *errors.entry(operation.to_string()).or_insert(0) += 1;}

    pub async fn get_metrics(&self) -> SecurityProviderMetrics {
        let operation_counters = self.operation_counters.read().await.clone();
        let latency_metrics = self.latency_metrics.read().await.clone();
        let error_counters = self.error_counters.read().await.clone();
        SecurityProviderMetrics {
            operation_counters,
            average_latencies: latency_metrics
                .into_iter()
                .map(|(op, latencies)| {
                    let avg = latencies.iter().sum::<f64>() / latencies.len() as f64;
                    (op, avg)
                })
                .collect(),
            error_counters,
            uptime_seconds: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),

pub struct SecurityProviderMetrics {
    pub operation_counters: HashMap<String, u64>,
    pub average_latencies: HashMap<String, f64>,
    pub error_counters: HashMap<String, u64>,
    pub uptime_seconds: u64,

impl Default for BridgeConfig {}

    fn default() -> Self {
            enable_authentication: true,
            enable_authorization: true,
            enable_audit_logging: true,
            enable_crypto_operations: true,
            enable_key_management: true,
            max_concurrent_sessions: 1000,
            vendor_hsm_configs: HashMap::with_capacity(16),
            enable_performance_monitoring: true,
            metrics_retention_hours: 24,
impl SecurityProviderBridge {

    pub async fn new() -> BearDogResult<Self> {
        let _config = SecurityProviderConfig::default();
        let security_provider = Arc::new(BearDogSecurityProvider::default());
        Ok(Self {
            security_provider,
            config: BridgeConfig::default(),
            sessions: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            vendor_integrations: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            metrics_collector: Arc::new(SecurityMetricsCollector::new()),
        })

    pub fn with_provider(
        security_provider: Arc<BearDogSecurityProvider>,
        config: BridgeConfig,
    ) -> Self {
            config,

    pub async fn register_vendor_integration(
        integration: Box<dyn VendorHsmIntegration>,
    ) -> BearDogResult<()> {
        let vendor_name = integration.vendor_name().to_string();
        info!("🏭 Registering vendor HSM integration: {}", vendor_name);

        if let Some(config) = self.config.vendor_hsm_configs.get(&vendor_name) {
            integration.initialize(config).await?;

        let mut integrations = self.vendor_integrations.write().await;
        integrations.insert(vendor_name.clone(), integration);
        info!("✅ Vendor HSM integration registered: {}", vendor_name);
        Ok(())

    pub async fn get_available_vendors(&self) -> Vec<String> {
        let integrations = self.vendor_integrations.read().await;
        integrations.keys().cloned().collect()

    pub async fn perform_vendor_operation(
        vendor: &str,
        operation: VendorOperation,
    ) -> BearDogResult<VendorOperationResult> {
        let start_time = std::time::Instant::now();
        let operation_name = format_args!("vendor_{}_{}", vendor, operation.operation_type().to_string());
        let result = self.internal_vendor_operation(vendor, operation).await;

        let duration_ms = start_time.elapsed().as_millis() as f64;
        let success = result.is_ok();
        self.metrics_collector
            .record_operation(&operation_name, duration_ms, success)
            .await;
        result

    async fn internal_vendor_operation(
        let integration = integrations.get(vendor).ok_or_else(|| {
            BearDogError::not_found(format_args!("No integration found for vendor: {}", vendor).to_string())
        })?;
        match operation {
            VendorOperation::GenerateKey { key_spec } => {
                let key_handle = integration.generate_vendor_key(&key_spec).await?;
                Ok(VendorOperationResult::KeyGenerated { key_handle })
            }
            VendorOperation::Sign { key_handle, data } => {
                let signature = integration.vendor_sign(&key_handle, &data).await?;
                Ok(VendorOperationResult::DataSigned { signature })
            VendorOperation::Verify {
                key_handle,
                data,
                signature,
            } => {
                let valid = integration
                    .vendor_verify(&key_handle, &data, &signature)
                    .await?;
                Ok(VendorOperationResult::SignatureVerified { valid })
            VendorOperation::GetCapabilities => {
                let capabilities = integration.get_vendor_capabilities().await?;
                Ok(VendorOperationResult::Capabilities { capabilities })
            VendorOperation::HealthCheck => {
                let health = integration.health_check().await?;
                Ok(VendorOperationResult::HealthStatus { health })

    pub async fn get_security_metrics(&self) -> BearDogResult<SecurityProviderMetrics> {
        if !self.config.enable_performance_monitoring {
            return Err(BearDogError::configuration(
                "Performance monitoring is disabled".to_string(),
            ));
        Ok(self.metrics_collector.get_metrics().await)

    pub async fn perform_multi_vendor_operation(
        preferred_vendors: Vec<&str>,
        let mut last_error = None;
        for vendor in preferred_vendors {
            match self
                .perform_vendor_operation(&vendor, operation.clone())
                .await
            {
                Ok(result) => {
                    info!("✅ Multi-vendor operation succeeded with: {}", vendor);
                    return Ok(result);
                }
                Err(e) => {
                    warn!("⚠️ Vendor {} failed for operation: {:?}", vendor, e);
                    last_error = Some(e);
        Err(last_error.unwrap_or_else(|| {
            BearDogError::not_found("No vendors available for operation".to_string())
        }))

    pub async fn get_vendor_health(&self, vendor: &str) -> BearDogResult<VendorHealthStatus> {
        integration.health_check().await

    pub async fn get_all_vendor_health(
    ) -> BearDogResult<HashMap<String, VendorHealthStatus>> {
        let mut health_statuses = HashMap::with_capacity(16);
        for (vendor, integration) in integrations.iter() {
            match integration.health_check().await {
                Ok(health) => {
                    health_statuses.insert(vendor.clone(), health);
                    warn!("Failed to get health for vendor {}: {:?}", vendor, e);
                    health_statuses.insert(
                        vendor.clone(),
                        VendorHealthStatus {
                            healthy: false,
                            response_time_ms: 0.0,
                            error_count: 1,
                            last_check: chrono::Utc::now(),
                            details: std::iter::once(("error".to_string(), e.to_string()))
                                .collect(),
                        },
                    );
        Ok(health_statuses)

    fn error_response(&self, system_id: &str, code: &str, message: &str) -> UniversalResponse {
        UniversalResponse {
            request_id: system_id,
            status: beardog_types::canonical::services::ResponseStatus::Error,
            data: Some(json!({
                "error": {
                    "code": code,
                    "message": message
            })),
            error: Some(message.to_string()),
            timestamp: chrono::Utc::now(),
            metadata: std::collections::HashMap::with_capacity(16),

pub enum VendorOperation {
    GenerateKey {
        key_spec: VendorKeySpec,
    Sign {
        key_handle: VendorKeyHandle,
        data: Vec<u8>,
    Verify {
        signature: Vec<u8>,
    GetCapabilities,
    HealthCheck,}

impl VendorOperation {}

    pub fn operation_type(&self) -> &'static str {
        match self {
            VendorOperation::GenerateKey { .. } => "generate_key",
            VendorOperation::Sign { .. } => "sign",
            VendorOperation::Verify { .. } => "verify",
            VendorOperation::GetCapabilities => "get_capabilities",
            VendorOperation::HealthCheck => "health_check",

pub enum VendorOperationResult {
    KeyGenerated { key_handle: VendorKeyHandle },
    DataSigned { signature: Vec<u8> },
    SignatureVerified { valid: bool },
    Capabilities { capabilities: VendorCapabilities },
    HealthStatus { health: VendorHealthStatus },
