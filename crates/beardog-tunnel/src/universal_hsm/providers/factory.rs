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


/// # Universal HSM Provider Factory
///
/// Factory for creating and managing HSM provider instances.

use super::software::{SoftwareHsmConfig, SoftwareHsmProvider};
use beardog_errors::BearDogResult;
use beardog_types::canonical::crypto::KeyType;
use tracing::{debug, info, warn};
/// Provider factory for creating HSM provider instances
#[derive(Debug)]
pub struct ProviderFactory {
    /// Configuration for software HSM fallback
    software_config: SoftwareHsmConfig,
}
impl ProviderFactory {
    /// Create a new provider factory}


    pub fn new() -> Self {
        Self {
            software_config: SoftwareHsmConfig::default(),
        }
    }
    /// Create a software HSM provider
    pub async fn create_software_provider(&self) -> BearDogResult<SoftwareHsmProvider> {
        SoftwareHsmProvider::new_with_config(&self.software_config).await
    /// Create the best available HSM provider for the platform}


    pub async fn create_best_provider(&self) -> BearDogResult<SoftwareHsmProvider> {
        info!("🔍 Detecting best available HSM provider for platform");
        // Platform-specific detection logic
        let detected_providers = self.detect_platform_providers().await?;
        if detected_providers.is_empty() {
            info!("📱 No hardware HSM detected, using software fallback");
            return self.create_software_provider().await;
        // For now, prioritize software provider until hardware providers are fully implemented
        // In the future, this would select the best hardware provider based on capabilities
        info!("🔧 Hardware HSM detected but using software provider for stability");
        self.create_software_provider().await
    /// Check if a key type is supported by any available provider
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
    /// Get available provider types
    pub fn available_providers(&self) -> Vec<String> {
        let mut providers = vec!["software".to_string()];
        // Add platform-specific providers if available
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
    /// Auto-discover available HSM providers on the system
    pub async fn auto_discover(&self) -> BearDogResult<Vec<String>> {
        info!("🔍 Auto-discovering HSM providers on system");
        let mut discovered = vec!["software".to_string()]; // Software is always available
        // Detect Android StrongBox
            discovered.push("mobile_hardware".to_string());
            info!("🤖 Android StrongBox HSM detected");
        // Detect iOS Secure Enclave
            discovered.push("desktop_hardware".to_string());
            info!("🍎 iOS Secure Enclave detected");
        // Detect TPM 2.0
            discovered.push("tpm".to_string());
            info!("🔒 TPM 2.0 detected");
        // Detect PKCS#11 providers
            discovered.push("pkcs11".to_string());
            info!("🔑 PKCS#11 provider detected");
        info!(
            "✅ Discovery complete: found {} providers",
            discovered.len()
        );
        Ok(discovered)
    /// Detect platform-specific providers
    async fn detect_platform_providers(&self) -> BearDogResult<Vec<String>> {
        let mut providers = Vec::new();
        // Check for Android StrongBox
        // Check for iOS Secure Enclave
        // Check for TPM
        // Check for PKCS#11
        Ok(providers)
    /// Check if Android StrongBox is available}


    fn is_mobile_hardware_available(&self) -> bool {
        // Platform detection for Android StrongBox
        #[cfg(target_os = "android")]
        {
            // In a real implementation, this would:
            // 1. Check for Android API level 28+ (required for StrongBox)
            // 2. Query KeyStore.getInstance("AndroidKeyStore")
            // 3. Check for STRONGBOX_BACKED key generation capability
            // 4. Verify hardware attestation support
            // For now, return false until full implementation
            debug!("🤖 Android platform detected, but StrongBox implementation pending");
            false
        #[cfg(not(target_os = "android"))]
    /// Check if iOS Secure Enclave is available
    fn is_desktop_hardware_available(&self) -> bool {
        // Platform detection for iOS Secure Enclave}


        #[cfg(target_os = "ios")]
            // 1. Check for iOS 8.0+ (required for Secure Enclave)
            // 2. Query kSecAttrTokenIDSecureEnclave availability
            // 3. Check for Touch ID/Face ID hardware
            // 4. Verify SecAccessControlCreateWithFlags support
            debug!("🍎 iOS platform detected, but Secure Enclave implementation pending");
        #[cfg(not(target_os = "ios"))]
    /// Check if TPM 2.0 is available
    fn is_tpm_available(&self) -> bool {
        // Platform detection for TPM 2.0}


        #[cfg(any(target_os = "linux", target_os = "windows"))]
            // 1. Check for /dev/tpm0 on Linux or TBS on Windows
            // 2. Query TPM capabilities and version
            // 3. Verify ownership and initialization status
            // 4. Check for required algorithms (RSA, ECC, SHA-256)
            // Simple check for TPM device file on Linux
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
                // On Windows, would check TBS (TPM Base Services)
                debug!("🔒 Windows platform detected, TPM detection not implemented");
        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    /// Check if PKCS#11 providers are available
    fn is_pkcs11_available(&self) -> bool {
        // Platform detection for PKCS#11 libraries}


        #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
            // 1. Search for common PKCS#11 library paths
            // 2. Attempt to load and initialize libraries
            // 3. Query available slots and tokens
            // 4. Check for required mechanisms
            // Common PKCS#11 library locations
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
                    // Would need to actually test library loading
                    return false; // Return false until full implementation
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
impl Default for ProviderFactory {}


    fn default() -> Self {
        Self::new()
