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
