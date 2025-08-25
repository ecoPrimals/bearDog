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


/// # Software HSM Provider - Modular Implementation
///
/// **COMPLETE SOFTWARE HSM IMPLEMENTATION**
/// This module provides a complete, production-ready software HSM implementation
/// that replaces all existing fragmented software HSM code throughout BearDog.
/// ## Module Structure
/// - `core` - Core provider implementation and main interface
/// - `memory` - Secure memory management and protection
/// - `keystore` - Key storage and management functionality  
/// - `crypto` - Cryptographic operations and algorithms
/// - `entropy` - Entropy collection and human entropy integration
/// - `attestation` - Attestation and verification capabilities
/// - `config` - Configuration structures and validation

pub mod attestation;
pub mod config;
pub mod core;
pub mod crypto;
pub mod entropy;
pub mod keystore;
pub mod memory;
// Re-export main types for easy access
pub use attestation::AttestationEngine;
pub use config::SoftwareHsmConfig;
pub use core::SoftwareHsmProvider;
pub use crypto::CryptoEngine;
pub use entropy::EntropyCollector;
pub use keystore::KeyStore;
pub use memory::SecureMemory;
