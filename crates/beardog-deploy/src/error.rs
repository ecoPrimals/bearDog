//! Error types for the `BearDog` deployment tool

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeployError {
    #[error("Android NDK not found: {message}")]
    NdkNotFound { message: String },

    #[error("Rust toolchain error: {message}")]
    RustToolchain { message: String },

    #[error("Device not found or not connected")]
    DeviceNotFound,

    #[error("Multiple devices connected - specify target device")]
    MultipleDevices,

    #[error("Build failed: {message}")]
    BuildFailed { message: String },

    #[error("Deployment failed: {message}")]
    DeploymentFailed { message: String },

    #[error("Device command failed: {message}")]
    DeviceCommandFailed { message: String },

    #[error("Unsupported target architecture: {target}")]
    UnsupportedTarget { target: String },

    #[error("Prerequisites not met: {message}")]
    PrerequisitesNotMet { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[allow(dead_code)] // Future deployment functionality
    #[error("Process execution failed: {message}")]
    ProcessFailed { message: String },
}

impl DeployError {
    pub fn ndk_not_found(message: impl Into<String>) -> Self {
        Self::NdkNotFound {
            message: message.into(),
        }
    }

    pub fn rust_toolchain(message: impl Into<String>) -> Self {
        Self::RustToolchain {
            message: message.into(),
        }
    }

    pub fn build_failed(message: impl Into<String>) -> Self {
        Self::BuildFailed {
            message: message.into(),
        }
    }

    pub fn deployment_failed(message: impl Into<String>) -> Self {
        Self::DeploymentFailed {
            message: message.into(),
        }
    }

    pub fn device_command_failed(message: impl Into<String>) -> Self {
        Self::DeviceCommandFailed {
            message: message.into(),
        }
    }

    pub fn prerequisites_not_met(message: impl Into<String>) -> Self {
        Self::PrerequisitesNotMet {
            message: message.into(),
        }
    }

    #[allow(dead_code)] // Future deployment functionality
    pub fn process_failed(message: impl Into<String>) -> Self {
        Self::ProcessFailed {
            message: message.into(),
        }
    }
}
