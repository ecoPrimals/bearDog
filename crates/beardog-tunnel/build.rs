// SPDX-License-Identifier: AGPL-3.0-only
//! Build script for the `beardog-tunnel` crate.
//!
//! Configures platform-specific native linking for tunnel and StrongBox-related code paths.
//! On **Android**, links NDK libraries (`log`, `android`, `keystore`) and sets
//! `cfg(feature = "android_native")` when appropriate. On other targets, consumers rely on
//! non-Android targets omit the Android `StrongBox` module (see `cargo:warning` during build).
//!
//! Environment variables observed: `ANDROID_NDK_HOME`, `CARGO_CFG_TARGET_OS`.

/// Cargo `build.rs` entry: emits link flags, search paths, and `cfg` for the library target.
fn main() {
    #[cfg(target_os = "android")]
    {
        println!("cargo:rerun-if-changed=build.rs");

        println!("cargo:rustc-link-lib=log"); // Android logging
        println!("cargo:rustc-link-lib=android"); // Android NDK
        println!("cargo:rustc-link-lib=keystore"); // Android Keystore

        println!("cargo:rustc-link-arg=-Wl,--allow-shlib-undefined");

        if let Ok(ndk_home) = std::env::var("ANDROID_NDK_HOME") {
            println!(
                "cargo:rustc-link-search=native={}/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib",
                ndk_home
            );
        }

        println!("cargo:rustc-cfg=feature=\"android_native\"");
        println!("cargo:warning=Building for Android with StrongBox support");
    }

    #[cfg(not(target_os = "android"))]
    println!(
        "cargo:warning=Building for non-Android target: Android StrongBox module is not compiled (expected)"
    );

    println!("cargo:rerun-if-env-changed=ANDROID_NDK_HOME");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS");
}
