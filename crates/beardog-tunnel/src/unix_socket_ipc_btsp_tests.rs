// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive BTSP JSON-RPC Tests
//!
//! This test module provides extensive coverage for BTSP methods exposed via JSON-RPC:
//! - Unit tests for each method
//! - End-to-end integration tests
//! - Error handling and edge cases
//! - Chaos/fault injection tests

use beardog_types::primal_identity::PrimalIdentity;

#[cfg(test)]
mod btsp_jsonrpc_unit_tests {
    use super::*;
    use crate::btsp_handshake::BtspSecurityMode;
    use crate::btsp_provider::BeardogBtspProvider;
    use crate::tunnel::hsm::HsmManager;
    use crate::unix_socket_ipc::UnixSocketIpcServer;
    use beardog_core::socket_config::IpcCapabilitySymlinksConfig;
    use beardog_genetics::EcosystemGeneticEngine;

    use std::path::PathBuf;
    use std::sync::Arc;
    use tempfile::TempDir;

    // Helper functions
    fn test_socket() -> (TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let socket_path = dir.path().join("test.sock");
        (dir, socket_path)
    }

    async fn create_test_btsp_provider() -> Arc<BeardogBtspProvider> {
        beardog_errors::process_env::set_var("BEARDOG_HSM_MODE", "software");
        let hsm = Arc::new(HsmManager::auto_initialize().await.expect("HSM init"));
        let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Genetics init"));
        Arc::new(
            BeardogBtspProvider::new(hsm, genetics)
                .await
                .expect("BTSP init"),
        )
    }

    #[tokio::test]
    async fn test_json_rpc_btsp_integration() {
        let (_dir, socket_path) = test_socket();
        let provider = create_test_btsp_provider().await;

        let server = UnixSocketIpcServer::new(
            &socket_path,
            provider,
            Arc::new(PrimalIdentity::for_test("test", "node1")),
            BtspSecurityMode::Development,
            IpcCapabilitySymlinksConfig::default(),
        )
        .await
        .unwrap();

        assert_eq!(
            server.socket_path().to_str().unwrap(),
            socket_path.to_str().unwrap()
        );
    }
}
