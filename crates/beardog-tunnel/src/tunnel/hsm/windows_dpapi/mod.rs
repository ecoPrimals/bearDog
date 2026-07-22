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
//! that `HsmKeyProviderBackend` can reference the type unconditionally
//! inside `#[cfg(windows)]` arms.  On non-Windows hosts, `is_available()`
//! returns `false` and every operation returns an error.

mod hsm_key_provider;

use beardog_errors::BearDogError;
use std::collections::BTreeMap;
use std::path::PathBuf;
use tokio::sync::RwLock;

/// Metadata for a single DPAPI-protected key stored on disk.
#[derive(Debug, Clone)]
struct DpapiKeyEntry {
    key_id: String,
    algorithm: beardog_types::hsm::HsmAlgorithm,
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
            Ok(PathBuf::from("/tmp/beardog-dpapi-stub"))
        }
    }

    /// Encrypt raw key material with DPAPI (Windows).
    ///
    /// # Safety invariants (for `unsafe` FFI blocks below)
    ///
    /// - `input.pbData` points to a valid, live `plaintext` slice for the duration of the call.
    /// - `output.pbData` is set by `CryptProtectData` on success; we copy its contents
    ///   into a `Vec<u8>` immediately and free via `LocalFree` before returning.
    /// - `from_raw_parts` reads exactly `output.cbData` bytes from `output.pbData`,
    ///   which is the contract documented by the Windows API.
    ///
    /// When cross-compiling for Windows, `#![forbid(unsafe_code)]` in `lib.rs` must
    /// be relaxed for this module (e.g. `#[allow(unsafe_code)]` on the containing mod).
    #[cfg(windows)]
    fn dpapi_protect(plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        use std::ptr;
        use windows_sys::Win32::Security::Cryptography::{CRYPTOAPI_BLOB, CryptProtectData};

        let mut input = CRYPTOAPI_BLOB {
            cbData: u32::try_from(plaintext.len()).unwrap_or(u32::MAX),
            pbData: plaintext.as_ptr().cast_mut(),
        };
        let mut output = CRYPTOAPI_BLOB {
            cbData: 0,
            pbData: ptr::null_mut(),
        };

        let ok = unsafe {
            CryptProtectData(
                &mut input,
                ptr::null(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                &mut output,
            )
        };
        if ok == 0 {
            return Err(BearDogError::internal(
                "CryptProtectData failed".to_string(),
            ));
        }

        let protected =
            unsafe { std::slice::from_raw_parts(output.pbData, output.cbData as usize) }.to_vec();
        unsafe { windows_sys::Win32::System::Memory::LocalFree(output.pbData.cast()) };
        Ok(protected)
    }

    /// Decrypt a DPAPI-protected blob back to plaintext key material.
    ///
    /// Same safety invariants as [`dpapi_protect`] — `output.pbData` is valid
    /// for `output.cbData` bytes after a successful `CryptUnprotectData` call,
    /// copied immediately, and freed via `LocalFree`.
    #[cfg(windows)]
    fn dpapi_unprotect(protected: &[u8]) -> Result<Vec<u8>, BearDogError> {
        use std::ptr;
        use windows_sys::Win32::Security::Cryptography::{CRYPTOAPI_BLOB, CryptUnprotectData};

        let mut input = CRYPTOAPI_BLOB {
            cbData: u32::try_from(protected.len()).unwrap_or(u32::MAX),
            pbData: protected.as_ptr().cast_mut(),
        };
        let mut output = CRYPTOAPI_BLOB {
            cbData: 0,
            pbData: ptr::null_mut(),
        };

        let ok = unsafe {
            CryptUnprotectData(
                &mut input,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                &mut output,
            )
        };
        if ok == 0 {
            return Err(BearDogError::internal(
                "CryptUnprotectData failed".to_string(),
            ));
        }

        let plaintext =
            unsafe { std::slice::from_raw_parts(output.pbData, output.cbData as usize) }.to_vec();
        unsafe { windows_sys::Win32::System::Memory::LocalFree(output.pbData.cast()) };
        Ok(plaintext)
    }

    /// Persist a DPAPI-protected key blob to disk.
    fn blob_path(&self, key_id: &str) -> PathBuf {
        self.storage_dir.join(format!("{key_id}.dpapi"))
    }

    /// Store a DPAPI-protected blob for `key_id`.
    async fn store_blob(&self, key_id: &str, blob: &[u8]) -> Result<(), BearDogError> {
        let path = self.blob_path(key_id);
        tokio::fs::write(&path, blob).await.map_err(|e| {
            BearDogError::internal(format!("Failed to write DPAPI blob for {key_id}: {e}"))
        })
    }

    /// Load a DPAPI-protected blob for `key_id`.
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
