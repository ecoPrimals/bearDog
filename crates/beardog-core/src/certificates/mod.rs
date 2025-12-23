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

pub mod issuer;
pub mod renewal;
pub mod store;

pub use issuer::CertificateIssuer;
pub use renewal::CertificateRenewal;
pub use store::CertificateStore;

