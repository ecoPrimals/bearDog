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


/// HSM Constants
///
/// Hardware Security Module related constants including capabilities,
/// key sizes, and operational parameters.

use std::time::Duration;
// ============================================================================
// HSM CAPABILITY CONSTANTS
/// Key generation capability identifier
pub const KEY_GENERATION: &str = "KeyGeneration";
/// Digital signing capability identifier
pub const SIGNING: &str = "Signing";
/// Data encryption capability identifier
pub const ENCRYPTION: &str = "Encryption";
/// Data decryption capability identifier
pub const DECRYPTION: &str = "Decryption";
/// Key attestation capability identifier
pub const KEY_ATTESTATION: &str = "KeyAttestation";
/// User presence validation capability identifier
pub const USER_PRESENCE_VALIDATION: &str = "UserPresenceValidation";
// PRIMAL CAPABILITY CONSTANTS
/// Communication mesh capability identifier
pub const COMMUNICATION_MESH_CAPABILITY: &str = "communication_mesh";
/// Storage services capability identifier
pub const STORAGE_SERVICES_CAPABILITY: &str = "storage_services";
/// Compute orchestration capability identifier
pub const COMPUTE_ORCHESTRATION_CAPABILITY: &str = "compute_orchestration";
/// AI intelligence capability identifier
pub const AI_INTELLIGENCE_CAPABILITY: &str = "ai_intelligence";
/// Security provider capability identifier
pub const SECURITY_PROVIDER_CAPABILITY: &str = "security_provider";
/// System integration capability identifier
pub const SYSTEM_INTEGRATION_CAPABILITY: &str = "system_integration";
/// Hardware security module capability identifier
pub const HSM_CAPABILITY: &str = "hardware_security_module";
/// Key management capability identifier
pub const KEY_MANAGEMENT_CAPABILITY: &str = "key_management";
/// Secure enclave capability identifier
pub const SECURE_ENCLAVE_CAPABILITY: &str = "secure_enclave";
// HSM OPERATIONAL CONSTANTS
/// Maximum number of HSM keys that can be stored
pub const MAX_HSM_KEYS: usize = 1000;
/// Standard key size limit in bytes
pub const STANDARD_KEY_SIZE_LIMIT: usize = 4096;
/// Zero-cost software HSM provider identifier for production deployments
pub const ZERO_COST_SOFTWARE_HSM_PROVIDER: &str = "ZeroCostSoftwareHSM";
/// Maximum key size for operations
pub const MAX_KEY_SIZE: usize = STANDARD_KEY_SIZE_LIMIT;
// HSM TIMING CONSTANTS
/// Default HSM operation timeout
pub const HSM_OPERATION_TIMEOUT: Duration = Duration::from_secs(30);
/// HSM health check interval
pub const HSM_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(60);
/// Key rotation interval
pub const KEY_ROTATION_INTERVAL: Duration = Duration::from_secs(86400 * 30); // 30 days
/// HSM connection timeout
pub const HSM_CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);
/// HSM discovery timeout
pub const HSM_DISCOVERY_TIMEOUT: Duration = Duration::from_secs(5);
// HSM SECURITY CONSTANTS
/// Minimum entropy bits for key generation
pub const MIN_ENTROPY_BITS: u32 = 256;
/// Default key derivation iterations
pub const DEFAULT_KEY_DERIVATION_ITERATIONS: u32 = 100_000;
/// HSM authentication timeout
pub const HSM_AUTH_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes
/// Maximum failed authentication attempts
pub const MAX_HSM_AUTH_ATTEMPTS: u32 = 3;
// HSM PROVIDER TYPES
/// Software HSM provider type
pub const SOFTWARE_HSM_PROVIDER: &str = "software";
/// Hardware HSM provider type
pub const HARDWARE_HSM_PROVIDER: &str = "hardware";
/// Mobile HSM provider type
pub const MOBILE_HSM_PROVIDER: &str = "mobile";
/// Cloud HSM provider type
pub const CLOUD_HSM_PROVIDER: &str = "cloud";
/// Network HSM provider type
pub const NETWORK_HSM_PROVIDER: &str = "network";
/// USB HSM provider type
pub const USB_HSM_PROVIDER: &str = "usb";
/// TPM provider type
pub const TPM_PROVIDER: &str = "tpm";
/// Smart card provider type
pub const SMARTCARD_PROVIDER: &str = "smartcard";
