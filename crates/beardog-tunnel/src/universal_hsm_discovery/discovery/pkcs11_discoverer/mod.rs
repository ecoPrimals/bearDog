// SPDX-License-Identifier: AGPL-3.0-or-later

//! PKCS#11 HSM Discoverer
//!
//! Provides discovery functionality for PKCS#11-based HSMs
//!
//! This module discovers HSMs accessible via PKCS#11 libraries, including:
//! - Hardware HSMs (Thales, Utimaco, etc.)
//! - SoftHSM instances
//! - SmartCard tokens
//! - TPM via PKCS#11 provider
//! - Other PKCS#11-compatible devices
//!
//! Module layout: `types` (paths + token model), `discoverer` (orchestration), `library_discovery`,
//! `token_enumeration`, `capability_profiles` (static capability matrices), `classification` (vendor routing).

mod types;

mod discoverer;
mod library_discovery;
mod token_enumeration;
mod capability_profiles;
mod classification;

#[cfg(test)]
#[path = "pkcs11_discoverer_tests.rs"]
mod tests;

pub use discoverer::Pkcs11Discoverer;
pub use types::Pkcs11TokenInfo;
