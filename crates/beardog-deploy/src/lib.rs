

use beardog_errors::BearDogError;

pub mod android;
pub mod builder;
pub mod device;
pub mod error;
pub mod optimization;

pub use android::*;
pub use builder::*;
pub use device::*;
pub use error::*;

#[derive(Debug, Clone)]
pub struct DeploymentManager {
    config: DeploymentConfig,
}
impl DeploymentManager {

    pub fn new(config: DeploymentConfig) -> Self {
        Self { config }
    }

    pub async fn initialize(&self) -> Result<(), BearDogError> {
        println!("🚀 Initializing BearDog deployment environment");

        self.validate_config().await?;
        println!("✅ Deployment environment initialized successfully");
        Ok(())
    }

    async fn validate_config(&self) -> Result<(), BearDogError> {
        if self.config.environment.is_empty() {
            return Err(BearDogError::system("Environment must be specified"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DeploymentConfig {
    pub environment: String,
    pub region: String,
    pub instance_count: u32,
    pub monitoring_enabled: bool,
}
impl Default for DeploymentConfig {
    fn default() -> Self {
        Self {
            environment: "development".to_string(),
            region: "local".to_string(),
            instance_count: 1,
            monitoring_enabled: true,
        }
    }
}
