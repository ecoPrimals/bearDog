// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Build script for beardog-tunnel with Android StrongBox support
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
    println!(
        "cargo:warning=Building for non-Android platform - using mock StrongBox implementation"
    );
    // Common build settings
    println!("cargo:rerun-if-env-changed=ANDROID_NDK_HOME");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS");
}
