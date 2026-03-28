// SPDX-License-Identifier: AGPL-3.0-only
//! Coverage for Android NDK env branches, builder setup success paths, and rustup error paths.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use crate::android::AndroidDeployment;
use crate::builder::RustBuilder;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn restore_path(old: Option<std::ffi::OsString>) {
    match old {
        Some(p) => beardog_errors::process_env::set_var("PATH", p),
        None => beardog_errors::process_env::remove_var("PATH"),
    }
}

#[test]
fn find_ndk_path_prefers_android_ndk_home_when_set() {
    let _g = ENV_LOCK.lock().expect("lock");
    let tmp = tempfile::tempdir().expect("tmp");
    let path = tmp.path().to_string_lossy().into_owned();
    let old_ndk = beardog_errors::process_env::var("ANDROID_NDK_HOME").ok();
    let old_home = beardog_errors::process_env::var("NDK_HOME").ok();
    beardog_errors::process_env::set_var("ANDROID_NDK_HOME", &path);
    beardog_errors::process_env::remove_var("NDK_HOME");
    let d = AndroidDeployment::new(None, 33);
    assert_eq!(d.find_ndk_path().expect("ndk"), path);
    match old_ndk {
        Some(v) => beardog_errors::process_env::set_var("ANDROID_NDK_HOME", v),
        None => beardog_errors::process_env::remove_var("ANDROID_NDK_HOME"),
    }
    match old_home {
        Some(v) => beardog_errors::process_env::set_var("NDK_HOME", v),
        None => beardog_errors::process_env::remove_var("NDK_HOME"),
    }
}

#[test]
fn find_ndk_path_falls_back_to_ndk_home() {
    let _g = ENV_LOCK.lock().expect("lock");
    let tmp = tempfile::tempdir().expect("tmp");
    let path = tmp.path().to_string_lossy().into_owned();
    let old_a = beardog_errors::process_env::var("ANDROID_NDK_HOME").ok();
    let old_n = beardog_errors::process_env::var("NDK_HOME").ok();
    beardog_errors::process_env::remove_var("ANDROID_NDK_HOME");
    beardog_errors::process_env::set_var("NDK_HOME", &path);
    let d = AndroidDeployment::new(None, 33);
    assert_eq!(d.find_ndk_path().expect("ndk from NDK_HOME"), path);
    match old_a {
        Some(v) => beardog_errors::process_env::set_var("ANDROID_NDK_HOME", v),
        None => beardog_errors::process_env::remove_var("ANDROID_NDK_HOME"),
    }
    match old_n {
        Some(v) => beardog_errors::process_env::set_var("NDK_HOME", v),
        None => beardog_errors::process_env::remove_var("NDK_HOME"),
    }
}

#[test]
fn find_ndk_path_struct_field_wins_over_env() {
    let _g = ENV_LOCK.lock().expect("lock");
    let tmp = tempfile::tempdir().expect("tmp");
    let p = tmp.path().to_string_lossy().into_owned();
    let other = tempfile::tempdir().expect("tmp2");
    let old_a = beardog_errors::process_env::var("ANDROID_NDK_HOME").ok();
    beardog_errors::process_env::set_var(
        "ANDROID_NDK_HOME",
        other.path().to_string_lossy().as_ref(),
    );
    let d = AndroidDeployment::new(Some(p.clone()), 33);
    assert_eq!(d.find_ndk_path().expect("struct ndk"), p);
    match old_a {
        Some(v) => beardog_errors::process_env::set_var("ANDROID_NDK_HOME", v),
        None => beardog_errors::process_env::remove_var("ANDROID_NDK_HOME"),
    }
}

#[test]
fn check_rust_compiler_errors_when_rustc_not_on_path() {
    let _g = ENV_LOCK.lock().expect("lock");
    let old = beardog_errors::process_env::var_os("PATH");
    beardog_errors::process_env::set_var("PATH", "/tmp/nonexistent_bin_dir_for_tests");
    let r = AndroidDeployment::check_rust_compiler();
    restore_path(old);
    assert!(r.is_err(), "rustc should be missing from synthetic PATH");
}

