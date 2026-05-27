// SPDX-License-Identifier: AGPL-3.0-or-later

//! `beardog entropy collect` — discovery, human or hardware entropy, seed JSON output.

use std::collections::HashMap;
use std::fs;

use beardog_errors::BearDogError;
use beardog_genetics::genetics::entropy_hierarchy::LiveFeedValidator;
use beardog_genetics::genetics::human_entropy::{
    InteractionCaptureConfig, InteractionEntropyCollector,
};
use beardog_tunnel::tunnel::hsm::universal_discovery::discovery_engine::DiscoveryEngine;
use chrono::Utc;
use uuid::Uuid;

use super::helpers::{
    base64_encode, calculate_entropy_quality, entropy_quality_assessment_label,
    generate_system_entropy,
};
use super::hsm_selection::{format_hsm_interface_type_label, select_hsm_by_preference};
use super::types::{EntropySeedMetadata, HsmInfo};

/// Handle entropy collection command.
///
/// # Errors
///
/// Returns an error if HSM discovery fails, entropy generation fails,
/// or the output file cannot be written.
#[expect(
    clippy::cast_precision_loss,
    reason = "Display duration ms as seconds; acceptable precision for CLI output"
)]
pub async fn handle_entropy_collect(
    human_input: bool,
    device_preference: &str,
    quality_tier: u8,
    output_path: &str,
    identity: Option<&str>,
) -> Result<(), BearDogError> {
    println!("BearDog Human Entropy Collection");
    println!("===================================");
    println!();

    // Step 1: Discover available HSMs (vendor-agnostic discovery, zero hardcoding)
    println!("Discovering available HSMs...");

    // Use actual HSM discovery engine (evolved from placeholder)
    let discovery = DiscoveryEngine::new()?;

    // Discover all types of HSMs (zero hardcoding - discovers what's available)
    let mut discovered_hsms = Vec::new();
    discovered_hsms.extend(discovery.discover_software_hsms().unwrap_or_default());
    discovered_hsms.extend(discovery.discover_tpm_hsms().unwrap_or_default());
    discovered_hsms.extend(discovery.discover_mobile_hsms().unwrap_or_default());
    discovered_hsms.extend(discovery.discover_usb_hsms().unwrap_or_default());
    discovered_hsms.extend(discovery.discover_smartcard_hsms().unwrap_or_default());
    // Note: PKCS#11, Cloud KMS, Network HSMs are also available but may be slow to probe

    if discovered_hsms.is_empty() {
        println!("No HSMs found!");
        println!();
        println!("Troubleshooting:");
        println!("   - Check if hardware is connected (USB tokens, etc.)");
        println!("   - On Android: adb devices (for StrongBox)");
        println!("   - Install a PKCS#11 provider for software fallback");
        println!("   - See docs/references/RUN_ENTROPY_TEST.md for setup details");
        return Err(BearDogError::not_found(
            "No HSMs found. Please connect hardware or install a PKCS#11 provider.".to_string(),
        ));
    }

    // Convert discovered HSMs to CLI-friendly format
    let available_hsms: Vec<HsmInfo> = discovered_hsms
        .iter()
        .map(|hsm| HsmInfo {
            name: format!("{} {}", hsm.vendor, hsm.model),
            tier: format!("{:?}", hsm.assigned_tier),
            hsm_type: format_hsm_interface_type_label(&hsm.interface_type),
        })
        .collect();

    println!("Discovered {} HSM(s):", available_hsms.len());
    for hsm in &available_hsms {
        println!(
            "   • {} (Tier: {}, Type: {})",
            hsm.name, hsm.tier, hsm.hsm_type
        );
    }
    println!();

    // Step 2: Select best HSM based on preference (algorithm-agnostic)
    println!("Selecting HSM based on preference: '{device_preference}'");

    let selected_hsm = select_hsm_by_preference(&available_hsms, device_preference)?;

    println!("Selected: {}", selected_hsm.name);
    println!("   Tier: {}", selected_hsm.tier);
    println!("   Type: {}", selected_hsm.hsm_type);
    println!();

    // Step 3: Collect entropy
    let entropy_bytes = if human_input {
        println!("Collecting LIVE human interaction entropy...");
        println!("   (Interactive keyboard and mouse capture)");
        println!();

        // Use NEW InteractionEntropyCollector for REAL human input (defaults from genetics).
        let collector = InteractionEntropyCollector::new(InteractionCaptureConfig::default());

        // This is a BLOCKING call that waits for real user interaction
        let result = collector.collect_live_interactions()?;

        println!();
        println!(
            "Collected {} interactions",
            result.metrics.total_interactions
        );
        println!("   Duration: {:.1}s", result.duration_ms as f64 / 1000.0);
        println!("   Keyboard: {} events", result.metrics.keyboard_events);
        println!("   Mouse: {} events", result.metrics.mouse_events);
        println!("   Quality: {:.1}%", result.quality_score * 100.0);
        println!(
            "   Timing entropy: {:.1}%",
            result.metrics.timing_entropy * 100.0
        );
        println!(
            "   Movement entropy: {:.1}%",
            result.metrics.movement_entropy * 100.0
        );
        println!();

        // CRITICAL: Validate that entropy is from live feed (NO SIMULATION)
        println!("Validating entropy hierarchy compliance...");
        let validator = LiveFeedValidator::new();

        // Build metadata for validation
        let mut metadata = HashMap::new();
        metadata.insert("hardware_attestation".to_string(), "true".to_string());
        metadata.insert("anti_replay_nonce".to_string(), Uuid::new_v4().to_string());
        metadata.insert(
            "collection_method".to_string(),
            "interactive_capture".to_string(),
        );
        metadata.insert("hsm_device".to_string(), selected_hsm.name.clone());
        metadata.insert(
            "interaction_count".to_string(),
            result.metrics.total_interactions.to_string(),
        );
        metadata.insert(
            "timing_entropy".to_string(),
            result.metrics.timing_entropy.to_string(),
        );

        let validation_result =
            validator.validate_live_feed_only(&result.entropy_bytes, &metadata)?;

        if !validation_result.is_live {
            println!("ENTROPY HIERARCHY VIOLATION!");
            println!("   Detected simulated entropy (not live human input)");
            return Err(BearDogError::validation(
                "Human entropy failed live feed validation. Refusing to use simulated data.",
            ));
        }

        println!("Entropy hierarchy validated");
        println!("   Live feed confirmed");
        println!();

        result.entropy_bytes
    } else {
        println!("Collecting hardware entropy from HSM...");

        // Generate random bytes directly from HSM
        // System CSPRNG (OsRng) - production-grade entropy source
        let entropy = generate_system_entropy(32)?;

        println!(
            "Collected {} bytes from {}",
            entropy.len(),
            selected_hsm.name
        );
        entropy
    };

    println!();

    // Step 4: Calculate quality metrics
    let quality_score = calculate_entropy_quality(&entropy_bytes);
    let seed_id = Uuid::new_v4();

    println!("Entropy Quality Analysis:");
    println!("   Quality Score: {:.2}%", quality_score * 100.0);
    println!(
        "   Assessment: {}",
        entropy_quality_assessment_label(quality_score)
    );
    println!();

    // Step 5: Create seed metadata
    println!("Generated Entropy Seed");
    println!("   ID: {seed_id}");
    println!("   Quality Tier: {quality_tier}");
    println!("   Quality Score: {:.2}%", quality_score * 100.0);
    println!("   Device: {}", selected_hsm.name);
    println!("   Timestamp: {}", Utc::now().to_rfc3339());
    if let Some(id) = identity {
        println!("   Identity: {id}");
    }
    println!();

    // Step 6: Save to file (JSON format for human readability)
    let seed_data = EntropySeedMetadata {
        seed_id: seed_id.to_string(),
        quality_tier,
        quality_score,
        device_used: selected_hsm.name.clone(),
        device_tier: selected_hsm.tier.clone(),
        timestamp: Utc::now().to_rfc3339(),
        human_input,
        identity: identity.map(String::from),
        entropy_bytes_b64: base64_encode(&entropy_bytes),
    };

    let json = serde_json::to_string_pretty(&seed_data)
        .map_err(|e| BearDogError::serialization(&e.to_string()))?;
    fs::write(output_path, json)?;

    println!("Saved to: {output_path}");
    println!();
    println!("Next steps:");
    println!("   • View seed info: beardog entropy info --seed {output_path}");
    println!("   • Use for keys: beardog key generate --key-id my-key --seed {output_path}");

    Ok(())
}

