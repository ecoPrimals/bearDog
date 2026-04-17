// SPDX-License-Identifier: AGPL-3.0-or-later
//! Shared imports for software HSM integration tests.

#![cfg(test)]

pub(crate) use crate::tunnel::hsm::GenerateKeyRequest;
pub(crate) use crate::tunnel::hsm::manager::HsmProvider;
pub(crate) use crate::tunnel::hsm::types::KeyType;
pub(crate) use crate::tunnel::hsm::types::config::CryptoBackendType;
pub(crate) use crate::tunnel::hsm::types::config::SoftwareHsmConfig;
pub(crate) use std::sync::Arc;

pub(crate) use super::super::*;
