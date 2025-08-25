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


//! World-Class Testing Framework - Modular Implementation
//!
//! This module provides the entry point for the world-class testing framework,
//! split into focused modules to maintain the 1000-line file limit.

pub mod framework;
pub mod traits;
pub mod metrics;
pub mod verification;

pub use framework::*;
pub use traits::*;
pub use metrics::*;
pub use verification::*; 