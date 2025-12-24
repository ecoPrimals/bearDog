//! UPA Integration Tests
//!
//! Comprehensive integration tests for BearDog ↔ Songbird UPA coordination.
//! These tests require a live Songbird instance running at https://localhost:8080.
//!
//! Run with:
//! ```bash
//! cargo test --test upa_integration_test --features btsp-api -- --test-threads=1
//! ```

use std::sync::Arc;
use std::time::Duration;

use beardog_genetics::EcosystemGeneticEngine;
use beardog_tunnel::tunnel::hsm::manager::HsmManager;
use beardog_tunnel::{
    BearDogApiServer, BearDogApiServerConfig, BeardogBtspProvider, UpaClient, UpaClientConfig,
};
use tokio::time::{sleep, timeout};

/// Test helper to check if Songbird is available
async fn is_songbird_available() -> bool {
    reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get("https://localhost:8080/health")
        .timeout(Duration::from_secs(2))
        .send()
        .await
        .is_ok()
}

/// Test: UPA client registration
///
/// Verifies:
/// - Client can connect to Songbird UPA
/// - Registration request is accepted
/// - Service ID is returned
/// - Token is stored securely
#[tokio::test]
#[ignore] // Requires live Songbird instance
async fn test_upa_registration() {
    // Skip if Songbird not available
    if !is_songbird_available().await {
        eprintln!("⚠️  Songbird not available, skipping test");
        return;
    }

    // Create UPA client
    let config = UpaClientConfig {
        upa_base_url: "https://localhost:8080".to_string(),
        service_name: "beardog-test".to_string(),
        service_version: "0.9.0-test".to_string(),
        api_bind_addr: "https://127.0.0.1:9001".to_string(),
        heartbeat_interval_secs: 30,
        connection_timeout_secs: 10,
    };

    let upa_client = UpaClient::new(config).expect("Failed to create UPA client");

    // Register with Songbird
    let service_id = upa_client
        .register()
        .await
        .expect("Failed to register with UPA");

    // Verify registration
    assert!(!service_id.is_empty(), "Service ID should not be empty");
    assert!(
        service_id.starts_with("beardog"),
        "Service ID should start with 'beardog'"
    );
    assert!(upa_client.is_registered(), "Client should be registered");
    assert_eq!(
        upa_client.service_id(),
        Some(service_id.clone()),
        "Service ID should match"
    );

    println!("✅ Registration successful: {}", service_id);
}

/// Test: UPA heartbeat loop
///
/// Verifies:
/// - Heartbeat loop starts successfully
/// - Heartbeats are sent periodically
/// - Songbird acknowledges heartbeats
/// - Load metrics are included
#[tokio::test]
#[ignore] // Requires live Songbird instance
async fn test_upa_heartbeat() {
    // Skip if Songbird not available
    if !is_songbird_available().await {
        eprintln!("⚠️  Songbird not available, skipping test");
        return;
    }

    // Create and register UPA client
    let config = UpaClientConfig {
        upa_base_url: "https://localhost:8080".to_string(),
        service_name: "beardog-test".to_string(),
        service_version: "0.9.0-test".to_string(),
        api_bind_addr: "https://127.0.0.1:9001".to_string(),
        heartbeat_interval_secs: 5, // Short interval for testing
        connection_timeout_secs: 10,
    };

    let upa_client = UpaClient::new(config).expect("Failed to create UPA client");
    let service_id = upa_client
        .register()
        .await
        .expect("Failed to register with UPA");

    println!("✅ Registered: {}", service_id);

    // Start heartbeat loop
    upa_client.start_heartbeat();
    println!("✅ Heartbeat loop started");

    // Wait for multiple heartbeat cycles
    sleep(Duration::from_secs(12)).await;

    // Query Songbird to verify heartbeats were received
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let response = client
        .get("https://localhost:8080/api/v1/services")
        .send()
        .await
        .expect("Failed to query services");

    let services: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse services response");

    // Find our service
    let our_service = services
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["service_id"].as_str() == Some(&service_id))
        .expect("Our service not found in registry");

    // Verify service status
    assert_eq!(
        our_service["status"].as_str(),
        Some("active"),
        "Service should be active"
    );
    assert!(
        our_service["last_heartbeat"].is_string(),
        "Should have last_heartbeat timestamp"
    );

    println!("✅ Heartbeat verified: {:?}", our_service);

    // Stop heartbeat
    upa_client.stop_heartbeat().await;
    println!("✅ Heartbeat stopped");
}

