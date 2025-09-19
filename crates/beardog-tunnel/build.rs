// Build script for beardog-tunnel crate
//
// This build script configures platform-specific linking for Android and non-Android targets.
// On Android, it links against the Android NDK libraries for hardware security module access.
// On other platforms, it uses mock implementations for development and testing.

/// Main build script entry point
///
/// - Android: Links against NDK libraries (log, android, keystore)
fn main() {
    #[cfg(target_os = "android")]
    {
        println!("cargo:rerun-if-changed=build.rs");

        println!("cargo:rustc-link-lib=log"); // Android logging
        println!("cargo:rustc-link-lib=android"); // Android NDK
        println!("cargo:rustc-link-lib=keystore"); // Android Keystore

        println!("cargo:rustc-link-arg=-Wl,--allow-shlib-undefined");

        if let Ok(ndk_home) = std::env::var("ANDROID_NDK_HOME") {
            println!("cargo:rustc-link-search=native={}/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib", ndk_home);
        }

        println!("cargo:rustc-cfg=feature=\"android_native\"");
        println!("cargo:warning=Building for Android with StrongBox support");
    }

    #[cfg(not(target_os = "android"))]
    println!(
        "cargo:warning=Building for non-Android platform - using mock StrongBox implementation"
    );

    println!("cargo:rerun-if-env-changed=ANDROID_NDK_HOME");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS");
}