#[test]
fn install_target_rejects_invalid_triple() {
    let _g = ENV_LOCK.lock().expect("lock");
    let r = AndroidDeployment::install_target("definitely-not-a-real-target-triple-999");
    assert!(r.is_err(), "rustup should reject invalid target");
}

#[test]
fn is_target_installed_errors_when_rustup_missing_from_path() {
    let _g = ENV_LOCK.lock().expect("lock");
    let old = beardog_errors::process_env::var_os("PATH");
    beardog_errors::process_env::set_var("PATH", "/tmp/nonexistent_bin_dir_for_tests");
    let r = AndroidDeployment::is_target_installed("aarch64-linux-android");
    restore_path(old);
    assert!(r.is_err(), "is_target_installed needs rustup on PATH");
}

#[test]
fn setup_build_environment_succeeds_with_ndk_overlay_aarch64() {
    if !cfg!(target_os = "linux") || !cfg!(target_arch = "x86_64") {
        return;
    }
    let _g = ENV_LOCK.lock().expect("lock");
    let tmp = tempfile::tempdir().expect("tmp");
    let old_a = beardog_errors::process_env::var("ANDROID_NDK_HOME").ok();
    let old_n = beardog_errors::process_env::var("NDK_HOME").ok();
    beardog_errors::process_env::set_var("ANDROID_NDK_HOME", tmp.path().to_string_lossy().as_ref());
    beardog_errors::process_env::remove_var("NDK_HOME");
    let r = RustBuilder::setup_build_environment("aarch64-linux-android");
    match old_a {
        Some(v) => beardog_errors::process_env::set_var("ANDROID_NDK_HOME", v),
        None => beardog_errors::process_env::remove_var("ANDROID_NDK_HOME"),
    }
    match old_n {
        Some(v) => beardog_errors::process_env::set_var("NDK_HOME", v),
        None => beardog_errors::process_env::remove_var("NDK_HOME"),
    }
    assert!(r.is_ok(), "setup with valid NDK + aarch64: {r:?}");
}

#[test]
fn setup_build_environment_succeeds_with_ndk_overlay_armv7() {
    if !cfg!(target_os = "linux") || !cfg!(target_arch = "x86_64") {
        return;
    }
    let _g = ENV_LOCK.lock().expect("lock");
    let tmp = tempfile::tempdir().expect("tmp");
    let old_a = beardog_errors::process_env::var("ANDROID_NDK_HOME").ok();
    let old_n = beardog_errors::process_env::var("NDK_HOME").ok();
    beardog_errors::process_env::set_var("ANDROID_NDK_HOME", tmp.path().to_string_lossy().as_ref());
    beardog_errors::process_env::remove_var("NDK_HOME");
    let r = RustBuilder::setup_build_environment("armv7-linux-androideabi");
    match old_a {
        Some(v) => beardog_errors::process_env::set_var("ANDROID_NDK_HOME", v),
        None => beardog_errors::process_env::remove_var("ANDROID_NDK_HOME"),
    }
    match old_n {
        Some(v) => beardog_errors::process_env::set_var("NDK_HOME", v),
        None => beardog_errors::process_env::remove_var("NDK_HOME"),
    }
    assert!(r.is_ok(), "armv7 toolchain branch: {r:?}");
}