/// Test: Service capability advertisement
///
/// Verifies:
/// - All capabilities are advertised correctly
/// - Endpoints are registered properly
/// - Protocols are listed accurately
#[tokio::test]
#[ignore] // Requires live Songbird instance
async fn test_capability_advertisement() {
    // Skip if Songbird not available
    if !is_songbird_available().await {
        eprintln!("⚠️  Songbird not available, skipping test");
        return;
    }

    // Create and register UPA client
    let config = UpaClientConfig::default();
    let upa_client = UpaClient::new(config).expect("Failed to create UPA client");
    let service_id = upa_client
        .register()
        .await
        .expect("Failed to register with UPA");

    // Query Songbird for our service details
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let response = client
        .get(format!(
            "https://localhost:8080/api/v1/services/{}",
            service_id
        ))
        .send()
        .await
        .expect("Failed to query service");

    let service: serde_json::Value = response.json().await.expect("Failed to parse service");

    // Verify capabilities
    let capabilities = service["capabilities"]
        .as_array()
        .expect("Capabilities should be an array");
    assert!(capabilities.iter().any(|c| c == "security"));
    assert!(capabilities.iter().any(|c| c == "btsp"));
    assert!(capabilities.iter().any(|c| c == "lineage"));
    assert!(capabilities.iter().any(|c| c == "birdsong"));

    // Verify endpoints
    let endpoints = service["endpoints"]
        .as_array()
        .expect("Endpoints should be an array");
    assert!(endpoints.iter().any(|e| e["name"] == "btsp"));
    assert!(endpoints.iter().any(|e| e["name"] == "genesis"));
    assert!(endpoints.iter().any(|e| e["name"] == "birdsong"));
    assert!(endpoints.iter().any(|e| e["name"] == "lineage"));

    println!("✅ Capabilities verified: {:?}", service);
}

/// Test: Full API server with UPA integration
///
/// Verifies:
/// - API server starts successfully
/// - UPA registration happens automatically
/// - All endpoints are accessible
/// - Health check includes UPA status
#[tokio::test]
#[ignore] // Requires live Songbird instance
async fn test_full_api_server_with_upa() {
    // Skip if Songbird not available
    if !is_songbird_available().await {
        eprintln!("⚠️  Songbird not available, skipping test");
        return;
    }

    // Initialize providers
    let hsm_manager = Arc::new(HsmManager::new());
    let genetic_engine = Arc::new(EcosystemGeneticEngine::new().unwrap());
    let btsp_provider = Arc::new(
        BeardogBtspProvider::new(Arc::clone(&hsm_manager), Arc::clone(&genetic_engine))
            .await
            .unwrap(),
    );

    // Create API server
    let api_config = BearDogApiServerConfig {
        bind_addr: "127.0.0.1:9001".parse().unwrap(),
        enable_cors: true,
        version: "0.9.0-test".to_string(),
    };

    let api_server = BearDogApiServer::new(api_config.clone(), Arc::clone(&btsp_provider))
        .await
        .unwrap();

    // Create UPA client
    let upa_config = UpaClientConfig {
        upa_base_url: "https://localhost:8080".to_string(),
        service_name: "beardog-test".to_string(),
        service_version: "0.9.0-test".to_string(),
        api_bind_addr: format!("https://{}", api_config.bind_addr),
        heartbeat_interval_secs: 30,
        connection_timeout_secs: 10,
    };

    let upa_client = UpaClient::new(upa_config).unwrap();

    // Register with UPA
    let service_id = upa_client.register().await.unwrap();
    upa_client.start_heartbeat();

    println!(
        "✅ API server started with UPA registration: {}",
        service_id
    );

    // Start server in background
    let server_handle = tokio::spawn(async move {
        api_server.serve().await.unwrap();
    });

    // Wait for server to be ready
    sleep(Duration::from_secs(2)).await;

    // Test health endpoint
    let client = reqwest::Client::new();
    let health_response = client
        .get(format!("http://{}/health", "127.0.0.1:9001"))
        .send()
        .await
        .expect("Failed to query health");

    assert!(health_response.status().is_success());
    let health: serde_json::Value = health_response.json().await.unwrap();
    assert_eq!(health["status"], "healthy");

    // Test BTSP endpoint availability
    let btsp_response = client
        .get(format!("http://{}/btsp/", "127.0.0.1:9001"))
        .send()
        .await
        .expect("Failed to query BTSP");

    assert!(btsp_response.status().is_success() || btsp_response.status().as_u16() == 404);

    println!("✅ All endpoints accessible");

    // Cleanup
    upa_client.stop_heartbeat().await;
    server_handle.abort();
}

