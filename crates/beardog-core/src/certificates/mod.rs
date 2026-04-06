// SPDX-License-Identifier: AGPL-3.0-or-later

//! Certificate management for adapter unlocking
//!
//! This module implements the certificate issuance and management system for
//! BearDog's "open for humans, locked for extraction" model.
//!
//! ## Architecture
//!
//! 1. **CertificateIssuer**: Issues short-lived certificates based on classification
//! 2. **CertificateStore**: Manages active certificates
//! 3. **CertificateRenewal**: Handles automatic renewal for valid uses
//!
//! ## Philosophy
//!
//! "Individual humans get automatic 24-hour access. Commercial extraction gets
//! 15-minute certificates that require continuous re-validation."

/// Request context supplied to classification and issuance (caller identity, routing metadata).
pub mod context;
mod extraction_detector;
/// Signs and enforces policy when issuing short-lived adapter unlock certificates.
pub mod issuer;
/// Automatic renewal workflows for certificates that are still within policy.
pub mod renewal;
/// Persistent or in-memory collection of active certificates keyed by adapter.
pub mod store;

pub use context::RequestContext;
pub use extraction_detector::CommercialExtractionDetector;
pub use issuer::CertificateIssuer;
pub use renewal::CertificateRenewal;
pub use store::CertificateStore;
