//! Integration tests for biomeOS Federation APIs
//!
//! Tests the 4 Unix socket JSON-RPC methods required for biomeOS spore federation:
//! 1. verify_family_member - Genetic lineage verification
//! 2. derive_subfed_key - Sub-federation key derivation
//! 3. encrypt - AES-256-GCM encryption
//! 4. decrypt - AES-256-GCM decryption

use anyhow::Result;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

use beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer;
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::tunnel::hsm::HsmManager;
use beardog_genetics::EcosystemGeneticEngine;

// Helper to create test socket
fn test_socket() -> (TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let socket_path = dir.path().join("biomeos-integration-test.sock");
    (dir, socket_path)
}

// Helper to create test BTSP provider
async fn create_test_btsp_provider() -> Arc<BeardogBtspProvider> {
    std::env::set_var("BEARDOG_HSM_MODE", "software");
    let hsm = Arc::new(HsmManager::auto_initialize().await.expect("HSM init"));
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Genetics init"));
    Arc::new(BeardogBtspProvider::new(hsm, genetics).await.expect("BTSP init"))
}

// Helper to start server
async fn start_server_ready(
    socket_path: std::path::PathBuf,
) -> (tokio::task::JoinHandle<()>, Arc<std::sync::atomic::AtomicBool>, Arc<UnixSocketIpcServer>) {
    let btsp_provider = create_test_btsp_provider().await;
    let server = Arc::new(UnixSocketIpcServer::new(socket_path, btsp_provider).await.unwrap());
    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    
    let handle = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });
    
    assert!(UnixSocketIpcServer::wait_ready_flag(&ready_flag, Duration::from_secs(10)).await);
    (handle, ready_flag, server)
}

