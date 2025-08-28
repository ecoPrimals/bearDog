use beardog_errors::BearDogError;
use beardog_types::canonical::configuration::adapters::VendorHsmConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// UNIFIED: Use canonical BridgeConfig
pub use beardog_types::canonical::configuration::adapters::BridgeConfig;

#[derive(Debug, Clone)]
pub struct SecuritySession {
    pub session_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub user_id: Option<String>,
}

#[derive(Debug)]
pub enum VendorHsmIntegrationImpl {
    Mock(MockVendorHsmIntegration),
    // Add other vendor implementations as needed
}

impl VendorHsmIntegrationImpl {
    pub async fn initialize(&self, config: &VendorHsmConfig) -> Result<(), BearDogError> {
        match self {
            Self::Mock(impl_) => impl_.initialize(config).await,
        }
    }

    pub async fn generate_vendor_key(
        &self,
        key_spec: &VendorKeySpec,
    ) -> Result<VendorKeyHandle, BearDogError> {
        match self {
            Self::Mock(impl_) => impl_.generate_vendor_key(key_spec).await,
        }
    }

    pub async fn vendor_sign(
        &self,
        key_handle: &VendorKeyHandle,
        data: &[u8],
        algorithm: &str,
    ) -> Result<Vec<u8>, BearDogError> {
        match self {
            Self::Mock(impl_) => impl_.vendor_sign(key_handle, data, algorithm).await,
        }
    }

    pub async fn vendor_verify(
        &self,
        key_handle: &VendorKeyHandle,
        data: &[u8],
        signature: &[u8],
        algorithm: &str,
    ) -> Result<bool, BearDogError> {
        match self {
            Self::Mock(impl_) => {
                impl_
                    .vendor_verify(key_handle, data, signature, algorithm)
                    .await
            }
        }
    }

    pub async fn get_vendor_capabilities(&self) -> Result<VendorCapabilities, BearDogError> {
        match self {
            Self::Mock(impl_) => impl_.get_vendor_capabilities().await,
        }
    }

    pub async fn health_check(&self) -> Result<VendorHealthStatus, BearDogError> {
        match self {
            Self::Mock(impl_) => impl_.health_check().await,
        }
    }
}

#[derive(Debug)]
pub struct MockVendorHsmIntegration;

impl MockVendorHsmIntegration {
    pub async fn initialize(&self, _config: &VendorHsmConfig) -> Result<(), BearDogError> {
        Ok(())
    }

    pub async fn generate_vendor_key(
        &self,
        key_spec: &VendorKeySpec,
    ) -> Result<VendorKeyHandle, BearDogError> {
        Ok(VendorKeyHandle {
            handle_id: key_spec.key_id.clone(),
            key_type: "mock".to_string(),
            metadata: HashMap::new(),
        })
    }

    pub async fn vendor_sign(
        &self,
        _key_handle: &VendorKeyHandle,
        _data: &[u8],
        _algorithm: &str,
    ) -> Result<Vec<u8>, BearDogError> {
        Ok(vec![0x00, 0x01, 0x02, 0x03])
    }

    pub async fn vendor_verify(
        &self,
        _key_handle: &VendorKeyHandle,
        _data: &[u8],
        _signature: &[u8],
        _algorithm: &str,
    ) -> Result<bool, BearDogError> {
        Ok(true)
    }

    pub async fn get_vendor_capabilities(&self) -> Result<VendorCapabilities, BearDogError> {
        Ok(VendorCapabilities::default())
    }

    pub async fn health_check(&self) -> Result<VendorHealthStatus, BearDogError> {
        Ok(VendorHealthStatus::Healthy)
    }
}

// VendorHsmConfig already imported from beardog_types::canonical::configuration::adapters above

#[derive(Debug, Clone)]
pub enum VendorAuthConfig {
    Pin { pin: String },
    Certificate { cert_path: String, key_path: String },
    SmartCard { card_id: String, pin: String },
    NetworkAuth { username: String, password: String },
    Custom { auth_data: HashMap<String, String> },
}

#[derive(Debug, Clone)]
pub struct VendorKeySpec {
    pub key_id: String,
    pub key_type: VendorKeyType,
    pub usage: Vec<VendorKeyUsage>,
    pub security_level: VendorSecurityLevel,
}

#[derive(Debug, Clone)]
pub enum VendorKeyType {
    Rsa {
        size: u32,
        public_exponent: Option<u32>,
    },
    EccP256,
    EccP384,
    EccP521,
    Aes {
        key_size: usize,
    },
    Custom {
        algorithm: String,
        parameters: HashMap<String, String>,
    },
}

