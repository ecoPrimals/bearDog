

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::super::*;
use beardog_errors::BearDogError;
use beardog_types::canonical::hsm::*;
use std::collections::HashMap;
use tracing::{debug, info};

#[derive(Debug)]
pub struct NetworkDiscoverer;
impl NetworkDiscoverer {}

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self)
    }
/// Discover operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover(&self, config: &DiscoveryConfig) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        debug!("🌐 Discovering Network HSMs");
        let mut hsms = Vec::new();

        hsms.extend(self.discover_pkcs11_hsms(config)?);

        hsms.extend(self.discover_rest_hsms(config)?);

        hsms.extend(self.discover_grpc_hsms(config)?);
        info!("Found {} Network HSMs", hsms.len());
        Ok(hsms)


    fn discover_pkcs11_hsms(&self, config: &DiscoveryConfig) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        debug!("Discovering PKCS#11 Network HSMs");

        let endpoints = vec![
            ("SafeNet Luna", "tcp://192.168.1.100:1792"),
            ("Thales nShield", "tcp://192.168.1.101:9004"),
            ("Utimaco HSM", "tcp://192.168.1.102:8080"),
        ];
        for (name, endpoint) in endpoints {
            if self.probe_pkcs11_endpoint(format!("{} Network HSM", name),
                    hsm_type: HsmType::Hardware,
                    provider: HsmProvider::NetworkPKCS11,
                    capabilities: self.get_network_pkcs11_capabilities(),
                    connection_info: HsmConnectionInfo {
                        endpoint: endpoint.to_string(),
                        authentication: Some("PKCS#11 PIN + Client Certificate".to_string()),
                        tls_config: Some(HsmHealthStatus::Available,
                    metadata: std::collections::HashMap::from([
                        ("protocol".to_string(), "pkcs11".to_string()),
                        ("vendor".to_string(), name.to_lowercase().replace(" ", "_")),
                    ]),
                });
            }
        }


    fn discover_rest_hsms(&self, config: &DiscoveryConfig) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        debug!("Discovering REST API HSMs");

            ("universal_cloud universal_hsm", "https://universal_hsm.us-east-1.amazonuniversal_cloud.com"),
            ("universal_cloud Key Vault", "https://vault.universal_cloud.net"),
            ("HashiCorp Vault", "https://vault.example.com:8200"),
            if self.probe_rest_endpoint(format!("{} REST HSM", name),
                    hsm_type: HsmType::Cloud,
                    provider: HsmProvider::RestApi,
                    capabilities: self.get_rest_hsm_capabilities(),
                        authentication: Some("API Key + OAuth".to_string()),
                        ("protocol".to_string(), "https".to_string()),
                        ("provider".to_string(), name.to_lowercase().replace(" ", "_")),


    fn discover_grpc_hsms(&self, config: &DiscoveryConfig) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        debug!("Discovering gRPC HSMs");

            ("Google Cloud KMS", "grpc://cloudkms.googleapis.com:443"),
            ("Custom HSM Service", "grpc://hsm.internal.com:9090"),
            if self.probe_grpc_endpoint(format!("{} gRPC HSM", name),
                    provider: HsmProvider::GRPC,
                    capabilities: self.get_grpc_hsm_capabilities(),
                        authentication: Some("mTLS + Service Account".to_string()),
                        tls_config: Some("TLS 1.3 + mTLS".to_string()),
                        ("protocol".to_string(), "grpc".to_string()),


    fn probe_pkcs11_endpoint(&self, endpoint: &str) -> Result<bool, BearDogError> {
        debug!("Probing PKCS#11 endpoint: {}", endpoint);

        Ok(false)


    fn probe_rest_endpoint(&self, endpoint: &str) -> Result<bool, BearDogError> {
        debug!("Probing REST endpoint: {}", endpoint);


    fn probe_grpc_endpoint(&self, endpoint: &str) -> Result<bool, BearDogError> {
        debug!("Probing gRPC endpoint: {}", endpoint);

    /// Gets network_pkcs11_capabilities
    fn get_network_pkcs11_capabilities(&self) -> HsmCapabilities {
        HsmCapabilities {
            supported_algorithms: vec![
                "AES-256-GCM".to_string(),
            hardware_backed: true,
            fips_certified: true,
            cc_certified: true,
            supports_key_generation: true,
            supports_key_import: true,
            supports_attestation: true,

    /// Gets rest_hsm_capabilities
    fn get_rest_hsm_capabilities(false,
            supports_key_import: false,
            supports_attestation: false,

    /// Gets grpc_hsm_capabilities
    fn get_grpc_hsm_capabilities(2048,
}
