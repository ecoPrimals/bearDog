// SPDX-License-Identifier: AGPL-3.0-or-later

use base64::Engine;

use super::config::current_unix_timestamp;
use super::crypto::{build_enrollment_message, compute_hmac_sha256, derive_enrollment_key};
use super::replay_cache::ReplayCache;
use crate::unix_socket_ipc::handlers::btsp::BtspHandler;

const TEST_SEED: &str = "test-family-seed-for-enrollment";

fn set_test_seed() {
    beardog_errors::process_env::set_var("FAMILY_SEED", TEST_SEED);
}

fn clear_test_env() {
    beardog_errors::process_env::remove_var("FAMILY_SEED");
    beardog_errors::process_env::remove_var("BEARDOG_FAMILY_SEED");
    beardog_errors::process_env::remove_var("BEARDOG_ENROLLMENT_SEED_GENERATION");
    beardog_errors::process_env::remove_var("FAMILY_ID");
}

fn make_valid_proof_gen(
    node_id: &str,
    public_key: &str,
    timestamp: u64,
    generation: u32,
) -> String {
    let enrollment_key = derive_enrollment_key(TEST_SEED.as_bytes(), generation);
    let message = build_enrollment_message(node_id, public_key, timestamp, generation);
    let proof = compute_hmac_sha256(&enrollment_key, message.as_bytes());
    base64::engine::general_purpose::STANDARD.encode(&proof)
}

fn make_valid_proof(node_id: &str, public_key: &str, timestamp: u64) -> String {
    make_valid_proof_gen(node_id, public_key, timestamp, 0)
}

// --- Unit tests for derivation ---

#[test]
fn enrollment_key_deterministic() {
    let k1 = derive_enrollment_key(b"seed", 0);
    let k2 = derive_enrollment_key(b"seed", 0);
    assert_eq!(k1, k2);
}

#[test]
fn enrollment_key_varies_by_generation() {
    let k0 = derive_enrollment_key(b"seed", 0);
    let k1 = derive_enrollment_key(b"seed", 1);
    assert_ne!(k0, k1);
}

#[test]
fn enrollment_key_varies_by_seed() {
    let ka = derive_enrollment_key(b"seed-a", 0);
    let kb = derive_enrollment_key(b"seed-b", 0);
    assert_ne!(ka, kb);
}

#[test]
fn enrollment_message_includes_generation() {
    let msg = build_enrollment_message("node", "pk", 1000, 3);
    assert_eq!(msg, "node|pk|1000|3");
}

#[test]
fn hmac_deterministic() {
    let key = b"test-seed";
    let data = b"southGate|pk|1234|0";
    let m1 = compute_hmac_sha256(key, data);
    let m2 = compute_hmac_sha256(key, data);
    assert_eq!(m1, m2);
    assert_eq!(m1.len(), 32);
}

#[test]
fn hmac_varies_by_message() {
    let key = b"test-seed";
    let m1 = compute_hmac_sha256(key, b"nodeA|pk|1000|0");
    let m2 = compute_hmac_sha256(key, b"nodeB|pk|1000|0");
    assert_ne!(m1, m2);
}

#[test]
fn timestamp_window_rejects_old() {
    let window = 300_u64;
    let now = current_unix_timestamp();
    let old_ts = now.saturating_sub(window + 100);
    let delta = now - old_ts;
    assert!(delta > window);
}

#[test]
fn timestamp_window_accepts_recent() {
    let window = 300_u64;
    let now = current_unix_timestamp();
    let recent_ts = now.saturating_sub(60);
    let delta = now - recent_ts;
    assert!(delta <= window);
}

#[test]
fn timestamp_window_rejects_future() {
    let window = 300_u64;
    let now = current_unix_timestamp();
    let future_ts = now + window + 100;
    let delta = future_ts - now;
    assert!(delta > window);
}

#[test]
fn replay_cache_detects_duplicate() {
    let cache = ReplayCache::new();
    let digest = [42u8; 32];
    assert!(!cache.check_and_record(digest, 1000));
    assert!(cache.check_and_record(digest, 1000));
}

#[test]
fn replay_cache_allows_different_digests() {
    let cache = ReplayCache::new();
    assert!(!cache.check_and_record([1u8; 32], 1000));
    assert!(!cache.check_and_record([2u8; 32], 1000));
}

// --- Integration tests (serial — env manipulation) ---

