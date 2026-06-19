// SPDX-License-Identifier: AGPL-3.0-or-later

//! BirdSong lineage-based encryption CLI handlers
//!
//! Implements privacy-preserving encryption where only lineage members can decrypt.

mod decrypt;
mod encrypt;
mod lineage;

#[allow(unused_imports)] // Public DI/test hooks preserved from pre-refactor API
pub use decrypt::{handle_birdsong_decrypt, handle_birdsong_decrypt_with_home};
#[allow(unused_imports)]
pub use encrypt::{handle_birdsong_encrypt, handle_birdsong_encrypt_with_home};

#[cfg(test)]
mod tests {
    use super::decrypt::{handle_birdsong_decrypt, handle_birdsong_decrypt_with_home};
    use super::encrypt::handle_birdsong_encrypt_with_home;
    use super::lineage::{compute_merkle_root, parse_lineage_hint};
    use crate::handlers::key_store;
    use tempfile::TempDir;

    fn sample_stored_key(id: &str, material: &[u8]) -> key_store::StoredKey {
        key_store::StoredKey {
            key_id: id.to_string(),
            algorithm: "aes256-gcm".to_string(),
            hsm_name: "test-hsm".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            key_material_b64: key_store::base64_encode(material),
            generation: 0,
            parent_key_id: None,
            derivation_purpose: None,
            children: vec![],
            lineage: Some(key_store::KeyLineageInfo {
                parent_key_id: None,
                depth: 0,
            }),
            expires_at: None,
            usage: None,
            purpose: None,
        }
    }

    #[test]
    fn test_parse_lineage_hint_direct_ancestors() {
        let hint =
            parse_lineage_hint("DirectAncestors", "root-123").expect("parse DirectAncestors hint");
        assert_eq!(hint.root_id, "root-123");
        assert_eq!(hint.min_depth, 0);
        assert_eq!(hint.max_depth, 1);
    }

    #[test]
    fn test_parse_lineage_hint_all_descendants() {
        let hint =
            parse_lineage_hint("AllDescendants", "root-456").expect("parse AllDescendants hint");
        assert_eq!(hint.root_id, "root-456");
        assert_eq!(hint.min_depth, 0);
        assert_eq!(hint.max_depth, 100);
    }

    #[test]
    fn test_parse_lineage_hint_depth_range() {
        let hint = parse_lineage_hint("Depth:2-5", "root-789").expect("parse Depth:2-5 hint");
        assert_eq!(hint.root_id, "root-789");
        assert_eq!(hint.min_depth, 2);
        assert_eq!(hint.max_depth, 5);
    }

    #[test]
    fn test_parse_lineage_hint_root_only() {
        let hint = parse_lineage_hint("RootOnly", "root-000").expect("parse RootOnly hint");
        assert_eq!(hint.root_id, "root-000");
        assert_eq!(hint.min_depth, 0);
        assert_eq!(hint.max_depth, 0);
    }