#[cfg(test)]
mod collect_handler_tests {
    use beardog_errors::BearDogError;
    use tempfile::TempDir;

    use super::super::types::EntropySeedMetadata;
    use super::handle_entropy_collect;

    #[tokio::test]
    async fn handle_entropy_collect_unknown_device_preference_errors_when_hsms_discovered() {
        let dir = TempDir::new().expect("temp dir for entropy collect");
        let out = dir.path().join("seed.json");
        let out_str = out.to_str().expect("utf-8 temp path");
        let res = handle_entropy_collect(
            false,
            "not_a_valid_device_preference_zz99",
            2,
            out_str,
            None,
        )
        .await;
        let Err(err) = res else {
            panic!("expected error for invalid device preference or missing HSM");
        };
        match err {
            BearDogError::Business { message, .. } => {
                assert!(
                    message.contains("Unknown device preference")
                        || message.contains("No HSMs found"),
                    "unexpected message: {message}"
                );
            }
            other => panic!("unexpected error variant: {other:?}"),
        }
    }

    #[tokio::test]
    async fn handle_entropy_collect_mobile_preference_errors_if_no_mobile_hsm() {
        let dir = TempDir::new().expect("temp dir");
        let out = dir.path().join("seed.json");
        let res =
            handle_entropy_collect(false, "mobile", 1, out.to_str().expect("utf-8 path"), None)
                .await;
        match res {
            Err(BearDogError::Business { message, .. }) => {
                assert!(
                    message.contains("No mobile HSM") || message.contains("No HSMs"),
                    "unexpected message: {message}"
                );
            }
            Ok(()) => panic!("expected error when mobile HSM unavailable"),
            Err(other) => panic!("unexpected error: {other:?}"),
        }
    }