#[serial_test::serial]
#[tokio::test]
async fn verify_valid_proof_gen0() {
    set_test_seed();
    let handler = BtspHandler::new();

    let timestamp = current_unix_timestamp();
    let proof = make_valid_proof("southGate", "wg-pubkey-abc123", timestamp);

    let params = serde_json::json!({
        "node_id": "southGate",
        "public_key": "wg-pubkey-abc123",
        "timestamp": timestamp,
        "proof": proof,
    });

    let result = handler.handle_enrollment_verify(Some(&params)).unwrap();

    assert_eq!(result["verified"], true);
    assert_eq!(result["verified_generation"], 0);
    clear_test_env();
}

#[serial_test::serial]
#[tokio::test]
async fn verify_valid_proof_gen1() {
    set_test_seed();
    beardog_errors::process_env::set_var("BEARDOG_ENROLLMENT_SEED_GENERATION", "1");
    let handler = BtspHandler::new();

    let timestamp = current_unix_timestamp();
    let proof = make_valid_proof_gen("southGate", "wg-pubkey", timestamp, 1);

    let params = serde_json::json!({
        "node_id": "southGate",
        "public_key": "wg-pubkey",
        "timestamp": timestamp,
        "proof": proof,
        "seed_generation": 1,
    });

    let result = handler.handle_enrollment_verify(Some(&params)).unwrap();

    assert_eq!(result["verified"], true);
    assert_eq!(result["verified_generation"], 1);
    clear_test_env();
}

#[serial_test::serial]
#[tokio::test]
async fn verify_grace_period_accepts_previous_gen() {
    set_test_seed();
    beardog_errors::process_env::set_var("BEARDOG_ENROLLMENT_SEED_GENERATION", "2");
    let handler = BtspHandler::new();

    let timestamp = current_unix_timestamp();
    let proof = make_valid_proof_gen("southGate", "wg-pubkey", timestamp, 1);

    let params = serde_json::json!({
        "node_id": "southGate",
        "public_key": "wg-pubkey",
        "timestamp": timestamp,
        "proof": proof,
        "seed_generation": 1,
    });

    let result = handler.handle_enrollment_verify(Some(&params)).unwrap();

    assert_eq!(result["verified"], true);
    assert_eq!(result["verified_generation"], 1);
    clear_test_env();
}

#[serial_test::serial]
#[tokio::test]
async fn verify_rejects_stale_generation() {
    set_test_seed();
    beardog_errors::process_env::set_var("BEARDOG_ENROLLMENT_SEED_GENERATION", "5");
    let handler = BtspHandler::new();

    let timestamp = current_unix_timestamp();
    let proof = make_valid_proof_gen("southGate", "wg-pubkey", timestamp, 2);

    let params = serde_json::json!({
        "node_id": "southGate",
        "public_key": "wg-pubkey",
        "timestamp": timestamp,
        "proof": proof,
        "seed_generation": 2,
    });

    let result = handler.handle_enrollment_verify(Some(&params)).unwrap();

    assert_eq!(result["verified"], false);
    assert!(result["reason"].as_str().unwrap().contains("not accepted"));
    clear_test_env();
}

#[serial_test::serial]
#[tokio::test]
async fn verify_invalid_proof() {
    set_test_seed();
    let handler = BtspHandler::new();

    let params = serde_json::json!({
        "node_id": "southGate",
        "public_key": "wg-pubkey-abc123",
        "timestamp": current_unix_timestamp(),
        "proof": base64::engine::general_purpose::STANDARD.encode(&[0u8; 32]),
    });

    let result = handler.handle_enrollment_verify(Some(&params)).unwrap();

    assert_eq!(result["verified"], false);
    assert!(
        result["reason"]
            .as_str()
            .unwrap()
            .contains("does not match")
    );
    clear_test_env();
}

#[serial_test::serial]
#[tokio::test]
async fn verify_fails_without_seed() {
    clear_test_env();
    let handler = BtspHandler::new();

    let params = serde_json::json!({
        "node_id": "southGate",
        "public_key": "pk",
        "timestamp": current_unix_timestamp(),
        "proof": base64::engine::general_purpose::STANDARD.encode(&[0u8; 32]),
    });

    let result = handler.handle_enrollment_verify(Some(&params));
    assert!(result.is_err());
}

#[serial_test::serial]
#[tokio::test]
async fn verify_rejects_expired_timestamp() {
    set_test_seed();
    let handler = BtspHandler::new();

    let old_ts = current_unix_timestamp().saturating_sub(600);
    let proof = make_valid_proof("southGate", "wg-pubkey", old_ts);

    let params = serde_json::json!({
        "node_id": "southGate",
        "public_key": "wg-pubkey",
        "timestamp": old_ts,
        "proof": proof,
    });

    let result = handler.handle_enrollment_verify(Some(&params)).unwrap();

    assert_eq!(result["verified"], false);
    assert!(
        result["reason"]
            .as_str()
            .unwrap()
            .contains("validity window")
    );
    clear_test_env();
}

