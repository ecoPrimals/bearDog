// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use crate::hsm::fido2::types::{Fido2Capabilities, Fido2DeviceInfo, Fido2Transport};
use beardog_traits::unified::BearDogProvider;
use beardog_traits::unified::hsm_multi_credential::{
    CredentialInfo, CredentialRequest, MultiCredentialHsmProvider,
};
use beardog_types::canonical::providers_unified::traits::HealthStatus;
use chrono::Utc;

fn make_device(
    resident_keys: bool,
    hmac_secret: bool,
    max_entropy: Option<usize>,
    supported_algorithms: Vec<String>,
) -> Fido2DeviceInfo {
    let mut caps = Fido2Capabilities::default();
    caps.resident_keys = resident_keys;
    caps.hmac_secret = hmac_secret;
    caps.max_entropy_size = max_entropy;
    caps.supported_algorithms = supported_algorithms;
    Fido2DeviceInfo {
        device_path: std::path::PathBuf::from("/dev/hidraw99"),
        vendor_id: 0x1209,
        product_id: 0xbeee,
        manufacturer: "TestCo".to_string(),
        product: "TestKey".to_string(),
        serial: None,
        aaguid: None,
        firmware_version: None,
        protocol_versions: vec!["FIDO_2_0".to_string()],
        extensions: vec![],
        transport: Fido2Transport::Usb,
        capabilities: caps,
    }
}

#[test]
fn test_credential_id_conversion() {
    let original = b"test_credential_id_12345";
    let string_id = Fido2MultiCredentialProvider::credential_id_to_string(original);
    let decoded = Fido2MultiCredentialProvider::string_to_credential_id(&string_id).unwrap();
    assert_eq!(original.to_vec(), decoded);
}

#[test]
fn test_string_to_credential_id_invalid() {
    let err =
        Fido2MultiCredentialProvider::string_to_credential_id("not!!!valid-base64!!!").unwrap_err();
    assert!(err.to_string().contains("credential") || err.to_string().contains("Invalid"));
}

#[test]
fn test_default_config() {
    let config = Fido2ProviderConfig::default();
    assert_eq!(config.rp_id, "beardog.ecoPrimals");
    assert!(!config.require_user_verification);
}

#[tokio::test]
async fn test_new_requires_resident_keys() {
    let mut d = make_device(false, false, Some(64), vec!["ES256".to_string()]);
    d.capabilities.resident_keys = false;
    let err = Fido2MultiCredentialProvider::new(d, None)
        .await
        .err()
        .expect("expected resident-keys error");
    assert!(err.to_string().contains("resident"));
}

#[tokio::test]
async fn test_create_credential_rejects_unknown_algorithm() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let req = CredentialRequest {
        role: "admin".to_string(),
        display_name: None,
        permissions: vec![],
        require_user_presence: false,
        require_user_verification: false,
        parent_credential: None,
        metadata: std::collections::HashMap::new(),
        algorithm: Some("RS256".to_string()),
    };
    let err = MultiCredentialHsmProvider::create_credential(&p, req)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("RS256") || err.to_string().contains("not supported"));
}

#[tokio::test]
async fn test_create_credential_parent_missing() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let req = CredentialRequest {
        role: "child".to_string(),
        display_name: None,
        permissions: vec![],
        require_user_presence: false,
        require_user_verification: false,
        parent_credential: Some("missing-parent-id".to_string()),
        metadata: std::collections::HashMap::new(),
        algorithm: Some("ES256".to_string()),
    };
    let err = MultiCredentialHsmProvider::create_credential(&p, req)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("Parent"));
}

#[tokio::test]
async fn test_create_hits_ctap2_stub() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let req = CredentialRequest {
        role: "admin".to_string(),
        display_name: None,
        permissions: vec![],
        require_user_presence: false,
        require_user_verification: false,
        parent_credential: None,
        metadata: std::collections::HashMap::new(),
        algorithm: Some("ES256".to_string()),
    };
    let err = MultiCredentialHsmProvider::create_credential(&p, req)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("Phase 2") || err.to_string().contains("CTAP2"));
}

#[tokio::test]
async fn test_list_credentials_empty_cache() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let list = MultiCredentialHsmProvider::list_credentials(&p)
        .await
        .unwrap();
    assert!(list.is_empty());
}

