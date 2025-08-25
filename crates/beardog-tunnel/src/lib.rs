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


/// BearDog tunnel module

pub mod tunnel;
pub mod universal_hsm_discovery;
// Re-export commonly used types and errors
pub use beardog_errors::{BearDogError, BearDogResult};
pub use tunnel::hsm::types::*;
pub use tunnel::hsm::HsmError;
pub use beardog_traits::canonical::HsmProvider;
// Re-export core functionality
pub use tunnel::*;
pub use universal_hsm_discovery::*;
