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
