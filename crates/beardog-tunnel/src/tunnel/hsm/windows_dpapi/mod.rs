// SPDX-License-Identifier: AGPL-3.0-or-later

//! Windows DPAPI HSM backend.
//!
//! Stores key material encrypted via the Windows Data Protection API
//! (`CryptProtectData` / `CryptUnprotectData`), binding keys to the
//! current user's login credentials.  Crypto operations (AES-GCM,
//! Ed25519, etc.) are performed in-process using `RustCrypto` after
//! unwrapping the DPAPI-protected key blob.
//!
//! ## Platform gating
//!
//! The struct and its `HsmKeyProvider` impl compile on all platforms so
//! that `HsmKeyProviderBackend` can reference the type unconditionally.
//! On non-Windows hosts, `is_available()` returns `false` and every
//! operation returns an error.
//!
//! ## Unsafe code
//!
//! Windows DPAPI requires FFI calls to CryptProtectData/CryptUnprotectData
//! (gated to `#[cfg(windows)]` functions below).

mod hsm_key_provider;

use beardog_errors::BearDogError;
use std::collections::BTreeMap;
use std::path::PathBuf;
use tokio::sync::RwLock;

/// Win32 `DATA_BLOB` / `CRYPTOAPI_BLOB` / `CRYPT_INTEGER_BLOB` — all are the
/// same layout.  Defined here to avoid `windows-sys` feature discovery issues
/// across binding generator versions.
#[cfg(windows)]
#[repr(C)]
struct DataBlob {
    cb_data: u32,
    pb_data: *mut u8,
}

/// RAII guard for DPAPI output buffers allocated by the Windows API.
///
/// `CryptProtectData` / `CryptUnprotectData` return heap memory that must be
/// freed with `LocalFree`.  This type ensures cleanup on drop, including early
/// returns after a successful FFI call.
#[cfg(windows)]
struct DpapiBlob {
    data: *mut u8,
    len: usize,
}

#[cfg(windows)]
#[expect(
    unsafe_code,
    reason = "Windows DPAPI FFI requires unsafe for CryptProtectData/CryptUnprotectData bindings"
)]
impl DpapiBlob {
    /// Copies the DPAPI output into an owned `Vec`.
    fn to_vec(&self) -> Vec<u8> {
        if self.data.is_null() || self.len == 0 {
            return Vec::new();
        }
        // SAFETY: `data` points to `len` valid bytes returned by a successful
        // CryptProtectData / CryptUnprotectData call.
        unsafe { std::slice::from_raw_parts(self.data, self.len) }.to_vec()
    }
}

#[cfg(windows)]
#[expect(
    unsafe_code,
    reason = "Windows DPAPI FFI requires unsafe for CryptProtectData/CryptUnprotectData bindings"
)]
impl Drop for DpapiBlob {
    fn drop(&mut self) {
        if !self.data.is_null() {
            // SAFETY: `data` was allocated by CryptProtectData / CryptUnprotectData
            // and must be released with LocalFree per the Windows API contract.
            unsafe { LocalFree(self.data.cast()) };
        }
    }
}