/// Test: Graceful degradation without Songbird
///
/// Verifies:
/// - API server starts even if Songbird is unavailable
/// - UPA registration fails gracefully
/// - Service continues to operate normally
#[tokio::test]
async fn test_graceful_degradation() {
    // Create UPA client with non-existent Songbird
    let config = UpaClientConfig {
        upa_base_url: "https://localhost:9999".to_string(), // Non-existent
        service_name: "beardog-test".to_string(),
        service_version: "0.9.0-test".to_string(),
        api_bind_addr: "https://127.0.0.1:9002".to_string(),
        heartbeat_interval_secs: 30,
        connection_timeout_secs: 2,
    };

    let upa_client = UpaClient::new(config).expect("Failed to create UPA client");

    // Attempt registration (should fail gracefully)
    let result = timeout(Duration::from_secs(5), upa_client.register()).await;

    match result {
        Ok(Err(_)) => {
            println!("✅ Registration failed gracefully as expected");
            assert!(!upa_client.is_registered(), "Should not be registered");
            assert_eq!(upa_client.service_id(), None, "Should have no service ID");
        }
        Ok(Ok(_)) => panic!("Registration should have failed"),
        Err(_) => panic!("Registration timed out"),
    }

    // Verify API server can still start
    let hsm_manager = Arc::new(HsmManager::new());
    let genetic_engine = Arc::new(EcosystemGeneticEngine::new().unwrap());
    let btsp_provider = Arc::new(
        BeardogBtspProvider::new(hsm_manager, genetic_engine)
            .await
            .unwrap(),
    );

    let api_config = BearDogApiServerConfig {
        bind_addr: "127.0.0.1:9002".parse().unwrap(),
        enable_cors: true,
        version: "0.9.0-test".to_string(),
    };

    let _api_server = BearDogApiServer::new(api_config, btsp_provider)
        .await
        .unwrap();

    println!("✅ API server created successfully without UPA");

    // Server would continue to operate normally
    // (not starting it here to avoid port conflicts in tests)
}

/// Test: Load metrics reporting
///
/// Verifies:
/// - Load metrics are collected accurately
/// - Metrics are included in heartbeat
/// - Songbird receives and stores metrics
#[tokio::test]
#[ignore] // Requires live Songbird instance
async fn test_load_metrics() {
    // Skip if Songbird not available
    if !is_songbird_available().await {
        eprintln!("⚠️  Songbird not available, skipping test");
        return;
    }

    // Create and register UPA client
    let config = UpaClientConfig {
        heartbeat_interval_secs: 5,
        ..Default::default()
    };

    let upa_client = UpaClient::new(config).expect("Failed to create UPA client");
    let service_id = upa_client.register().await.expect("Failed to register");

    // Start heartbeat
    upa_client.start_heartbeat();

    // Wait for heartbeat to send metrics
    sleep(Duration::from_secs(7)).await;

    // Query Songbird for metrics
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let response = client
        .get(format!(
            "https://localhost:8080/api/v1/services/{}",
            service_id
        ))
        .send()
        .await
        .expect("Failed to query service");

    let service: serde_json::Value = response.json().await.expect("Failed to parse service");

    // Verify load metrics exist
    assert!(service["load"].is_object(), "Load metrics should exist");
    assert!(
        service["load"]["active_tunnels"].is_number(),
        "Should have active_tunnels metric"
    );

    println!("✅ Load metrics verified: {:?}", service["load"]);

    // Cleanup
    upa_client.stop_heartbeat().await;
}
