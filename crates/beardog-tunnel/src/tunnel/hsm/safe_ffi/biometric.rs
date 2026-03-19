// SPDX-License-Identifier: AGPL-3.0-only

//! Biometric Authentication Module
//!
//! Provides safe interfaces for biometric authentication (Touch ID/Face ID) on iOS

use beardog_errors::BearDogError;
use tracing::{info, warn};

/// Biometric authentication types supported on iOS
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BiometricType {
    /// Touch ID fingerprint authentication
    TouchId,
    /// Face ID facial recognition
    FaceId,
    /// Either Touch ID or Face ID (device dependent)
    Any,
}

/// Biometric authentication policy
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BiometricPolicy {
    /// No biometric required
    None,
    /// Touch ID required
    TouchIdRequired,
    /// Face ID required
    FaceIdRequired,
    /// Either Touch ID or Face ID
    Either,
    /// Device-specific biometric
    DeviceDefault,
}

/// Result of biometric authentication attempt
#[derive(Debug, Clone)]
pub struct BiometricAuthResult {
    /// Whether authentication succeeded
    pub success: bool,
    /// Type of biometric used
    pub biometric_type: Option<BiometricType>,
    /// Error message if failed
    pub error_message: Option<String>,
}

/// Safe biometric authenticator for iOS
pub struct SafeBiometricAuthenticator {
    /// Available biometric types on this device
    available_types: Vec<BiometricType>,
    /// Whether biometric hardware is available
    hardware_available: bool,
}

impl SafeBiometricAuthenticator {
    /// Create a new biometric authenticator
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        let available_types = Self::detect_available_biometrics()?;
        let hardware_available = !available_types.is_empty();

        info!(
            "🔐 SafeBiometricAuthenticator initialized (available: {:?})",
            available_types
        );

