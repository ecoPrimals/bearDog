// SPDX-License-Identifier: AGPL-3.0-or-later

//! Universal HSM Module
//!
//! Provides unified traits and types for Hardware Security Module operations
//! across all supported backends (software, TPM, Android `StrongBox`, iOS
//! Secure Enclave, cloud).
//!
//! The `traits` and `entropy` sub-modules are compiled and used by HSM
//! providers.

pub mod entropy;
pub mod traits;

pub use entropy::{EntropyCollector, LiveFeedValidator};
pub use traits::*;
