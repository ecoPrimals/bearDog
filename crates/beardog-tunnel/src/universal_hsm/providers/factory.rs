//! Provider factory for creating HSM provider instances

use beardog_errors::BearDogError;

/// Factory for creating HSM providers
#[derive(Debug, Clone, Default)]
pub struct ProviderFactory;

impl ProviderFactory {
    /// Create a new provider factory instance
    pub fn new() -> Self {
        Self
    }

    /// Create a provider by name
    ///
    /// This is a stub implementation. In production, this would create
    /// the appropriate provider based on configuration.
    pub fn create_provider(&self, provider_name: &str) -> Result<(), BearDogError> {
        match provider_name {
            "software" => {
                // TODO: Create software provider
                Ok(())
            }
            "android" => {
                // TODO: Create Android StrongBox provider
                Ok(())
            }
            "ios" => {
                // TODO: Create iOS Secure Enclave provider
                Ok(())
            }
            _ => Err(BearDogError::not_supported(format!(
                "Unknown provider: {}",
                provider_name
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_creation() {
        let factory = ProviderFactory::new();
        // Basic creation test
        assert!(factory.create_provider("software").is_ok());
    }
}
