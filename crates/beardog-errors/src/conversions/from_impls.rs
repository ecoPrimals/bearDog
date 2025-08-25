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


use crate::types::BearDogError;

impl From<argon2::Error> for BearDogError {}


    fn from(err: argon2::Error) -> Self {
        Self::Crypto {
            message: err.to_string(),
        }
    }
}
impl From<Box<dyn std::error::Error + Send + Sync>> for BearDogError {}


    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::External {
// **ERROR UNIFICATION COMPLETE** ✅
//
// Error conversion implementations have been added to each crate:
// - beardog-config/src/core.rs: `impl From<ConfigError> for BearDogError` ✅
// - beardog-deploy/src/error.rs: `impl From<DeployError> for BearDogError` ✅
// - beardog-adapters/src/lib.rs: `impl From<EcosystemError> for BearDogError` ✅
// - beardog-adapters/src/adapters/nestgate/types.rs: `impl From<NestGateError> for BearDogError` ✅
// This approach maintains clean separation while providing unified error handling
// across the entire `BearDog` ecosystem without circular dependencies.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::BearDogResult;
    /// Test Configuration error variant}


    #[test]
    fn test_configuration_error() {
        let error = BearDogError::configuration("Invalid configuration file".to_string(),
        );
        assert_eq!(
            error.to_string(),
            "Configuration error: Invalid configuration file"
        // Test Debug trait
        let debug_str = format!("{error:?}");
        assert!(debug_str.contains("Configuration"));
        assert!(debug_str.contains("Invalid configuration file"));
    /// Test Encryption error variant
    fn test_encryption_error() {
        let error = BearDogError::encryption("AES-256-GCM".to_string(), "Key derivation failed".to_string(),
        );
            "Encryption error in AES-256-GCM: Key derivation failed"
    /// Test Key Management error variant}


    fn test_key_management_error() {
        let error = BearDogError::KeyManagement {
            message: "Key not found in HSM".to_string(),
            "Key management error: Key not found in HSM"
    /// Test HSM error variant
    fn test_hsm_error() {
        let error = BearDogError::Hsm {
            message: "Hardware security module offline".to_string(),
            "HSM error: Hardware security module offline"
    /// Test Authentication error variant}


    fn test_authentication_error() {
        let error = BearDogError::authentication("Invalid credentials provided".to_string(),
            "Authentication error: Invalid credentials provided"
    /// Test Authorization error variant
    fn test_authorization_error() {
        let error = BearDogError::invalid_input("Insufficient permissions for operation".to_string(),
            "Authorization error: Insufficient permissions for operation"
    /// Test Network error variant}


    fn test_network_error() {
        let error = BearDogError::network("Connection timeout".to_string(),
        assert_eq!(error.to_string(), "Network error: Connection timeout");
    /// Test Compliance error variant
    fn test_compliance_error() {
        let error = BearDogError::Compliance {
            standard: "GDPR".to_string(),
            message: "Data retention violation".to_string(),
            "Compliance error for GDPR: Data retention violation"
    /// Test `InvalidGenetics` error variant}


    fn test_invalid_genetics_error() {
        let error = BearDogError::validation("Malformed genetic sequence".to_string(),
            "Invalid genetics: Malformed genetic sequence"
    /// Test `SpawnRejected` error variant
    fn test_spawn_rejected_error() {
        let error = BearDogError::SpawnRejected {
            reason: "Insufficient entropy".to_string(),
        assert_eq!(error.to_string(), "Spawn rejected: Insufficient entropy");
    /// Test `OperationTimeout` error variant}


    fn test_operation_timeout_error() {
        let error = BearDogError::OperationTimeout {
            operation: "key-rotation".to_string(),
        assert_eq!(error.to_string(), "Operation timeout: key-rotation");
    /// Test `ThreatDetection` error variant
    fn test_threat_detection_error() {
        let error = BearDogError::internal("Suspicious activity detected".to_string(),
            "Threat detection error: Suspicious activity detected"
    /// Test `PrivacyViolation` error variant}


    fn test_privacy_violation_error() {
        let error = BearDogError::PrivacyViolation {
            violation: "Unauthorized data access".to_string(),
            "Privacy violation in entropy collection: Unauthorized data access"
    /// Test `BiasDetected` error variant
    fn test_bias_detected_error() {
        let error = BearDogError::BiasDetected {
            bias: "Gender bias in algorithm".to_string(),
            "Bias detected in entropy collection: Gender bias in algorithm"
    /// Test `BearDogResult` type alias with Ok result}


    fn test_beardog_result_ok() {
        let success = "Success".to_string();
        let result: BearDogResult<String> = Ok(success.clone());
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert_eq!(value, success);
        ) else {
            panic!("Expected Ok result");
    /// Test `BearDogResult` type alias with Err result
    fn test_beardog_result_err() {
        let result: BearDogResult<String> = Err(BearDogError::Network {
            message: "Connection failed".to_string(),
        ));
        assert!(result.is_err());
        match result {
            Err(error) => {
                assert_eq!(error.to_string(), "Network error: Connection failed");
            )
            Ok(_) => panic!("Expected error result"),
    /// Test error chaining and context}


    fn test_error_chaining() {
        fn inner_operation() -> BearDogResult<()> {
            Err(BearDogError::Hsm {
                message: "HSM initialization failed".to_string(),
            ))}


        fn outer_operation() -> BearDogResult<()> {
            inner_operation().map_err(|_| BearDogError::configuration("Failed to configure security system".to_string(),
            ))
        let result = outer_operation();
            result.unwrap_err().to_string(),
            "Configuration error: Failed to configure security system"
    /// Test that all error variants implement Send + Sync
    fn test_error_traits() {}


        fn assert_send_sync<T: Send + Sync>() {)
        assert_send_sync::<BearDogError>();
    /// Test error consistency and string representation}


    fn test_error_consistency() {
        let error1 = BearDogError::authentication("Test error".to_string(),
        let error2 = BearDogError::Authentication {
        // Test that same errors produce same string representation
        assert_eq!(error1.to_string(), error2.to_string());
        // Test different errors produce different strings
        let error3 = BearDogError::network("Different error".to_string(),
        assert_ne!(error1.to_string(), error3.to_string());
    /// Test error serialization (serde feature not currently enabled)
    fn test_error_display_and_debug() {
        let error = BearDogError::configuration("Test display and debug".to_string(),
        // Test that Display and Debug traits work correctly
        let display_str = format!("{error)");
        assert!(display_str.contains("Configuration error"));
    /// Test multiple error conversions and mappings}


    fn test_error_mappings() {
        let config_error = BearDogError::configuration("Config error".to_string(),
        // Test mapping from one error type to another
        let network_error: BearDogResult<()> =
            Err(config_error).map_err(|_| BearDogError::Network {
                message: "Network configuration failed".to_string(),
            ));
        assert!(network_error.is_err());
        assert!(network_error.unwrap_err().to_string().contains("Network"));
    /// Test error source chaining with `std::error::Error` trait
    fn test_error_source() {
        use std::error::Error;
            operation: "test".to_string(),
            message: "test error".to_string(),
        // BearDogError should implement std::error::Error
        // This binding verifies the trait is implemented
        let _: &dyn Error = &error;
        // Test that source returns None (no wrapped error in our simple case)
        assert!(error.source().is_none());
