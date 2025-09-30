

// MODERNIZATION NOTE: This file contains vendor-specific references that should be migrated
// to universal adapter patterns. See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md
// Target: Replace with capability-based discovery for vendor/primal agnosticism
// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod universal_kms;
pub mod tpm;
pub mod vault;

pub use universal_kms::AwsKmsCapabilityHandler;
pub use tpm::TpmCapabilityHandler;
pub use vault::VaultCapabilityHandler;
