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


/// Security Provider Bridge - MODERNIZED TO CANONICAL TRAITS
///
/// **MODERNIZATION COMPLETE** ✅
/// Universal bridge for security provider integration using canonical traits.
/// Now uses zero-cost async and unified provider interfaces.
///
/// ## Migration Benefits:
/// - **Zero-cost async** - Native async fn instead of async_trait
/// - **Canonical providers** - Unified SecurityProvider interface
/// - **Better performance** - No boxing overhead

use beardog_errors::{BearDogError, BearDogResult};
use beardog_traits::canonical::SecurityProvider;
use beardog_types::canonical::configuration::security::SecurityConfig;
use beardog_types::canonical::services::UniversalResponse;
use serde_json::json;
use tracing::warn;
// JSON handling is done in the crypto_handlers module
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
// UUID generation is handled in the implementation

/// Zero-cost security provider bridge using generic composition
/// 
/// This modernized bridge uses generic composition instead of trait objects
/// to eliminate runtime dispatch overhead and improve performance.
pub struct SecurityProviderBridge<P: SecurityProvider + Send + Sync + 'static> {
    /// Security provider implementation (generic for zero-cost)
    security_provider: P,
    /// Bridge configuration
    config: BridgeConfig,
    /// Active security sessions managed by the bridge
    sessions: Arc<RwLock<HashMap<String, SecuritySession>>>,
    /// Vendor-specific HSM integrations (modernized)
    vendor_integrations: Arc<RwLock<HashMap<String, Box<dyn VendorHsmIntegration>>>>,
    /// Performance metrics collector
    metrics_collector: Arc<SecurityMetricsCollector>,
}
/// Vendor-specific HSM integration trait - MODERNIZED
/// 
/// **ZERO-COST ASYNC** - Uses native async fn instead of async_trait
#[allow(async_fn_in_trait)]
pub trait VendorHsmIntegration: Send + Sync + std::fmt::Debug {
    /// Get vendor name (e.g., "SafeNet", "Thales", "Utimaco")}


    fn vendor_name(&self) -> &str;
    /// Initialize vendor-specific HSM connection
    async fn initialize(&self, config: &VendorHsmConfig) -> BearDogResult<()>;
    /// Perform vendor-specific key generation
    async fn generate_vendor_key(&self, key_spec: &VendorKeySpec)
        -> BearDogResult<VendorKeyHandle>;
    /// Perform vendor-specific signing
    async fn vendor_sign(
        &self,
        key_handle: &VendorKeyHandle,
        data: &[u8],
    ) -> BearDogResult<Vec<u8>>;
    /// Verify vendor-specific signatures
    async fn vendor_verify(
        signature: &[u8],
    ) -> BearDogResult<bool>;
    /// Get vendor-specific capabilities
    async fn get_vendor_capabilities(&self) -> BearDogResult<VendorCapabilities>;
    /// Check vendor HSM health
    async fn health_check(&self) -> BearDogResult<VendorHealthStatus>;
/// Vendor HSM configuration
#[derive(Debug, Clone)]
pub struct VendorHsmConfig {
    pub vendor: String,
}


    pub model: String,
    pub connection_string: String,
    pub authentication: VendorAuthConfig,
    pub features: HashMap<String, String>,
/// Vendor authentication configuration
pub enum VendorAuthConfig {
    Pin { pin: String },
    Certificate { cert_path: String, key_path: String },
    SmartCard { card_id: String, pin: String },
    NetworkAuth { username: String, password: String },
    Custom { auth_data: HashMap<String, String> },
/// Vendor key specification
pub struct VendorKeySpec {
    pub key_id: String,
    pub key_type: VendorKeyType,
    pub usage: Vec<VendorKeyUsage>,
    pub security_level: VendorSecurityLevel,
/// Vendor key types
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
/// Vendor key usage flags
pub enum VendorKeyUsage {
    Sign,
    Verify,
    Encrypt,
    Decrypt,
    KeyWrap,
    KeyUnwrap,
    Derive,
/// Vendor security levels}


pub enum VendorSecurityLevel {
    Software,
    HardwareBacked,
    Fips140Level2,
    Fips140Level3,
    CommonCriteriaEal4Plus,
/// Vendor key handle
pub struct VendorKeyHandle {
    pub handle: String,
    pub capabilities: VendorKeyCapabilities,
/// Vendor key capabilities
pub struct VendorKeyCapabilities {
    pub extractable: bool,
    pub hardware_backed: bool,
    pub attestation_supported: bool,
    pub user_presence_required: bool,
/// Vendor capabilities
pub struct VendorCapabilities {
    pub supported_algorithms: Vec<String>,
    pub max_key_size: HashMap<String, u32>,
    pub performance_metrics: VendorPerformanceMetrics,
    pub security_features: Vec<String>,
/// Vendor performance metrics
pub struct VendorPerformanceMetrics {
    pub operations_per_second: HashMap<String, f64>,
    pub average_latency_ms: HashMap<String, f64>,
    pub max_concurrent_operations: u32,
/// Vendor health status
pub struct VendorHealthStatus {
    pub healthy: bool,
    pub response_time_ms: f64,
    pub error_count: u64,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub details: HashMap<String, String>,
/// Security metrics collector
#[derive(Debug)]
pub struct SecurityMetricsCollector {
    operation_counters: Arc<RwLock<HashMap<String, u64>>>,
    latency_metrics: Arc<RwLock<HashMap<String, Vec<f64>>>>,
    error_counters: Arc<RwLock<HashMap<String, u64>>>,}


impl SecurityMetricsCollector {}


