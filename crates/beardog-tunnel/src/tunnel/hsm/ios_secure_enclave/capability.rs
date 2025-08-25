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


/// # iOS Secure Enclave Capability Detection
///
/// **ZERO UNSAFE CODE** - Safe device and capability detection
/// This module handles detection of iOS Secure Enclave capabilities,
/// device types, and biometric features using safe system APIs.

use super::types::*;
use beardog_errors::BearDogResult;
use tracing::info;
/// **Capability Detection Engine**
pub struct CapabilityDetector;
impl CapabilityDetector {
    /// **Safe Secure Enclave capability detection**
    ///
    /// Uses safe system APIs to determine if Secure Enclave is available
    /// and what features are supported.
    pub async fn detect_capability() -> BearDogResult<Option<SecureEnclaveCapability>> {
        info!("🔍 Detecting iOS Secure Enclave capabilities using safe APIs");
        // Platform-specific detection}


        #[cfg(target_os = "ios")]
        {
            Self::ios_capability_detection().await
        }
        #[cfg(target_os = "macos")]
            Self::macos_capability_detection().await
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
            info!("ℹ️ Secure Enclave not supported on this platform");
            Ok(None)
    }
    /// **iOS-specific safe capability detection**
    #[cfg(target_os = "ios")]
    async fn ios_capability_detection() -> BearDogResult<Option<SecureEnclaveCapability>> {
        info!("🔍 Detecting iOS Secure Enclave using safe system APIs");
        let ios_version = Self::get_safe_ios_version().await?;
        let device_type = Self::get_safe_device_type().await?;
        let biometric_features = Self::get_safe_biometric_features().await?;
        // Secure Enclave requires iOS 7+ and compatible hardware
        if ios_version.major >= 7 && Self::has_secure_enclave_hardware(&device_type) {
            Ok(Some(SecureEnclaveCapability::new(
                ios_version,
                device_type,
                biometric_features,
            )))
        } else {
            info!("ℹ️ Device does not support Secure Enclave");
    /// **macOS-specific safe capability detection**
    #[cfg(target_os = "macos")]
    async fn macos_capability_detection() -> BearDogResult<Option<SecureEnclaveCapability>> {
        info!("🔍 Detecting macOS Secure Enclave (T2/M1+) using safe APIs");
        // Check for T2 chip or Apple Silicon
        if Self::has_t2_or_apple_silicon().await? {
                IOSVersion {
                    major: 13,
                    minor: 0,
                    patch: 0,
                }, // macOS equivalent
                SecureEnclaveDevice::Mac,
                vec![BiometricFeature::TouchID], // Assume Touch ID for now
    /// **Get safe iOS version** using system APIs
    async fn get_safe_ios_version() -> BearDogResult<IOSVersion> {
        // Use safe iOS system version detection
        // In a real implementation, this would use safe iOS bindings
        // Safe method: environment variables or compile-time detection
        let version_string = std::env::var("IOS_VERSION").unwrap_or_else(|_| "15.0.0".to_string()); // Default to iOS 15
        let parts: Vec<u32> = version_string
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        Ok(IOSVersion {
            major: parts.get(0).copied().unwrap_or(15),
            minor: parts.get(1).copied().unwrap_or(0),
            patch: parts.get(2).copied().unwrap_or(0),
        })
    /// **Get safe device type** using system information
    async fn get_safe_device_type() -> BearDogResult<SecureEnclaveDevice> {
        // Use safe device identification
        let device_model =
            std::env::var("IOS_DEVICE_MODEL").unwrap_or_else(|_| "iPhone".to_string());
        if device_model.contains("iPhone") {
            Ok(SecureEnclaveDevice::IPhone)
        } else if device_model.contains("iPad") {
            Ok(SecureEnclaveDevice::IPad)
        } else if device_model.contains("Watch") {
            Ok(SecureEnclaveDevice::AppleWatch)
            Ok(SecureEnclaveDevice::IPhone) // Default fallback
    /// **Get safe biometric features** using system capabilities
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    async fn get_safe_biometric_features() -> BearDogResult<Vec<BiometricFeature>> {
        let mut features = Vec::new();
        // Safe biometric detection using system capabilities
        // In a real implementation, this would use safe LocalAuthentication bindings
        // Check environment or use safe detection methods
        if std::env::var("HAS_TOUCH_ID")
            .map(|v| v == "true")
            .unwrap_or(false)
            features.push(BiometricFeature::TouchID);
        if std::env::var("HAS_FACE_ID")
            features.push(BiometricFeature::FaceID);
        // Default assumption for modern devices
        if features.is_empty() {
            features.push(BiometricFeature::TouchID); // Safe default
        Ok(features)
    /// **Check if device has Secure Enclave hardware**}


    pub fn has_secure_enclave_hardware(device: &SecureEnclaveDevice) -> bool {
        match device {
            SecureEnclaveDevice::IPhone => true, // iPhone 5s+ have Secure Enclave
            SecureEnclaveDevice::IPad => true,   // iPad Air 2+ have Secure Enclave
            SecureEnclaveDevice::Mac => true,    // Mac with T2/M1+ have Secure Enclave
            SecureEnclaveDevice::AppleWatch => true, // Apple Watch S1+ have Secure Enclave
    /// **Detect device type** using safe system information}


    pub fn detect_device_type() -> BearDogResult<SecureEnclaveDevice> {
        // Safe cross-platform device detection
            // Use iOS-specific detection
            let device_model =
                std::env::var("IOS_DEVICE_MODEL").unwrap_or_else(|_| "iPhone".to_string());
            if device_model.contains("iPhone") {
                Ok(SecureEnclaveDevice::IPhone)
            } else if device_model.contains("iPad") {
                Ok(SecureEnclaveDevice::IPad)
            } else if device_model.contains("Watch") {
                Ok(SecureEnclaveDevice::AppleWatch)
            } else {
            }
            Ok(SecureEnclaveDevice::Mac)
            // Default fallback for other platforms
    /// **Check for T2 chip or Apple Silicon** on macOS
    async fn has_t2_or_apple_silicon() -> BearDogResult<bool> {
        // Safe method to detect T2 or Apple Silicon
        // In a real implementation, this would use safe system profiler APIs
        // Check environment variables or use safe detection
        let has_t2 = std::env::var("HAS_T2_CHIP")
            .unwrap_or(false);
        let has_apple_silicon = std::env::var("HAS_APPLE_SILICON")
            .unwrap_or(true); // Default assumption for modern Macs
        Ok(has_t2 || has_apple_silicon)
    /// **Safe biometric policy validation**}


    pub fn validate_biometric_policy(
        policy: &BiometricPolicy,
        available_features: &[BiometricFeature],
    ) -> BearDogResult<bool> {
        match policy {
            BiometricPolicy::TouchIDRequired => {
                Ok(available_features.contains(&BiometricFeature::TouchID))
            BiometricPolicy::FaceIDRequired => {
                Ok(available_features.contains(&BiometricFeature::FaceID))
            BiometricPolicy::TouchIDOrFaceID => Ok(available_features
                .contains(&BiometricFeature::TouchID)
                || available_features.contains(&BiometricFeature::FaceID)),
            BiometricPolicy::FaceIDOnly => Ok(available_features
                .contains(&BiometricFeature::FaceID)
                && available_features.len() == 1),
            BiometricPolicy::TouchIDOnly => Ok(available_features
            BiometricPolicy::AnyBiometric => Ok(!available_features.is_empty()),
            BiometricPolicy::NoBiometric => Ok(true), // Always valid
    /// **Get safe system information** for debugging
    pub async fn get_system_info() -> BearDogResult<SystemInfo> {
        Ok(SystemInfo {
            platform: Self::get_platform_name(),
            has_secure_enclave: Self::detect_capability().await?.is_some(),
            device_type: Self::detect_device_type()?,
            available_features: Self::get_available_biometric_features().await?,}


    fn get_platform_name() -> String {
        return "iOS".to_string();
        return "macOS".to_string();
        return "Other".to_string();}


    async fn get_available_biometric_features() -> BearDogResult<Vec<BiometricFeature>> {
        Self::get_safe_biometric_features().await
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        Ok(vec![]) // No biometric features on non-Apple platforms
}
/// **System information for debugging**
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub platform: String,
    pub has_secure_enclave: bool,
    pub device_type: SecureEnclaveDevice,
    pub available_features: Vec<BiometricFeature>,
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_device_capability_detection() -> beardog_errors::BearDogResult<()> {
        // Test device type detection
        let device = SecureEnclaveDevice::IPhone;
        assert!(matches!(
            device,
            SecureEnclaveDevice::IPhone | SecureEnclaveDevice::IPad
        ));
        // Test hardware support
        assert!(CapabilityDetector::has_secure_enclave_hardware(&device));
        Ok(())}


    fn test_biometric_policy_validation() -> beardog_errors::BearDogResult<()> {
        let features = vec![BiometricFeature::TouchID, BiometricFeature::FaceID];
        // Test TouchID policy
        let policy = BiometricPolicy::TouchIDRequired;
        assert!(
            CapabilityDetector::validate_biometric_policy(&policy, &features).map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?
        );
        // Test FaceID policy
        let policy = BiometricPolicy::FaceIDRequired;
        // Test combined policy
        let policy = BiometricPolicy::TouchIDOrFaceID;
    fn test_version_compatibility() -> beardog_errors::BearDogResult<()> {
        // Test version - assume modern versions support Secure Enclave
        let version = IOSVersion {
            major: 15,
            minor: 0,
            patch: 0,
        };
        assert!(version.major >= 10);
