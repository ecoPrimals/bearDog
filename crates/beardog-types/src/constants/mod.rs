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


/// # Constants Module - Canonical
///
/// **UNIFIED CONSTANTS SYSTEM** for the BearDog ecosystem
/// This is the single source of truth for all constants across the system

// Re-export unified constants module
pub mod unified;

// Clean re-exports for common constants
pub use self::unified::network::ports::API;
pub use self::unified::network::ports::HTTPS;
pub use self::unified::network::ports::GRPC;
pub use self::unified::network::ports::METRICS;
pub use self::unified::network::ports::HEALTH;
