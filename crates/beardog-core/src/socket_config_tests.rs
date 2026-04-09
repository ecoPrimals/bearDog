// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;

use super::*;
use crate::self_knowledge::discovered_simple_capabilities;

fn ok(inputs: &SocketPathInputs) -> SocketConfig {
    SocketConfig::from_inputs(inputs).expect("from_inputs should succeed for test")
}

#[test]
fn test_env_var_override_takes_priority() {
    let config = ok(&SocketPathInputs {
        beardog_socket: Some("/tmp/custom-override.sock".to_string()),
        family_id: Some("test0".to_string()),
        ..Default::default()
    });

    assert_eq!(
        config.socket_path_string(),
        "/tmp/custom-override.sock",
        "BEARDOG_SOCKET env var must take highest priority (Tier 1)"
    );
    assert_eq!(config.source(), SocketPathSource::PrimalEnvVar);
    assert_eq!(config.family_id(), "test0");
}

#[test]
fn test_empty_socket_path_rejected() {
    let config = ok(&SocketPathInputs {
        beardog_socket: Some(String::new()),
        family_id: Some("test".to_string()),
        primal_namespace_root_exists: false,
        ..Default::default()
    });

    assert_ne!(config.socket_path_string(), "");
    assert_ne!(config.source(), SocketPathSource::PrimalEnvVar);
    assert!(
        config.source() == SocketPathSource::XdgRuntime
            || config.source() == SocketPathSource::TempDir
    );
}

#[test]
fn test_empty_biomeos_socket_rejected() {
    let config = ok(&SocketPathInputs {
        biomeos_socket_path: Some(String::new()),
        family_id: Some("test".to_string()),
        primal_namespace_root_exists: false,
        ..Default::default()
    });

    assert_ne!(config.socket_path_string(), "");
    assert_ne!(config.source(), SocketPathSource::OrchestratorEnvVar);
    assert!(
        config.source() == SocketPathSource::XdgRuntime
            || config.source() == SocketPathSource::TempDir
    );
}

#[test]
fn test_biomeos_socket_path_tier2() {
    let config = ok(&SocketPathInputs {
        biomeos_socket_path: Some("/tmp/beardog-default-default.sock".to_string()),
        family_id: Some("nat0".to_string()),
        primal_namespace_root_exists: false,
        ..Default::default()
    });

    assert_eq!(
        config.socket_path_string(),
        "/tmp/beardog-default-default.sock",
        "BIOMEOS_SOCKET_PATH should be honored (Tier 2)"
    );
    assert_eq!(config.source(), SocketPathSource::OrchestratorEnvVar);
}

#[test]
fn test_beardog_socket_overrides_biomeos_socket_path() {
    let config = ok(&SocketPathInputs {
        beardog_socket: Some("/custom/beardog-specific.sock".to_string()),
        biomeos_socket_path: Some("/tmp/biomeos-generic.sock".to_string()),
        ..Default::default()
    });

    assert_eq!(config.socket_path_string(), "/custom/beardog-specific.sock");
    assert_eq!(config.source(), SocketPathSource::PrimalEnvVar);
}

#[test]
fn test_xdg_runtime_preferred_over_tmp() {
    let config = ok(&SocketPathInputs {
        primal_namespace_root_exists: false,
        node_id: Some("default".to_string()),
        ..Default::default()
    });

    assert!(
        matches!(
            config.source(),
            SocketPathSource::XdgRuntime | SocketPathSource::TempDir
        ),
        "Unexpected source: {:?}",
        config.source(),
    );
    if config.source() == SocketPathSource::XdgRuntime {
        assert!(config.socket_path_string().contains("/run/user/"));
        assert!(
            config.socket_path_string().contains("biomeos/beardog.sock"),
            "development mode (no family_id) should produce beardog.sock"
        );
    } else {
        assert!(
            config
                .socket_path_string()
                .contains("beardog-default-default.sock")
        );
    }
}

#[test]
fn test_fallback_to_tmp_with_node_id() {
    let config = ok(&SocketPathInputs {
        family_id: Some("fallback".to_string()),
        node_id: Some("node123".to_string()),
        primal_namespace_root_exists: false,
        ..Default::default()
    });

    if config.source() == SocketPathSource::TempDir {
        let p = std::env::temp_dir().join("beardog-fallback-node123.sock");
        assert_eq!(config.socket_path_string(), p.display().to_string());
    } else if config.source() == SocketPathSource::XdgRuntime {
        assert!(config.socket_path_string().contains("/run/user/"));
    }
}

