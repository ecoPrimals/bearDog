// SPDX-License-Identifier: AGPL-3.0-only

//! Genetics Tests Module

#[cfg(test)]
mod comprehensive_genetics_tests;

#[cfg(test)]
mod genetics_manager_comprehensive_tests;

#[cfg(test)]
mod lib_coverage_tests;

#[cfg(test)]
mod spawning_comprehensive_tests;

#[cfg(test)]
mod error_path_comprehensive_tests;

#[cfg(test)]
mod edge_case_comprehensive_tests;

// December 1, 2025: Algorithm error path coverage expansion
#[cfg(test)]
mod algorithm_error_paths_tests;

// December 1, 2025: Substantive population evolution integration tests
#[cfg(test)]
mod population_evolution_integration_tests;

// December 6, 2025: Phase 2 Advanced Test Coverage - Edge cases and error paths
#[cfg(test)]
mod genetics_advanced_coverage_tests;

// February 2026: Coverage gap tests targeting 90% threshold
#[cfg(test)]
mod coverage_gap_tests;