#[tokio::test]
async fn test_get_credential_info_not_found() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let err = MultiCredentialHsmProvider::get_credential_info(&p, "nope")
        .await
        .unwrap_err();
    assert!(err.to_string().contains("nope"));
}

#[tokio::test]
async fn test_delete_invalid_credential_id_encoding() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let err = MultiCredentialHsmProvider::delete_credential(&p, "@@@")
        .await
        .unwrap_err();
    assert!(!err.to_string().is_empty());
}

#[tokio::test]
async fn test_delete_ctap2_stub() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let id = Fido2MultiCredentialProvider::credential_id_to_string(b"id");
    let err = MultiCredentialHsmProvider::delete_credential(&p, &id)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("Phase 2") || err.to_string().contains("CTAP2"));
}

#[tokio::test]
async fn test_sign_invalid_base64() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let err = MultiCredentialHsmProvider::sign_with_credential(&p, "bad@@@", b"data", false)
        .await
        .unwrap_err();
    assert!(!err.to_string().is_empty());
}

#[tokio::test]
async fn test_sign_ctap2_stub() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let id = Fido2MultiCredentialProvider::credential_id_to_string(b"cid");
    let err = MultiCredentialHsmProvider::sign_with_credential(&p, &id, b"data", true)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("Phase 2") || err.to_string().contains("CTAP2"));
}

#[tokio::test]
async fn test_entropy_exceeds_max() {
    let d = make_device(true, true, Some(8), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let err = MultiCredentialHsmProvider::generate_hardware_entropy(&p, 999)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("exceeds"));
}

#[tokio::test]
async fn test_entropy_hmac_secret_unsupported() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let err = MultiCredentialHsmProvider::generate_hardware_entropy(&p, 8)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("hmac-secret"));
}

#[tokio::test]
async fn test_entropy_hmac_phase2_stub() {
    let d = make_device(true, true, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let err = MultiCredentialHsmProvider::generate_hardware_entropy(&p, 8)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("Phase 2") || err.to_string().contains("CTAP2"));
}

#[tokio::test]
async fn test_derive_child_parent_missing() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let child = CredentialRequest {
        role: "child".to_string(),
        display_name: None,
        permissions: vec![],
        require_user_presence: false,
        require_user_verification: false,
        parent_credential: None,
        metadata: std::collections::HashMap::new(),
        algorithm: Some("ES256".to_string()),
    };
    let err = MultiCredentialHsmProvider::derive_child_credential(&p, "parent-missing", child)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("Parent"));
}

#[tokio::test]
async fn test_credential_hierarchy_empty() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let h = MultiCredentialHsmProvider::get_credential_hierarchy(&p)
        .await
        .unwrap();
    assert!(h.roots.is_empty());
}

#[tokio::test]
async fn test_multi_credential_capabilities_and_provider_trait() {
    let d = make_device(
        true,
        true,
        Some(64),
        vec!["ES256".to_string(), "EdDSA".to_string()],
    );
    let p = Fido2MultiCredentialProvider::new(d.clone(), None)
        .await
        .unwrap();
    let caps = MultiCredentialHsmProvider::get_multi_credential_capabilities(&p);
    assert_eq!(
        caps.protocol,
        beardog_traits::unified::hsm_multi_credential::HsmProtocol::Fido2
    );
    assert!(caps.supports_hierarchical_credentials);

    assert_eq!(p.provider_id(), "fido2_multi_credential");
    assert!(!p.provider_version().is_empty());

    let h = BearDogProvider::health_check(&p).await.unwrap();
    assert_eq!(h.status, HealthStatus::Healthy);

    let m = BearDogProvider::metrics(&p).await.unwrap();
    assert!(!m.custom_metrics.is_empty());
}

#[tokio::test]
async fn test_prepare_replication_missing_credential() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let err = MultiCredentialHsmProvider::prepare_credential_replication(&p, "missing", b"seed")
        .await
        .unwrap_err();
    assert!(err.to_string().contains("missing"));
}

#[test]
fn test_fido2_provider_config_debug_clone() {
    let c = Fido2ProviderConfig::default();
    let _ = format!("{c:?}");
    let c2 = c.clone();
    assert_eq!(c2.rp_id, c.rp_id);
}