#[derive(Debug, Clone)]
pub enum VendorKeyUsage {
    Sign,
    Verify,
    Encrypt,
    Decrypt,
    KeyWrap,
    KeyUnwrap,
    Derive,
}

#[derive(Debug, Clone)]
pub enum VendorSecurityLevel {
    Software,
    HardwareBacked,
    Fips140Level2,
    Fips140Level3,
    CommonCriteriaEal4Plus,
}

#[derive(Debug, Clone)]
pub struct VendorKeyHandle {
    pub handle_id: String,
    pub key_type: String,
    pub metadata: std::collections::HashMap<String, String>,
}

impl Default for VendorKeyHandle {
    fn default() -> Self {
        Self {
            handle_id: String::new(),
            key_type: "unknown".to_string(),
            metadata: std::collections::HashMap::new(),
        }
    }
}

pub struct VendorKeyCapabilities {
    pub extractable: bool,
    pub hardware_backed: bool,
    pub attestation_supported: bool,
    pub user_presence_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VendorCapabilities {
    pub supported_algorithms: Vec<String>,
    pub key_types: Vec<String>,
    pub max_key_size: u32,
    pub hardware_backed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VendorHealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

impl Default for VendorHealthStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone, Default)]
pub struct SecurityMetricsCollector {
    pub sessions_created: u64,
    pub operations_performed: u64,
    pub errors_encountered: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionData {
    pub session_id: String,
    pub user_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub permissions: Vec<String>,
}

#[derive(Debug)]
pub struct SecurityProviderBridge {
    config: BridgeConfig,
    sessions: Arc<RwLock<HashMap<String, SessionData>>>,
    vendor_integrations: Arc<RwLock<HashMap<String, VendorHsmIntegrationImpl>>>,
    metrics_collector: Arc<SecurityMetricsCollector>,
}

impl SecurityProviderBridge {
    /// Get bridge configuration
    pub fn config(&self) -> &BridgeConfig {
        &self.config
    }

    /// Get active session count
    pub async fn session_count(&self) -> usize {
        self.sessions.read().await.len()
    }

    /// Get security metrics
    pub fn metrics(&self) -> &Arc<SecurityMetricsCollector> {
        &self.metrics_collector
    }
}

impl Default for SecurityProviderBridge {
    fn default() -> Self {
        Self {
            config: BridgeConfig {
                enabled: true,
                bridge_endpoint: "http://localhost:8080".to_string(),
                timeout: std::time::Duration::from_secs(30),
                retry_attempts: 3,
                max_sessions: 1000,
                session_timeout_seconds: 3600,
                enable_metrics: true,
                vendor_integrations_enabled: true,
            },
            sessions: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            vendor_integrations: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            metrics_collector: Arc::new(SecurityMetricsCollector::default()),
        }
    }
}

impl SecurityProviderBridge {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(config: BridgeConfig) -> Self {
        Self {
            config,
            sessions: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            vendor_integrations: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            metrics_collector: Arc::new(SecurityMetricsCollector::default()),
        }
    }

    pub async fn add_vendor_integration(
        &self,
        vendor: String,
        integration: VendorHsmIntegrationImpl,
    ) -> Result<(), BearDogError> {
        let mut integrations = self.vendor_integrations.write().await;
        integrations.insert(vendor, integration);
        Ok(())
    }
}

#[derive(Debug)]
pub enum VendorOperation {
    GenerateKey {
        key_spec: VendorKeySpec,
    },
    Sign {
        key_handle: VendorKeyHandle,
        data: Vec<u8>,
    },
    Verify {
        key_handle: VendorKeyHandle,
        data: Vec<u8>,
        signature: Vec<u8>,
    },
    GetCapabilities,
    HealthCheck,
}

impl VendorOperation {
    pub fn operation_type(&self) -> &'static str {
        match self {
            VendorOperation::GenerateKey { .. } => "generate_key",
            VendorOperation::Sign { .. } => "sign",
            VendorOperation::Verify { .. } => "verify",
            VendorOperation::GetCapabilities => "get_capabilities",
            VendorOperation::HealthCheck => "health_check",
        }
    }
}

pub enum VendorOperationResult {
    KeyGenerated { key_handle: VendorKeyHandle },
    DataSigned { signature: Vec<u8> },
    SignatureVerified { valid: bool },
    Capabilities { capabilities: VendorCapabilities },
    HealthStatus { health: VendorHealthStatus },
}
