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


/// Universal Adapters for External System Integration
///
/// **Universal, agnostic adapters following ecoPrimals architecture principles**
/// This module provides the universal adapter system that enables BearDog to integrate
/// with ANY external system (non-ecoPrimals) through a consistent, capability-based interface.
// Legacy NestGate module removed - replaced with universal adapter pattern
pub mod universal;

pub use universal::*;
