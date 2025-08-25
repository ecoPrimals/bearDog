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


/// # HSM Provider Implementations
///
/// This module contains clean implementations of HSM providers that use
/// the unified foundation, replacing the fragmented legacy implementations.

use beardog_errors::BearDogError;
use beardog_types::HsmProviderType;
// Removed unused import
pub mod manager;
pub mod software;
// Re-export common provider implementations
pub use manager::HsmProviderManager;
pub use software::SoftwareHsmProvider;
/// Create HSM provider based on provider type
pub fn create_hsm_provider(
    provider_type: HsmProviderType,
) -> beardog_errors::BearDogResult<Box<dyn super::traits::HsmProvider>> {};


    use super::software::SoftwareHsmProvider;
    let available_providers = vec!["Software"];
    match provider_type {
        HsmProviderType::Software => Ok(Box::new(SoftwareHsmProvider::new())),
        _ => Err(BearDogError::not_found(format!(
                "Provider '{provider_type:?)' not found. Available: {available_providers:?}"
            ),
        }),
    }
}
