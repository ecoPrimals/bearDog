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


//! Integration Test Modules
//! 
//! Modular organization of integration tests for better maintainability.
//! Replaces the monolithic integration_tests.rs file.

pub mod core_initialization;
pub mod api_endpoints;
pub mod threat_detection;
pub mod nestgate_adapter;
pub mod compliance_engine;
pub mod workflow_engine;
pub mod security_provider;
pub mod data_flow_integration;
pub mod performance_tests;
pub mod error_handling;

// Common test utilities and helpers
pub mod common;

/// Re-export common test utilities for convenience
pub use common::*; 