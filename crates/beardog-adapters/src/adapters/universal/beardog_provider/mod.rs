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


/// BearDog Universal PrimalProvider Implementation
///
/// **BearDog's implementation of universal ecosystem patterns**
/// This module shows how BearDog integrates with the universal ecosystem
/// by implementing the PrimalProvider trait as a security provider.
/// It follows SongBird's established patterns for interoperable ecosystem components.

pub mod capabilities;
pub mod core;
pub mod handlers;
pub mod helpers;
pub mod provider;
// Re-export the main struct for backward compatibility
pub use core::BearDogPrimalProvider;
