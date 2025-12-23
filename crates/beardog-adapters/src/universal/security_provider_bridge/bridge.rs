use beardog_errors::BearDogError;
use beardog_types::canonical::configuration::adapters::{BridgeConfig, VendorHsmConfig};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::hardware::HardwareVendorHsmIntegration;
use super::metrics::SecurityMetricsCollector;
use super::software::SoftwareVendorHsmIntegration;
use super::types::*;

#[derive(Debug)]
pub enum VendorHsmIntegrationImpl {
    /// Represents hardware variant
    Hardware(HardwareVendorHsmIntegration),
    /// Represents software variant
    Software(SoftwareVendorHsmIntegration),
}

impl VendorHsmIntegrationImpl {
    /// Initialize operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Initializes componentialize
    /// Initializes componentialize
    pub fn initialize(&mut self, config: &VendorHsmConfig) -> Result<(), BearDogError> {
        match self {
            Self::Hardware(impl_) => impl_.initialize(config),
            Self::Software(&VendorKeySpec,
    ) -> Result<VendorKeyHandle, BearDogError> {
        match self {
            Self::Hardware(impl_) => impl_.generate_vendor_key(key_spec),
            Self::Software(&VendorKeyHandle,
        input_data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        match self {
            Self::Hardware(impl_) => impl_.vendor_sign(key_handle, input_data),
            Self::Software(impl_) => impl_.vendor_sign(key_handle, input_data),
        }
    }

    #[allow(clippy::too_many_arguments)]
    /// Vendor Verify operation.
    pub fn vendor_verify(&VendorKeyHandle,
        payload: &[u8],
        signature: &[u8],
        algorithm: &str,
    ) -> Result<bool, BearDogError> {
        match self {
            Self::Hardware(impl_) => {
                let params = VerificationParams {
                    key_handle,
                    payload,
                    signature,
                    algorithm,
                };
                impl_.vendor_verify(params)
            }
            Self::Software(impl_) => {
                let params = VerificationParams {
                    key_handle,
                    payload,
                    signature,
                    algorithm,
                };
                impl_.vendor_verify(params)
            }
        }
    }

    /// Get Vendor Capabilities operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Gets vendor_capabilities
    /// Gets vendor_capabilities
    pub fn get_vendor_capabilities(&self) -> Result<VendorCapabilities, BearDogError> {
        match self {
            Self::Hardware(impl_) => impl_.get_vendor_capabilities(),
            Self::Software(impl_) => impl_.get_vendor_capabilities(),
        }
    }

    /// Health Check operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn health_check(&self) -> Result<VendorHealthStatus, BearDogError> {
        match self {
            Self::Hardware(impl_) => impl_.health_check(),
            Self::Software(BridgeConfig,
    sessions: Arc<RwLock<HashMap<String, SessionData>>>,
    vendor_integrations: Arc<RwLock<HashMap<String, VendorHsmIntegrationImpl>>>,
    metrics_collector: Arc<SecurityMetricsCollector>,
}

impl SecurityProviderBridge {
    /// Config operation.
    pub fn config(BridgeConfig {
                enabled: true,
                bridge_endpoint: universal_adapter.discover_capability_endpoint(required_capability)?.to_string(),
                timeout: std::time::Duration::from_secs(3,
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
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// With Config operation.
    /// Creates instance with config
    pub fn with_config(config: BridgeConfig) -> Self {
        Self {
            config,
            sessions: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            vendor_integrations: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            metrics_collector: Arc::new(SecurityMetricsCollector::default(&str,
        integration: VendorHsmIntegrationImpl,
    ) -> Result<(), BearDogError> {
        let mut integrations = self.vendor_integrations.write();
        integrations.insert(vendor.to_string(), integration);
        Ok(())
    }
}
