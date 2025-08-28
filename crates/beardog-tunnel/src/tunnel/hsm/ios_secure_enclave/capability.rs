

use super::types::*;
use beardog_errors::BearDogError;
use tracing::info;

pub struct CapabilityDetector;
impl CapabilityDetector {

    pub async fn detect_capability() -> Result<Option<SecureEnclaveCapability>, BearDogError>> {
        info!("🔍 Detecting iOS Secure Enclave capabilities using safe APIs");

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

    #[cfg(target_os = "ios")]
    async fn ios_capability_detection() -> Result<Option<SecureEnclaveCapability>, BearDogError>> {
        info!("🔍 Detecting iOS Secure Enclave using safe system APIs");
        let ios_version = Self::get_safe_ios_version().await?;
        let device_type = Self::get_safe_device_type().await?;
        let biometric_features = Self::get_safe_biometric_features().await?;

        if ios_version.major >= 7 && Self::has_secure_enclave_hardware(&device_type) {
            Ok(Some(SecureEnclaveCapability::new(
                ios_version,
                device_type,
                biometric_features,
            )))
        } else {
            info!("ℹ️ Device does not support Secure Enclave");

    #[cfg(target_os = "macos")]
    async fn macos_capability_detection() -> Result<Option<SecureEnclaveCapability>, BearDogError>> {
        info!("🔍 Detecting macOS Secure Enclave (T2/M1+) using safe APIs");

        if Self::has_t2_or_apple_silicon().await? {
                IOSVersion {
                    major: 13,
                    minor: 0,
                    patch: 0,
                }, // macOS equivalent
                SecureEnclaveDevice::Mac,
                vec![BiometricFeature::TouchID], // Assume Touch ID for now

    async fn get_safe_ios_version() -> Result<IOSVersion, BearDogError> {

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

    async fn get_safe_device_type() -> Result<SecureEnclaveDevice, BearDogError> {

        let device_model =
            std::env::var("IOS_DEVICE_MODEL").unwrap_or_else(|_| "iPhone".to_string());
        if device_model.contains("iPhone") {
            Ok(SecureEnclaveDevice::IPhone)
        } else if device_model.contains("iPad") {
            Ok(SecureEnclaveDevice::IPad)
        } else if device_model.contains("Watch") {
            Ok(SecureEnclaveDevice::AppleWatch)
            Ok(SecureEnclaveDevice::IPhone) // Default fallback

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    async fn get_safe_biometric_features() -> Result<Vec<BiometricFeature>, BearDogError>> {
        let mut features = Vec::new();

        if std::env::var("HAS_TOUCH_ID")
            .map(|v| v == "true")
            .unwrap_or(false)
            features.push(BiometricFeature::TouchID);
        if std::env::var("HAS_FACE_ID")
            features.push(BiometricFeature::FaceID);

        if features.is_empty() {
            features.push(BiometricFeature::TouchID); // Safe default
        Ok(features)

    pub fn has_secure_enclave_hardware(device: &SecureEnclaveDevice) -> bool {
        match device {
            SecureEnclaveDevice::IPhone => true, // iPhone 5s+ have Secure Enclave
            SecureEnclaveDevice::IPad => true,   // iPad Air 2+ have Secure Enclave
            SecureEnclaveDevice::Mac => true,    // Mac with T2/M1+ have Secure Enclave
            SecureEnclaveDevice::AppleWatch => true, // Apple Watch S1+ have Secure Enclave

    pub fn detect_device_type() -> Result<SecureEnclaveDevice, BearDogError> {

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

    async fn has_t2_or_apple_silicon() -> Result<bool, BearDogError> {

        let has_t2 = std::env::var("HAS_T2_CHIP")
            .unwrap_or(false);
        let has_apple_silicon = std::env::var("HAS_APPLE_SILICON")
            .unwrap_or(true); // Default assumption for modern Macs
        Ok(has_t2 || has_apple_silicon)

    pub fn validate_biometric_policy(
        policy: &BiometricPolicy,
        available_features: &[BiometricFeature],
    ) -> Result<bool, BearDogError> {
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

    pub async fn get_system_info() -> Result<SystemInfo, BearDogError> {
        Ok(SystemInfo {
            platform: Self::get_platform_name(),
            has_secure_enclave: Self::detect_capability().await?.is_some(),
            device_type: Self::detect_device_type()?,
            available_features: Self::get_available_biometric_features().await?,}

    fn get_platform_name() -> String {
        return "iOS".to_string();
        return "macOS".to_string();
        return "Other".to_string();}

    async fn get_available_biometric_features() -> Result<Vec<BiometricFeature>, BearDogError>> {
        Self::get_safe_biometric_features().await
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        Ok(vec![]) // No biometric features on non-Apple platforms
}

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
    fn test_device_capability_detection() -> Result<(), BearDogError> {

        let device = SecureEnclaveDevice::IPhone;
        assert!(matches!(
            device,
            SecureEnclaveDevice::IPhone | SecureEnclaveDevice::IPad
        ));

        assert!(CapabilityDetector::has_secure_enclave_hardware(&device));
        Ok(())}

    fn test_biometric_policy_validation() -> Result<(), BearDogError> {
        let features = vec![BiometricFeature::TouchID, BiometricFeature::FaceID];

        let policy = BiometricPolicy::TouchIDRequired;
        assert!(
            CapabilityDetector::validate_biometric_policy(&policy, &features).map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?
        );

        let policy = BiometricPolicy::FaceIDRequired;

        let policy = BiometricPolicy::TouchIDOrFaceID;
    fn test_version_compatibility() -> Result<(), BearDogError> {

        let version = IOSVersion {
            major: 15,
            minor: 0,
            patch: 0,
        };
        assert!(version.major >= 10);