#[cfg(windows)]
#[expect(
    unsafe_code,
    reason = "Windows DPAPI FFI requires unsafe for CryptProtectData/CryptUnprotectData bindings"
)]
unsafe extern "system" {
    fn CryptProtectData(
        p_data_in: *const DataBlob,
        sz_data_descr: *const u16,
        p_optional_entropy: *const DataBlob,
        pv_reserved: *const core::ffi::c_void,
        p_prompt_struct: *const core::ffi::c_void,
        dw_flags: u32,
        p_data_out: *mut DataBlob,
    ) -> i32;

    fn CryptUnprotectData(
        p_data_in: *const DataBlob,
        ppsz_data_descr: *mut *mut u16,
        p_optional_entropy: *const DataBlob,
        pv_reserved: *const core::ffi::c_void,
        p_prompt_struct: *const core::ffi::c_void,
        dw_flags: u32,
        p_data_out: *mut DataBlob,
    ) -> i32;

    fn LocalFree(hmem: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
}

/// Metadata for a single DPAPI-protected key stored on disk.
#[derive(Debug, Clone)]
struct DpapiKeyEntry {
    #[expect(dead_code, reason = "retained for key listing and audit logging")]
    key_id: String,
    #[expect(dead_code, reason = "retained for algorithm-specific crypto operations")]
    algorithm: beardog_types::hsm::HsmAlgorithm,
    #[expect(dead_code, reason = "retained for key expiration policy")]
    created_at_ms: u64,
}

/// Windows DPAPI HSM backend.
///
/// On Windows, keys are generated in-process via `OsRng`, encrypted
/// with `CryptProtectData` (user-scope), and persisted as opaque blobs
/// under `%LOCALAPPDATA%/beardog/keys/<key_id>.dpapi`.
///
/// On non-Windows platforms the provider reports as unavailable and all
/// operations return `BearDogError::not_yet_available`.
#[derive(Debug)]
pub struct WindowsDpapiHsm {
    keys: RwLock<BTreeMap<String, DpapiKeyEntry>>,
    storage_dir: PathBuf,
}

impl WindowsDpapiHsm {
    /// Create a new DPAPI HSM, resolving the key storage directory from
    /// the platform-appropriate local app data location.
    ///
    /// # Errors
    ///
    /// Returns an error if the storage directory cannot be determined or created.
    pub fn new() -> Result<Self, BearDogError> {
        let storage_dir = Self::resolve_storage_dir()?;
        Ok(Self {
            keys: RwLock::new(BTreeMap::new()),
            storage_dir,
        })
    }

    fn resolve_storage_dir() -> Result<PathBuf, BearDogError> {
        #[cfg(windows)]
        {
            let base = std::env::var("LOCALAPPDATA").map_err(|_| {
                BearDogError::internal(
                    "LOCALAPPDATA not set — cannot determine DPAPI key storage path".to_string(),
                )
            })?;
            let dir = PathBuf::from(base).join("beardog").join("keys");
            std::fs::create_dir_all(&dir).map_err(|e| {
                BearDogError::internal(format!("Failed to create DPAPI key dir: {e}"))
            })?;
            Ok(dir)
        }
        #[cfg(not(windows))]
        {
            Ok(std::env::temp_dir().join("beardog-dpapi-stub"))
        }
    }

    /// Encrypt raw key material with DPAPI (Windows).
    #[cfg(windows)]
    #[expect(
    unsafe_code,
    reason = "Windows DPAPI FFI requires unsafe for CryptProtectData/CryptUnprotectData bindings"
)]
    fn dpapi_protect(plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        use std::ptr;

        let mut input = DataBlob {
            cb_data: u32::try_from(plaintext.len()).unwrap_or(u32::MAX),
            pb_data: plaintext.as_ptr().cast_mut(),
        };
        let mut output = DataBlob {
            cb_data: 0,
            pb_data: ptr::null_mut(),
        };

        let ok = unsafe {
            // SAFETY: `input.pb_data` points to a valid, live `plaintext` slice for the
            // duration of the call.  Optional pointer parameters are null as documented.
            CryptProtectData(
                &mut input,
                ptr::null(),
                ptr::null(),
                ptr::null(),
                ptr::null(),
                0,
                &mut output,
            )
        };
        if ok == 0 {
            return Err(BearDogError::internal(
                "CryptProtectData failed".to_string(),
            ));
        }

        let blob = DpapiBlob {
            data: output.pb_data,
            len: output.cb_data as usize,
        };
        Ok(blob.to_vec())
    }

    /// Decrypt a DPAPI-protected blob back to plaintext key material.
    #[cfg(windows)]
    #[expect(
    unsafe_code,
    reason = "Windows DPAPI FFI requires unsafe for CryptProtectData/CryptUnprotectData bindings"
)]
    fn dpapi_unprotect(protected: &[u8]) -> Result<Vec<u8>, BearDogError> {
        use std::ptr;

        let mut input = DataBlob {
            cb_data: u32::try_from(protected.len()).unwrap_or(u32::MAX),
            pb_data: protected.as_ptr().cast_mut(),
        };
        let mut output = DataBlob {
            cb_data: 0,
            pb_data: ptr::null_mut(),
        };

        let ok = unsafe {
            // SAFETY: `input.pb_data` points to a valid, live `protected` slice for the
            // duration of the call.  Optional pointer parameters are null as documented.
            CryptUnprotectData(
                &mut input,
                ptr::null_mut(),
                ptr::null(),
                ptr::null(),
                ptr::null(),
                0,
                &mut output,
            )
        };
        if ok == 0 {
            return Err(BearDogError::internal(
                "CryptUnprotectData failed".to_string(),
            ));
        }

        let blob = DpapiBlob {
            data: output.pb_data,
            len: output.cb_data as usize,
        };
        Ok(blob.to_vec())
    }

    /// Persist a DPAPI-protected key blob to disk.
    fn blob_path(&self, key_id: &str) -> PathBuf {
        self.storage_dir.join(format!("{key_id}.dpapi"))
    }

    /// Store a DPAPI-protected blob for `key_id`.
    #[expect(dead_code, reason = "used by Windows DPAPI key generation path")]
    async fn store_blob(&self, key_id: &str, blob: &[u8]) -> Result<(), BearDogError> {
        let path = self.blob_path(key_id);
        tokio::fs::write(&path, blob).await.map_err(|e| {
            BearDogError::internal(format!("Failed to write DPAPI blob for {key_id}: {e}"))
        })
    }

    /// Load a DPAPI-protected blob for `key_id`.
    #[expect(dead_code, reason = "used by Windows DPAPI key unwrap path")]
    async fn load_blob(&self, key_id: &str) -> Result<Vec<u8>, BearDogError> {
        let path = self.blob_path(key_id);
        tokio::fs::read(&path).await.map_err(|e| {
            BearDogError::internal(format!("Failed to read DPAPI blob for {key_id}: {e}"))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsm_reports_unavailable_on_non_windows() {
        if !cfg!(windows) {
            let hsm = WindowsDpapiHsm::new().expect("stub construction");
            assert!(!beardog_traits::hsm::HsmKeyProvider::is_available(&hsm));
        }
    }

    #[test]
    fn blob_path_uses_key_id() {
        let hsm = WindowsDpapiHsm::new().expect("stub construction");
        let p = hsm.blob_path("my-key");
        assert!(p.to_string_lossy().contains("my-key.dpapi"));
    }
}
