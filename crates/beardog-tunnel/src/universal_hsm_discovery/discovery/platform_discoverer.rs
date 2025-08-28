

use super::super::*;
use beardog_errors::BearDogError;
use tracing::{debug, info};

#[derive(Debug)]
pub struct PlatformDiscoverer;
impl PlatformDiscoverer {}

    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self)
    }
    pub async fn discover(&self, config: &DiscoveryConfig) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        debug!("💻 Discovering Platform HSMs");
        let mut hsms = Vec::new();

        if let Ok(tpm_hsm) = self.discover_tpm().await {
            hsms.push(tpm_hsm);
        }

        if config.enable_platform_discovery {
            if let Ok(platform_hsms) = self.discover_platform_hsms().await {
                hsms.extend(platform_hsms);
            }
        info!("Found {} Platform HSMs", hsms.len());
        Ok(hsms)

    async fn discover_tpm(&self) -> Result<DiscoveredHsm, BearDogError> {
        debug!("🔍 Checking for TPM 2.0");

        let tpm_paths = ["/dev/tpm0", "/dev/tpmrm0"];
        for path in &tpm_paths {
            if std::path::Path::new(path).exists() {
                return Ok(DiscoveredHsm {
                    id: "tpm2".to_string(),
                    name: "TPM 2.0".to_string(),
                    hsm_type: HsmType::Platform,
                    connection_info: ConnectionInfo::Platform {
                        device_path: path.to_string(),
                    },
                    capabilities: vec![
                        HsmCapability::KeyGeneration,
                        HsmCapability::Signing,
                        HsmCapability::RandomGeneration,
                    ],
                    status: HsmStatus::Available,
                });
        Err(BearDogError::not_found("TPM 2.0 device not found"))

    async fn discover_platform_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        debug!("🔍 Checking for platform-specific HSMs");

        if self.check_intel_txt().await {
            hsms.push(DiscoveredHsm {
                id: "intel_txt".to_string(),
                name: "Intel TXT".to_string(),
                hsm_type: HsmType::Platform,
                connection_info: ConnectionInfo::Platform {
                    device_path: "/dev/txt".to_string(),
                },
                capabilities: vec![HsmCapability::Attestation],
                status: HsmStatus::Available,
            });

    async fn check_intel_txt(&self) -> bool {

        std::path::Path::new("/sys/kernel/security/tpm0").exists()
}
