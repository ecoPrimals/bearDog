

use super::software::{SoftwareHsmConfig, SoftwareHsmProvider};
use beardog_errors::BearDogError;
use beardog_types::canonical::crypto::KeyType;
use tracing::{debug, info, warn};

#[derive(Debug)]
pub struct ProviderFactory {

    software_config: SoftwareHsmConfig,
}
impl ProviderFactory {

    pub fn new() -> Self {
        Self {
            software_config: SoftwareHsmConfig::default(),
        }
    }

    pub async fn create_software_provider(&self) -> Result<SoftwareHsmProvider, BearDogError> {
        SoftwareHsmProvider::new_with_config(&self.software_config).await

    pub async fn create_best_provider(&self) -> Result<SoftwareHsmProvider, BearDogError> {
        info!("🔍 Detecting best available HSM provider for platform");

        let detected_providers = self.detect_platform_providers().await?;
        if detected_providers.is_empty() {
            info!("📱 No hardware HSM detected, using software fallback");
            return self.create_software_provider().await;

        info!("🔧 Hardware HSM detected but using software provider for stability");
        self.create_software_provider().await

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

    pub fn available_providers(&self) -> Vec<String> {
        let mut providers = vec!["software".to_string()];

        if self.is_mobile_hardware_available() {
            providers.push("mobile_hardware".to_string());
        if self.is_desktop_hardware_available() {
            providers.push("desktop_hardware".to_string());
        if self.is_tpm_available() {
            providers.push("tpm".to_string());
        if self.is_pkcs11_available() {
            providers.push("pkcs11".to_string());
        debug!("📋 Available HSM providers: {:?}", providers);
        providers

    pub async fn auto_discover(&self) -> Result<Vec<String>, BearDogError>> {
        info!("🔍 Auto-discovering HSM providers on system");
        let mut discovered = vec!["software".to_string()]; // Software is always available

            discovered.push("mobile_hardware".to_string());
            info!("🤖 Android StrongBox HSM detected");

            discovered.push("desktop_hardware".to_string());
            info!("🍎 iOS Secure Enclave detected");

            discovered.push("tpm".to_string());
            info!("🔒 TPM 2.0 detected");

            discovered.push("pkcs11".to_string());
            info!("🔑 PKCS#11 provider detected");
        info!(
            "✅ Discovery complete: found {} providers",
            discovered.len()
        );
        Ok(discovered)

    async fn detect_platform_providers(&self) -> Result<Vec<String>, BearDogError>> {
        let mut providers = Vec::new();

        Ok(providers)

    fn is_mobile_hardware_available(&self) -> bool {

        #[cfg(target_os = "android")]
        {

            debug!("🤖 Android platform detected, but StrongBox implementation pending");
            false
        #[cfg(not(target_os = "android"))]

    fn is_desktop_hardware_available(&self) -> bool {

        #[cfg(target_os = "ios")]

            debug!("🍎 iOS platform detected, but Secure Enclave implementation pending");
        #[cfg(not(target_os = "ios"))]

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
                if std::path::Path::new(path).exists() {
                    debug!("🔑 PKCS#11 library path found: {}", path);

                    return false; // Return false until full implementation
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
impl Default for ProviderFactory {}

    fn default() -> Self {
        Self::new()
