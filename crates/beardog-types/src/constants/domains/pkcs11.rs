// SPDX-License-Identifier: AGPL-3.0-only

//! PKCS#11 Constants
//!
//! Standard PKCS#11 (Cryptoki) return codes and constants.
//! Reference: PKCS #11 Cryptographic Token Interface Base Specification
//!
//! ## Return Codes (CKR_*)
//!
//! These are the standard return values from PKCS#11 operations.
//! All codes are defined in the PKCS#11 specification.

/// PKCS#11 return codes
pub mod return_codes {
    /// Success - the operation completed successfully
    pub const CKR_OK: u32 = 0x0000_0000;

    /// Operation was cancelled by user
    pub const CKR_CANCEL: u32 = 0x0000_0001;

    /// Host memory allocation failed
    pub const CKR_HOST_MEMORY: u32 = 0x0000_0002;

    /// Slot ID is invalid for this system
    pub const CKR_SLOT_ID_INVALID: u32 = 0x0000_0003;

    /// General unspecified error
    pub const CKR_GENERAL_ERROR: u32 = 0x0000_0005;

    /// Function failed for an unspecified reason
    pub const CKR_FUNCTION_FAILED: u32 = 0x0000_0006;

    /// Invalid or bad arguments provided
    pub const CKR_ARGUMENTS_BAD: u32 = 0x0000_0007;

    /// No event is available
    pub const CKR_NO_EVENT: u32 = 0x0000_0008;

    /// Module needs to create threads but cannot
    pub const CKR_NEED_TO_CREATE_THREADS: u32 = 0x0000_0009;

    /// Module cannot lock resources
    pub const CKR_CANT_LOCK: u32 = 0x0000_000A;

    /// Attribute type is invalid
    pub const CKR_ATTRIBUTE_TYPE_INVALID: u32 = 0x0000_0012;

    /// Attribute value is invalid
    pub const CKR_ATTRIBUTE_VALUE_INVALID: u32 = 0x0000_0013;

    /// Operation is not valid in current state
    pub const CKR_OPERATION_ACTIVE: u32 = 0x0000_0090;

    /// No operation is currently active
    pub const CKR_OPERATION_NOT_INITIALIZED: u32 = 0x0000_0091;

    /// User PIN is not yet set
    pub const CKR_USER_PIN_NOT_INITIALIZED: u32 = 0x0000_0092;

    /// User PIN is incorrect
    pub const CKR_PIN_INCORRECT: u32 = 0x0000_00A0;

    /// User PIN is locked
    pub const CKR_PIN_LOCKED: u32 = 0x0000_00A4;

    /// User PIN has expired
    pub const CKR_PIN_EXPIRED: u32 = 0x0000_00A3;

    /// Session handle is invalid
    pub const CKR_SESSION_HANDLE_INVALID: u32 = 0x0000_00B3;

    /// Object handle is invalid  
    pub const CKR_OBJECT_HANDLE_INVALID: u32 = 0x0000_0082;

    /// Token is not present in the slot
    pub const CKR_TOKEN_NOT_PRESENT: u32 = 0x0000_00E0;

    /// Token is write-protected
    pub const CKR_TOKEN_WRITE_PROTECTED: u32 = 0x0000_00B6;
}