#[serial_test::serial]
#[tokio::test]
async fn verify_rejects_replay() {
    set_test_seed();
    let handler = BtspHandler::new();

    let timestamp = current_unix_timestamp();
    let proof = make_valid_proof("southGate", "wg-pubkey-abc", timestamp);

    let params = serde_json::json!({
        "node_id": "southGate",
        "public_key": "wg-pubkey-abc",
        "timestamp": timestamp,
        "proof": proof,
    });

    let result = handler.handle_enrollment_verify(Some(&params)).unwrap();
    assert_eq!(result["verified"], true);

    let result2 = handler.handle_enrollment_verify(Some(&params)).unwrap();
    assert_eq!(result2["verified"], false);
    assert!(result2["reason"].as_str().unwrap().contains("replay"));

    clear_test_env();
}

#[serial_test::serial]
#[tokio::test]
async fn verify_rejects_far_future_timestamp() {
    set_test_seed();
    let handler = BtspHandler::new();

    let future_ts = current_unix_timestamp() + 600;
    let proof = make_valid_proof("southGate", "wg-pubkey", future_ts);

    let params = serde_json::json!({
        "node_id": "southGate",
        "public_key": "wg-pubkey",
        "timestamp": future_ts,
        "proof": proof,
    });

    let result = handler.handle_enrollment_verify(Some(&params)).unwrap();

    assert_eq!(result["verified"], false);
    assert!(
        result["reason"]
            .as_str()
            .unwrap()
            .contains("validity window")
    );
    clear_test_env();
}

#[serial_test::serial]
#[tokio::test]
async fn cross_gen_proof_rejected_without_grace() {
    set_test_seed();
    // Current gen is 0 (default), proof uses gen 1 → rejected
    let handler = BtspHandler::new();

    let timestamp = current_unix_timestamp();
    let proof = make_valid_proof_gen("southGate", "wg-pubkey", timestamp, 1);

    let params = serde_json::json!({
        "node_id": "southGate",
        "public_key": "wg-pubkey",
        "timestamp": timestamp,
        "proof": proof,
        "seed_generation": 1,
    });

    let result = handler.handle_enrollment_verify(Some(&params)).unwrap();

    assert_eq!(result["verified"], false);
    assert!(result["reason"].as_str().unwrap().contains("not accepted"));
    clear_test_env();
}

// --- Nuclear lineage distance tests ---

#[serial_test::serial]
#[tokio::test]
async fn verify_with_lineage_proof_returns_tier() {
    set_test_seed();
    beardog_errors::process_env::set_var("BEARDOG_NODE_ID", "root");
    let handler = BtspHandler::new();

    let timestamp = current_unix_timestamp();
    let proof = make_valid_proof("child-1", "wg-pubkey", timestamp);

    let params = serde_json::json!({
        "node_id": "child-1",
        "public_key": "wg-pubkey",
        "timestamp": timestamp,
        "proof": proof,
        "lineage_proof": {
            "chain_id": "test-chain",
            "path": ["root", "child-1"],
            "generation": 0
        }
    });

    let result = handler.handle_enrollment_verify(Some(&params)).unwrap();

    assert_eq!(result["verified"], true);
    // Verifier is "root", enrollee is "child-1" in path [root, child-1]
    // Verifier is at position 0 (depth 0), enrollee at depth 1 → distance 1 → "kin"
    assert_eq!(result["enrollment_tier"], "kin");
    assert_eq!(result["genetic_distance"], 1);
    clear_test_env();
    beardog_errors::process_env::remove_var("BEARDOG_NODE_ID");
}

#[serial_test::serial]
#[tokio::test]
async fn verify_without_lineage_proof_omits_tier() {
    set_test_seed();
    let handler = BtspHandler::new();

    let timestamp = current_unix_timestamp();
    let proof = make_valid_proof("southGate", "wg-pubkey", timestamp);

    let params = serde_json::json!({
        "node_id": "southGate",
        "public_key": "wg-pubkey",
        "timestamp": timestamp,
        "proof": proof,
    });

    let result = handler.handle_enrollment_verify(Some(&params)).unwrap();

    assert_eq!(result["verified"], true);
    assert!(result.get("enrollment_tier").is_none() || result["enrollment_tier"].is_null());
    assert!(result.get("genetic_distance").is_none() || result["genetic_distance"].is_null());
    clear_test_env();
}