#[tokio::test]
async fn test_create_credential_default_algorithm_es256() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let req = CredentialRequest {
        role: "r".to_string(),
        display_name: None,
        permissions: vec![],
        require_user_presence: false,
        require_user_verification: false,
        parent_credential: None,
        metadata: std::collections::HashMap::new(),
        algorithm: None,
    };
    let err = MultiCredentialHsmProvider::create_credential(&p, req)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("Phase 2") || err.to_string().contains("CTAP2"));
}

#[tokio::test]
async fn test_create_warns_when_child_permission_not_in_parent() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let parent_id = Fido2MultiCredentialProvider::credential_id_to_string(b"parent");
    {
        let mut creds = p.credentials.write().await;
        creds.insert(
            parent_id.clone(),
            CredentialInfo {
                credential_id: parent_id.clone(),
                role: "parent".to_string(),
                display_name: None,
                permissions: vec!["read".to_string()],
                public_key: vec![],
                algorithm: "ES256".to_string(),
                created_at: Utc::now(),
                last_used: None,
                use_count: 0,
                parent_credential_id: None,
                metadata: std::collections::HashMap::new(),
                requires_user_presence: false,
                requires_user_verification: false,
            },
        );
    }
    let req = CredentialRequest {
        role: "child".to_string(),
        display_name: None,
        permissions: vec!["read".to_string(), "write".to_string()],
        require_user_presence: false,
        require_user_verification: false,
        parent_credential: Some(parent_id),
        metadata: std::collections::HashMap::new(),
        algorithm: Some("ES256".to_string()),
    };
    let err = MultiCredentialHsmProvider::create_credential(&p, req)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("Phase 2") || err.to_string().contains("CTAP2"));
}

#[tokio::test]
async fn test_get_credential_hierarchy_with_nested() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let root_id = Fido2MultiCredentialProvider::credential_id_to_string(b"root");
    let child_id = Fido2MultiCredentialProvider::credential_id_to_string(b"child");
    {
        let mut creds = p.credentials.write().await;
        creds.insert(
            root_id.clone(),
            CredentialInfo {
                credential_id: root_id.clone(),
                role: "root".to_string(),
                display_name: None,
                permissions: vec![],
                public_key: vec![],
                algorithm: "ES256".to_string(),
                created_at: Utc::now(),
                last_used: None,
                use_count: 0,
                parent_credential_id: None,
                metadata: std::collections::HashMap::new(),
                requires_user_presence: false,
                requires_user_verification: false,
            },
        );
        creds.insert(
            child_id.clone(),
            CredentialInfo {
                credential_id: child_id.clone(),
                role: "leaf".to_string(),
                display_name: None,
                permissions: vec![],
                public_key: vec![],
                algorithm: "ES256".to_string(),
                created_at: Utc::now(),
                last_used: None,
                use_count: 0,
                parent_credential_id: Some(root_id.clone()),
                metadata: std::collections::HashMap::new(),
                requires_user_presence: false,
                requires_user_verification: false,
            },
        );
    }
    let h = MultiCredentialHsmProvider::get_credential_hierarchy(&p)
        .await
        .unwrap();
    assert_eq!(h.roots.len(), 1);
    assert_eq!(h.roots[0].children.len(), 1);
    assert_eq!(h.roots[0].children[0].credential.credential_id, child_id);
}

#[tokio::test]
async fn test_prepare_replication_success() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let cid = Fido2MultiCredentialProvider::credential_id_to_string(b"rep");
    {
        let mut creds = p.credentials.write().await;
        creds.insert(
            cid.clone(),
            CredentialInfo {
                credential_id: cid.clone(),
                role: "role-a".to_string(),
                display_name: Some("dn".to_string()),
                permissions: vec!["p".to_string()],
                public_key: vec![1, 2, 3],
                algorithm: "ES256".to_string(),
                created_at: Utc::now(),
                last_used: None,
                use_count: 0,
                parent_credential_id: None,
                metadata: std::collections::HashMap::from([("k".to_string(), "v".to_string())]),
                requires_user_presence: true,
                requires_user_verification: false,
            },
        );
    }
    let data = MultiCredentialHsmProvider::prepare_credential_replication(&p, &cid, b"shared")
        .await
        .expect("replication prep");
    assert_eq!(data.source_credential.credential_id, cid);
    assert_eq!(data.entropy_hash.len(), 32);
    assert_eq!(data.target_request.role, "role-a");
    assert!(data.target_request.parent_credential.is_none());
}

