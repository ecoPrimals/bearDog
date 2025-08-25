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


/// Cross-node authorization module
///
/// **Cryptographic proof of permissions for secure distributed storage operations.**
/// This module was refactored from a large file to improve maintainability.
/// The authorization system enables "friends helpin friends store data securely
/// with mathematical proof of permission."
/// ## Key Features
/// * **Authorization Proofs**: Cryptographic signatures proving operation permissions
/// * **Node Registry**: Trust relationships and capability management  
/// * **Genetic Spawning**: Dynamic BearDog instance creation with inherited capabilities
/// * **Multi-Party Workflows**: Consensus-based approval for sensitive operations
/// * **Ecosystem Integration**: Network effects with ToadStool, SongBird, NestGate, Squirrel

// Re-export public types and functions from submodules
pub use handlers::*;
pub use types::*;
// Module declarations
pub mod handlers;
pub mod types;
#[cfg(test)]
mod tests;
// Core module functionality will be implemented here
