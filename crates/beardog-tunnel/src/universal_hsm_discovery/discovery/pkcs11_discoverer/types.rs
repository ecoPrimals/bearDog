// SPDX-License-Identifier: AGPL-3.0-only

//! PKCS#11-specific data shapes and well-known library paths.

use std::path::PathBuf;

/// Known PKCS#11 library locations by platform
pub(crate) const PKCS11_COMMON_PATHS: &[&str] = &[
    // SoftHSM
    "/usr/lib/softhsm/libsofthsm2.so",
    "/usr/lib/x86_64-linux-gnu/softhsm/libsofthsm2.so",
    "/usr/local/lib/softhsm/libsofthsm2.so",
    "/opt/homebrew/lib/softhsm/libsofthsm2.so",
    // OpenSC (smartcard)
    "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so",
    "/usr/lib/opensc-pkcs11.so",
    "/usr/local/lib/opensc-pkcs11.so",
    // TPM PKCS#11
    "/usr/lib/x86_64-linux-gnu/libtpm2_pkcs11.so",
    "/usr/lib/libtpm2_pkcs11.so",
    // YubiKey
    "/usr/lib/x86_64-linux-gnu/libykcs11.so",
    "/usr/local/lib/libykcs11.so",
    // Thales/nCipher
    "/opt/nfast/toolkits/pkcs11/libcknfast.so",
    // Utimaco
    "/opt/utimaco/lib/libcs_pkcs11_R2.so",
    // AWS CloudHSM
    "/opt/cloudhsm/lib/libcloudhsm_pkcs11.so",
];

/// PKCS#11 token information
#[derive(Debug, Clone)]
pub struct Pkcs11TokenInfo {
    /// Library path
    pub library_path: PathBuf,
    /// Token label
    pub label: String,
    /// Manufacturer ID
    pub manufacturer_id: String,
    /// Model
    pub model: String,
    /// Serial number
    pub serial_number: String,
    /// Slot ID
    pub slot_id: u64,
}
