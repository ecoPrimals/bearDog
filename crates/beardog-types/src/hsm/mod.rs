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


/// # HSM Types - CANONICAL RE-EXPORTS
///
/// This module provides clean re-exports from the canonical HSM module.
/// All HSM types are now unified under `beardog_types::canonical::hsm`.

// CANONICAL RE-EXPORTS - Clean access to canonical types
pub use crate::canonical::hsm::{
    HsmCapabilities, HsmConfig, HsmHealth, HsmKey, KeyMetadata
};
pub use crate::canonical::hsm::status::HsmTier;
