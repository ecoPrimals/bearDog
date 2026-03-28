// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod android;
pub mod ios;
pub mod pkcs11;
pub mod registry;
pub mod software;
pub mod tpm;

pub use android::AndroidUniversalProvider;
pub use ios::IosUniversalProvider;
pub use pkcs11::Pkcs11UniversalProvider;
pub use registry::HsmProviderRegistry;
pub use registry::UniversalProviderRegistry;
pub use software::SoftwareUniversalProvider;
pub use tpm::TpmUniversalProvider;
