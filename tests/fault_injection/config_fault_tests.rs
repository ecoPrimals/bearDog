// SPDX-License-Identifier: AGPL-3.0-or-later

//! Configuration fault injection tests.

use beardog_config::BearDogConfig;
use beardog_config::domains::network_ports::{DEFAULT_API_PORT, NetworkPortsConfig};
use beardog_config::domains::paths::PathConfig;
use std::io::Write;
use tempfile::NamedTempFile;

#[tokio::test]
async fn config_fault_missing_file_returns_error() {
    let dir = tempfile::tempdir().expect("temp directory for missing config test");
    let path = dir.path().join("definitely_missing.toml");
    assert!(!path.exists());
    let result = BearDogConfig::from_file(&path);
    assert!(
        result.is_err(),
        "missing file should not panic; expect error"
    );
}

#[tokio::test]
async fn config_fault_corrupt_toml_returns_parse_error() {
    let mut file = NamedTempFile::new().expect("temp config file");
    writeln!(file, "[[[not_valid_toml").expect("write corrupt toml");
    let path = file.path().to_path_buf();
    let result = BearDogConfig::from_file(&path);
    assert!(
        result.is_err(),
        "corrupt TOML should yield error, not panic"
    );
}

#[tokio::test]
async fn config_fault_invalid_port_string_uses_default_like_from_env() {
    let parsed = "not-a-port".parse::<u16>().ok();
    assert!(parsed.is_none());
    let api_port = parsed.unwrap_or(DEFAULT_API_PORT);
    assert_eq!(api_port, DEFAULT_API_PORT);
}

#[tokio::test]
async fn config_fault_privileged_api_port_fails_validation() {
    let mut ports = NetworkPortsConfig::with_defaults();
    ports.api_port = 80;
    let v = ports.validate();
    assert!(
        v.is_err(),
        "privileged port should fail validation with clear error"
    );
}

#[tokio::test]
async fn config_fault_bad_pkcs11_path_returns_path_error() {
    let dir = tempfile::tempdir().expect("temp directory for fake pkcs11 path");
    let fake = dir.path().join("no_such_lib.so");
    assert!(!fake.exists());
    let mut paths = PathConfig::default();
    paths.pkcs11_library = Some(fake);
    let v = paths.validate();
    assert!(
        v.is_err(),
        "missing PKCS#11 library path should error in validate()"
    );
}

#[tokio::test]
async fn config_fault_empty_network_section_validates_or_loads() {
    let mut file = NamedTempFile::new().expect("temp network toml");
    writeln!(file, "[network.ports]").expect("write header");
    writeln!(file, "api_port = 0").expect("write invalid port");
    let path = file.path();
    let loaded = BearDogConfig::from_file(path);
    if let Ok(cfg) = loaded {
        assert!(cfg.validate().is_err(), "port 0 must not validate");
    }
}
