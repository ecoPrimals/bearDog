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


/// # Zero-Cost HSM Provider Architecture
///
/// High-performance HSM provider implementations that eliminate async_trait boxing overhead
/// while providing full HSM functionality with compile-time optimizations.
/// This module has been split from a single 1269-line file into focused sub-modules
/// to maintain the 1000-line limit while preserving functionality.

pub mod core; // Core ZeroCostSoftwareHsm implementation
pub mod factory; // Factory and manager implementations
pub mod metrics; // Metrics, stats, and health monitoring
pub mod traits; // Zero-cost trait definitions
// Re-export the main types for backward compatibility
pub use core::*;
pub use factory::*;
pub use metrics::*;
pub use traits::*;
