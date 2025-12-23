// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Adapter Certificate System
//!
//! Cryptographically-enforced adapter locking that implements the philosophy:
//! **"Open gates for humans, locked tight for commercial extraction"**
//!
//! ## Architecture
//!
//! External adapters (Prometheus, Grafana, Consul, etc.) require unlock certificates
//! to operate. Certificates are issued based on commercial classification:
//!
//! - **Human/Individual**: Automatic unlock, long-lived certificates (24h)
//! - **Small Teams**: Automatic unlock, medium-lived certificates (12h)
//! - **Commercial/Corporate**: Requires license, short-lived certificates (15min)
//!
//! ## Security Model
//!
//! ```text
//! ┌────────────────────┐
//! │   Request Arrives   │
//! └─────────┬──────────┘
//!           │
//!           ▼
//! ┌────────────────────────────┐
//! │ Commercial Classification  │ (Detector analyzes request)
//! └─────────┬──────────────────┘
//!           │
//!     ┌─────┴─────┐
//!     ▼           ▼
//! Human       Commercial
//!     │           │
//!     │           ▼
//!     │      ┌─────────────┐
//!     │      │ Has License?│
//!     │      └─────┬───────┘
//!     │            │
//!     │       ┌────┴────┐
//!     │       ▼         ▼
//!     │      Yes        No
//!     │       │         │
//!     └───────┤         └──► ❌ DENY
//!             │
//!             ▼
//!     ┌────────────────┐
//!     │ Issue Cert     │ (BearDog daemon signs)
//!     └────────┬───────┘
//!              │
//!              ▼
//!     ┌────────────────┐
//!     │ Adapter Unlocks│
//!     └────────────────┘
//! ```
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_adapters::certificates::*;
//!
//! // Request arrives
//! let request = IncomingRequest { /* ... */ };
//!
//! // Classify
//! let classification = CommercialExtractionDetector::classify(&request)?;
//!
//! // Issue certificate if allowed
//! let cert = CertificateIssuer::issue(
//!     &classification,
//!     "prometheus",
//!     &beardog_signing_key,
//! )?;
//!
//! // Adapter verifies certificate
//! let adapter = PrometheusAdapter::new();
//! adapter.initialize_with_certificate(cert)?;
//! ```

pub mod issuance;
pub mod types;
pub mod verification;

pub use issuance::CertificateIssuer;
pub use types::{AdapterUnlockCertificate, CertificateExpiry, CertificateScope};
pub use verification::{CertificateVerifier, VerificationError};
