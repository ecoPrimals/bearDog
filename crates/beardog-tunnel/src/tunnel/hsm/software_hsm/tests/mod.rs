// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive tests for Software HSM
//!
//! Covers:
//! - Core HSM initialization and lifecycle
//! - Key generation for all supported types
//! - Cryptographic operations (sign, verify, encrypt, decrypt)
//! - Key management (import, export, delete)
//! - Memory protection and secure zeroing
//! - Audit logging
//! - Health monitoring
//! - Error handling and edge cases

#![cfg(test)]

mod common;
mod concurrency;
mod crypto_operations;
mod edge_cases;
mod errors;
mod initialization;
mod key_generation;
mod key_management;
