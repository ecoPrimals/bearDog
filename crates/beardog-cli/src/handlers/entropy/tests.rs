// SPDX-License-Identifier: AGPL-3.0-or-later

use super::helpers::{entropy_quality_assessment_label, generate_system_entropy};
use super::hsm_selection::{format_hsm_interface_type_label, select_hsm_by_preference};
use super::info::handle_entropy_info;
use super::types::{EntropySeedMetadata, HsmInfo};
use super::{
    base64_decode, base64_encode, calculate_entropy_quality, load_entropy_file, save_entropy_file,
};
use beardog_types::constants::network::HTTPS_PORT;
use tempfile::TempDir;

#[test]
fn test_entropy_seed_metadata_serde_roundtrip() {
    let meta = EntropySeedMetadata {
        seed_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
        quality_tier: 3,
        quality_score: 0.92,
        device_used: "Test HSM".to_string(),
        device_tier: "Software".to_string(),
        timestamp: "2025-01-01T00:00:00Z".to_string(),
        human_input: true,
        identity: Some("alice".to_string()),
        entropy_bytes_b64: base64_encode(b"entropy-bytes"),
    };
    let json =
        serde_json::to_string(&meta).expect("serialize EntropySeedMetadata for roundtrip test");
    let back: EntropySeedMetadata =
        serde_json::from_str(&json).expect("deserialize EntropySeedMetadata in roundtrip test");
    assert_eq!(back.seed_id, meta.seed_id);
    assert_eq!(back.quality_tier, meta.quality_tier);
    assert_eq!(back.identity, meta.identity);
}

#[test]
fn test_generate_system_entropy_output_lengths() {
    assert_eq!(
        generate_system_entropy(16)
            .expect("generate 16 bytes system entropy")
            .len(),
        16
    );
    assert_eq!(
        generate_system_entropy(32)
            .expect("generate 32 bytes system entropy")
            .len(),
        32
    );
    assert_eq!(
        generate_system_entropy(48)
            .expect("generate 48 bytes system entropy")
            .len(),
        48
    );
    assert_eq!(
        generate_system_entropy(64)
            .expect("generate 64 bytes system entropy")
            .len(),
        64
    );
}

#[test]
fn test_calculate_entropy_quality_single_byte() {
    let q = calculate_entropy_quality(&[42; 32]);
    assert!(q < 0.2);
}

#[tokio::test]
async fn test_handle_entropy_info_reads_seed_file() {
    let dir = TempDir::new().expect("create temp directory for entropy info read test");
    let seed_path = dir.path().join("seed.json");
    let meta = EntropySeedMetadata {
        seed_id: "id-1".to_string(),
        quality_tier: 2,
        quality_score: 0.88,
        device_used: "dev".to_string(),
        device_tier: "Hardware".to_string(),
        timestamp: "2025-06-01T12:00:00Z".to_string(),
        human_input: false,
        identity: None,
        entropy_bytes_b64: base64_encode(&[0u8; 40]),
    };
    std::fs::write(
        &seed_path,
        serde_json::to_string_pretty(&meta).expect("pretty-print seed metadata for test"),
    )
    .expect("write seed.json fixture");
    let result =
        handle_entropy_info(seed_path.to_str().expect("seed path must be valid UTF-8")).await;
    assert!(result.is_ok());
}

#[test]
fn test_base64_decode_error() {
    assert!(base64_decode("not-valid-base64!!!").is_err());
}

#[test]
fn test_calculate_entropy_quality_empty() {
    assert_eq!(calculate_entropy_quality(&[]), 0.0);
}

#[test]
#[expect(
    clippy::cast_possible_truncation,
    reason = "u16 0..256 maps to u8 for full byte alphabet; range is exact"
)]
fn test_calculate_entropy_quality_near_uniform() {
    let v: Vec<u8> = (0u16..256).map(|i| i as u8).collect();
    let q = calculate_entropy_quality(&v);
    assert!(q > 0.95);
}

#[test]
fn test_save_and_load_entropy_file_roundtrip() {
    let dir = TempDir::new().expect("create temp directory for entropy file roundtrip");
    let p = dir.path().join("raw.bin");
    let data = [7u8, 8, 9];
    save_entropy_file(&data, p.to_str().expect("raw.bin path must be valid UTF-8"))
        .expect("save_entropy_file in roundtrip test");
    assert_eq!(
        load_entropy_file(p.to_str().expect("raw.bin path must be valid UTF-8"))
            .expect("load_entropy_file in roundtrip test"),
        data
    );
}

#[tokio::test]
async fn test_handle_entropy_info_invalid_json() {
    let dir = TempDir::new().expect("create temp directory for invalid JSON entropy test");
    let seed_path = dir.path().join("bad.json");
    std::fs::write(&seed_path, "{not json").expect("write invalid JSON fixture");
    assert!(
        handle_entropy_info(
            seed_path
                .to_str()
                .expect("bad.json path must be valid UTF-8"),
        )
        .await
        .is_err()
    );
}

