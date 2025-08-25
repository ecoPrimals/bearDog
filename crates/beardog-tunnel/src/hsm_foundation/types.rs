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


/// HSM Foundation Types
///
/// **MIGRATION COMPLETE** - This module now uses the consolidated HSM types from beardog-types::hsm
/// All HSM types have been unified into beardog-types::hsm with the following structure:
/// - beardog-types::hsm::tiers - HsmTier, HsmSecurityTier, etc.
/// - beardog-types::hsm::status - HsmOperationResult, HsmHealth, etc.
/// - beardog-types::hsm::config - HsmConfig, HsmProviderType, etc.
/// - beardog-types::hsm::discovery - DiscoveredHsm, HsmInterfaceType, etc.

// Import consolidated types from beardog-types (canonical)
pub use beardog_types::{
    HsmCapabilities, HsmConfig, HsmHealth, HsmHealthStatus, HsmOperationResult, HsmProviderType,
    HsmTierConfig, KeyHealth, KeyMaterial, KeyMetadata, KeyOperation, KeyUsagePolicy,
};
// Import canonical HsmKey from the correct location
pub use beardog_types::canonical::hsm::HsmKey;
// Import tier types from canonical location
pub use beardog_types::canonical::hsm::tiers::{
    AttestationLevel, HsmSecurityTier, HsmTier, TamperResistanceLevel,
// Import other canonical types};


pub use beardog_errors::{BearDogError, BearDogResult};
// - HsmTier (now in beardog-types::hsm::tiers)
// - HsmOperationResult (now in beardog-types::hsm::status)
// - HsmConfig (now in beardog-types::hsm::config)
//
// This eliminates fragmentation and ensures single source of truth.
