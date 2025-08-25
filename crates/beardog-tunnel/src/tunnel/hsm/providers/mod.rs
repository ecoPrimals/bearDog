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


/// # Modern Vendor-Agnostic HSM Providers
///
/// **ZERO VENDOR LOCK-IN** - Concrete implementations of universal HSM traits
/// This module provides real-world implementations of the vendor-agnostic HSM architecture:
/// - Android StrongBox Provider (works on any Android device with StrongBox)
/// - iOS Secure Enclave Provider (works on any iOS device with Secure Enclave)
/// - Software HSM Provider (works on any platform)
/// - PKCS#11 Provider (works with any PKCS#11 compliant HSM)
/// - TPM Provider (works with any TPM 2.0 device)

pub mod android;
pub mod ios;
pub mod software;
pub mod pkcs11;
pub mod tpm;
pub mod registry;
// Re-export the universal providers
pub use android::AndroidUniversalProvider;
pub use ios::IosUniversalProvider;
pub use software::SoftwareUniversalProvider;
pub use pkcs11::Pkcs11UniversalProvider;
pub use tpm::TpmUniversalProvider;
pub use registry::UniversalProviderRegistry; 
