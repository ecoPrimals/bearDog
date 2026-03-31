// SPDX-License-Identifier: AGPL-3.0-only

use super::*;
use chrono::Utc;
use tempfile::TempDir;

#[test]
fn test_revocation_list_new() {
    let list = RevocationList::new();
    assert_eq!(list.revoked_keys.len(), 0);
    assert_eq!(list.version, 1);
}

#[test]
fn test_revoke_key() {
    let mut list = RevocationList::new();

    assert!(!list.is_revoked("test-key"));

    list.revoke(
        "test-key".to_string(),
        Some("Testing".to_string()),
        None,
        false,
    );

    assert!(list.is_revoked("test-key"));
    assert_eq!(list.version, 2); // Incremented
    assert_eq!(list.revoked_keys.len(), 1);
}

#[test]
fn test_unrevoke_key() {
    let mut list = RevocationList::new();

    list.revoke("test-key".to_string(), None, None, false);
    assert!(list.is_revoked("test-key"));

    let removed = list.unrevoke("test-key");
    assert!(removed);
    assert!(!list.is_revoked("test-key"));
    assert_eq!(list.version, 3); // Incremented twice
}

#[test]
fn test_merge_revocation_lists() {
    let mut list1 = RevocationList::new();
    list1.revoke("key1".to_string(), None, None, false);

    let mut list2 = RevocationList::new();
    list2.revoke("key2".to_string(), None, None, false);

    list1.merge(&list2);

    assert!(list1.is_revoked("key1"));
    assert!(list1.is_revoked("key2"));
    assert_eq!(list1.revoked_keys.len(), 2);
}

#[test]
fn test_unrevoke_nonexistent() {
    let mut list = RevocationList::new();
    let removed = list.unrevoke("nonexistent");
    assert!(!removed);
}

#[test]
fn test_revocation_list_load_save_roundtrip_via_home() {
    let dir = TempDir::new().expect("create temp directory for revocation roundtrip test");

    let mut list = RevocationList::new();
    list.revoke(
        "roundtrip-key".to_string(),
        Some("test".to_string()),
        None,
        false,
    );
    list.save_to_home(dir.path()).expect("save");

    let loaded = RevocationList::load_from_home(dir.path()).expect("load");
    assert!(loaded.is_revoked("roundtrip-key"));
    assert_eq!(
        loaded.revoked_keys["roundtrip-key"].reason,
        Some("test".to_string())
    );
}

#[test]
fn test_revocation_export_import_merge() {
    let dir = TempDir::new().expect("create temp directory for export/import merge test");
    let export_a = dir.path().join("a.json");
    let mut a = RevocationList::new();
    a.revoke("ka".to_string(), None, None, false);
    a.export(
        export_a
            .to_str()
            .expect("export path must be valid UTF-8 for test"),
    )
    .expect("export revocation list a");

    let mut b = RevocationList::new();
    b.revoke("kb".to_string(), None, None, false);
    b.merge(
        &RevocationList::import(
            export_a
                .to_str()
                .expect("export path must be valid UTF-8 for test"),
        )
        .expect("import revocation list from export file"),
    );
    assert!(b.is_revoked("ka"));
    assert!(b.is_revoked("kb"));
}

#[test]
fn test_merge_prefers_newer_timestamp() {
    let mut older = RevocationList::new();
    older.revoked_keys.insert(
        "k".to_string(),
        RevocationEntry {
            key_id: "k".to_string(),
            revoked_at: "2020-01-01T00:00:00Z".to_string(),
            effective_at: None,
            reason: Some("old".to_string()),
            revoked_by: "tester".to_string(),
            cascade: false,
        },
    );

    let mut newer = RevocationList::new();
    newer.revoked_keys.insert(
        "k".to_string(),
        RevocationEntry {
            key_id: "k".to_string(),
            revoked_at: "2025-01-01T00:00:00Z".to_string(),
            effective_at: None,
            reason: Some("newer".to_string()),
            revoked_by: "tester".to_string(),
            cascade: false,
        },
    );

    older.merge(&newer);
    assert_eq!(
        older
            .revoked_keys
            .get("k")
            .expect("key k must exist after merge with newer entry")
            .reason,
        Some("newer".to_string())
    );
}

#[tokio::test]
async fn test_handle_key_check_revocation_not_revoked() {
    let dir = TempDir::new().expect("create temp directory for revocation check test");
    handle_key_check_revocation_with_home("fresh-key-id", dir.path())
        .await
        .expect("handle_key_check_revocation_with_home for non-revoked key");
}

#[tokio::test]
async fn test_handle_key_list_revocations_empty() {
    let dir = TempDir::new().expect("create temp directory for empty revocation list test");
    handle_key_list_revocations_with_home(dir.path())
        .await
        .expect("handle_key_list_revocations_with_home on empty store");
}

