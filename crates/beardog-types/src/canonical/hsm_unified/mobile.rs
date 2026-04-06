// SPDX-License-Identifier: AGPL-3.0-or-later

// Mobile HSM Configuration

use serde::{Deserialize, Serialize};

/// Mobile HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MobileHsmConfig {
    /// Android `StrongBox` enabled
    pub android_strongbox_enabled: bool,

    /// iOS Secure Enclave enabled
    /// Whether `ios_secure_enclave` is enabled
    pub ios_secure_enclave_enabled: bool,

    /// Biometric authentication enabled
    /// Whether `biometric_auth` is enabled
    pub biometric_auth_enabled: bool,
}
