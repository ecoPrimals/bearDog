

use super::super::*;
use crate::tunnel::hsm::types::{
    capability::HsmCapabilities, status::HsmHealthStatus, tier::HsmTier, AuthenticationMethod,
    HsmConnectionInfo, HsmInterfaceType,
};
use beardog_errors::BearDogResult;
use beardog_types::config::network::security::SslConfig;
use std::path::Path;
use std::time::Duration;
use tracing::{debug, info};

#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
}
pub type ConnectionType = String;

#[derive(Debug)]
pub struct CloudDiscoverer;
impl CloudDiscoverer {}

    pub fn new() -> BearDogResult<Self> {
        Ok(Self)
    }
    pub async fn discover(&self, config: &DiscoveryConfig) -> BearDogResult<Vec<DiscoveredHsm>> {
        debug!("☁️ Discovering Cloud HSMs");
        let mut hsms = Vec::new();

        if let Ok(aws_hsms) = self.discover_aws_kms(config).await {
            hsms.extend(aws_hsms);
        }

        if let Ok(azure_hsms) = self.discover_azure_kv(config).await {
            hsms.extend(azure_hsms);

        if let Ok(gcp_hsms) = self.discover_gcp_kms(config).await {
            hsms.extend(gcp_hsms);
        info!("Found {} Cloud HSMs", hsms.len());
        Ok(hsms)
    async fn discover_aws_kms(
        &self,
        config: &DiscoveryConfig,
    ) -> BearDogResult<Vec<DiscoveredHsm>> {
        debug!("🔍 Checking for AWS KMS availability");

        if !self.has_aws_credentials() {
            return Ok(Vec::new());
        let regions = vec!["us-east-1", "us-west-2", "eu-west-1", "ap-southeast-1"];
        for region in regions {
            let hsm = DiscoveredHsm {
                hsm_id: format_args!("aws-cloudhsm-{}", region).to_string(),
                name: format_args!("AWS CloudHSM ({})", region).to_string(),
                hsm_type: crate::universal_hsm_discovery::HsmType::Cloud,
                endpoint: crate::universal_hsm_discovery::HsmEndpoint {
                    address: format_args!("cloudhsm.{}.amazonaws.com", region).to_string(),
                    port: Some(443),
                    protocol: "https".to_string(),
                    secure: true,
                },
                capabilities: HsmCapabilities::default(),
                assigned_tier: HsmTier::CertifiedHardware,
                supports_human_entropy: false,
                health_status: HsmHealthStatus::healthy(),
                discovered_at: chrono::Utc::now(),
                last_health_check: chrono::Utc::now(),
                integration_status: IntegrationStatus::Discovered,
            };
            hsms.push(hsm);
    async fn discover_azure_kv(
        debug!("🔍 Checking for Azure Key Vault availability");
        if !self.has_azure_credentials() {
        let mut discovered_hsms = Vec::new();

        for region in &["eastus", "westus2", "northeurope"] {
                hsm_id: format_args!("azure-keyvault-{}", region).to_string(),
                name: format_args!("Azure Key Vault ({})", region).to_string(),
                    address: format_args!("{}.vault.azure.net", region).to_string(),
            discovered_hsms.push(hsm);

        for region in &["us-central1", "europe-west1", "asia-northeast1"] {
                hsm_id: format_args!("gcp-kms-{}", region).to_string(),
                name: format_args!("Google Cloud KMS ({})", region).to_string(),
                    address: format!("cloudkms.googleapis.com"),
                assigned_tier: HsmTier::HighSecurity,
        Ok(discovered_hsms)
    async fn discover_gcp_kms(
        debug!("🔍 Checking for Google Cloud KMS availability");
        if !self.has_gcp_credentials() {
        let project_locations = vec![
            ("example-project", "global"),
            ("example-project", "us-central1"),
        ];
        for (project_id, location) in project_locations {
                hsm_id: format_args!("gcp-kms-{}-{}", project_id, location).to_string(),
                vendor: "Google Cloud Platform".to_string(),
                model: "Google Cloud KMS".to_string(),
                version: "Current".to_string(),
                interface_type: HsmInterfaceType::GcpKms {
                    project_id: project_id.to_string(),
                    location: location.to_string(),
                connection_info: HsmConnectionInfo {
                    connection_type: ConnectionType::Cloud,
                    authentication: AuthenticationMethod::Token {
                        token: "GCP Service Account".to_string(),
                    },
                    endpoint: Some("cloudkms.googleapis.com".to_string()),
                    timeout: config.timeout,
                    retry_policy: RetryPolicy {
                        max_retries: 3,
                        base_delay: Duration::from_millis(500),
                        max_delay: Duration::from_secs(30),
                        backoff_multiplier: 2.0,
                    ssl_config: Some(SslConfig {
                        ca_cert_path: None,
                        verify_hostname: true,
                        min_tls_version: "1.2".to_string(),
                        cipher_suites: vec!["ECDHE-RSA-AES256-GCM-SHA384".to_string()],
                    }),
                discovery_timestamp: chrono::Utc::now(),
    fn has_aws_credentials(&self) -> bool {

        std::env::var("AWS_ACCESS_KEY_ID").is_ok()
            || std::env::var("AWS_PROFILE").is_ok()
            || Path::new(&format!(
                "{}/.aws/credentials",
                std::env::var("HOME").unwrap_or_default()
            ))
            .exists()
    fn has_azure_credentials(&self) -> bool {

        std::env::var("AZURE_CLIENT_ID").is_ok() || std::env::var("AZURE_TENANT_ID").is_ok()}

    fn has_gcp_credentials(&self) -> bool {

        std::env::var("GOOGLE_APPLICATION_CREDENTIALS").is_ok()
            || std::env::var("GCLOUD_PROJECT").is_ok()