/// Helper function to get human-readable description of return code
pub const fn return_code_description(code: u32) -> &'static str {
    use return_codes::{
        CKR_ARGUMENTS_BAD, CKR_ATTRIBUTE_TYPE_INVALID, CKR_ATTRIBUTE_VALUE_INVALID, CKR_CANCEL,
        CKR_CANT_LOCK, CKR_FUNCTION_FAILED, CKR_GENERAL_ERROR, CKR_HOST_MEMORY,
        CKR_NEED_TO_CREATE_THREADS, CKR_NO_EVENT, CKR_OBJECT_HANDLE_INVALID, CKR_OK,
        CKR_OPERATION_ACTIVE, CKR_OPERATION_NOT_INITIALIZED, CKR_PIN_EXPIRED, CKR_PIN_INCORRECT,
        CKR_PIN_LOCKED, CKR_SESSION_HANDLE_INVALID, CKR_SLOT_ID_INVALID, CKR_TOKEN_NOT_PRESENT,
        CKR_TOKEN_WRITE_PROTECTED, CKR_USER_PIN_NOT_INITIALIZED,
    };

    match code {
        CKR_OK => "Success",
        CKR_CANCEL => "Operation cancelled",
        CKR_HOST_MEMORY => "Host memory allocation failed",
        CKR_SLOT_ID_INVALID => "Invalid slot ID",
        CKR_GENERAL_ERROR => "General error",
        CKR_FUNCTION_FAILED => "Function failed",
        CKR_ARGUMENTS_BAD => "Invalid arguments",
        CKR_NO_EVENT => "No event available",
        CKR_NEED_TO_CREATE_THREADS => "Cannot create threads",
        CKR_CANT_LOCK => "Cannot lock resources",
        CKR_ATTRIBUTE_TYPE_INVALID => "Invalid attribute type",
        CKR_ATTRIBUTE_VALUE_INVALID => "Invalid attribute value",
        CKR_OPERATION_ACTIVE => "Operation already active",
        CKR_OPERATION_NOT_INITIALIZED => "Operation not initialized",
        CKR_USER_PIN_NOT_INITIALIZED => "User PIN not set",
        CKR_PIN_INCORRECT => "Incorrect PIN",
        CKR_PIN_LOCKED => "PIN locked",
        CKR_PIN_EXPIRED => "PIN expired",
        CKR_SESSION_HANDLE_INVALID => "Invalid session handle",
        CKR_OBJECT_HANDLE_INVALID => "Invalid object handle",
        CKR_TOKEN_NOT_PRESENT => "Token not present",
        CKR_TOKEN_WRITE_PROTECTED => "Token write-protected",
        _ => "Unknown error code",
    }
}

/// PKCS#11 object classes
/// Note: These constants match the PKCS#11 specification exactly
#[expect(
    clippy::unreadable_literal,
    reason = "PKCS#11 constants match spec values"
)]
pub mod object_classes {
    /// Data object
    pub const CKO_DATA: u32 = 0x00000000;

    /// Certificate object
    pub const CKO_CERTIFICATE: u32 = 0x00000001;

    /// Public key object
    pub const CKO_PUBLIC_KEY: u32 = 0x00000002;

    /// Private key object
    pub const CKO_PRIVATE_KEY: u32 = 0x00000003;

    /// Secret key object
    pub const CKO_SECRET_KEY: u32 = 0x00000004;
}

/// PKCS#11 key types
/// Note: These constants match the PKCS#11 specification exactly
#[expect(
    clippy::unreadable_literal,
    reason = "PKCS#11 constants match spec values"
)]
pub mod key_types {
    /// RSA key
    pub const CKK_RSA: u32 = 0x00000000;

    /// DSA key
    pub const CKK_DSA: u32 = 0x00000001;

    /// Diffie-Hellman key
    pub const CKK_DH: u32 = 0x00000002;

    /// Elliptic Curve key
    pub const CKK_EC: u32 = 0x00000003;

    /// AES key
    pub const CKK_AES: u32 = 0x0000001F;

    /// Generic secret key
    pub const CKK_GENERIC_SECRET: u32 = 0x00000010;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_return_code_descriptions() {
        assert_eq!(return_code_description(return_codes::CKR_OK), "Success");
        assert_eq!(
            return_code_description(return_codes::CKR_PIN_INCORRECT),
            "Incorrect PIN"
        );
        #[expect(
            clippy::unreadable_literal,
            reason = "PKCS#11 constants match spec values"
        )]
        {
            assert_eq!(return_code_description(0xFFFFFFFF), "Unknown error code");
        }
    }
}