#[tokio::test]
async fn test_handle_entropy_info_invalid_entropy_b64() {
    let dir = TempDir::new().expect("create temp directory for invalid b64 entropy test");
    let seed_path = dir.path().join("seed.json");
    let meta = EntropySeedMetadata {
        seed_id: "id-1".to_string(),
        quality_tier: 1,
        quality_score: 0.5,
        device_used: "dev".to_string(),
        device_tier: "Software".to_string(),
        timestamp: "2025-06-01T12:00:00Z".to_string(),
        human_input: false,
        identity: Some("x".to_string()),
        entropy_bytes_b64: "!!!".to_string(),
    };
    std::fs::write(
        &seed_path,
        serde_json::to_string_pretty(&meta).expect("serialize meta with bad b64"),
    )
    .expect("write seed with invalid entropy b64");
    assert!(
        handle_entropy_info(seed_path.to_str().expect("seed path must be valid UTF-8"),)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_handle_entropy_info_missing_file() {
    assert!(
        handle_entropy_info("/nonexistent/path/seed.json")
            .await
            .is_err()
    );
}

#[test]
fn test_generate_system_entropy_branch_over_32_bytes() {
    let v = generate_system_entropy(100).expect("generate 100 bytes system entropy");
    assert_eq!(v.len(), 100);
}

#[test]
fn test_select_hsm_by_preference_auto_order() {
    let hsms = vec![
        HsmInfo {
            name: "sw".to_string(),
            tier: "Software".to_string(),
            hsm_type: "Software".to_string(),
        },
        HsmInfo {
            name: "mob".to_string(),
            tier: "Mobile".to_string(),
            hsm_type: "Mobile".to_string(),
        },
    ];
    let picked = select_hsm_by_preference(&hsms, "auto").expect("select auto with mobile+sw");
    assert_eq!(picked.tier, "Mobile");
}

#[test]
fn test_select_hsm_by_preference_software() {
    let hsms = vec![HsmInfo {
        name: "pkcs11".to_string(),
        tier: "Software".to_string(),
        hsm_type: "Software".to_string(),
    }];
    let picked = select_hsm_by_preference(&hsms, "software").expect("select software HSM");
    assert_eq!(picked.name, "pkcs11");
}

#[test]
fn test_select_hsm_by_preference_unknown() {
    let hsms = vec![HsmInfo {
        name: "x".to_string(),
        tier: "Software".to_string(),
        hsm_type: "Software".to_string(),
    }];
    assert!(select_hsm_by_preference(&hsms, "nope").is_err());
}

#[test]
fn test_select_hsm_by_preference_hardware_usb_alias() {
    let hsms = vec![HsmInfo {
        name: "token".to_string(),
        tier: "Hardware".to_string(),
        hsm_type: "USB".to_string(),
    }];
    let a = select_hsm_by_preference(&hsms, "usb").expect("select usb alias");
    let b = select_hsm_by_preference(&hsms, "hardware").expect("select hardware alias");
    assert_eq!(a.name, b.name);
}

#[test]
fn test_select_hsm_by_preference_auto_prefers_mobile_then_hardware() {
    let hsms = vec![
        HsmInfo {
            name: "sw".to_string(),
            tier: "Software".to_string(),
            hsm_type: "Software".to_string(),
        },
        HsmInfo {
            name: "hw".to_string(),
            tier: "Hardware".to_string(),
            hsm_type: "Hardware".to_string(),
        },
    ];
    assert_eq!(
        select_hsm_by_preference(&hsms, "auto")
            .expect("auto select hardware when no mobile")
            .name,
        "hw"
    );

    let with_mobile = vec![
        HsmInfo {
            name: "m".to_string(),
            tier: "Mobile".to_string(),
            hsm_type: "Mobile".to_string(),
        },
        hsms[1].clone(),
    ];
    assert_eq!(
        select_hsm_by_preference(&with_mobile, "auto")
            .expect("auto prefer mobile")
            .tier,
        "Mobile"
    );
}

#[test]
fn test_select_hsm_by_preference_auto_software_only() {
    let hsms = vec![HsmInfo {
        name: "only-soft".to_string(),
        tier: "Software".to_string(),
        hsm_type: "Software".to_string(),
    }];
    assert_eq!(
        select_hsm_by_preference(&hsms, "auto")
            .expect("auto fallback to software")
            .name,
        "only-soft"
    );
}

#[test]
fn test_select_hsm_by_preference_mobile_not_found() {
    let hsms = vec![HsmInfo {
        name: "sw".to_string(),
        tier: "Software".to_string(),
        hsm_type: "Software".to_string(),
    }];
    assert!(select_hsm_by_preference(&hsms, "mobile").is_err());
}

#[test]
fn test_select_hsm_by_preference_software_not_found() {
    let hsms = vec![HsmInfo {
        name: "hw".to_string(),
        tier: "Hardware".to_string(),
        hsm_type: "Hardware".to_string(),
    }];
    assert!(select_hsm_by_preference(&hsms, "software").is_err());
}

#[test]
fn test_select_hsm_by_preference_hardware_not_found() {
    let hsms = vec![HsmInfo {
        name: "sw".to_string(),
        tier: "Software".to_string(),
        hsm_type: "Software".to_string(),
    }];
    assert!(select_hsm_by_preference(&hsms, "hardware").is_err());
}

#[test]
#[expect(
    clippy::cast_possible_truncation,
    reason = "enumerate index fits u8 for 256-slot histogram (i % 17)"
)]
fn test_calculate_entropy_quality_moderate_distribution() {
    let mut v = vec![0u8; 256];
    for (i, slot) in v.iter_mut().enumerate() {
        *slot = (i % 17) as u8;
    }
    let q = calculate_entropy_quality(&v);
    assert!(q > 0.2 && q < 0.99, "unexpected quality {q}");
}

