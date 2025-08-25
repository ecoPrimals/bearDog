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


/// # BearDog - Unified Security and Genetic Computing Platform
///
/// BearDog is a next-generation security and genetic computing platform that provides
/// unified cryptographic operations, hardware security module integration, and
/// evolutionary algorithms for optimization.

// Re-export core modules for easy access
pub use beardog_core as core;
pub use beardog_types::config as config;
pub use beardog_errors as errors;
pub use beardog_traits as traits;

// Re-export key types for convenience
pub use beardog_types::canonical::*;
pub use beardog_errors::{BearDogError, BearDogResult};
/// BearDog version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// BearDog mission statement
pub const MISSION: &str = "Democratizing enterprise-grade security for everyone";
