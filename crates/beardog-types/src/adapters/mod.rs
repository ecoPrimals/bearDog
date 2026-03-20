// SPDX-License-Identifier: AGPL-3.0-only

//! # Adapter Types - External Integration Support
//!
//! This module contains types for adapter locking and external integration support.
//!
//! ## Philosophy
//!
//! "Open for humans, locked tight for commercial extraction."
//!
//! ## Architecture
//!
//! Adapters for external systems (Prometheus, Grafana, Consul, etc.) require
//! cryptographically signed unlock certificates from BearDog. This ensures:
//! - Individual developers get automatic access
//! - Commercial extraction requires licensing
//! - Certificates are time-limited and renewable

pub mod certificates;

pub use certificates::{
    AdapterUnlockCertificate, CertificateClassification, CertificateStatus, ExtractionRisk,
};