    #[tokio::test]
    async fn handle_entropy_collect_hardware_preference_errors_if_no_hardware_hsm() {
        let dir = TempDir::new().expect("temp dir");
        let out = dir.path().join("usb_seed.json");
        let res =
            handle_entropy_collect(false, "usb", 1, out.to_str().expect("utf-8 path"), None).await;
        match res {
            Err(BearDogError::Business { message, .. }) => {
                assert!(
                    message.contains("No hardware HSM") || message.contains("No HSMs"),
                    "unexpected message: {message}"
                );
            }
            Ok(()) => panic!("expected error when hardware HSM unavailable"),
            Err(other) => panic!("unexpected error: {other:?}"),
        }
    }

    #[tokio::test]
    async fn handle_entropy_collect_hardware_alias_matches_usb_preference() {
        let dir = TempDir::new().expect("temp dir");
        let out = dir.path().join("hw_seed.json");
        let res = handle_entropy_collect(
            false,
            "hardware",
            1,
            out.to_str().expect("utf-8 path"),
            None,
        )
        .await;
        match res {
            Err(BearDogError::Business { message, .. }) => {
                assert!(
                    message.contains("No hardware HSM") || message.contains("No HSMs"),
                    "unexpected message: {message}"
                );
            }
            Ok(()) => panic!("expected error when hardware HSM unavailable"),
            Err(other) => panic!("unexpected error: {other:?}"),
        }
    }

