// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    DelegateParams, DelegationConstraints, derive_delegated_key, expand_weekday_range,
    format_bytes, handle_key_delegate_with_home, is_valid_weekday, parse_memory_quota,
    parse_time_range, parse_weekdays,
};
use crate::handlers::key_store;
use chrono::Utc;
use tempfile::TempDir;

#[test]
fn test_parse_time_range() {
    let (start, end) = parse_time_range("9:00-17:00").expect("parse_time_range valid input");
    assert_eq!(start, "9:00");
    assert_eq!(end, "17:00");

    assert!(parse_time_range("invalid").is_err());
    assert!(parse_time_range("9:00").is_err());
}

#[test]
fn test_parse_weekdays_range() {
    let days = parse_weekdays("mon-fri").expect("parse_weekdays range");
    assert_eq!(days, vec!["mon", "tue", "wed", "thu", "fri"]);
}

#[test]
fn test_parse_weekdays_list() {
    let days = parse_weekdays("mon,wed,fri").expect("parse_weekdays list");
    assert_eq!(days, vec!["mon", "wed", "fri"]);
}

#[test]
fn test_expand_weekday_range() {
    let days = expand_weekday_range("mon", "fri").expect("expand_weekday_range mon-fri");
    assert_eq!(days, vec!["mon", "tue", "wed", "thu", "fri"]);

    let weekend = expand_weekday_range("sat", "sun").expect("expand_weekday_range weekend");
    assert_eq!(weekend, vec!["sat", "sun"]);

    assert!(expand_weekday_range("fri", "mon").is_err());
}

#[test]
fn test_parse_memory_quota() {
    assert_eq!(
        parse_memory_quota("8GB").expect("parse 8GB"),
        8 * 1024 * 1024 * 1024
    );
    assert_eq!(
        parse_memory_quota("512MB").expect("parse 512MB"),
        512 * 1024 * 1024
    );
    assert_eq!(
        parse_memory_quota("1024KB").expect("parse 1024KB"),
        1024 * 1024
    );
    assert_eq!(parse_memory_quota("100").expect("parse raw bytes"), 100);

    assert!(parse_memory_quota("invalid").is_err());
    assert!(parse_memory_quota("10XB").is_err());
}

#[test]
fn test_format_bytes() {
    assert_eq!(format_bytes(100), "100 bytes");
    assert_eq!(format_bytes(1024), "1.00 KB");
    assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
    assert_eq!(format_bytes(8 * 1024 * 1024 * 1024), "8.00 GB");
}

#[test]
fn test_is_valid_weekday() {
    assert!(is_valid_weekday("mon"));
    assert!(is_valid_weekday("MON"));
    assert!(is_valid_weekday("fri"));
    assert!(!is_valid_weekday("invalid"));
    assert!(!is_valid_weekday("monday"));
}

#[test]
fn test_derive_delegated_key_deterministic() {
    let master = [3u8; 32];
    let a = derive_delegated_key(&master, b"delegate:alice").expect("derive alice");
    let b = derive_delegated_key(&master, b"delegate:alice").expect("derive alice again");
    assert_eq!(a, b);
    assert_eq!(a.len(), 32);
    assert_ne!(
        derive_delegated_key(&master, b"delegate:bob").expect("derive bob"),
        a
    );
}

#[test]
fn test_parse_time_range_missing_colon_in_part() {
    assert!(parse_time_range("900-17:00").is_err());
}

#[test]
fn test_delegation_constraints_to_constraints_and_composite() {
    let dc = DelegationConstraints {
        time_range: Some(("09:00".to_string(), "17:00".to_string())),
        weekdays: Some(vec!["mon".to_string()]),
        cpu_quota: Some(50),
        memory_quota: Some(1024),
        expires_at: "2099-01-01T00:00:00Z".to_string(),
        delegated_to: "user".to_string(),
    };
    let v = dc.to_constraints();
    assert!(v.len() >= 4);
    let _ = dc.as_composite();
}

#[tokio::test]
async fn test_handle_key_delegate_full_flow() {
    let dir = TempDir::new().expect("tempdir for key delegate flow test");
    let home = dir.path();

    let master = key_store::StoredKey {
        key_id: "master-delegate".to_string(),
        algorithm: "aes256-gcm".to_string(),
        hsm_name: "hsm-x".to_string(),
        created_at: Utc::now().to_rfc3339(),
        key_material_b64: key_store::base64_encode(&[11u8; 32]),
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
    };
    key_store::save_key_to_home(&master, home).expect("save master key in test");

    let params = DelegateParams {
        master_key_id: "master-delegate",
        delegate_to: "delegatee",
        output_key_id: "delegated-out",
        time_range: Some("9:00-17:00"),
        weekdays: Some("mon,wed"),
        cpu_quota: Some(25),
        memory_quota: Some("512MB"),
        expires_in: "24h",
    };
    handle_key_delegate_with_home(&params, home)
        .await
        .expect("delegate");

    let del = key_store::load_key_from_home("delegated-out", home).expect("delegated key");
    assert_eq!(del.generation, 1);
    assert_eq!(del.parent_key_id.as_deref(), Some("master-delegate"));
}

#[test]
fn test_parse_weekdays_invalid_day_in_list() {
    assert!(parse_weekdays("mon,bad").is_err());
}

#[test]
fn test_parse_memory_quota_bytes_unit() {
    assert_eq!(parse_memory_quota("4096B").expect("parse 4096B"), 4096);
    assert_eq!(
        parse_memory_quota("2TB").expect("parse 2TB"),
        2 * 1024_u64.pow(4)
    );
}

#[test]
fn test_format_bytes_terabyte() {
    let tb = 1024_u64.pow(4) * 3;
    let s = format_bytes(tb);
    assert!(s.contains("TB"));
}

#[test]
fn test_delegation_constraints_is_satisfied_expired() {
    let dc = DelegationConstraints {
        time_range: None,
        weekdays: None,
        cpu_quota: None,
        memory_quota: None,
        expires_at: "2000-01-01T00:00:00Z".to_string(),
        delegated_to: "u".to_string(),
    };
    assert!(
        !dc.is_satisfied()
            .expect("is_satisfied on expired constraints")
    );
}
