// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used, missing_docs)]

//! Phase 6 Crypto — comprehensive integration tests for TLS 1.3–style primitives.
//!
//! Handlers under test: SHA-256/384/512, ECDH P-256/P-384, AES-GCM, Argon2id, PBKDF2.
//!
//! Layout (same pattern as `tests/crypto_api/`): `common`, `enhanced_unit`, `e2e`, `chaos`, `fault`.

#[path = "phase6_crypto_comprehensive/chaos.rs"]
mod chaos;
#[path = "phase6_crypto_comprehensive/common.rs"]
mod common;
#[path = "phase6_crypto_comprehensive/e2e.rs"]
mod e2e;
#[path = "phase6_crypto_comprehensive/enhanced_unit.rs"]
mod enhanced_unit;
#[path = "phase6_crypto_comprehensive/fault.rs"]
mod fault;
