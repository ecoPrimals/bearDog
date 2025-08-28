use beardog_errors::BearDogError;

#[allow(dead_code)]
pub struct DeploymentErrorHandler;

#[allow(dead_code)]
impl DeploymentErrorHandler {
    pub fn ndk_not_found<'a>(message: impl Into<&'a str>) -> BearDogError {
        BearDogError::system(format!("NDK Detection Error: {}", message.into()))
    }

    pub fn rust_toolchain<'a>(message: impl Into<&'a str>) -> BearDogError {
        BearDogError::system(format!("Toolchain Setup Error: {}", message.into()))
    }

    pub fn android_build_env(message: impl Into<String>) -> BearDogError {
        BearDogError::system(format!(
            "Android Build Environment Error: {}",
            message.into()
        ))
    }

    pub fn ios_build_env(message: impl Into<String>) -> BearDogError {
        BearDogError::system(format!("iOS Build Environment Error: {}", message.into()))
    }

    pub fn device_not_found() -> BearDogError {
        BearDogError::system("Device Detection Error: Device not found or not connected")
    }

    pub fn multiple_devices() -> BearDogError {
        BearDogError::system(
            "Device Selection Error: Multiple devices connected - specify target device",
        )
    }

    pub fn build_failed<'a>(message: impl Into<&'a str>) -> BearDogError {
        BearDogError::system(format!("Build Error: {}", message.into()))
    }

    pub fn deployment_failed<'a>(message: impl Into<&'a str>) -> BearDogError {
        BearDogError::system(format!("Deployment Error: {}", message.into()))
    }

    pub fn device_command_failed<'a>(message: impl Into<&'a str>) -> BearDogError {
        BearDogError::system(format!("Device Command Error: {}", message.into()))
    }

    pub fn unsupported_target<'a>(target: impl Into<&'a str>) -> BearDogError {
        BearDogError::system(format!(
            "Target Architecture Error: Unsupported target architecture: {}",
            target.into()
        ))
    }

    pub fn prerequisites_not_met<'a>(message: impl Into<&'a str>) -> BearDogError {
        BearDogError::system(format!("Prerequisites Error: {}", message.into()))
    }

    pub fn process_failed<'a>(message: impl Into<&'a str>) -> BearDogError {
        BearDogError::system(format!("Process Execution Error: {}", message.into()))
    }

    pub fn io_error(error: std::io::Error) -> BearDogError {
        BearDogError::system(format!("IO Operation Error: {error}"))
    }
}
