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


/// # Unified HSM Error System
///
/// **CANONICAL ERROR HANDLING** ✅
/// This module provides direct access to BearDogError for HSM operations,
/// eliminating helper structs and providing clean, canonical error handling.

use beardog_errors::{BearDogError, BearDogResult};
// ✅ CANONICAL APPROACH: Direct BearDogError usage
// All HSM operations use BearDogError directly without helper structs
/// **CANONICAL ERROR PATTERNS** ✅
/// All HSM errors now use BearDogError directly:
/// - Configuration errors → `BearDogError::configuration(message)`
/// - Provider errors → `BearDogError::not_found(message)`  
/// - Key errors → `BearDogError::internal(message)`
/// - Crypto errors → `BearDogError::encryption(operation, message)`
/// - Auth errors → `BearDogError::authentication(message)`
/// - Permission errors → `BearDogError::invalid_input(message)`
/// - Hardware errors → `BearDogError::internal(message)`
/// - Attestation errors → `BearDogError::validation(message)`
// ✅ HELPER STRUCT ELIMINATED - Use BearDogError directly instead
// 
// Example usage:
// ```rust
// // OLD: HsmErrorHelpers::hsm_configuration_error("details")
// // NEW: BearDogError::configuration(format!("HSM configuration error: {}", details))
// // OLD: HsmErrorHelpers::key_management_error("details")  
// // NEW: BearDogError::internal(details.to_string())
// // OLD: HsmErrorHelpers::from_json_error(e)
// // NEW: BearDogError::from(e) // Uses automatic conversion
// ```
/// Re-export BearDogError and BearDogResult for convenience
pub use beardog_errors::{BearDogError, BearDogResult};
// ✅ MODERNIZATION COMPLETE
// All HSM operations should use BearDogError directly for:
// - Better performance (no helper function overhead)
// - Cleaner code (direct usage patterns)
// - Canonical consistency (same patterns across all crates)
// - Easier maintenance (fewer abstractions to maintain)