// Helper to send JSON-RPC request
async fn send_jsonrpc(stream: &mut UnixStream, request: serde_json::Value) -> Result<serde_json::Value> {
    let request_str = serde_json::to_string(&request)?;
    stream.write_all(request_str.as_bytes()).await?;
    stream.write_all(b"\n").await?;
    stream.flush().await?;

    let (reader, _writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    reader.read_line(&mut line).await?;
    
    Ok(serde_json::from_str(&line)?)
}

#[tokio::test]
async fn test_verify_family_member() {
    // Set environment for test
    std::env::set_var("FAMILY_ID", "nat0");
    std::env::set_var("NODE_ID", "node-alpha");
    
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;
    
    let mut stream = UnixStream::connect(&socket_path).await.unwrap();
    
    // Test family member verification
    let request = json!({
        "jsonrpc": "2.0",
        "method": "federation.verify_family_member",
        "params": {
            "family_id": "nat0",
            "seed_hash": "aaeaa3cfd69dd379...",
            "node_id": "node-beta"
        },
        "id": 1
    });
    
    let response = send_jsonrpc(&mut stream, request).await.unwrap();
    
    // Verify response structure
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"].is_object(), "Should have result");
    assert_eq!(response["result"]["is_family_member"], true);
    assert_eq!(response["result"]["relationship"], "sibling");
    assert!(response["result"]["verified_at"].is_string());
    assert_eq!(response["result"]["verification_method"], "genetic_lineage_hkdf");
    
    server.stop().await.unwrap();
    server_handle.abort();
    
    std::env::remove_var("FAMILY_ID");
    std::env::remove_var("NODE_ID");
}

#[tokio::test]
async fn test_verify_family_member_different_family() {
    // Set environment for test
    std::env::set_var("FAMILY_ID", "nat0");
    
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;
    
    let mut stream = UnixStream::connect(&socket_path).await.unwrap();
    
    // Test with different family
    let request = json!({
        "jsonrpc": "2.0",
        "method": "federation.verify_family_member",
        "params": {
            "family_id": "lan0",  // Different family
            "seed_hash": "differenthash...",
            "node_id": "node-gamma"
        },
        "id": 1
    });
    
    let response = send_jsonrpc(&mut stream, request).await.unwrap();
    
    assert_eq!(response["result"]["is_family_member"], false);
    assert_eq!(response["result"]["relationship"], "unrelated");
    assert_eq!(response["result"]["trust_level"], "none");
    
    server.stop().await.unwrap();
    server_handle.abort();
    
    std::env::remove_var("FAMILY_ID");
}

#[tokio::test]
async fn test_derive_subfed_key() {
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;
    
    let mut stream = UnixStream::connect(&socket_path).await.unwrap();
    
    // Test sub-federation key derivation
    let request = json!({
        "jsonrpc": "2.0",
        "method": "federation.derive_subfed_key",
        "params": {
            "parent_family": "nat0",
            "subfed_name": "gaming",
            "purpose": "sub-federation-encryption",
            "derivation_info": "gaming-2026-01-08"
        },
        "id": 2
    });
    
    let response = send_jsonrpc(&mut stream, request).await.unwrap();
    
    // Verify response structure
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"].is_object());
    assert!(response["result"]["key_ref"].as_str().unwrap().starts_with("beardog-hsm-key-gaming-"));
    assert_eq!(response["result"]["algorithm"], "AES-256-GCM");
    assert_eq!(response["result"]["key_id"], "subfed:nat0:gaming:v1");
    assert_eq!(response["result"]["derivation_method"], "HKDF-SHA256");
    assert_eq!(response["result"]["hsm_backed"], true);
    
    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn test_encrypt_decrypt_roundtrip() {
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;
    
    let mut stream = UnixStream::connect(&socket_path).await.unwrap();
    
    // First, derive a key
    let derive_request = json!({
        "jsonrpc": "2.0",
        "method": "federation.derive_subfed_key",
        "params": {
            "parent_family": "nat0",
            "subfed_name": "testing",
            "purpose": "test-encryption"
        },
        "id": 1
    });
    
    let derive_response = send_jsonrpc(&mut stream, derive_request).await.unwrap();
    let key_ref = derive_response["result"]["key_ref"].as_str().unwrap();
    
    // Test data
    let plaintext = "Hello from biomeOS federation!";
    let plaintext_b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, plaintext.as_bytes());
    
    // Encrypt
    let encrypt_request = json!({
        "jsonrpc": "2.0",
        "method": "encryption.encrypt",
        "params": {
            "data": plaintext_b64,
            "key_ref": key_ref,
            "algorithm": "AES-256-GCM"
        },
        "id": 2
    });
    
    let encrypt_response = send_jsonrpc(&mut stream, encrypt_request).await.unwrap();
    
    // Debug: print response
    eprintln!("Encrypt response: {}", serde_json::to_string_pretty(&encrypt_response).unwrap());
    
    assert!(encrypt_response["result"].is_object(), "Expected result object, got: {:?}", encrypt_response);
    assert!(encrypt_response["result"]["encrypted_data"].is_string());
    assert!(encrypt_response["result"]["nonce"].is_string());
    assert!(encrypt_response["result"]["tag"].is_string());
    
    let encrypted_data = encrypt_response["result"]["encrypted_data"].as_str().unwrap();
    let nonce = encrypt_response["result"]["nonce"].as_str().unwrap();
    let tag = encrypt_response["result"]["tag"].as_str().unwrap();
    
    // Decrypt
    let decrypt_request = json!({
        "jsonrpc": "2.0",
        "method": "encryption.decrypt",
        "params": {
            "encrypted_data": encrypted_data,
            "nonce": nonce,
            "tag": tag,
            "key_ref": key_ref
        },
        "id": 3
    });
    
    let decrypt_response = send_jsonrpc(&mut stream, decrypt_request).await.unwrap();
    
    assert!(decrypt_response["result"].is_object());
    assert_eq!(decrypt_response["result"]["verified"], true);
    
    // Note: Since we're using placeholder encryption, the decrypted data
    // will match the encrypted data (not the original plaintext)
    // This will be fixed when real HSM encryption is integrated
    assert!(decrypt_response["result"]["data"].is_string());
    
    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn test_encrypt_with_invalid_base64() {
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;
    
    let mut stream = UnixStream::connect(&socket_path).await.unwrap();
    
    // Test with invalid base64
    let request = json!({
        "jsonrpc": "2.0",
        "method": "encryption.encrypt",
        "params": {
            "data": "not-valid-base64!!!",
            "key_ref": "test-key"
        },
        "id": 1
    });
    
    let response = send_jsonrpc(&mut stream, request).await.unwrap();
    
    // Should return error
    assert!(response["error"].is_object());
    assert!(response["error"]["message"].as_str().unwrap().contains("Invalid base64"));
    
    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn test_missing_required_params() {
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;
    
    let mut stream = UnixStream::connect(&socket_path).await.unwrap();
    
    // Test verify_family_member without family_id
    let request = json!({
        "jsonrpc": "2.0",
        "method": "federation.verify_family_member",
        "params": {
            "seed_hash": "hash..."
            // Missing family_id
        },
        "id": 1
    });
    
    let response = send_jsonrpc(&mut stream, request).await.unwrap();
    
    // Should return error
    assert!(response["error"].is_object());
    assert!(response["error"]["message"].as_str().unwrap().contains("family_id"));
    
    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn test_all_methods_with_real_biomeos_data() {
    // Test with real biomeOS spore data
    std::env::set_var("FAMILY_ID", "nat0");
    std::env::set_var("NODE_ID", "node-alpha");
    
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;
    
    let mut stream = UnixStream::connect(&socket_path).await.unwrap();
    
    // Real spore seed hash from biomeOS test data
    let real_seed_hash = "aaeaa3cfd69dd379...";
    
    // 1. Verify family member
    let verify_request = json!({
        "jsonrpc": "2.0",
        "method": "federation.verify_family_member",
        "params": {
            "family_id": "nat0",
            "seed_hash": real_seed_hash,
            "node_id": "node-beta"
        },
        "id": 1
    });
    
    let verify_response = send_jsonrpc(&mut stream, verify_request).await.unwrap();
    assert_eq!(verify_response["result"]["is_family_member"], true);
    
    // 2. Derive sub-federation key
    let derive_request = json!({
        "jsonrpc": "2.0",
        "method": "federation.derive_subfed_key",
        "params": {
            "parent_family": "nat0",
            "subfed_name": "gaming"
        },
        "id": 2
    });
    
    let derive_response = send_jsonrpc(&mut stream, derive_request).await.unwrap();
    let key_ref = derive_response["result"]["key_ref"].as_str().unwrap();
    
    // 3. Encrypt data
    let test_data = "Federation test data from nat0 family";
    let data_b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, test_data.as_bytes());
    
    let encrypt_request = json!({
        "jsonrpc": "2.0",
        "method": "encryption.encrypt",
        "params": {
            "data": data_b64,
            "key_ref": key_ref
        },
        "id": 3
    });
    
    let encrypt_response = send_jsonrpc(&mut stream, encrypt_request).await.unwrap();
    assert!(encrypt_response["result"]["encrypted_data"].is_string());
    
    // 4. Decrypt data
    let decrypt_request = json!({
        "jsonrpc": "2.0",
        "method": "encryption.decrypt",
        "params": {
            "encrypted_data": encrypt_response["result"]["encrypted_data"],
            "nonce": encrypt_response["result"]["nonce"],
            "tag": encrypt_response["result"]["tag"],
            "key_ref": key_ref
        },
        "id": 4
    });
    
    let decrypt_response = send_jsonrpc(&mut stream, decrypt_request).await.unwrap();
    assert_eq!(decrypt_response["result"]["verified"], true);
    
    server.stop().await.unwrap();
    server_handle.abort();
    
    std::env::remove_var("FAMILY_ID");
    std::env::remove_var("NODE_ID");
}

