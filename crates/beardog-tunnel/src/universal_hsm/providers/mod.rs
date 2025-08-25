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


/// # Universal HSM Providers
///
/// This module contains implementations for various HSM providers that integrate
/// with the Universal HSM interface.

pub mod factory;
pub mod software;
// Import actual provider implementations
mod android;
mod ios;
mod pkcs11;
mod tpm;
// Re-export the main software HSM components
pub use software::{
    AttestationEngine, CryptoEngine, EntropyCollector, KeyStore, SecureMemory, SoftwareHsmConfig,
    SoftwareHsmProvider,
};
pub use factory::ProviderFactory;
// Re-export actual provider implementations
pub use android::MobileHardwareProvider;
pub use ios::DesktopHardwareProvider;
pub use pkcs11::Pkcs11Provider;
pub use tpm::TpmProvider;
// Factory alias for convenience
pub type UniversalHsmFactory = ProviderFactory;
