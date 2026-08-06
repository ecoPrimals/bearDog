// SPDX-License-Identifier: AGPL-3.0-or-later

//! # libtower — C ABI for `BearDog` Tower Atomic crypto
//!
//! Chimera Phase 0: shared library (`libtower.so` / `libtower.dylib` / `tower.dll`)
//! exposing `BearDog`'s pure Rust crypto primitives via a stable C ABI.
//!
//! Other primals can `dlopen` this library for hot-path crypto without JSON-RPC
//! overhead, while the JSON-RPC IPC path remains the primary integration method.
//!
//! ## Thread Safety
//!
//! All exported functions are thread-safe. Error messages use thread-local storage.
//!
//! ## Error Model
//!
//! Functions return `0` on success, negative values on error. Call `tower_last_error`
//! to retrieve a NUL-terminated error message string (valid until the next call on
//! the same thread).

#![allow(
    unsafe_code,
    reason = "C ABI exports require unsafe for raw pointer and slice operations"
)]

use std::cell::RefCell;
use std::ptr;
use std::slice;

thread_local! {
    static LAST_ERROR: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

fn set_error(msg: &str) {
    LAST_ERROR.with(|e| {
        let mut buf = e.borrow_mut();
        buf.clear();
        buf.extend_from_slice(msg.as_bytes());
        buf.push(0);
    });
}

/// Returns a pointer to the last error message (NUL-terminated, thread-local).
///
/// Returns null if no error has occurred. Valid until the next libtower call
/// on the same thread.
///
/// # Safety
///
/// The returned pointer is only valid until the next libtower function call
/// on the same thread.
#[unsafe(no_mangle)]
pub extern "C" fn tower_last_error() -> *const u8 {
    LAST_ERROR.with(|e| {
        let buf = e.borrow();
        if buf.is_empty() { ptr::null() } else { buf.as_ptr() }
    })
}

/// Returns the library version as a NUL-terminated string.
///
/// # Safety
///
/// The returned pointer is a static string and is always valid.
#[unsafe(no_mangle)]
pub const extern "C" fn tower_version() -> *const u8 {
    c"0.9.0".as_ptr().cast()
}

// ── Hashing ──────────────────────────────────────────────────────────────

/// Compute BLAKE3 hash of `data_ptr[0..data_len]`.
///
/// Writes 32 bytes to `out_ptr`. Returns 0 on success, -1 on error.
///
/// # Safety
///
/// `data_ptr` must point to at least `data_len` readable bytes.
/// `out_ptr` must point to at least 32 writable bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tower_hash_blake3(
    data_ptr: *const u8,
    data_len: usize,
    out_ptr: *mut u8,
) -> i32 {
    if data_ptr.is_null() || out_ptr.is_null() {
        set_error("null pointer");
        return -1;
    }
    let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    let hash = beardog_crypto::hash_blake3(data);
    let out = unsafe { slice::from_raw_parts_mut(out_ptr, 32) };
    let len = hash.len().min(32);
    out[..len].copy_from_slice(&hash[..len]);
    0
}

/// Compute HMAC-SHA256 of `data_ptr[0..data_len]` with `key_ptr[0..key_len]`.
///
/// Writes 32 bytes to `out_ptr`. Returns 0 on success, -1 on error.
///
/// # Safety
///
/// All pointers must be valid for their respective lengths.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tower_hmac_sha256(
    data_ptr: *const u8,
    data_len: usize,
    key_ptr: *const u8,
    key_len: usize,
    out_ptr: *mut u8,
) -> i32 {
    if data_ptr.is_null() || key_ptr.is_null() || out_ptr.is_null() {
        set_error("null pointer");
        return -1;
    }
    let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    let key = unsafe { slice::from_raw_parts(key_ptr, key_len) };
    match beardog_crypto::hmac_sha256(data, key) {
        Ok(mac) => {
            let out = unsafe { slice::from_raw_parts_mut(out_ptr, 32) };
            let len = mac.len().min(32);
            out[..len].copy_from_slice(&mac[..len]);
            0
        }
        Err(e) => {
            set_error(&e.to_string());
            -1
        }
    }
}

// ── Symmetric Encryption ─────────────────────────────────────────────────

