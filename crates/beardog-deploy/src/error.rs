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


/// **CANONICAL DEPLOYMENT ERROR HANDLING** ✅ **COMPLETE**
/// 
/// This module now uses the unified BearDogError system exclusively.
/// All deployment errors are handled through BearDogError::Deployment variant.
/// 
/// **Usage Pattern**:
/// ```rust
/// use beardog_errors::{BearDogError, BearDogResult};
/// 
/// // NDK not found error
/// BearDogError::deployment("Android NDK not found", Some("ndk_setup"))
/// 
/// // Build failed error
/// BearDogError::deployment("Build process failed", Some("compilation"))
/// 
/// // Device connection error
/// BearDogError::deployment("Target device not found", Some("device_connection"))
/// ```
use beardog_errors::BearDogError;

/// **CANONICAL DEPLOYMENT ERROR HANDLING** ✅
/// Modern error handling using the unified BearDogError system
pub struct DeploymentErrorHandler;

impl DeploymentErrorHandler {
    /// Create NDK not found error
    pub fn ndk_not_found(message: impl Into<String>) -> BearDogError {
        BearDogError::deployment_with_stage(message.into(), "ndk_detection")
    }

    /// Create Rust toolchain error
    pub fn rust_toolchain(message: impl Into<String>) -> BearDogError {
        BearDogError::deployment_with_stage(message.into(), "toolchain_setup")
    }

    /// Create device not found error
    pub fn device_not_found() -> BearDogError {
        BearDogError::deployment_with_stage("Device not found or not connected", "device_detection")
    }

    /// Create multiple devices error
    pub fn multiple_devices() -> BearDogError {
        BearDogError::deployment_with_stage("Multiple devices connected - specify target device", "device_selection")
    }

    /// Create build failed error
    pub fn build_failed(message: impl Into<String>) -> BearDogError {
        BearDogError::deployment_with_stage(message.into(), "build")
    }

    /// Create deployment failed error
    pub fn deployment_failed(message: impl Into<String>) -> BearDogError {
        BearDogError::deployment_with_stage(message.into(), "deployment")
    }

    /// Create device command failed error
    pub fn device_command_failed(message: impl Into<String>) -> BearDogError {
        BearDogError::deployment_with_stage(message.into(), "device_command")
    }

    /// Create unsupported target error
    pub fn unsupported_target(target: impl Into<String>) -> BearDogError {
        BearDogError::deployment_with_stage(
            format!("Unsupported target architecture: {}", target.into()),
            "target_validation"
        )
    }

    /// Create prerequisites not met error
    pub fn prerequisites_not_met(message: impl Into<String>) -> BearDogError {
        BearDogError::deployment_with_stage(message.into(), "prerequisites")
    }

    /// Create process failed error
    pub fn process_failed(message: impl Into<String>) -> BearDogError {
        BearDogError::deployment_with_stage(message.into(), "process_execution")
    }

    /// Create IO error
    pub fn io_error(error: std::io::Error) -> BearDogError {
        BearDogError::deployment_with_stage(format!("IO error: {error}"), "io_operation")
    }
}
