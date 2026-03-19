// SPDX-License-Identifier: AGPL-3.0-only

//! Modernization Test Module
//!
//! This module contains all configuration modernization tests, split into logical
//! groups for better maintainability and compliance with the 1000-line file limit.
//!
//! ## Organization
//!
//! Tests are organized by configuration domain:
//! - `core_tests`: Bootstrap, Database, and System configuration tests
//! - `network_tests`: Network, Connection Pool, Timeout, and Testing configuration tests
//! - `security_tests`: Security, Authentication, Rate Limiting, and Failover tests
//! - `production_tests`: Production, Deployment, Operations, and Observability tests
//! - `meta_tests`: Meta-tests for determinism, constants, and pattern compliance
//!
//! ## Migration Note
//!
//! This module replaces the previous `config_modernization_tests.rs` file (1,079 lines).
//! The file has been split into 5 smaller, focused modules (~200-300 lines each).

#[cfg(test)]
mod core_tests;

#[cfg(test)]
mod network_tests;

#[cfg(test)]
mod security_tests;

#[cfg(test)]
mod production_tests;

#[cfg(test)]
mod meta_tests;