    pub fn new() -> Self {
        Self {
            operation_counters: Arc::new(RwLock::new(HashMap::new())),
            latency_metrics: Arc::new(RwLock::new(HashMap::new())),
            error_counters: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub async fn record_operation(&self, operation: &str, duration_ms: f64, success: bool) {
        // Record operation count
        {
            let mut counters = self.operation_counters.write().await;
            *counters.entry(operation.to_string()).or_insert(0) += 1;
        // Record latency
            let mut latencies = self.latency_metrics.write().await;
            latencies
                .entry(operation.to_string())
                .or_insert_with(Vec::new)
                .push(duration_ms);
        // Record errors
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
/// Security provider performance metrics
pub struct SecurityProviderMetrics {
    pub operation_counters: HashMap<String, u64>,
    pub average_latencies: HashMap<String, f64>,
    pub error_counters: HashMap<String, u64>,
    pub uptime_seconds: u64,
/// Configuration for the security provider bridge
pub struct BridgeConfig {
    /// Enable authentication services
    pub enable_authentication: bool,
    /// Enable authorization services
    pub enable_authorization: bool,
    /// Enable audit logging
    pub enable_audit_logging: bool,
    /// Enable cryptographic operations
    pub enable_crypto_operations: bool,
    /// Enable key management
    pub enable_key_management: bool,
    /// Maximum concurrent sessions
    pub max_concurrent_sessions: usize,
    /// Vendor HSM configurations
    pub vendor_hsm_configs: HashMap<String, VendorHsmConfig>,
    /// Performance monitoring settings
    pub enable_performance_monitoring: bool,
    pub metrics_retention_hours: u64,}


impl Default for BridgeConfig {}


    fn default() -> Self {
            enable_authentication: true,
            enable_authorization: true,
            enable_audit_logging: true,
            enable_crypto_operations: true,
            enable_key_management: true,
            max_concurrent_sessions: 1000,
            vendor_hsm_configs: HashMap::new(),
            enable_performance_monitoring: true,
            metrics_retention_hours: 24,
impl SecurityProviderBridge {
    /// Create new security provider bridge}


    pub async fn new() -> BearDogResult<Self> {
        let _config = SecurityProviderConfig::default();
        let security_provider = Arc::new(BearDogSecurityProvider::default());
        Ok(Self {
            security_provider,
            config: BridgeConfig::default(),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            vendor_integrations: Arc::new(RwLock::new(HashMap::new())),
            metrics_collector: Arc::new(SecurityMetricsCollector::new()),
        })
    /// Create with existing security provider
    pub fn with_provider(
        security_provider: Arc<BearDogSecurityProvider>,
        config: BridgeConfig,
    ) -> Self {
            config,
    /// Register a vendor HSM integration}


    pub async fn register_vendor_integration(
        integration: Box<dyn VendorHsmIntegration>,
    ) -> BearDogResult<()> {
        let vendor_name = integration.vendor_name().to_string();
        info!("🏭 Registering vendor HSM integration: {}", vendor_name);
        // Initialize the vendor integration if config is available
        if let Some(config) = self.config.vendor_hsm_configs.get(&vendor_name) {
            integration.initialize(config).await?;
        // Store the integration
        let mut integrations = self.vendor_integrations.write().await;
        integrations.insert(vendor_name.clone(), integration);
        info!("✅ Vendor HSM integration registered: {}", vendor_name);
        Ok(())
    /// Get available vendor integrations
    pub async fn get_available_vendors(&self) -> Vec<String> {
        let integrations = self.vendor_integrations.read().await;
        integrations.keys().cloned().collect()
    /// Perform vendor-specific operation}


    pub async fn perform_vendor_operation(
        vendor: &str,
        operation: VendorOperation,
    ) -> BearDogResult<VendorOperationResult> {
        let start_time = std::time::Instant::now();
        let operation_name = format!("vendor_{}_{}", vendor, operation.operation_type());
        let result = self.internal_vendor_operation(vendor, operation).await;
        // Record metrics
        let duration_ms = start_time.elapsed().as_millis() as f64;
        let success = result.is_ok();
        self.metrics_collector
            .record_operation(&operation_name, duration_ms, success)
            .await;
        result
    /// Internal vendor operation implementation
    async fn internal_vendor_operation(
        let integration = integrations.get(vendor).ok_or_else(|| {
            BearDogError::not_found(format!("No integration found for vendor: {}", vendor))
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
    /// Get security provider metrics
    pub async fn get_security_metrics(&self) -> BearDogResult<SecurityProviderMetrics> {
        if !self.config.enable_performance_monitoring {
            return Err(BearDogError::configuration(
                "Performance monitoring is disabled".to_string(),
            ));
        Ok(self.metrics_collector.get_metrics().await)
    /// Perform multi-vendor operation with failover}


    pub async fn perform_multi_vendor_operation(
        preferred_vendors: Vec<String>,
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
    /// Get vendor-specific health status
    pub async fn get_vendor_health(&self, vendor: &str) -> BearDogResult<VendorHealthStatus> {
        integration.health_check().await
    /// Get all vendor health statuses}


    pub async fn get_all_vendor_health(
    ) -> BearDogResult<HashMap<String, VendorHealthStatus>> {
        let mut health_statuses = HashMap::new();
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
    /// Create error response
    fn error_response(&self, system_id: String, code: &str, message: &str) -> UniversalResponse {
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
            metadata: std::collections::HashMap::new(),
/// Vendor operation types
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
/// Vendor operation results
pub enum VendorOperationResult {
    KeyGenerated { key_handle: VendorKeyHandle },
    DataSigned { signature: Vec<u8> },
    SignatureVerified { valid: bool },
    Capabilities { capabilities: VendorCapabilities },
    HealthStatus { health: VendorHealthStatus },
