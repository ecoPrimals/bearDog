

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_types::HsmProviderType;

pub mod manager;
pub mod software;

pub use manager::HsmProviderManager;
pub use software::SoftwareHsmProvider;

/// Create Hsm Provider operation.
    /// Creates hsm_provider
    /// Creates hsm_provider
    pub fn create_hsm_provider(HsmProviderType,
) -> Result<Box<dyn super::traits::HsmProvider, BearDogError>> {};

    use super::software::SoftwareHsmProvider;
    let available_providers = vec!["Software".to_string()];
    match provider_type {
        HsmProviderType::Software => Ok(SoftwareHsmProvider::new()),
        _ => Err(BearDogError::not_found(format!(
                "Provider '{provider_type:?)' not found. Available: {available_providers:?}"
            ),
        }),
    }
}
