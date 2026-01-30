//! Test Helpers for BTSP and Unix Socket Testing
//!
//! This module provides mock implementations and test utilities for testing
//! BTSP functionality without requiring actual HSM hardware.

#[cfg(test)]
pub mod mocks {
    use crate::btsp_provider::ContactInfo;
    use async_trait::async_trait;
    use beardog_capabilities::traits::{
        PeerEndpoint, SecureTunnelProvider, TunnelHandle, TunnelStatus,
    };
    use beardog_errors::BearDogError;
    use std::sync::{Arc, Mutex};

    /// Mock BTSP provider for testing
    ///
    /// This mock implements SecureTunnelProvider without requiring HSM hardware,
    /// making it suitable for unit and integration testing.
    pub struct MockBtspProvider {
        /// Track established tunnels
        tunnels: Arc<Mutex<Vec<String>>>,
        /// Simulate failures for testing error paths
        should_fail: Arc<Mutex<bool>>,
    }

    impl MockBtspProvider {
        /// Create a new mock BTSP provider
        pub fn new() -> Self {
            Self {
                tunnels: Arc::new(Mutex::new(Vec::new())),
                should_fail: Arc::new(Mutex::new(false)),
            }
        }

        /// Create a mock that will fail operations (for testing error paths)
        pub fn new_failing() -> Self {
            Self {
                tunnels: Arc::new(Mutex::new(Vec::new())),
                should_fail: Arc::new(Mutex::new(true)),
            }
        }

        /// Set whether operations should fail
        pub fn set_should_fail(&self, should_fail: bool) {
            *self.should_fail.lock().unwrap() = should_fail;
        }

        /// Get list of established tunnels
        pub fn get_tunnels(&self) -> Vec<String> {
            self.tunnels.lock().unwrap().clone()
        }
    }

    impl Default for MockBtspProvider {
        fn default() -> Self {
            Self::new()
        }
    }

    #[async_trait]
    impl SecureTunnelProvider for MockBtspProvider {
        async fn establish_tunnel(&self, peer: PeerEndpoint) -> Result<TunnelHandle, BearDogError> {
            if *self.should_fail.lock().unwrap() {
                return Err(BearDogError::system(
                    "Mock tunnel establishment failed".to_string(),
                ));
            }

            let tunnel_id = format!("mock-tunnel-{}", uuid::Uuid::new_v4());
            self.tunnels.lock().unwrap().push(tunnel_id.clone());

            Ok(TunnelHandle {
                id: tunnel_id,
                peer_id: peer.id,
                established_at: chrono::Utc::now().to_rfc3339(),
            })
        }

        async fn tunnel_encrypt(
            &self,
            _tunnel: &TunnelHandle,
            data: &[u8],
        ) -> Result<Vec<u8>, BearDogError> {
            if *self.should_fail.lock().unwrap() {
                return Err(BearDogError::system("Mock encryption failed".to_string()));
            }

            // Simple mock encryption: reverse the bytes
            Ok(data.iter().rev().copied().collect())
        }

        async fn tunnel_decrypt(
            &self,
            _tunnel: &TunnelHandle,
            data: &[u8],
        ) -> Result<Vec<u8>, BearDogError> {
            if *self.should_fail.lock().unwrap() {
                return Err(BearDogError::system("Mock decryption failed".to_string()));
            }

            // Simple mock decryption: reverse back
            Ok(data.iter().rev().copied().collect())
        }

        async fn tunnel_status(&self, tunnel: &TunnelHandle) -> Result<TunnelStatus, BearDogError> {
            if *self.should_fail.lock().unwrap() {
                return Err(BearDogError::system("Mock status query failed".to_string()));
            }

            let tunnels = self.tunnels.lock().unwrap();
            let active = tunnels.contains(&tunnel.id);

            Ok(TunnelStatus {
                tunnel_id: tunnel.id.clone(),
                active,
                bytes_sent: 1024,
                bytes_received: 2048,
                last_activity: chrono::Utc::now().to_rfc3339(),
            })
        }

        async fn close_tunnel(&self, tunnel: &TunnelHandle) -> Result<(), BearDogError> {
            if *self.should_fail.lock().unwrap() {
                return Err(BearDogError::system("Mock tunnel close failed".to_string()));
            }

            let mut tunnels = self.tunnels.lock().unwrap();
            tunnels.retain(|t| t != &tunnel.id);
            Ok(())
        }
    }

    // Implement contact_exchange as a separate impl block (not part of trait)
    impl MockBtspProvider {
        pub async fn contact_exchange(
            &self,
            target_peer_id: &str,
            _requester_lineage: &str,
            _max_hops: usize,
        ) -> Result<ContactInfo, BearDogError> {
            if *self.should_fail.lock().unwrap() {
                return Err(BearDogError::system(
                    "Mock contact exchange failed".to_string(),
                ));
            }

            Ok(ContactInfo {
                peer_id: target_peer_id.to_string(),
                addresses: vec![format!("192.168.1.100:8080")],
                lineage_proof: "mock-lineage-proof".to_string(),
                lineage_path: vec!["mock-relay".to_string()],
                search_depth: 1,
                last_seen: chrono::Utc::now(),
            })
        }
    }

    /// Create a mock BTSP provider wrapped in Arc for use in tests
    pub fn create_mock_btsp_provider() -> Arc<MockBtspProvider> {
        Arc::new(MockBtspProvider::new())
    }

    /// Create a failing mock BTSP provider for error path testing
    pub fn create_failing_btsp_provider() -> Arc<MockBtspProvider> {
        Arc::new(MockBtspProvider::new_failing())
    }

    /// Create a minimal safe BeardogBtspProvider for testing
    ///
    /// This provides a safe alternative to `unsafe { std::mem::zeroed() }`
    /// for tests that need to pass a BeardogBtspProvider but don't actually use it.
    ///
    /// Uses `new_for_testing()` which bypasses HSM initialization, making it suitable
    /// for handler tests that don't actually invoke BTSP functionality.
    ///
    /// # Returns
    /// A properly initialized but minimal BeardogBtspProvider (suitable for tests only)
    pub async fn create_minimal_beardog_provider() -> Arc<crate::btsp_provider::BeardogBtspProvider>
    {
        use crate::tunnel::hsm::HsmManager;
        use beardog_genetics::ecosystem_evolution::engine::EcosystemGeneticEngine;

        let hsm_manager = Arc::new(HsmManager::new());

        let genetic_engine =
            Arc::new(EcosystemGeneticEngine::new().expect("Failed to create genetic engine"));

        Arc::new(
            crate::btsp_provider::BeardogBtspProvider::new_for_testing(hsm_manager, genetic_engine)
                .await
                .expect("Failed to create test BTSP provider"),
        )
    }
}

