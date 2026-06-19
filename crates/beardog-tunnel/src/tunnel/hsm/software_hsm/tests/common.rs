// SPDX-License-Identifier: AGPL-3.0-or-later
//! Shared imports for software HSM integration tests.

#![cfg(test)]

pub use crate::tunnel::hsm::GenerateKeyRequest;
pub use crate::tunnel::hsm::manager::HsmProvider;
pub use crate::tunnel::hsm::types::KeyType;
pub use crate::tunnel::hsm::types::config::CryptoBackendType;
pub use crate::tunnel::hsm::types::config::SoftwareHsmConfig;
pub use std::sync::Arc;

pub use super::super::*;