#[test]
fn test_entropy_quality_assessment_label_branches() {
    assert!(entropy_quality_assessment_label(0.96).contains("Excellent"));
    assert!(entropy_quality_assessment_label(0.90).contains("Good"));
    assert!(entropy_quality_assessment_label(0.75).contains("Acceptable"));
    assert!(entropy_quality_assessment_label(0.50).contains("Poor"));
}

#[tokio::test]
async fn test_handle_entropy_info_with_identity_and_short_entropy() {
    let dir = TempDir::new().expect("create temp directory for tiny entropy seed test");
    let seed_path = dir.path().join("tiny.json");
    let meta = EntropySeedMetadata {
        seed_id: "id-2".to_string(),
        quality_tier: 4,
        quality_score: 0.91,
        device_used: "dev".to_string(),
        device_tier: "Software".to_string(),
        timestamp: "2025-06-01T12:00:00Z".to_string(),
        human_input: true,
        identity: Some("bob".to_string()),
        entropy_bytes_b64: base64_encode(&[1u8, 2, 3]),
    };
    std::fs::write(
        &seed_path,
        serde_json::to_string_pretty(&meta).expect("serialize tiny seed metadata"),
    )
    .expect("write tiny.json");
    handle_entropy_info(
        seed_path
            .to_str()
            .expect("tiny.json path must be valid UTF-8"),
    )
    .await
    .expect("handle_entropy_info for short entropy");
}

#[test]
fn format_hsm_interface_type_label_covers_all_variants() {
    use beardog_tunnel::tunnel::hsm::universal_discovery::HsmInterfaceType as T;
    assert!(
        format_hsm_interface_type_label(&T::Tpm {
            version: "2.0".to_string(),
        })
        .contains("TPM")
    );
    assert!(
        format_hsm_interface_type_label(&T::SoftwareHsm {
            implementation: "SoftHSM2".to_string(),
        })
        .contains("SoftHSM2")
    );
    assert!(
        format_hsm_interface_type_label(&T::MobileHsm {
            platform: "Android".to_string(),
            chip: None,
        })
        .contains("Android")
    );
    assert!(
        format_hsm_interface_type_label(&T::CloudKms {
            provider: "aws".to_string(),
            region: None,
        })
        .contains("aws")
    );
    assert!(
        format_hsm_interface_type_label(&T::NetworkHsm {
            endpoint: "10.0.0.1".to_string(),
            port: HTTPS_PORT,
        })
        .contains("10.0.0.1")
    );
    assert!(
        format_hsm_interface_type_label(&T::UsbHsm {
            device_id: "deadbeef".to_string(),
        })
        .contains("deadbeef")
    );
    assert!(
        format_hsm_interface_type_label(&T::SmartCard {
            reader: "reader-1".to_string(),
        })
        .contains("reader-1")
    );
    assert!(
        format_hsm_interface_type_label(&T::CustomApi {
            api_type: "rest".to_string(),
            endpoint: "https://hsm".to_string(),
        })
        .contains("rest")
    );
}

#[test]
fn select_hsm_by_preference_empty_list_errors() {
    let empty: Vec<HsmInfo> = vec![];
    assert!(select_hsm_by_preference(&empty, "auto").is_err());
    assert!(select_hsm_by_preference(&empty, "software").is_err());
    assert!(select_hsm_by_preference(&empty, "mobile").is_err());
    assert!(select_hsm_by_preference(&empty, "usb").is_err());
}

#[test]
fn select_hsm_by_preference_case_insensitive() {
    let hsms = vec![HsmInfo {
        name: "s".to_string(),
        tier: "Software".to_string(),
        hsm_type: "t".to_string(),
    }];
    assert!(select_hsm_by_preference(&hsms, "SOFTWARE").is_ok());
    assert!(select_hsm_by_preference(&hsms, "Auto").is_ok());
}

#[test]
fn base64_roundtrip_ascii() {
    let raw = b"entropy-test-bytes";
    assert_eq!(
        base64_decode(&base64_encode(raw)).expect("decode"),
        raw.as_slice()
    );
}
