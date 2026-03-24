// SPDX-License-Identifier: AGPL-3.0-only

//! Targeted coverage for receipt validation, adapter certificate helpers, and constraint builtins.

use chrono::{Datelike, Utc};
use std::fs;
use std::path::Path;

use crate::adapter_certificates::{
    AdapterClassification, AdapterUnlockCertificate, CertificateUsageRecord,
    CertificateVerificationResult, ResourceUsage,
};
use crate::constraints::Constraint;
use crate::constraints::ConstraintContext;
use crate::constraints::builtin::{
    CpuQuotaConstraint, ExpiryConstraint, MemoryQuotaConstraint, TimeRangeConstraint,
    WeekdayConstraint,
};
use crate::genetics_constraints::{
    ComputeQuota, ComputeUsage, KeyConstraints, KeyOperation, ScopeConstraint,
};
use crate::receipt::{KeyInfo, OperationReceipt, OperationResult, generate_receipt_filename};
use serde_json::json;

// --- receipt.rs: validation & I/O edge paths ---

#[test]
fn receipt_validate_rejects_empty_operation_field() {
    let mut r = OperationReceipt::new("ok-op");
    r.operation = String::new();
    let err = r.validate().expect_err("empty operation");
    assert!(err.to_string().contains("operation") || err.to_string().contains("Missing"));
}

#[test]
fn receipt_validate_rejects_invalid_timestamp() {
    let mut r = OperationReceipt::new("op");
    r.timestamp = "not-rfc3339".to_string();
    let err = r.validate().expect_err("bad timestamp");
    assert!(err.to_string().contains("timestamp") || err.to_string().contains("ISO"));
}

#[test]
fn receipt_validate_rejects_bad_uuid() {
    let mut r = OperationReceipt::new("op");
    r.receipt_id = "not-a-uuid".to_string();
    let err = r.validate().expect_err("invalid uuid");
    assert!(err.to_string().contains("receipt") || err.to_string().contains("UUID"));
}