#[tokio::test]
async fn test_list_and_get_credential_after_insert() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let cid = Fido2MultiCredentialProvider::credential_id_to_string(b"x");
    {
        let mut creds = p.credentials.write().await;
        creds.insert(
            cid.clone(),
            CredentialInfo {
                credential_id: cid.clone(),
                role: "r".to_string(),
                display_name: None,
                permissions: vec![],
                public_key: vec![],
                algorithm: "ES256".to_string(),
                created_at: Utc::now(),
                last_used: None,
                use_count: 0,
                parent_credential_id: None,
                metadata: std::collections::HashMap::new(),
                requires_user_presence: false,
                requires_user_verification: false,
            },
        );
    }
    let listed = MultiCredentialHsmProvider::list_credentials(&p)
        .await
        .unwrap();
    assert_eq!(listed.len(), 1);
    let got = MultiCredentialHsmProvider::get_credential_info(&p, &cid)
        .await
        .unwrap();
    assert_eq!(got.role, "r");
}

#[tokio::test]
async fn test_capabilities_trait_hardware_entropy_toggle() {
    let mut d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    d.capabilities.user_verification = true;
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let caps = BearDogProvider::capabilities(&p);
    assert_eq!(caps.len(), 3);
    assert!(
        !caps
            .iter()
            .any(|c| c.name == "hardware_entropy" && c.enabled)
    );

    let mut d2 = make_device(true, true, Some(64), vec!["ES256".to_string()]);
    d2.capabilities.user_verification = true;
    let p2 = Fido2MultiCredentialProvider::new(d2, None).await.unwrap();
    let caps2 = BearDogProvider::capabilities(&p2);
    assert!(
        caps2
            .iter()
            .any(|c| c.name == "hardware_entropy" && c.enabled)
    );
    let mc = MultiCredentialHsmProvider::get_multi_credential_capabilities(&p2);
    assert!(mc.supports_user_verification);
}

#[tokio::test]
async fn test_metrics_reflects_credential_count_and_max_keys() {
    let d = make_device(true, false, Some(64), vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    {
        let mut creds = p.credentials.write().await;
        creds.insert(
            "a".to_string(),
            CredentialInfo {
                credential_id: "a".to_string(),
                role: "r".to_string(),
                display_name: None,
                permissions: vec![],
                public_key: vec![],
                algorithm: "ES256".to_string(),
                created_at: Utc::now(),
                last_used: None,
                use_count: 0,
                parent_credential_id: None,
                metadata: std::collections::HashMap::new(),
                requires_user_presence: false,
                requires_user_verification: false,
            },
        );
    }
    let m = BearDogProvider::metrics(&p).await.unwrap();
    let count = m
        .custom_metrics
        .iter()
        .find(|c| c.name == "credentials_count")
        .map(|c| c.value)
        .unwrap();
    assert!((count - 1.0).abs() < f64::EPSILON);

    let mut d2 = make_device(true, false, None, vec!["ES256".to_string()]);
    d2.capabilities.max_resident_keys = None;
    let p2 = Fido2MultiCredentialProvider::new(d2, None).await.unwrap();
    let m2 = BearDogProvider::metrics(&p2).await.unwrap();
    let max_m = m2
        .custom_metrics
        .iter()
        .find(|c| c.name == "max_credentials")
        .unwrap();
    assert!(max_m.value.abs() < f64::EPSILON);
}

#[tokio::test]
async fn test_entropy_max_uses_default_64_when_unlimited() {
    let d = make_device(true, true, None, vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let err = MultiCredentialHsmProvider::generate_hardware_entropy(&p, 999)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("exceeds"));
}

#[tokio::test]
async fn test_multi_cap_max_entropy_none_reports_none() {
    let d = make_device(true, true, None, vec!["ES256".to_string()]);
    let p = Fido2MultiCredentialProvider::new(d, None).await.unwrap();
    let caps = MultiCredentialHsmProvider::get_multi_credential_capabilities(&p);
    assert!(caps.max_entropy_bytes.is_none());
}