#[serial_test::serial]
#[tokio::test]
async fn verify_lineage_proof_sibling_distance() {
    set_test_seed();
    beardog_errors::process_env::set_var("BEARDOG_NODE_ID", "child-1");
    let handler = BtspHandler::new();

    let timestamp = current_unix_timestamp();
    let proof = make_valid_proof("child-2", "wg-pubkey", timestamp);

    // child-2's path: [root, child-2]. Verifier is "child-1" which is NOT
    // in child-2's path, so distance falls back to enrollee's depth (1)
    let params = serde_json::json!({
        "node_id": "child-2",
        "public_key": "wg-pubkey",
        "timestamp": timestamp,
        "proof": proof,
        "lineage_proof": {
            "chain_id": "test-chain",
            "path": ["root", "child-2"],
            "generation": 0
        }
    });

    let result = handler.handle_enrollment_verify(Some(&params)).unwrap();

    assert_eq!(result["verified"], true);
    // Verifier not in path → fallback to enrollee depth = 1 → "kin"
    assert_eq!(result["enrollment_tier"], "kin");
    assert_eq!(result["genetic_distance"], 1);
    clear_test_env();
    beardog_errors::process_env::remove_var("BEARDOG_NODE_ID");
}

#[serial_test::serial]
#[tokio::test]
async fn verify_lineage_proof_deep_enrollee() {
    set_test_seed();
    beardog_errors::process_env::set_var("BEARDOG_NODE_ID", "root");
    let handler = BtspHandler::new();

    let timestamp = current_unix_timestamp();
    let proof = make_valid_proof("gc-3", "wg-pubkey", timestamp);

    // Deep enrollee: root → child-1 → gc-1 → gc-2 → gc-3 (depth 4)
    // Verifier at root (pos 0), enrollee depth 4 → distance 4 → "extended"
    let params = serde_json::json!({
        "node_id": "gc-3",
        "public_key": "wg-pubkey",
        "timestamp": timestamp,
        "proof": proof,
        "lineage_proof": {
            "chain_id": "test-chain",
            "path": ["root", "child-1", "gc-1", "gc-2", "gc-3"],
            "generation": 0
        }
    });

    let result = handler.handle_enrollment_verify(Some(&params)).unwrap();

    assert_eq!(result["verified"], true);
    assert_eq!(result["enrollment_tier"], "extended");
    assert_eq!(result["genetic_distance"], 4);
    clear_test_env();
    beardog_errors::process_env::remove_var("BEARDOG_NODE_ID");
}

#[serial_test::serial]
#[tokio::test]
async fn verify_lineage_proof_distant_enrollee() {
    set_test_seed();
    beardog_errors::process_env::set_var("BEARDOG_NODE_ID", "root");
    let handler = BtspHandler::new();

    let timestamp = current_unix_timestamp();
    let proof = make_valid_proof("far-node", "wg-pubkey", timestamp);

    // Very deep: 6 hops from root → distance 6 → "distant"
    let params = serde_json::json!({
        "node_id": "far-node",
        "public_key": "wg-pubkey",
        "timestamp": timestamp,
        "proof": proof,
        "lineage_proof": {
            "chain_id": "test-chain",
            "path": ["root", "a", "b", "c", "d", "e", "far-node"],
            "generation": 0
        }
    });

    let result = handler.handle_enrollment_verify(Some(&params)).unwrap();

    assert_eq!(result["verified"], true);
    assert_eq!(result["enrollment_tier"], "distant");
    assert_eq!(result["genetic_distance"], 6);
    clear_test_env();
    beardog_errors::process_env::remove_var("BEARDOG_NODE_ID");
}

#[serial_test::serial]
#[tokio::test]
async fn verify_lineage_proof_self_enrollment() {
    set_test_seed();
    beardog_errors::process_env::set_var("BEARDOG_NODE_ID", "southGate");
    let handler = BtspHandler::new();

    let timestamp = current_unix_timestamp();
    let proof = make_valid_proof("southGate", "wg-pubkey", timestamp);

    let params = serde_json::json!({
        "node_id": "southGate",
        "public_key": "wg-pubkey",
        "timestamp": timestamp,
        "proof": proof,
        "lineage_proof": {
            "chain_id": "test-chain",
            "path": ["root", "southGate"],
            "generation": 0
        }
    });

    let result = handler.handle_enrollment_verify(Some(&params)).unwrap();

    assert_eq!(result["verified"], true);
    // node_id == verifier_id → distance 0 → "identity"
    assert_eq!(result["enrollment_tier"], "identity");
    assert_eq!(result["genetic_distance"], 0);
    clear_test_env();
    beardog_errors::process_env::remove_var("BEARDOG_NODE_ID");
}
