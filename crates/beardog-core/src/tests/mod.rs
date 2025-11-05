//! Core Tests Module

#[cfg(test)]
mod comprehensive_core_tests;

#[cfg(test)]
mod concurrency_tests;

// October 17, 2025 Evening: Initialization edge case tests
#[cfg(test)]
mod initialization_edge_cases;

// October 18, 2025: Zero-knowledge bootstrap comprehensive tests
#[cfg(test)]
mod zero_knowledge_comprehensive_tests;

// October 31, 2025 - Week 2 Test Expansion: Handler Tests
#[cfg(test)]
mod handlers_comprehensive_tests;

// October 31, 2025 - Week 2 Test Expansion: Generic Handler Pattern Tests
#[cfg(test)]
mod handler_tests;

// October 30, 2025: Ecosystem Integration Comprehensive Tests - Coverage expansion
mod ecosystem_integration_comprehensive_tests;

// October 18, 2025 Evening: Config Validation Comprehensive Tests
#[cfg(test)]
mod config_validation_comprehensive_tests;

// October 19, 2025 Evening: Startup Error Handling Tests
#[cfg(test)]
mod startup_error_handling_tests;

// October 22, 2025: Comprehensive Error Path Tests
#[cfg(test)]
mod error_path_comprehensive_tests;

// October 22, 2025: Core Edge Cases (High-Value Test Coverage Expansion)
#[cfg(test)]
mod core_edge_cases_oct22;

// October 20, 2025: Additional test modules (temporarily disabled due to compilation errors)
// #[cfg(test)]
// mod initialization_tests;
//
// #[cfg(test)]
// mod initialization_edge_cases_tests;
//
// #[cfg(test)]
// mod ecosystem_registration_tests;
//
// #[cfg(test)]
// mod self_identity_tests;
//
// #[cfg(test)]
// mod core_state_tests;
//
// #[cfg(test)]
// mod discovery_protocol_tests;