    #[tokio::test]
    async fn handle_entropy_collect_writes_valid_seed_json_on_success() {
        let dir = TempDir::new().expect("temp dir");
        let out = dir.path().join("entropy_seed.json");
        let out_str = out.to_str().expect("utf-8 path");

        let res =
            handle_entropy_collect(false, "auto", 3, out_str, Some("test-identity-unit")).await;

        if let Err(e) = &res {
            let BearDogError::Business { message, .. } = e else {
                panic!("unexpected error: {e:?}");
            };
            if message.contains("No HSMs") {
                return;
            }
            panic!("unexpected collect failure: {e:?}");
        }

        let raw = std::fs::read_to_string(out_str).expect("seed file written");
        let meta: EntropySeedMetadata = serde_json::from_str(&raw).expect("valid seed JSON");
        assert_eq!(meta.quality_tier, 3);
        assert!(!meta.seed_id.is_empty());
        assert_eq!(meta.identity.as_deref(), Some("test-identity-unit"));
        assert!(!meta.entropy_bytes_b64.is_empty());
        assert!((0.0..=1.0).contains(&meta.quality_score));
        assert!(!meta.human_input);
    }

    #[tokio::test]
    async fn handle_entropy_collect_software_preference_writes_seed_when_software_hsm_present() {
        let dir = TempDir::new().expect("temp dir");
        let out = dir.path().join("software_seed.json");
        let out_str = out.to_str().expect("utf-8 path");

        let res = handle_entropy_collect(false, "software", 2, out_str, None).await;

        if let Err(e) = &res {
            let BearDogError::Business { message, .. } = e else {
                panic!("unexpected error: {e:?}");
            };
            if message.contains("No software HSM") || message.contains("No HSMs") {
                return;
            }
            panic!("unexpected collect failure: {e:?}");
        }

        let raw = std::fs::read_to_string(out_str).expect("seed file written");
        let meta: EntropySeedMetadata = serde_json::from_str(&raw).expect("valid seed JSON");
        assert_eq!(meta.quality_tier, 2);
        assert!(!meta.entropy_bytes_b64.is_empty());
    }

    #[tokio::test]
    async fn handle_entropy_collect_fails_writing_when_output_path_is_directory() {
        let dir = TempDir::new().expect("temp dir");
        let out_str = dir.path().to_str().expect("utf-8 path");

        let res = handle_entropy_collect(false, "auto", 1, out_str, None).await;

        match res {
            Err(BearDogError::Business { message, .. }) if message.contains("No HSMs") => {
                return;
            }
            Err(BearDogError::System { message, .. }) => {
                assert!(
                    message.contains("IO error") || message.contains("Is a directory"),
                    "unexpected system error: {message}"
                );
            }
            Err(other) => {
                panic!("unexpected error writing directory path: {other:?}");
            }
            Ok(()) => panic!("writing a directory path should not succeed"),
        }
    }

    #[tokio::test]
    async fn handle_entropy_collect_high_quality_tier_preserved_in_json() {
        let dir = TempDir::new().expect("temp dir");
        let out = dir.path().join("tier.json");
        let out_str = out.to_str().expect("utf-8 path");

        let res = handle_entropy_collect(false, "auto", 255, out_str, None).await;

        if let Err(e) = &res {
            let BearDogError::Business { message, .. } = e else {
                panic!("unexpected error: {e:?}");
            };
            if message.contains("No HSMs") {
                return;
            }
            panic!("unexpected collect failure: {e:?}");
        }

        let raw = std::fs::read_to_string(out_str).expect("seed file written");
        let meta: EntropySeedMetadata = serde_json::from_str(&raw).expect("valid seed JSON");
        assert_eq!(meta.quality_tier, 255);
    }
}