#[test]
fn test_default_family_and_node_ids() {
    let config = ok(&SocketPathInputs {
        primal_namespace_root_exists: false,
        node_id: Some("default".to_string()),
        ..Default::default()
    });

    assert_eq!(config.family_id(), "default");
    assert_eq!(config.node_id(), "default");

    if config.source() == SocketPathSource::TempDir {
        let p = std::env::temp_dir().join("beardog-default-default.sock");
        assert_eq!(config.socket_path_string(), p.display().to_string());
    }
}

#[test]
fn test_missing_node_id_uses_ephemeral_standalone_id() {
    let config = ok(&SocketPathInputs {
        primal_namespace_root_exists: false,
        node_id: None,
        ..Default::default()
    });
    assert!(
        config.node_id().starts_with("standalone-"),
        "expected standalone-* node id, got {}",
        config.node_id()
    );
    if config.source() == SocketPathSource::TempDir {
        assert!(config.socket_path_string().contains(config.node_id()));
    }
}

#[test]
fn test_primal_namespace_tier3_when_root_exists() {
    let config = ok(&SocketPathInputs {
        primal_namespace_root_exists: true,
        primal_name: None,
        ..Default::default()
    });
    assert_eq!(config.source(), SocketPathSource::PrimalNamespace);
    assert_eq!(config.socket_path_string(), "/primal/beardog");
}

#[test]
fn test_description_format() {
    let config = ok(&SocketPathInputs {
        beardog_socket: Some("/custom/socket.sock".to_string()),
        ..Default::default()
    });

    let desc = config.description();
    assert!(desc.contains("/custom/socket.sock"));
    assert!(desc.contains("BEARDOG_SOCKET"));
}

#[test]
fn test_custom_config() {
    let config = SocketConfig::custom(
        "/test/custom.sock",
        "family1".to_string(),
        "node1".to_string(),
    );

    assert_eq!(config.socket_path_string(), "/test/custom.sock");
    assert_eq!(config.family_id(), "family1");
    assert_eq!(config.node_id(), "node1");
    assert!(config.production_mode());
}

#[test]
fn test_prepare_removes_old_socket() {
    let test_dir = std::env::temp_dir().join("beardog-socket-test");
    fs::create_dir_all(&test_dir).expect("Failed to create test directory");

    let socket_path = test_dir.join("test-socket.sock");
    fs::write(&socket_path, b"old socket").expect("Failed to write test socket file");
    assert!(socket_path.exists());

    let config = SocketConfig::custom(socket_path.clone(), "test".to_string(), "test".to_string());

    config.prepare().expect("prepare() should succeed for test");
    assert!(!socket_path.exists());

    let _ = fs::remove_dir_all(&test_dir);
}

#[test]
fn test_prepare_creates_parent_directory() {
    let test_dir = std::env::temp_dir().join("beardog-socket-test-nested");
    let socket_path = test_dir.join("subdir").join("test.sock");
    let _ = fs::remove_dir_all(&test_dir);

    let config = SocketConfig::custom(socket_path.clone(), "test".to_string(), "test".to_string());

    config
        .prepare()
        .expect("prepare() should succeed for nested test");
    assert!(
        socket_path
            .parent()
            .expect("socket path should have parent")
            .exists()
    );

    let _ = fs::remove_dir_all(&test_dir);
}

// --- BTSP family-scoped socket tests (BLOCKING fix) ---

#[test]
fn test_family_scoped_socket_via_biomeos_dir_production() {
    let config = ok(&SocketPathInputs {
        biomeos_socket_dir: Some("/run/user/1000/biomeos".to_string()),
        family_id: Some("cluster-7".to_string()),
        primal_namespace_root_exists: false,
        uid: 99999, // avoid hitting real XDG tier
        ..Default::default()
    });

    assert_eq!(
        config.socket_path_string(),
        "/run/user/1000/biomeos/beardog-cluster-7.sock",
        "Tier 2 (BIOMEOS_SOCKET_DIR) with FAMILY_ID must produce family-scoped socket"
    );
    assert_eq!(config.source(), SocketPathSource::OrchestratorEnvVar);
    assert!(config.production_mode());
}

#[test]
fn test_development_mode_socket_via_biomeos_dir() {
    let config = ok(&SocketPathInputs {
        biomeos_socket_dir: Some("/run/user/1000/biomeos".to_string()),
        primal_namespace_root_exists: false,
        uid: 99999,
        ..Default::default()
    });

    assert_eq!(
        config.socket_path_string(),
        "/run/user/1000/biomeos/beardog.sock",
        "Tier 2 (BIOMEOS_SOCKET_DIR) without FAMILY_ID must produce plain beardog.sock"
    );
    assert!(!config.production_mode());
}

