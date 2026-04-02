// SPDX-License-Identifier: AGPL-3.0-only

//! Universal HSM Module
//!
//! Provides unified traits and types for Hardware Security Module operations
//! across all supported backends (software, TPM, Android `StrongBox`, iOS
//! Secure Enclave, cloud).
//!
//! The `traits` and `entropy` sub-modules are compiled and used by HSM
//! providers.  The `providers/`, `health`, `registry`, and `provider`
//! sub-modules exist on disk but are not yet wired into the module tree — they
//! reference older API surfaces and need alignment before activation.

pub mod entropy;
pub mod traits;

pub use entropy::{EntropyCollector, LiveFeedValidator};
pub use traits::*;
