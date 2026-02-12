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
#[allow(
    unused_imports,
    clippy::module_inception,
    dead_code,
    deprecated,
    clippy::float_cmp
)]
mod coverage_gap_tests_1;
mod coverage_gap_tests_10;
mod coverage_gap_tests_11;
#[allow(
    unused_imports,
    clippy::module_inception,
    dead_code,
    deprecated,
    clippy::float_cmp
)]
mod coverage_gap_tests_12;
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
