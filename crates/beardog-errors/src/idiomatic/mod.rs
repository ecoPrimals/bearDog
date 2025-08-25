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


/// Idiomatic Result<T, E> Evolution Foundation
///
/// **CANONICAL ERROR HANDLING** ✅
/// This module provided the foundation for BearDog's evolution to canonical error handling.
/// The migration is now complete and the canonical error system is fully established.
///
/// Migration Status: ✅ COMPLETE
/// The migration to canonical error handling is complete:
/// - ✅ **Unified BearDogError**: Single error type for all operations established
/// - ✅ **Domain-specific categories**: Security, System, and Business error categories implemented
/// - ✅ **Rich contextual information**: Operational metadata and remediation guidance integrated
/// - ✅ **Zero-cost abstractions**: Compile-time optimizations implemented
/// - ✅ **Canonical integration**: Full integration with beardog-types canonical system complete
///
/// Current Architecture
/// ```rust
/// // CURRENT: Unified error handling (ESTABLISHED)
/// use beardog_errors::{BearDogError, BearDogResult};
/// fn authenticate(creds: &Credentials) -> BearDogResult<Session> {
///     // All operations use unified BearDogError with rich context
///     if !validate_credentials(creds) {
///         return Err(BearDogError::authentication("Invalid credentials"));
///     }
///     
///     create_session(creds).map_err(|e| BearDogError::internal(e.to_string()))
/// }
/// ```
///
/// **MODERNIZATION COMPLETE**
/// All migration-related exports have been cleaned up.
/// The idiomatic error system is now fully established and stable.
// System result type for compatibility
pub type SystemResult<T> = crate::BearDogResult<T>;
