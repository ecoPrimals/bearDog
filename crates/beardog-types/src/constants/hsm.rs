

use std::time::Duration;

pub const KEY_GENERATION: &str = "KeyGeneration";

pub const SIGNING: &str = "Signing";

pub const ENCRYPTION: &str = "Encryption";

pub const DECRYPTION: &str = "Decryption";

pub const KEY_ATTESTATION: &str = "KeyAttestation";

pub const USER_PRESENCE_VALIDATION: &str = "UserPresenceValidation";

pub const COMMUNICATION_MESH_CAPABILITY: &str = "communication_mesh";

pub const STORAGE_SERVICES_CAPABILITY: &str = "storage_services";

pub const COMPUTE_ORCHESTRATION_CAPABILITY: &str = "compute_orchestration";

pub const AI_INTELLIGENCE_CAPABILITY: &str = "ai_intelligence";

pub const SECURITY_PROVIDER_CAPABILITY: &str = "security_provider";

pub const SYSTEM_INTEGRATION_CAPABILITY: &str = "system_integration";

pub const HSM_CAPABILITY: &str = "hardware_security_module";

pub const KEY_MANAGEMENT_CAPABILITY: &str = "key_management";

pub const SECURE_ENCLAVE_CAPABILITY: &str = "secure_enclave";

pub const MAX_HSM_KEYS: usize = 1000;

pub const STANDARD_KEY_SIZE_LIMIT: usize = 4096;

pub const ZERO_COST_SOFTWARE_HSM_PROVIDER: &str = "ZeroCostSoftwareHSM";

pub const MAX_KEY_SIZE: usize = STANDARD_KEY_SIZE_LIMIT;

pub const HSM_OPERATION_TIMEOUT: Duration = Duration::from_secs(30);

pub const HSM_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(60);

pub const KEY_ROTATION_INTERVAL: Duration = Duration::from_secs(86400 * 30); // 30 days

pub const HSM_CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);

pub const HSM_DISCOVERY_TIMEOUT: Duration = Duration::from_secs(5);

pub const MIN_ENTROPY_BITS: u32 = 256;

pub const DEFAULT_KEY_DERIVATION_ITERATIONS: u32 = 100_000;

pub const HSM_AUTH_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes

pub const MAX_HSM_AUTH_ATTEMPTS: u32 = 3;

pub const SOFTWARE_HSM_PROVIDER: &str = "software";

pub const HARDWARE_HSM_PROVIDER: &str = "hardware";

pub const MOBILE_HSM_PROVIDER: &str = "mobile";

pub const CLOUD_HSM_PROVIDER: &str = "cloud";

pub const NETWORK_HSM_PROVIDER: &str = "network";

pub const USB_HSM_PROVIDER: &str = "usb";

pub const TPM_PROVIDER: &str = "tpm";

pub const SMARTCARD_PROVIDER: &str = "smartcard";
