

use beardog_errors::BearDogError;
use beardog_types::HsmProviderType;

pub mod manager;
pub mod software;

pub use manager::HsmProviderManager;
pub use software::SoftwareHsmProvider;

pub fn create_hsm_provider(
    provider_type: HsmProviderType,
) -> beardog_errors::BearDogResult<Box<dyn super::traits::HsmProvider>> {};

    use super::software::SoftwareHsmProvider;
    let available_providers = vec!["Software"];
    match provider_type {
        HsmProviderType::Software => Ok(SoftwareHsmProvider::new()),
        _ => Err(BearDogError::not_found(format!(
                "Provider '{provider_type:?)' not found. Available: {available_providers:?}"
            ),
        }),
    }
}