#[test]
fn test_family_scoped_xdg_runtime() {
    let config = ok(&SocketPathInputs {
        family_id: Some("alpha".to_string()),
        primal_namespace_root_exists: false,
        ..Default::default()
    });

    if config.source() == SocketPathSource::XdgRuntime {
        assert!(
            config.socket_path_string().ends_with("beardog-alpha.sock"),
            "XDG tier with FAMILY_ID must use family-scoped name, got: {}",
            config.socket_path_string()
        );
    }
}

#[test]
fn test_insecure_with_family_id_is_fatal() {
    let result = SocketConfig::from_inputs(&SocketPathInputs {
        family_id: Some("cluster-7".to_string()),
        biomeos_insecure: true,
        ..Default::default()
    });

    let err = result.expect_err("FAMILY_ID + BIOMEOS_INSECURE must be fatal");
    assert!(
        matches!(err, SocketConfigError::InsecureWithFamily { .. }),
        "Expected InsecureWithFamily, got: {err:?}"
    );
}

#[test]
fn test_insecure_without_family_id_is_ok() {
    let config = ok(&SocketPathInputs {
        biomeos_insecure: true,
        ..Default::default()
    });

    assert!(!config.production_mode());
}

#[test]
fn test_default_family_is_not_production() {
    let config = ok(&SocketPathInputs {
        family_id: Some("default".to_string()),
        ..Default::default()
    });

    assert!(!config.production_mode());
}

#[test]
fn test_production_mode_accessor() {
    let dev = SocketConfig::custom("/tmp/a.sock", "default".to_string(), "n".to_string());
    assert!(!dev.production_mode());

    let prod = SocketConfig::custom("/tmp/b.sock", "cluster-7".to_string(), "n".to_string());
    assert!(prod.production_mode());
}

#[test]
fn test_is_production_mode_inputs() {
    assert!(!SocketPathInputs::default().is_production_mode());
    assert!(
        SocketPathInputs {
            family_id: Some("real".to_string()),
            ..Default::default()
        }
        .is_production_mode()
    );
    assert!(
        !SocketPathInputs {
            family_id: Some("default".to_string()),
            ..Default::default()
        }
        .is_production_mode()
    );
    assert!(
        !SocketPathInputs {
            family_id: Some(String::new()),
            ..Default::default()
        }
        .is_production_mode()
    );
}

#[cfg(unix)]
#[test]
fn test_ipc_capability_symlinks_development_mode() {
    let test_dir = std::env::temp_dir().join("beardog-symlink-test-dev");
    let _ = fs::remove_dir_all(&test_dir);
    fs::create_dir_all(&test_dir).expect("create test dir");

    let socket_path = test_dir.join("beardog.sock");
    fs::write(&socket_path, b"").expect("create mock socket");

    let config = SocketConfig::custom(socket_path, "default".to_string(), "default".to_string());
    let stems = ipc_capability_domain_stems_from_capabilities(&discovered_simple_capabilities());
    let links = config.install_ipc_capability_symlinks(&stems);

    assert!(!stems.is_empty());
    assert_eq!(links.len(), stems.len());

    for link in &links {
        let dest = fs::read_link(link).expect("read link");
        assert_eq!(dest.to_string_lossy(), "beardog.sock");
    }

    config.remove_ipc_capability_symlinks(&stems);
    for stem in &stems {
        assert!(
            !test_dir
                .join(format!("{}{}", stem, config.ipc_symlink_filename_suffix()))
                .exists()
        );
    }

    let _ = fs::remove_dir_all(&test_dir);
}

#[cfg(unix)]
#[test]
fn test_ipc_capability_symlinks_production_mode() {
    let test_dir = std::env::temp_dir().join("beardog-symlink-test-prod");
    let _ = fs::remove_dir_all(&test_dir);
    fs::create_dir_all(&test_dir).expect("create test dir");

    let socket_path = test_dir.join("beardog-cluster7.sock");
    fs::write(&socket_path, b"").expect("create mock socket");

    let config = SocketConfig::custom(socket_path, "cluster7".to_string(), "node1".to_string());
    assert!(config.production_mode());

    let stems = ipc_capability_domain_stems_from_capabilities(&discovered_simple_capabilities());
    let links = config.install_ipc_capability_symlinks(&stems);

    assert!(!stems.is_empty());
    assert_eq!(links.len(), stems.len());

    for link in &links {
        let dest = fs::read_link(link).expect("read link");
        assert_eq!(dest.to_string_lossy(), "beardog-cluster7.sock");
    }

    config.remove_ipc_capability_symlinks(&stems);
    for stem in &stems {
        assert!(
            !test_dir
                .join(format!("{}{}", stem, config.ipc_symlink_filename_suffix()))
                .exists()
        );
    }

    let _ = fs::remove_dir_all(&test_dir);
}
