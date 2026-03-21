// SPDX-License-Identifier: AGPL-3.0-only

//! # 🚀 tarpc Server for BearDog Crypto Operations
//!
//! **HIGH-PERFORMANCE CRYPTO RPC SERVER** (v1.0.0)
//!
//! Implements the `BearDogCrypto` tarpc service trait, providing binary RPC
//! for cryptographic operations with ~10-20μs latency.
//!
//! ## Architecture
//! - tarpc server binds to TCP port (default 9901)
//! - Delegates to existing crypto implementations (same as JSON-RPC handlers)
//! - Zero unsafe code
//! - Modern async/await
//!
//! ## Philosophy: Walk → Run
//! This server provides the "running" (fast) path for operations that have
//! been stabilized through JSON-RPC "walking" (flexible) experimentation.

mod server;

pub use server::BearDogCryptoServer;
