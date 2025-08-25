// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! HSM Comprehensive Tests - Modular Entry Point
//!
//! Comprehensive test suite for BearDog's Hardware Security Module integration.
//! Now organized into focused, maintainable modules.

mod hsm;

pub use hsm::*;

/// Main HSM comprehensive test suite
#[tokio::test]
async fn test_hsm_comprehensive_suite() -> beardog::BearDogResult<()> {
    hsm::integration_tests::run_comprehensive_hsm_tests().await
}

/// HSM error handling test
#[tokio::test]
async fn test_hsm_error_handling() -> beardog::BearDogResult<()> {
    hsm::integration_tests::test_hsm_error_handling().await
}

/// HSM system integration end-to-end test
#[tokio::test]
async fn test_hsm_system_integration_e2e() -> beardog::BearDogResult<()> {
    hsm::integration_tests::test_hsm_system_integration_e2e().await
} 