#[tokio::test]
async fn test_handle_key_revoke_idempotent() {
    let dir = TempDir::new().expect("create temp directory for idempotent revoke test");

    handle_key_revoke_with_home("dup-key", Some("r1"), None, false, dir.path())
        .await
        .expect("first revoke of dup-key");
    handle_key_revoke_with_home("dup-key", Some("r2"), None, false, dir.path())
        .await
        .expect("second revoke of dup-key (idempotent)");
}

#[tokio::test]
async fn test_handle_key_revoke_with_cascade() {
    let dir = TempDir::new().expect("create temp directory for cascade revoke test");

    use crate::handlers::key_store;

    let parent = key_store::StoredKey {
        key_id: "parent-k".to_string(),
        algorithm: "aes256-gcm".to_string(),
        hsm_name: "h".to_string(),
        created_at: Utc::now().to_rfc3339(),
        key_material_b64: key_store::base64_encode(&[1u8; 32]),
        generation: 0,
        parent_key_id: None,
        derivation_purpose: None,
        children: vec!["child-k".to_string()],
        lineage: None,
        expires_at: None,
        usage: None,
        purpose: None,
    };
    key_store::save_key_to_home(&parent, dir.path())
        .expect("save parent key for cascade revoke test");

    handle_key_revoke_with_home("parent-k", Some("rotate"), None, true, dir.path())
        .await
        .expect("revoke parent with cascade");

    let list = RevocationList::load_from_home(dir.path()).expect("load revocation list");
    assert!(list.is_revoked("parent-k"));
    assert!(list.is_revoked("child-k"));
}

#[tokio::test]
async fn test_handle_revocation_export_and_import_handlers() {
    let dir = TempDir::new().expect("create temp directory for export handler test");

    let mut list = RevocationList::new();
    list.revoke("export-me".to_string(), None, None, false);
    list.save_to_home(dir.path())
        .expect("save revocation list before export");

    let export_path = dir.path().join("rev.json");
    handle_revocation_export_with_home(
        export_path
            .to_str()
            .expect("export path must be valid UTF-8"),
        dir.path(),
    )
    .await
    .expect("export revocation list");

    let home2 = TempDir::new().expect("create second temp home for import test");
    handle_revocation_import_with_home(
        export_path
            .to_str()
            .expect("export path must be valid UTF-8"),
        home2.path(),
    )
    .await
    .expect("import revocation list");
    let merged = RevocationList::load_from_home(home2.path()).expect("load merged list");
    assert!(merged.is_revoked("export-me"));
}

#[test]
fn test_load_from_home_invalid_json_errors() {
    let dir = TempDir::new().expect("create temp directory for invalid JSON test");
    let path = RevocationList::revocation_file_path_for_home(dir.path());
    std::fs::create_dir_all(
        path.parent()
            .expect("revocation file path must have a parent directory"),
    )
    .expect("create parent dirs for revocation file");
    std::fs::write(&path, "{ not json").expect("write invalid JSON fixture");
    assert!(RevocationList::load_from_home(dir.path()).is_err());
}

#[tokio::test]
async fn test_handle_key_check_revocation_with_home_revoked() {
    let dir = TempDir::new().expect("create temp directory for revoked key check test");
    let mut list = RevocationList::new();
    list.revoke(
        "bad-key".to_string(),
        Some("compromise".to_string()),
        None,
        false,
    );
    list.save_to_home(dir.path())
        .expect("save revocation list with bad-key");

    handle_key_check_revocation_with_home("bad-key", dir.path())
        .await
        .expect("check revocation for revoked key");
}

#[tokio::test]
async fn test_handle_key_list_revocations_with_home_nonempty() {
    let dir = TempDir::new().expect("create temp directory for nonempty revocation list test");
    let mut list = RevocationList::new();
    list.revoke("k1".to_string(), None, None, false);
    list.save_to_home(dir.path())
        .expect("save revocation list with k1");

    handle_key_list_revocations_with_home(dir.path())
        .await
        .expect("list revocations with entries");
}

#[test]
fn test_merge_keeps_existing_when_other_is_older() {
    let mut current = RevocationList::new();
    current.revoked_keys.insert(
        "same".to_string(),
        RevocationEntry {
            key_id: "same".to_string(),
            revoked_at: "2025-06-01T00:00:00Z".to_string(),
            effective_at: None,
            reason: Some("current".to_string()),
            revoked_by: "a".to_string(),
            cascade: false,
        },
    );

    let mut older = RevocationList::new();
    older.revoked_keys.insert(
        "same".to_string(),
        RevocationEntry {
            key_id: "same".to_string(),
            revoked_at: "2020-01-01T00:00:00Z".to_string(),
            effective_at: None,
            reason: Some("stale".to_string()),
            revoked_by: "b".to_string(),
            cascade: false,
        },
    );

    current.merge(&older);
    assert_eq!(
        current
            .revoked_keys
            .get("same")
            .expect("key same must exist after merge")
            .reason
            .as_deref(),
        Some("current")
    );
}
