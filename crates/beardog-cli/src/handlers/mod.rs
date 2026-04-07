// SPDX-License-Identifier: AGPL-3.0-or-later

//! CLI command handlers (entropy, keys, HSM, server, etc.).
//!
//! Each submodule implements one area of the `beardog` binary.

/// BirdSong lineage-based encryption
pub mod birdsong;
/// Interactive client mode
pub mod client;
/// Cross-primal secure messaging (Workflow 3)
pub mod cross_primal;
/// Daemon mode (background service)
pub mod daemon;
/// File decryption
pub mod decrypt;
/// Health diagnostics
pub mod doctor;
/// File encryption
pub mod encrypt;
/// Entropy collection and seed files
pub mod entropy;
/// HSM discovery and tests
pub mod hsm;
/// Vendor-agnostic HSM discovery helpers
pub mod hsm_agnostic;
/// Key derivation function configuration
pub mod kdf;
/// Key generation, list, info, delete (`handlers/key/` submodules)
pub mod key;
/// Key delegation with constraints
pub mod key_delegate;
/// Derive keys from a master key
pub mod key_derive;
/// Key export/import for inter-primal sharing
pub mod key_export;
/// Key lineage tree display
pub mod key_lineage;
/// Mix two keys into a derived key
pub mod key_mix;
/// Key revocation list and checks
pub mod key_revoke;
/// JSON key storage under `~/.beardog/keys`
pub mod key_store;
/// Server mode (long-running IPC service)
pub mod server;
/// Status and version output
pub mod status;
/// Streaming encryption/decryption for large files
pub mod streaming;

// Test modules
#[cfg(test)]
mod client_tests;
#[cfg(test)]
mod coverage_boost_wave10;
#[cfg(test)]
mod daemon_tests;
#[cfg(test)]
mod doctor_tests;
#[cfg(test)]
mod entropy_tests;
#[cfg(test)]
mod hsm_tests;
#[cfg(test)]
mod server_tests;
#[cfg(test)]
mod status_tests;
