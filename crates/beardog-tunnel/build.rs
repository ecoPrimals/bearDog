//! Build script for beardog-tunnel with Android StrongBox support

fn main() {
    // Android-specific build configuration
    #[cfg(target_os = "android")]
    {
        println!("cargo:rerun-if-changed=build.rs");

        // Link Android system libraries
        println!("cargo:rustc-link-lib=log"); // Android logging
        println!("cargo:rustc-link-lib=android"); // Android NDK
        println!("cargo:rustc-link-lib=keystore"); // Android Keystore

        // Add Android-specific compiler flags
        println!("cargo:rustc-link-arg=-Wl,--allow-shlib-undefined");

        // Set up NDK paths if available
        if let Ok(ndk_home) = std::env::var("ANDROID_NDK_HOME") {
            println!("cargo:rustc-link-search=native={}/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib", ndk_home);
        }

        // Enable Android-specific features
        println!("cargo:rustc-cfg=feature=\"android_native\"");

        println!("cargo:warning=Building for Android with StrongBox support");
    }

    // Standard build for other platforms
    #[cfg(not(target_os = "android"))]
    {
        println!(
            "cargo:warning=Building for non-Android platform - using mock StrongBox implementation"
        );
    }

    // Common build settings
    println!("cargo:rerun-if-env-changed=ANDROID_NDK_HOME");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS");
}
