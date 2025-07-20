//! Build script for Android NDK integration

use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();

    // Configure for Android targets
    if target.contains("android") {
        println!("cargo:rustc-cfg=target_os=\"android\"");

        // Link Android system libraries
        println!("cargo:rustc-link-lib=log"); // Android logging
        println!("cargo:rustc-link-lib=android"); // Android native API

        // Set up JNI
        if let Ok(ndk_home) = env::var("ANDROID_NDK_HOME") {
            let ndk_path = PathBuf::from(ndk_home);
            let toolchain_path = ndk_path.join("toolchains/llvm/prebuilt");

            // Find the appropriate toolchain
            if let Ok(host_tag) = env::var("NDK_HOST_TAG") {
                let include_path = toolchain_path.join(&host_tag).join("sysroot/usr/include");
                println!("cargo:rustc-link-search={}", include_path.display());
            }
        }

        println!("cargo:rerun-if-env-changed=ANDROID_NDK_HOME");
        println!("cargo:rerun-if-env-changed=NDK_HOST_TAG");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
