// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used, reason = "expect/unwrap acceptable for invariant failures in tests and bootstrap code"))]
#![doc = include_str!("../README.md")]

//! ACME certificate lifecycle client for `BearDog` TLS sovereignty.
//!
//! Implements RFC 8555 (ACME) to enable automated certificate issuance and
//! renewal, replacing Cloudflare's managed TLS with self-sovereign certificate
//! management.

mod account;
mod challenge;
mod client;
mod error;
pub mod hot_reload;
mod jws;
mod order;
mod rustls_provider;
pub mod shadow_metrics;
mod storage;

pub use account::AcmeAccount;
pub use challenge::{ChallengeToken, Http01Solver};
pub use client::{AcmeClient, AcmeConfig};
pub use error::AcmeError;
pub use hot_reload::{HotReloadAcceptor, HotReloadController, create_hot_reload_pair};
pub use order::CertificateOrder;
pub use shadow_metrics::ShadowMetricsCollector;
pub use storage::CertificateStore;

/// ACME directory URLs for well-known CAs.
pub mod directories {
    /// Let's Encrypt production directory.
    pub const LETS_ENCRYPT_PRODUCTION: &str = "https://acme-v02.api.letsencrypt.org/directory";

    /// Let's Encrypt staging directory (for testing).
    pub const LETS_ENCRYPT_STAGING: &str = "https://acme-staging-v02.api.letsencrypt.org/directory";
}