/// Encrypt with ChaCha20-Poly1305.
///
/// On success, writes ciphertext to `out_ptr`, nonce (12 bytes) to `nonce_out`,
/// tag (16 bytes) to `tag_out`, and ciphertext length to `out_len`. Returns 0.
///
/// `out_ptr` must have space for at least `data_len` bytes.
///
/// # Safety
///
/// All pointers must be valid. `key_ptr` must point to exactly 32 bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tower_encrypt_chacha20(
    data_ptr: *const u8,
    data_len: usize,
    key_ptr: *const u8,
    nonce_out: *mut u8,
    tag_out: *mut u8,
    out_ptr: *mut u8,
    out_len: *mut usize,
) -> i32 {
    if data_ptr.is_null() || key_ptr.is_null() || out_ptr.is_null() {
        set_error("null pointer");
        return -1;
    }
    let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    let key_slice = unsafe { slice::from_raw_parts(key_ptr, 32) };
    let Ok(key): Result<&[u8; 32], _> = key_slice.try_into() else {
        set_error("key must be 32 bytes");
        return -1;
    };

    match beardog_crypto::encrypt_chacha20_poly1305(data, key, None) {
        Ok((ciphertext, nonce, tag)) => {
            let out = unsafe { slice::from_raw_parts_mut(out_ptr, ciphertext.len()) };
            out.copy_from_slice(&ciphertext);
            if !nonce_out.is_null() {
                let n = unsafe { slice::from_raw_parts_mut(nonce_out, 12) };
                let nlen = nonce.len().min(12);
                n[..nlen].copy_from_slice(&nonce[..nlen]);
            }
            if !tag_out.is_null() {
                let t = unsafe { slice::from_raw_parts_mut(tag_out, 16) };
                let tlen = tag.len().min(16);
                t[..tlen].copy_from_slice(&tag[..tlen]);
            }
            if !out_len.is_null() {
                unsafe { *out_len = ciphertext.len(); }
            }
            0
        }
        Err(e) => {
            set_error(&e.to_string());
            -1
        }
    }
}

// ── Signatures ───────────────────────────────────────────────────────────

/// Sign data with Ed25519.
///
/// Writes 64-byte signature to `sig_out`. Returns 0 on success.
///
/// # Safety
///
/// `data_ptr` must point to `data_len` bytes. `secret_key_ptr` must point to
/// 64 bytes (Ed25519 expanded secret key). `sig_out` must have 64 bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tower_sign_ed25519(
    data_ptr: *const u8,
    data_len: usize,
    secret_key_ptr: *const u8,
    sig_out: *mut u8,
) -> i32 {
    if data_ptr.is_null() || secret_key_ptr.is_null() || sig_out.is_null() {
        set_error("null pointer");
        return -1;
    }
    let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    let sk_slice = unsafe { slice::from_raw_parts(secret_key_ptr, 64) };
    let Ok(sk): Result<&[u8; 64], _> = sk_slice.try_into() else {
        set_error("secret key must be 64 bytes");
        return -1;
    };

    match beardog_crypto::sign_ed25519(data, sk) {
        Ok(sig) => {
            let out = unsafe { slice::from_raw_parts_mut(sig_out, 64) };
            let slen = sig.len().min(64);
            out[..slen].copy_from_slice(&sig[..slen]);
            0
        }
        Err(e) => {
            set_error(&e.to_string());
            -1
        }
    }
}

/// Verify an Ed25519 signature.
///
/// Returns 0 if valid, -1 on error, 1 if signature is invalid.
///
/// # Safety
///
/// `data_ptr` must point to `data_len` bytes. `sig_ptr` must point to 64 bytes.
/// `pubkey_ptr` must point to 32 bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tower_verify_ed25519(
    data_ptr: *const u8,
    data_len: usize,
    sig_ptr: *const u8,
    pubkey_ptr: *const u8,
) -> i32 {
    if data_ptr.is_null() || sig_ptr.is_null() || pubkey_ptr.is_null() {
        set_error("null pointer");
        return -1;
    }
    let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    let sig = unsafe { slice::from_raw_parts(sig_ptr, 64) };
    let pubkey = unsafe { slice::from_raw_parts(pubkey_ptr, 32) };

    match beardog_crypto::verify_ed25519(data, sig, pubkey) {
        Ok(valid) => i32::from(!valid),
        Err(e) => {
            set_error(&e.to_string());
            -1
        }
    }
}

// ── Discovery ────────────────────────────────────────────────────────────

/// Write the library's capability list as a JSON string to `buf_ptr`.
///
/// On success, writes the JSON string (NUL-terminated) and sets `*buf_len`
/// to the string length (excluding NUL). Returns 0.
///
/// If `buf_ptr` is null, only sets `*buf_len` to the required size (for sizing).
///
/// # Safety
///
/// `buf_len` must be non-null. If `buf_ptr` is non-null, it must have
/// `*buf_len` writable bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tower_capabilities(
    buf_ptr: *mut u8,
    buf_len: *mut usize,
) -> i32 {
    if buf_len.is_null() {
        set_error("buf_len is null");
        return -1;
    }

    let caps = beardog_crypto::discover_algorithms();
    let json = match serde_json::to_string(&caps) {
        Ok(j) => j,
        Err(e) => { set_error(&e.to_string()); return -1; }
    };
    let needed = json.len() + 1;

    if buf_ptr.is_null() {
        unsafe { *buf_len = needed; }
        return 0;
    }

    let available = unsafe { *buf_len };
    if available < needed {
        unsafe { *buf_len = needed; }
        set_error("buffer too small");
        return -2;
    }

    let out = unsafe { slice::from_raw_parts_mut(buf_ptr, needed) };
    out[..json.len()].copy_from_slice(json.as_bytes());
    out[json.len()] = 0;
    unsafe { *buf_len = json.len(); }
    0
}