    #[test]
    fn test_parse_lineage_hint_invalid() {
        let result = parse_lineage_hint("InvalidHint", "root-123");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_lineage_hint_invalid_depth_format() {
        let result = parse_lineage_hint("Depth:invalid", "root-123");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_lineage_hint_depth_too_many_parts() {
        assert!(parse_lineage_hint("Depth:0-1-2", "r").is_err());
    }

    #[test]
    fn test_compute_merkle_root_empty() {
        assert!(compute_merkle_root(&[]).is_empty());
    }

    #[test]
    fn test_compute_merkle_root_single_leaf() {
        let leaf = vec![1u8; 32];
        let root = compute_merkle_root(std::slice::from_ref(&leaf));
        assert_eq!(root, leaf);
    }

    #[test]
    fn test_compute_merkle_root_two_leaves() {
        let a = vec![1u8; 32];
        let b = vec![2u8; 32];
        let root = compute_merkle_root(&[a, b]);
        assert_eq!(root.len(), 32);
    }

    #[test]
    fn test_compute_merkle_root_three_leaves() {
        let leaves: Vec<Vec<u8>> = (0u8..3).map(|i| vec![i; 32]).collect();
        let root = compute_merkle_root(&leaves);
        assert_eq!(root.len(), 32);
    }

    #[tokio::test]
    async fn test_birdsong_encrypt_decrypt_roundtrip_full_key_material() {
        let dir = TempDir::new().expect("create temp directory for birdsong roundtrip test");
        let home = dir.path();

        let root = sample_stored_key("root-lineage-1", &[9u8; 32]);
        key_store::save_key_to_home(&root, home).expect("save root-lineage-1");

        let out = dir.path().join("out.birdsong");
        handle_birdsong_encrypt_with_home(
            "hello-roundtrip",
            "DirectAncestors",
            "root-lineage-1",
            Some(out.to_str().expect("out.birdsong path must be valid UTF-8")),
            home,
        )
        .await
        .expect("encrypt");

        handle_birdsong_decrypt_with_home(
            out.to_str().expect("out.birdsong path must be valid UTF-8"),
            "root-lineage-1",
            home,
        )
        .await
        .expect("decrypt");
    }

    #[tokio::test]
    async fn test_birdsong_encrypt_uses_short_key_material_hkdf_branch() {
        let dir =
            TempDir::new().expect("create temp directory for short key material birdsong test");
        let home = dir.path();

        let root = sample_stored_key("short-root", &[1u8; 16]);
        key_store::save_key_to_home(&root, home).expect("save short-root");

        let out = dir.path().join("short.birdsong");
        handle_birdsong_encrypt_with_home(
            "m",
            "RootOnly",
            "short-root",
            Some(
                out.to_str()
                    .expect("short.birdsong path must be valid UTF-8"),
            ),
            home,
        )
        .await
        .expect("encrypt");

        handle_birdsong_decrypt_with_home(
            out.to_str()
                .expect("short.birdsong path must be valid UTF-8"),
            "short-root",
            home,
        )
        .await
        .expect("decrypt");
    }

    #[tokio::test]
    async fn test_birdsong_decrypt_lineage_mismatch() {
        let dir = TempDir::new().expect("create temp directory for birdsong lineage mismatch test");
        let home = dir.path();

        let a = sample_stored_key("root-a", &[2u8; 32]);
        let b = sample_stored_key("root-b", &[3u8; 32]);
        key_store::save_key_to_home(&a, home).expect("save root-a");
        key_store::save_key_to_home(&b, home).expect("save root-b");

        let out = dir.path().join("mismatch.birdsong");
        handle_birdsong_encrypt_with_home(
            "x",
            "AllDescendants",
            "root-a",
            Some(
                out.to_str()
                    .expect("mismatch.birdsong path must be valid UTF-8"),
            ),
            home,
        )
        .await
        .expect("encrypt for lineage mismatch test");

        let err = handle_birdsong_decrypt_with_home(
            out.to_str()
                .expect("mismatch.birdsong path must be valid UTF-8"),
            "root-b",
            home,
        )
        .await
        .expect_err("lineage mismatch");
        assert!(
            err.to_string().to_lowercase().contains("lineage")
                || err.to_string().contains("Lineage")
        );
    }

    #[tokio::test]
    async fn test_birdsong_decrypt_missing_file() {
        assert!(
            handle_birdsong_decrypt("/no/such/file.birdsong", "k")
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn test_birdsong_decrypt_invalid_json() {
        let dir = TempDir::new().expect("create temp directory for invalid birdsong JSON test");
        let p = dir.path().join("bad.json");
        std::fs::write(&p, b"not json").expect("write invalid JSON fixture");
        assert!(
            handle_birdsong_decrypt(p.to_str().expect("bad.json path must be valid UTF-8"), "k",)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn test_birdsong_decrypt_key_without_lineage() {
        let dir = TempDir::new().expect("create temp directory for decrypt without lineage test");
        let home = dir.path();

        let mut k = sample_stored_key("no-lineage", &[5u8; 32]);
        k.lineage = None;
        key_store::save_key_to_home(&k, home).expect("save no-lineage key");

        let out = dir.path().join("x.birdsong");
        handle_birdsong_encrypt_with_home(
            "z",
            "RootOnly",
            "no-lineage",
            Some(out.to_str().expect("x.birdsong path must be valid UTF-8")),
            home,
        )
        .await
        .expect("encrypt for no-lineage decrypt test");

        let err = handle_birdsong_decrypt_with_home(
            out.to_str().expect("x.birdsong path must be valid UTF-8"),
            "no-lineage",
            home,
        )
        .await
        .expect_err("no lineage on decrypt key");
        assert!(err.to_string().contains("lineage") || err.to_string().contains("Lineage"));
    }

    #[tokio::test]
    async fn test_birdsong_decrypt_with_child_key_lineage_chain() {
        let dir = TempDir::new().expect("temp dir for child lineage decrypt test");
        let home = dir.path();

        let root = sample_stored_key("lineage-root", &[9u8; 32]);
        let mut child = sample_stored_key("lineage-child", &[9u8; 32]);
        child.lineage = Some(key_store::KeyLineageInfo {
            parent_key_id: Some("lineage-root".to_string()),
            depth: 1,
        });
        key_store::save_key_to_home(&root, home).expect("save lineage-root");
        key_store::save_key_to_home(&child, home).expect("save lineage-child");

        let out = dir.path().join("chain.birdsong");
        handle_birdsong_encrypt_with_home(
            "child-lineage-msg",
            "AllDescendants",
            "lineage-root",
            Some(
                out.to_str()
                    .expect("chain.birdsong path must be valid UTF-8"),
            ),
            home,
        )
        .await
        .expect("encrypt for child lineage test");

        handle_birdsong_decrypt_with_home(
            out.to_str()
                .expect("chain.birdsong path must be valid UTF-8"),
            "lineage-child",
            home,
        )
        .await
        .expect("decrypt with child key in lineage");
    }
}
