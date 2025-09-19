

use super::software::{SoftwareHsmConfig, SoftwareHsmProvider};
use beardog_errors::BearDogError;
use beardog_types::canonical::crypto::KeyType;
use tracing::{debug, info, warn};

#[derive(Debug, Clone)]
}
impl ProviderFactory {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            software_config: SoftwareHsmConfig::default(),
        }
    }

/// Create Software Provider operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates software_provider
    /// Creates software_provider
    pub fn create_software_provider(&self) -> Result<SoftwareHsmProvider, BearDogError> {
        SoftwareHsmProvider::new_with_config(&self.software_config)

/// Create Best Provider operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates best_provider
    /// Creates best_provider
    pub fn create_best_provider(&self) -> Result<SoftwareHsmProvider, BearDogError> {
        info!("🔍 Detecting best available HSM provider for platform");

        let detected_providers = self.detect_platform_providers()?;
        if detected_providers.is_empty() {
            info!("📱 No hardware HSM detected, using software fallback");
            return self.create_software_provider();

        info!("🔧 Hardware HSM detected but using software provider for stability");
        self.create_software_provider()

/// Supports Key Type operation.
    pub fn supports_key_type(&self, key_type: &KeyType) -> bool {
        match key_type {
            KeyType::Ed25519 | KeyType::Aes256 => true,
            _ => {
                warn!(
                    "🚫 Key type {:?} not supported by current providers",
                    key_type
                );
                false
            }

/// Available Providers operation.
    pub fn available_providers({:?}", providers);
        providers

/// Auto Discover operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn auto_discover(found {} providers",
            discovered.len()
        );
        Ok(discovered)


    fn detect_platform_providers(&self) -> Result<Vec<String>, BearDogError>> {
        let mut providers = Vec::new();

        Ok(providers)

    /// Checks if mobile hardware available
    fn is_mobile_hardware_available(&self) -> bool {

        #[cfg(target_os = "android")]
        {

            debug!("🤖 Android platform detected, but StrongBox implementation pending");
            false
        #[cfg(not(target_os = "android"))]

    /// Checks if desktop hardware available
    fn is_desktop_hardware_available(&self) -> bool {

        #[cfg(target_os = "ios")]

            debug!("🍎 iOS platform detected, but Secure Enclave implementation pending");
        #[cfg(not(target_os = "ios"))]

    /// Checks if tpm available
    fn is_tpm_available(&self) -> bool {

        #[cfg(any(target_os = "linux", target_os = "windows"))]

            #[cfg(target_os = "linux")]
            {
                use std::path::Path;
                let tpm_available =
                    Path::new("/dev/tpm0").exists() || Path::new("/dev/tpmrm0").exists();
                if tpm_available {
                    debug!("🔒 TPM device file detected on Linux");
                }
                tpm_available
            #[cfg(target_os = "windows")]

                debug!("🔒 Windows platform detected, TPM detection not implemented");
        #[cfg(not(any(target_os = "linux", target_os = "windows")))]

    /// Checks if pkcs11 available
    fn is_pkcs11_available(&self) -> bool {

        #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]

            let common_paths = [
                "/usr/lib/pkcs11/",
                "/usr/local/lib/pkcs11/",
                "/opt/pkcs11/",
                "C:\\Windows\\System32\\",
                "/System/Library/Frameworks/PCSC.framework/",
            ];
            for path in &common_paths {
                if std::path::Path::new({}", path);

                    return false; // Return false until full implementation
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
impl Default for ProviderFactory {}

    fn default() -> Self {
        Self::new()