#[test]
fn get_ndk_path_uses_ndk_home_when_android_ndk_home_missing() {
    let _g = ENV_LOCK.lock().expect("lock");
    let tmp = tempfile::tempdir().expect("tmp");
    let p = tmp.path().to_string_lossy().into_owned();
    let old_a = beardog_errors::process_env::var("ANDROID_NDK_HOME").ok();
    let old_n = beardog_errors::process_env::var("NDK_HOME").ok();
    beardog_errors::process_env::remove_var("ANDROID_NDK_HOME");
    beardog_errors::process_env::set_var("NDK_HOME", &p);
    let got = RustBuilder::get_ndk_path().expect("NDK_HOME fallback");
    match old_a {
        Some(v) => beardog_errors::process_env::set_var("ANDROID_NDK_HOME", v),
        None => beardog_errors::process_env::remove_var("ANDROID_NDK_HOME"),
    }
    match old_n {
        Some(v) => beardog_errors::process_env::set_var("NDK_HOME", v),
        None => beardog_errors::process_env::remove_var("NDK_HOME"),
    }
    assert_eq!(got, p);
}

#[tokio::test]
async fn build_android_library_fails_without_cargo_workspace() {
    if !cfg!(target_os = "linux") || !cfg!(target_arch = "x86_64") {
        return;
    }
    let tmp = tempfile::tempdir().expect("tmp");
    let builder_path = tmp.path().to_owned();

    // Synchronous env manipulation under lock — dropped before await
    {
        let _g = ENV_LOCK.lock().expect("lock");
        beardog_errors::process_env::set_var(
            "ANDROID_NDK_HOME",
            tmp.path().to_string_lossy().as_ref(),
        );
        beardog_errors::process_env::remove_var("NDK_HOME");
        std::fs::write(
            tmp.path().join("Cargo.toml"),
            "[package]\nname = \"empty_proj\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .expect("write manifest");
    }

    let b = RustBuilder::new(&builder_path);
    let err = b
        .build_android_app(false, "aarch64-linux-android")
        .await
        .expect_err("no lib target / build failure");
    let msg = err.to_string();
    assert!(!msg.is_empty(), "{msg}");
}

#[test]
fn verify_android_ndk_ok_when_ndk_path_in_struct() {
    let _g = ENV_LOCK.lock().expect("lock");
    let tmp = tempfile::tempdir().expect("tmp");
    let d = AndroidDeployment::new(Some(tmp.path().to_string_lossy().into_owned()), 33);
    assert!(d.verify_android_ndk().is_ok());
}

#[test]
fn set_aarch64_and_armv7_toolchains_set_expected_env_keys() {
    if !cfg!(target_os = "linux") || !cfg!(target_arch = "x86_64") {
        return;
    }
    let _g = ENV_LOCK.lock().expect("lock");
    let tmp = tempfile::tempdir().expect("tmp");
    let old_a = beardog_errors::process_env::var("ANDROID_NDK_HOME").ok();
    beardog_errors::process_env::set_var("ANDROID_NDK_HOME", tmp.path().to_string_lossy().as_ref());
    RustBuilder::setup_build_environment("aarch64-linux-android").expect("aarch64");
    assert!(
        beardog_errors::process_env::var("CC_aarch64_linux_android").is_ok(),
        "CC_aarch64_linux_android should be set"
    );
    RustBuilder::setup_build_environment("armv7-linux-androideabi").expect("armv7");
    assert!(
        beardog_errors::process_env::var("CC_armv7_linux_androideabi").is_ok(),
        "CC_armv7_linux_androideabi should be set"
    );
    for key in [
        "CC_aarch64_linux_android",
        "CXX_aarch64_linux_android",
        "AR_aarch64_linux_android",
        "RANLIB_aarch64_linux_android",
        "CC_armv7_linux_androideabi",
        "CXX_armv7_linux_androideabi",
        "AR_armv7_linux_androideabi",
        "RANLIB_armv7_linux_androideabi",
    ] {
        beardog_errors::process_env::remove_var(key);
    }
    match old_a {
        Some(v) => beardog_errors::process_env::set_var("ANDROID_NDK_HOME", v),
        None => beardog_errors::process_env::remove_var("ANDROID_NDK_HOME"),
    }
}
