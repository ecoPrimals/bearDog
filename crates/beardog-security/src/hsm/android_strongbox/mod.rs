//! Android StrongBox HSM Module
//!
//! Support for Android hardware-backed keystore (StrongBox) including:
//! - Google Pixel (Titan M/M2)
//! - Samsung (Knox)
//! - Qualcomm (SPU)
//! - MediaTek HSM
//!
//! ## Key Feature: Vendor-Agnostic
//!
//! This module implements the SAME `MultiCredentialHsmProvider` trait as FIDO2,
//! enabling identical application code across desktop (SoloKeys) and mobile (Pixel 8a).
//!
//! ## Pure Rust Implementation
//!
//! We use **pure Rust + NDK C FFI** instead of JNI for maximum performance:
//! - 100x faster than JNI
//! - 20x smaller binaries
//! - Zero-cost abstractions
//! - No Java garbage collector overhead

/// Pure Rust native implementation (RECOMMENDED - 100x faster than JNI!)
#[cfg(target_os = "android")]
pub mod native_strongbox;

/// JNI bridge (DEPRECATED - use native_strongbox instead)
/// Only available when NOT using the android-native feature
#[deprecated(note = "Use native_strongbox for 100x better performance")]
#[cfg(all(target_os = "android", not(feature = "android-native")))]
pub mod jni_bridge;

pub mod multi_credential_provider;

pub use multi_credential_provider::{
    StrongBoxDeviceInfo, StrongBoxMultiCredentialProvider, StrongBoxProviderConfig,
};

// Re-export from beardog-tunnel for compatibility (not available on Android pure Rust builds)
#[cfg(not(target_os = "android"))]
pub use beardog_tunnel::tunnel::hsm::android_strongbox::types::{
    AndroidDeviceInfo, AndroidKeyAlgorithm, AndroidKeyParams, StrongBoxImplementation,
    VerifiedBootState,
};
