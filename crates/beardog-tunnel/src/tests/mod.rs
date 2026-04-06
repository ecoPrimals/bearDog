// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tunnel Tests Module

#[cfg(test)]
mod comprehensive_tunnel_tests;

#[cfg(test)]
mod connection_lifecycle_tests;

// October 18, 2025 Evening: HSM Provider Selection Tests
#[cfg(test)]
mod hsm_provider_selection_tests;

// October 25, 2025: Week 2 Test Expansion
#[cfg(test)]
mod hsm_comprehensive_tests;

#[cfg(test)]
mod hsm_provider_edge_cases_tests;

#[cfg(test)]
mod security_comprehensive_tests;

// #[cfg(test)]
// mod hsm_error_paths; // Disabled - tests outdated HsmConfig API

// November 19, 2025: Comprehensive tunnel recovery tests (modern concurrent patterns)
#[cfg(test)]
mod tunnel_recovery_comprehensive_tests;

// November 22, 2025: Error recovery and edge case coverage expansion
#[cfg(test)]
mod error_recovery_tests;

#[cfg(test)]
mod edge_cases_tests;

// December 1, 2025: Connection error path coverage expansion
#[cfg(test)]
mod connection_error_paths_tests;

// December 1, 2025: HSM provider integration tests (production paths)
#[cfg(test)]
mod hsm_provider_integration_tests;

#[cfg(test)]
mod tunnel_coverage_wave2;

#[cfg(test)]
mod tunnel_coverage_wave3;
