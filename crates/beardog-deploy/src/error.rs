// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;

#[allow(dead_code)]
pub struct DeploymentErrorHandler;

impl DeploymentErrorHandler {
    /// Ndk Not Found operation.
    #[allow(dead_code)]
    pub fn ndk_not_found<'a>(message: impl Into<&'a str>) -> BearDogError {
        BearDogError::system(format!("NDK Error: {}", message.into()))
    }

    /// Rust Toolchain operation.
    #[allow(dead_code)]
    pub fn rust_toolchain<'a>(message: impl Into<&'a str>) -> BearDogError {
        BearDogError::system(format!("Rust Toolchain Error: {}", message.into()))
    }

    /// Android Build Env operation.
    #[allow(dead_code)]
    pub fn android_build_env(message: impl Into<String>) -> BearDogError {
        BearDogError::system(format!(
            "Android Build Environment Error: {}",
            message.into()
        ))
    }

    /// Ios Build Env operation.
    #[allow(dead_code)]
    pub fn ios_build_env(message: impl Into<String>) -> BearDogError {
        BearDogError::system(format!("iOS Build Environment Error: {}", message.into()))
    }

    /// Device Not Found operation.
    #[must_use]
    #[allow(dead_code)]
    pub fn device_not_found() -> BearDogError {
        BearDogError::system("No suitable devices found for deployment".to_string())
    }

    /// Multiple Devices operation.
    #[must_use]
    #[allow(dead_code)]
    pub fn multiple_devices() -> BearDogError {
        BearDogError::system("Multiple devices found, please specify target".to_string())
    }

    /// Build Failed operation.
    #[allow(dead_code)]
    /// Builds failed
    /// Builds failed
    pub fn build_failed<'a>(message: impl Into<&'a str>) -> BearDogError {
        BearDogError::system(format!("Build Failed: {}", message.into()))
    }

    /// Deployment Failed operation.
    #[allow(dead_code)]
    pub fn deployment_failed<'a>(message: impl Into<&'a str>) -> BearDogError {
        BearDogError::system(format!("Deployment Failed: {}", message.into()))
    }

    /// Device Command Failed operation.
    #[allow(dead_code)]
    pub fn device_command_failed<'a>(message: impl Into<&'a str>) -> BearDogError {
        BearDogError::system(format!("Device Command Failed: {}", message.into()))
    }

    /// Unsupported Target operation.
    #[allow(dead_code)]
    pub fn unsupported_target<'a>(target: impl Into<&'a str>) -> BearDogError {
        BearDogError::system(format!(
            "Unsupported target architecture: {}",
            target.into()
        ))
    }

    /// Prerequisites Not Met operation.
    #[allow(dead_code)]
    pub fn prerequisites_not_met<'a>(message: impl Into<&'a str>) -> BearDogError {
        BearDogError::system(format!("Prerequisites Not Met: {}", message.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ndk_not_found() {
        let error = DeploymentErrorHandler::ndk_not_found("NDK r21 not found");
        let msg = format!("{}", error);
        assert!(msg.contains("NDK Error"));
        assert!(msg.contains("NDK r21 not found"));
    }

    #[test]
    fn test_rust_toolchain() {
        let error =
            DeploymentErrorHandler::rust_toolchain("stable-aarch64-linux-android not installed");
        let msg = format!("{}", error);
        assert!(msg.contains("Rust Toolchain Error"));
    }

    #[test]
    fn test_android_build_env() {
        let error = DeploymentErrorHandler::android_build_env("ANDROID_HOME not set");
        let msg = format!("{}", error);
        assert!(msg.contains("Android Build Environment Error"));
    }

    #[test]
    fn test_ios_build_env() {
        let error = DeploymentErrorHandler::ios_build_env("Xcode not found");
        let msg = format!("{}", error);
        assert!(msg.contains("iOS Build Environment Error"));
    }

    #[test]
    fn test_device_not_found() {
        let error = DeploymentErrorHandler::device_not_found();
        let msg = format!("{}", error);
        assert!(msg.contains("No suitable devices found"));
    }

    #[test]
    fn test_multiple_devices() {
        let error = DeploymentErrorHandler::multiple_devices();
        let msg = format!("{}", error);
        assert!(msg.contains("Multiple devices found"));
    }

    #[test]
    fn test_build_failed() {
        let error = DeploymentErrorHandler::build_failed("cargo build failed");
        let msg = format!("{}", error);
        assert!(msg.contains("Build Failed"));
    }

    #[test]
    fn test_deployment_failed() {
        let error = DeploymentErrorHandler::deployment_failed("APK installation failed");
        let msg = format!("{}", error);
        assert!(msg.contains("Deployment Failed"));
    }

    #[test]
    fn test_device_command_failed() {
        let error = DeploymentErrorHandler::device_command_failed("adb shell failed");
        let msg = format!("{}", error);
        assert!(msg.contains("Device Command Failed"));
    }

    #[test]
    fn test_unsupported_target() {
        let error = DeploymentErrorHandler::unsupported_target("riscv64");
        let msg = format!("{}", error);
        assert!(msg.contains("Unsupported target architecture"));
        assert!(msg.contains("riscv64"));
    }

    #[test]
    fn test_prerequisites_not_met() {
        let error = DeploymentErrorHandler::prerequisites_not_met("cmake required");
        let msg = format!("{}", error);
        assert!(msg.contains("Prerequisites Not Met"));
    }
}
