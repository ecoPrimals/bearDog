//! Universal HSM Provider trait definition

use super::{ProviderInfo, ProviderHealth, HumanEntropyCapabilities, HumanEntropyData};
use beardog_errors::BearDogError;

/// Universal HSM Provider trait
/// 
/// This trait defines the interface for all HSM providers in the system,
/// providing a unified API for cryptographic operations regardless of the
/// underlying hardware or software implementation.
#[async_trait::async_trait]
pub trait UniversalHsmProvider: Send + Sync {
    /// Get provider information
    fn get_provider_info(&self) -> ProviderInfo;
    
    /// Check provider health
    async fn check_health(&self) -> Result<ProviderHealth, BearDogError>;
    
    /// Initialize the provider
    async fn initialize(&self) -> Result<(), BearDogError> {
        Ok(())
    }
    
    /// Shutdown the provider
    async fn shutdown(&self) -> Result<(), BearDogError> {
        Ok(())
    }

    /// Generate a key
    async fn generate_key(&self, _key_id: String, _key_type: String) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::not_supported("generate_key not implemented"))
    }

    /// Sign data
    async fn sign_data(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::not_supported("sign_data not implemented"))
    }

    /// Verify signature
    async fn verify_signature(&self, _key_id: &str, _data: &[u8], _signature: &[u8]) -> Result<bool, BearDogError> {
        Err(BearDogError::not_supported("verify_signature not implemented"))
    }

    /// Get human entropy capabilities
    async fn get_human_entropy_capabilities(&self) -> Result<HumanEntropyCapabilities, BearDogError> {
        Err(BearDogError::not_supported("get_human_entropy_capabilities not implemented"))
    }

    /// Collect human entropy
    async fn collect_human_entropy(&self) -> Result<HumanEntropyData, BearDogError> {
        Err(BearDogError::not_supported("collect_human_entropy not implemented"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: Tests for trait implementations would go in the implementing types
}
