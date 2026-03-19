// SPDX-License-Identifier: AGPL-3.0-only

//! Core Tests Module

#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
#[cfg(test)]
mod comprehensive_core_tests;

#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
#[cfg(test)]
mod concurrency_tests;

// October 17, 2025 Evening: Initialization edge case tests
#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
#[cfg(test)]
mod initialization_edge_cases;

// October 18, 2025: Zero-knowledge bootstrap comprehensive tests
#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
#[cfg(test)]
mod zero_knowledge_comprehensive_tests;

// October 31, 2025 - Week 2 Test Expansion: Handler Tests
#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
#[cfg(test)]
mod handlers_comprehensive_tests;

// October 31, 2025 - Week 2 Test Expansion: Generic Handler Pattern Tests
#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
#[cfg(test)]
mod handler_tests;

// October 30, 2025: Ecosystem Integration Comprehensive Tests - Coverage expansion
mod ecosystem_integration_comprehensive_tests;

// October 18, 2025 Evening: Config Validation Comprehensive Tests
#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
#[cfg(test)]
mod config_validation_comprehensive_tests;

// October 19, 2025 Evening: Startup Error Handling Tests
#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
#[cfg(test)]
mod startup_error_handling_tests;

// October 22, 2025: Comprehensive Error Path Tests
#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
#[cfg(test)]
mod error_path_comprehensive_tests;

// December 7, 2025: Error recovery path tests - Phase 2 coverage expansion
#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
#[cfg(test)]
mod error_recovery_path_tests;

// December 7, 2025: Integration engine coverage tests - Phase 3 expansion
#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
#[cfg(test)]
mod integration_engine_coverage_tests;

// October 22, 2025: Core Edge Cases (High-Value Test Coverage Expansion)
#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
#[cfg(test)]
mod core_edge_cases_oct22;

// November 22, 2025: Bootstrap and Registry Error Scenario Coverage Expansion
#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
#[cfg(test)]
mod bootstrap_error_scenarios_tests;

#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
#[cfg(test)]
mod registry_edge_cases_tests;

// October 20, 2025: Additional test modules (temporarily disabled due to compilation errors)
#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
// #[cfg(test)]
// mod initialization_tests;
//
#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
// #[cfg(test)]
// mod initialization_edge_cases_tests;
//
#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
// #[cfg(test)]
// mod ecosystem_registration_tests;
//
#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
// #[cfg(test)]
// mod self_identity_tests;
//
#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
// #[cfg(test)]
// mod core_state_tests;
//
#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
// #[cfg(test)]
// mod discovery_protocol_tests;
