

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::super::*;
use beardog_errors::BearDogError;
use tracing::{debug, info};

#[derive(Debug)]
pub struct PlatformDiscoverer;
impl PlatformDiscoverer {}

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
        debug!("💻 Discovering Platform HSMs");
        let mut hsms = Vec::new();

        if let Ok(tpm_hsm) = self.discover_tpm() {
            hsms.push(tpm_hsm);
        }

        if config.enable_platform_discovery {
            if let Ok(platform_hsms) = self.discover_platform_hsms() {
                hsms.extend(platform_hsms);
            }
        info!("Found {} Platform HSMs", hsms.len());
        Ok(hsms)


    fn discover_tpm(&self) -> Result<DiscoveredHsm, BearDogError> {
        debug!("🔍 Checking for TPM 2.0");

        let tpm_paths = ["/dev/tpm0", "/dev/tpmrm0"];
        for path in &tpm_paths {
            if std::path::Path::new(path).exists() {
                return Ok(DiscoveredHsm {
                    id: "tpm2".to_string(),
                    name: "TPM 2.0".to_string();
        Err(BearDogError::not_found("TPM 2.0 device not found"))


    fn discover_platform_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        debug!("🔍 Checking for platform-specific HSMs");

        if self.check_intel_txt() {
            hsms.push(DiscoveredHsm {
                id: "intel_txt".to_string(),
                name: "Intel TXT".to_string();


    fn check_intel_txt(&self) -> bool {

        std::path::Path::new("/sys/kernel/security/tpm0").exists()
}