#[test]
fn receipt_save_and_load_roundtrip() {
    let dir = std::env::temp_dir().join(format!("beardog_receipt_m26_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("roundtrip.json");

    let original = OperationReceipt::new("sign-data")
        .with_metadata("unit", json!("coverage"))
        .with_key_info(KeyInfo {
            key_id: "k1".to_string(),
            algorithm: "Ed25519".to_string(),
            generation: 1,
            parent_key_id: None,
            expires_at: None,
            usage: None,
            purpose: None,
        });

    original.save_to_file(&path).expect("save");
    let loaded = OperationReceipt::load_from_file(&path).expect("load");
    assert_eq!(loaded.operation, original.operation);
    assert_eq!(loaded.receipt_id, original.receipt_id);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn receipt_load_from_file_missing_returns_error() {
    let p = Path::new("/nonexistent/beardog/receipt_xyz.json");
    assert!(OperationReceipt::load_from_file(p).is_err());
}

#[test]
fn receipt_load_from_file_invalid_json_returns_error() {
    let dir = std::env::temp_dir().join(format!("beardog_bad_json_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("bad.json");
    fs::write(&path, b"{ not json").expect("write");
    assert!(OperationReceipt::load_from_file(&path).is_err());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn generate_receipt_filename_has_operation_and_json_suffix() {
    let name = generate_receipt_filename("encrypt");
    assert!(name.starts_with("receipt-encrypt-"));
    assert!(
        std::path::Path::new(&name)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
    );
}

#[test]
fn operation_result_failure_roundtrips_serde() {
    let r = OperationResult::Failure {
        error: "e".to_string(),
    };
    let j = serde_json::to_string(&r).expect("ser");
    let back: OperationResult = serde_json::from_str(&j).expect("de");
    match back {
        OperationResult::Failure { error } => assert_eq!(error, "e"),
        OperationResult::Success => panic!("expected Failure"),
    }
}

// --- genetics_constraints: description & quota branches ---

#[test]
fn key_constraints_description_includes_operations_scope() {
    let c = KeyConstraints {
        scope: ScopeConstraint::Operations {
            allowed_operations: vec!["read".to_string(), "custom_op".to_string()],
        },
        ..Default::default()
    };
    let d = c.description();
    assert!(d.contains("read") || d.contains("Allowed"));
}

#[test]
fn key_constraints_description_notes_biometric_when_set() {
    let mut c = KeyConstraints::default();
    c.behavior.biometric_required = true;
    let d = c.description();
    assert!(d.to_lowercase().contains("biometric"));
}

#[test]
fn verify_compute_quota_rejects_memory_over_max() {
    let c = KeyConstraints {
        compute_quota: Some(ComputeQuota {
            max_hours: 100.0,
            max_memory_bytes: 100,
            max_cpu_percent: 50,
            current_usage: ComputeUsage::default(),
        }),
        ..Default::default()
    };
    let op = KeyOperation::ComputeAllocation {
        hours: 0.0,
        memory_bytes: 200,
    };
    assert!(c.verify_operation(&op).is_err());
}

// --- adapter_certificates.rs: ancillary types ---

#[test]
fn adapter_classification_descriptions_distinct() {
    assert_ne!(
        AdapterClassification::Human.description(),
        AdapterClassification::Commercial.description()
    );
}

#[test]
fn resource_usage_default_billing_is_zero() {
    let u = ResourceUsage::default();
    assert!((u.calculate_billing_amount()).abs() < f64::EPSILON);
}

#[test]
fn certificate_usage_record_serde_roundtrip() {
    let rec = CertificateUsageRecord {
        cert_id: "cid".to_string(),
        adapter_id: "a::b".to_string(),
        classification: AdapterClassification::Human,
        timestamp: Utc::now(),
        operation_type: "ping".to_string(),
        resource_usage: ResourceUsage {
            cpu_seconds: 1.0,
            memory_bytes: 0,
            network_bytes: 0,
            storage_bytes: 0,
            custom_units: 0.0,
        },
    };
    let j = serde_json::to_string(&rec).expect("ser");
    let back: CertificateUsageRecord = serde_json::from_str(&j).expect("de");
    assert_eq!(back.cert_id, rec.cert_id);
}

#[test]
fn certificate_verification_result_invalid_signature_message() {
    let m = CertificateVerificationResult::InvalidSignature
        .error_message()
        .expect("msg");
    assert!(m.to_lowercase().contains("signature"));
}

#[test]
fn certificate_verification_result_constraints_tampered_message() {
    let m = CertificateVerificationResult::ConstraintsTampered
        .error_message()
        .expect("msg");
    assert!(m.to_lowercase().contains("constraint"));
}

#[test]
fn adapter_unlock_certificate_metadata_string_includes_cert_prefix_and_adapter() {
    let cert = AdapterUnlockCertificate {
        cert_id: "long-enough-id-xxxxxxxx".to_string(),
        issuer_key_id: "k".to_string(),
        adapter_id: "adapter::x".to_string(),
        classification: AdapterClassification::Human,
        constraints: None,
        issued_at: Utc::now() - chrono::Duration::minutes(1),
        expires_at: Utc::now() + chrono::Duration::hours(1),
        signature: vec![0u8; 64],
        issuer_public_key: vec![0u8; 32],
    };
    let s = cert.metadata_string();
    assert!(s.contains("long-eno"));
    assert!(s.contains("adapter::x"));
}

#[test]
fn signable_data_deterministic_for_same_certificate() {
    let c1 = AdapterUnlockCertificate {
        cert_id: "id1".to_string(),
        issuer_key_id: "k".to_string(),
        adapter_id: "a".to_string(),
        classification: AdapterClassification::Commercial,
        constraints: None,
        issued_at: Utc::now(),
        expires_at: Utc::now() + chrono::Duration::hours(1),
        signature: vec![],
        issuer_public_key: vec![],
    };
    let b1 = c1.signable_data();
    let b2 = c1.signable_data();
    assert_eq!(b1, b2);
}

// --- constraints/builtin.rs ---

#[test]
fn time_range_constraint_description_and_json() {
    let t = TimeRangeConstraint {
        start: "00:00".to_string(),
        end: "23:59".to_string(),
    };
    assert!(t.description().contains("00:00"));
    let j = t.serialize_json().expect("json");
    assert!(j.contains("00:00"));
}

#[test]
fn weekday_constraint_matches_current_weekday_name() {
    let ctx = ConstraintContext::new();
    let w = match ctx.current_time.weekday() {
        chrono::Weekday::Mon => "mon",
        chrono::Weekday::Tue => "tue",
        chrono::Weekday::Wed => "wed",
        chrono::Weekday::Thu => "thu",
        chrono::Weekday::Fri => "fri",
        chrono::Weekday::Sat => "sat",
        chrono::Weekday::Sun => "sun",
    };
    let c = WeekdayConstraint {
        allowed_days: vec![w.to_string()],
    };
    assert!(c.is_satisfied(&ctx).expect("ok"));
}

#[test]
fn cpu_quota_under_limit_satisfied() {
    let ctx = ConstraintContext {
        system_state: crate::constraints::SystemState {
            cpu_usage_percent: Some(10.0),
            ..Default::default()
        },
        ..ConstraintContext::new()
    };
    let c = CpuQuotaConstraint { max_percent: 50 };
    assert!(c.is_satisfied(&ctx).expect("eval"));
}

#[test]
fn memory_quota_over_limit_not_satisfied() {
    let ctx = ConstraintContext {
        system_state: crate::constraints::SystemState {
            memory_used_bytes: Some(200),
            ..Default::default()
        },
        ..ConstraintContext::new()
    };
    let c = MemoryQuotaConstraint { max_bytes: 100 };
    assert!(!c.is_satisfied(&ctx).expect("eval"));
}

#[test]
fn expiry_constraint_satisfied_before_deadline() {
    let c = ExpiryConstraint {
        expires_at: (Utc::now() + chrono::Duration::days(1)).to_rfc3339(),
    };
    assert!(c.is_satisfied(&ConstraintContext::new()).expect("eval"));
}

#[test]
fn cpu_quota_missing_usage_allows() {
    let ctx = ConstraintContext::new();
    let c = CpuQuotaConstraint { max_percent: 1 };
    assert!(c.is_satisfied(&ctx).expect("eval"));
}

#[test]
fn memory_quota_missing_usage_allows() {
    let ctx = ConstraintContext::new();
    let c = MemoryQuotaConstraint { max_bytes: 1 };
    assert!(c.is_satisfied(&ctx).expect("eval"));
}

#[test]
fn key_constraints_hash_stable_across_calls() {
    let k = KeyConstraints::default();
    let a = k.hash().expect("h1");
    let b = k.hash().expect("h2");
    assert_eq!(a, b);
}
