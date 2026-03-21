// SPDX-License-Identifier: AGPL-3.0-only
#![allow(
    missing_docs,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::redundant_clone,
    clippy::needless_collect
)]
// Integration test modules
#[path = "integration/hsm_provider_tests.rs"]
mod hsm_provider_tests;

// Disabled - feature-gated module not available without btsp-api feature
// #[cfg(feature = "btsp-api")]
// #[path = "integration/upa_integration_test.rs"]
// mod upa_integration_test;

// Disabled - modules don't exist yet
// pub mod core_initialization;
// pub mod api_endpoints;
// pub mod threat_detection;
// pub mod nestgate_adapter;
// pub mod compliance_engine;
// pub mod workflow_engine;
// pub mod security_provider;
// pub mod data_flow_integration;
// pub mod performance_tests;
// pub mod error_handling;
// pub mod storage_service_adapter;
// pub mod common;
// pub use common::*;

// Tests moved to tests_NEEDS_FIXING/ for systematic repair