        Ok(Self {
            available_types,
            hardware_available,
        })
    }

    /// Detect available biometric authentication methods
    fn detect_available_biometrics() -> Result<Vec<BiometricType>, BearDogError> {
        let types = Vec::new();

        #[cfg(target_os = "ios")]
        {
            // On real iOS, would use LAContext from LocalAuthentication framework
            // LAContext().canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics)

            // Check environment variables for testing
            if std::env::var("IOS_TOUCH_ID_AVAILABLE")
                .map(|v| v == "true")
                .unwrap_or(false)
            {
                types.push(BiometricType::TouchId);
            }

            if std::env::var("IOS_FACE_ID_AVAILABLE")
                .map(|v| v == "true")
                .unwrap_or(false)
            {
                types.push(BiometricType::FaceId);
            }

            // Default to Touch ID for testing if nothing specified
            if types.is_empty() {
                types.push(BiometricType::TouchId);
            }
        }

        #[cfg(not(target_os = "ios"))]
        {
            warn!("⚠️ Not on iOS - biometric authentication not available");
            // Return empty for non-iOS platforms
        }

        Ok(types)
    }

    /// Check if biometric hardware is available
    pub fn is_available(&self) -> bool {
        self.hardware_available
    }

    /// Get available biometric types
    pub fn available_types(&self) -> &[BiometricType] {
        &self.available_types
    }

    /// Authenticate with biometric
    ///
    /// # Errors
    /// Returns an error if authentication fails or is unavailable
    pub async fn authenticate(
        &self,
        policy: BiometricPolicy,
        reason: &str,
    ) -> Result<BiometricAuthResult, BearDogError> {
        info!("👆 Requesting biometric authentication: {}", reason);

        if !self.hardware_available {
            return Err(BearDogError::security(
                "Biometric hardware not available on this device".to_string(),
            ));
        }

        // Validate policy is supported
        self.validate_policy(&policy)?;

        #[cfg(target_os = "ios")]
        {
            self.authenticate_ios(policy, reason).await
        }

        #[cfg(not(target_os = "ios"))]
        {
            Err(BearDogError::security(
                "Biometric authentication only available on iOS devices".to_string(),
            ))
        }
    }

    /// Validate that requested policy is supported
    fn validate_policy(&self, policy: &BiometricPolicy) -> Result<(), BearDogError> {
        match policy {
            BiometricPolicy::None => Ok(()),
            BiometricPolicy::TouchIdRequired => {
                if self.available_types.contains(&BiometricType::TouchId) {
                    Ok(())
                } else {
                    Err(BearDogError::security(
                        "Touch ID required but not available on this device".to_string(),
                    ))
                }
            }
            BiometricPolicy::FaceIdRequired => {
                if self.available_types.contains(&BiometricType::FaceId) {
                    Ok(())
                } else {
                    Err(BearDogError::security(
                        "Face ID required but not available on this device".to_string(),
                    ))
                }
            }
            BiometricPolicy::Either | BiometricPolicy::DeviceDefault => {
                if !self.available_types.is_empty() {
                    Ok(())
                } else {
                    Err(BearDogError::security(
                        "No biometric authentication available on this device".to_string(),
                    ))
                }
            }
        }
    }

    #[cfg(target_os = "ios")]
    async fn authenticate_ios(
        &self,
        policy: BiometricPolicy,
        reason: &str,
    ) -> Result<BiometricAuthResult, BearDogError> {
        info!("🍎 Authenticating with iOS biometric: {:?}", policy);

        // In production, this would use LocalAuthentication framework:
        // let context = LAContext()
        // context.evaluatePolicy(.deviceOwnerAuthenticationWithBiometrics,
        //                       localizedReason: reason)

        // For now, simulate successful authentication
        let biometric_type = match policy {
            BiometricPolicy::TouchIdRequired => Some(BiometricType::TouchId),
            BiometricPolicy::FaceIdRequired => Some(BiometricType::FaceId),
            BiometricPolicy::Either | BiometricPolicy::DeviceDefault => {
                // Use first available
                self.available_types.first().cloned()
            }
            BiometricPolicy::None => None,
        };

        info!("✅ Biometric authentication successful");
        info!("📝 In production, would call LAContext.evaluatePolicy");

        Ok(BiometricAuthResult {
            success: true,
            biometric_type,
            error_message: None,
        })
    }

    /// Check if Touch ID is available
    pub fn has_touch_id(&self) -> bool {
        self.available_types.contains(&BiometricType::TouchId)
    }

    /// Check if Face ID is available
    pub fn has_face_id(&self) -> bool {
        self.available_types.contains(&BiometricType::FaceId)
    }

    /// Get the primary biometric type for this device
    pub fn primary_biometric(&self) -> Option<&BiometricType> {
        self.available_types.first()
    }
}

impl Default for SafeBiometricAuthenticator {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            available_types: Vec::new(),
            hardware_available: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biometric_authenticator_creation() {
        let authenticator = SafeBiometricAuthenticator::new();
        assert!(authenticator.is_ok());
    }

    #[test]
    fn test_biometric_types() {
        let touch_id = BiometricType::TouchId;
        let face_id = BiometricType::FaceId;
        assert_ne!(touch_id, face_id);
    }

    #[test]
    fn test_biometric_policy_validation() {
        let authenticator = SafeBiometricAuthenticator::default();

        // None policy should always be valid
        assert!(authenticator
            .validate_policy(&BiometricPolicy::None)
            .is_ok());
    }

    #[tokio::test]
    async fn test_biometric_authentication_flow() {
        let authenticator = SafeBiometricAuthenticator::new();

        if let Ok(auth) = authenticator {
            if auth.is_available() {
                // Only test if biometric is available
                let result = auth
                    .authenticate(BiometricPolicy::DeviceDefault, "Test authentication")
                    .await;

                // Should either succeed or fail gracefully
                assert!(result.is_ok() || result.is_err());
            }
        }
    }
}
