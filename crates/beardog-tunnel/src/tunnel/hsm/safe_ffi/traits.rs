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


/// Safe Hardware Provider Traits - Canonical System
///
/// This module provides the canonical HSM provider traits for safe hardware operations.
/// All HSM providers should implement the unified HsmProvider trait for consistency.

// Re-export the canonical HsmProvider trait
pub use beardog_traits::canonical::HsmProvider as SafeHardwareProvider;

// Re-export necessary types for backward compatibility
pub use beardog_types::canonical::hsm::{HsmKey, KeyType};
pub use beardog_errors::BearDogResult;
