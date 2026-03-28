// SPDX-License-Identifier: AGPL-3.0-only

#[cfg(test)]
mod serialization_property_tests;

mod capabilities_tests;
mod health_tests;
mod validation_tests;

// October 18, 2025: Comprehensive test expansion
mod capabilities_comprehensive_tests;

// October 22, 2025: Production monitoring comprehensive tests
// Split into 2 modules on October 24, 2025 (file size compliance: 1031→~515 lines each)
mod production_monitoring_advanced_tests;
mod production_monitoring_basic_tests;

// February 2026: Coverage gap tests
mod coverage_gap_11_config_trait;
mod coverage_gap_11_constants_tail;
mod coverage_gap_11_domains_network;
mod coverage_gap_11_hsm_monitoring;
mod coverage_gap_11_migration_providers;
#[allow(
    unused_imports,
    clippy::module_inception,
    dead_code,
    deprecated,
    clippy::float_cmp
)]
mod coverage_gap_12_adapter_network;
#[allow(
    unused_imports,
    clippy::module_inception,
    dead_code,
    deprecated,
    clippy::float_cmp
)]
mod coverage_gap_12_config_genetics;
#[allow(
    unused_imports,
    clippy::module_inception,
    dead_code,
    deprecated,
    clippy::float_cmp
)]
mod coverage_gap_12_hsm_config_status;
#[allow(
    unused_imports,
    clippy::module_inception,
    dead_code,
    deprecated,
    clippy::float_cmp
)]
mod coverage_gap_12_hsm_keys;
#[allow(
    unused_imports,
    clippy::module_inception,
    dead_code,
    deprecated,
    clippy::float_cmp
)]
mod coverage_gap_12_resilience_monitoring;
#[allow(
    unused_imports,
    clippy::module_inception,
    dead_code,
    deprecated,
    clippy::float_cmp
)]
mod coverage_gap_12_system_loader;
#[allow(
    unused_imports,
    clippy::module_inception,
    dead_code,
    deprecated,
    clippy::float_cmp
)]
mod coverage_gap_tests_1;
mod coverage_gap_tests_10;
#[allow(
    unused_imports,
    clippy::module_inception,
    dead_code,
    deprecated,
    clippy::float_cmp
)]
mod coverage_gap_tests_2;
#[allow(
    unused_imports,
    clippy::module_inception,
    dead_code,
    deprecated,
    clippy::float_cmp
)]
mod coverage_gap_tests_3;
#[allow(
    unused_imports,
    clippy::module_inception,
    dead_code,
    deprecated,
    clippy::float_cmp
)]
mod coverage_gap_tests_4;
#[allow(
    unused_imports,
    clippy::module_inception,
    dead_code,
    deprecated,
    clippy::float_cmp
)]
mod coverage_gap_tests_5;
#[allow(
    unused_imports,
    clippy::module_inception,
    dead_code,
    deprecated,
    clippy::float_cmp
)]
mod coverage_gap_tests_6;
#[allow(
    unused_imports,
    clippy::module_inception,
    dead_code,
    deprecated,
    clippy::float_cmp
)]
mod coverage_gap_tests_7;
#[allow(
    unused_imports,
    clippy::module_inception,
    dead_code,
    deprecated,
    clippy::float_cmp
)]
mod coverage_gap_tests_8;
mod coverage_gap_tests_9;

// March 2026: receipt, adapter certificates, constraint builtins
mod coverage_march26_types_wave;

// March 2026: wave 18 — cloud HSM, discovery, workflow/security, constants, HSM config, performance
#[allow(
    unused_imports,
    clippy::module_inception,
    dead_code,
    deprecated,
    clippy::float_cmp
)]
mod coverage_gap_wave18;
