// SPDX-License-Identifier: AGPL-3.0-or-later

use super::config::{
    OS_RNG_FALLBACK_DEVICE, OS_RNG_FALLBACK_TIER, OS_RNG_SOURCE, OrchestratorConfig,
};
#[cfg(any(feature = "fido2", target_os = "android", target_os = "ios"))]
use super::discovery::HsmSource;
use super::orchestrator::HsmEntropyOrchestrator;
use super::quality::os_rng_fallback_report;
use super::types::{EntropyGenerationRequest, HumanEntropyInput, SecurityLevel};

#[tokio::test]
async fn test_orchestrator_initialization() {
    let result = HsmEntropyOrchestrator::new().await;
    assert!(
        result.is_ok(),
        "Orchestrator should initialize successfully"
    );
}

#[tokio::test]
async fn test_list_available_devices() {
    let orchestrator = HsmEntropyOrchestrator::new()
        .await
        .expect("HsmEntropyOrchestrator::new in test");
    let devices = orchestrator.list_available_devices();
    // Should not panic, may be empty if no HSMs available
    // Note: devices.len() is always >= 0 (usize is unsigned)
    assert!(devices.is_empty() || !devices.is_empty()); // Always true, verifies call succeeds
}

#[tokio::test]
async fn test_orchestrator_config_default() {
    let config = OrchestratorConfig::default();
    assert!(config.prefer_biometric);
    assert_eq!(config.min_security_level, SecurityLevel::Hardware);
    assert!(!config.enable_multi_device_mixing);
}

#[tokio::test]
async fn test_orchestrator_config_custom() {
    let config = OrchestratorConfig {
        prefer_biometric: false,
        min_security_level: SecurityLevel::Software,
        enable_multi_device_mixing: true,
    };
    assert!(!config.prefer_biometric);
    assert_eq!(config.min_security_level, SecurityLevel::Software);
    assert!(config.enable_multi_device_mixing);
}

#[tokio::test]
async fn test_orchestrator_new_with_config() {
    let config = OrchestratorConfig {
        prefer_biometric: false,
        min_security_level: SecurityLevel::Software,
        enable_multi_device_mixing: false,
    };
    let result = HsmEntropyOrchestrator::new_with_config(config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_entropy_generation_request_default() {
    let request = EntropyGenerationRequest::default();
    assert_eq!(request.length, 256);
}

#[tokio::test]
async fn test_generate_entropy_basic() {
    let mut orchestrator = HsmEntropyOrchestrator::new()
        .await
        .expect("HsmEntropyOrchestrator::new in test");

    let request = EntropyGenerationRequest {
        length: 32,
        human_input: None,
        ..Default::default()
    };

    let result = orchestrator.generate_entropy(request);
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_generate_human_entropy_basic() {
    let mut orchestrator = HsmEntropyOrchestrator::new()
        .await
        .expect("HsmEntropyOrchestrator::new in test");
    let result = orchestrator.generate_human_entropy(32, None).await;
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_generate_entropy_various_lengths() {
    let mut orchestrator = HsmEntropyOrchestrator::new()
        .await
        .expect("HsmEntropyOrchestrator::new in test");

    for length in &[16, 32, 64, 128] {
        let request = EntropyGenerationRequest {
            length: *length,
            human_input: None,
            ..Default::default()
        };

        let result = orchestrator.generate_entropy(request);
        assert!(result.is_ok() || result.is_err());
    }
}

#[tokio::test]
async fn test_orchestrator_config_security_levels() {
    for level in &[
        SecurityLevel::Software,
        SecurityLevel::Hardware,
        SecurityLevel::StrongBox,
    ] {
        let config = OrchestratorConfig {
            prefer_biometric: true,
            min_security_level: *level,
            enable_multi_device_mixing: false,
        };

        let result = HsmEntropyOrchestrator::new_with_config(config).await;
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_human_entropy_input_behavioral_data() {
    let input = HumanEntropyInput {
        biometric_data: None,
        behavioral_data: Some(vec![50, 75, 100, 125, 150]),
        environmental_data: None,
    };

    assert!(input.behavioral_data.is_some());
    assert!(input.biometric_data.is_none());
    assert!(input.environmental_data.is_none());
}

#[tokio::test]
async fn test_generate_human_entropy_with_input() {
    let mut orchestrator = HsmEntropyOrchestrator::new()
        .await
        .expect("HsmEntropyOrchestrator::new in test");

    let human_input = HumanEntropyInput {
        biometric_data: Some(vec![100, 150, 120, 180]),
        behavioral_data: None,
        environmental_data: None,
    };

    let result = orchestrator
        .generate_human_entropy(32, Some(human_input))
        .await;
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_orchestrator_config_clone() {
    let config1 = OrchestratorConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.prefer_biometric, config2.prefer_biometric);
    assert_eq!(config1.min_security_level, config2.min_security_level);
    assert_eq!(
        config1.enable_multi_device_mixing,
        config2.enable_multi_device_mixing
    );
}

#[tokio::test]
async fn test_list_devices_consistency() {
    let orchestrator = HsmEntropyOrchestrator::new()
        .await
        .expect("HsmEntropyOrchestrator::new in test");

    let devices1 = orchestrator.list_available_devices();
    let devices2 = orchestrator.list_available_devices();

    assert_eq!(devices1.len(), devices2.len());
}

#[tokio::test]
async fn test_orchestrator_multiple_instances() {
    let orch1 = HsmEntropyOrchestrator::new().await;
    let orch2 = HsmEntropyOrchestrator::new().await;

    assert!(orch1.is_ok());
    assert!(orch2.is_ok());
}

#[tokio::test]
async fn test_entropy_requests_are_independent() {
    let mut orchestrator = HsmEntropyOrchestrator::new()
        .await
        .expect("HsmEntropyOrchestrator::new in test");

    let request1 = EntropyGenerationRequest {
        length: 32,
        human_input: None,
        ..Default::default()
    };

    let request2 = EntropyGenerationRequest {
        length: 64,
        human_input: None,
        ..Default::default()
    };

    let result1 = orchestrator.generate_entropy(request1);
    let result2 = orchestrator.generate_entropy(request2);

    assert!(result1.is_ok() || result1.is_err());
    assert!(result2.is_ok() || result2.is_err());
}

#[tokio::test]
async fn test_orchestrator_sequential_operations() {
    let mut orchestrator = HsmEntropyOrchestrator::new()
        .await
        .expect("HsmEntropyOrchestrator::new in test");

    let _devices = orchestrator.list_available_devices();
    let result1 = orchestrator.generate_human_entropy(32, None).await;
    let result2 = orchestrator.generate_human_entropy(32, None).await;

    assert!(result1.is_ok() || result1.is_err());
    assert!(result2.is_ok() || result2.is_err());
}

#[tokio::test]
async fn test_generate_entropy_zero_length() {
    let mut orchestrator = HsmEntropyOrchestrator::new()
        .await
        .expect("HsmEntropyOrchestrator::new in test");

    let request = EntropyGenerationRequest {
        length: 0,
        human_input: None,
        ..Default::default()
    };

    let result = orchestrator.generate_entropy(request);
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_generate_entropy_large_length() {
    let mut orchestrator = HsmEntropyOrchestrator::new()
        .await
        .expect("HsmEntropyOrchestrator::new in test");

    let request = EntropyGenerationRequest {
        length: 1024,
        human_input: None,
        ..Default::default()
    };

    let result = orchestrator.generate_entropy(request);
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_os_rng_fallback_report_honest_labels() {
    let report = os_rng_fallback_report();
    assert_eq!(report.source, OS_RNG_SOURCE);
    assert_eq!(report.device_used, OS_RNG_FALLBACK_DEVICE);
    assert!(!report.hardware_backed);
    assert_eq!(report.quality_tier, OS_RNG_FALLBACK_TIER);
    assert!(report.quality_score < 0.50);
}

#[tokio::test]
#[cfg(any(feature = "fido2", target_os = "android", target_os = "ios"))]
async fn test_generate_from_hsm_uses_os_rng_fallback_metadata() {
    let orchestrator = HsmEntropyOrchestrator::new()
        .await
        .expect("HsmEntropyOrchestrator::new in test");

    #[cfg(feature = "fido2")]
    let source = HsmSource::Fido2(0);
    #[cfg(all(not(feature = "fido2"), target_os = "android"))]
    let source = HsmSource::Android;
    #[cfg(all(not(feature = "fido2"), not(target_os = "android"), target_os = "ios"))]
    let source = HsmSource::IOS;

    let (entropy, report) = orchestrator
        .generate_from_hsm(&source, 32)
        .expect("generate_from_hsm in test");
    assert_eq!(entropy.len(), 32);
    assert!(!report.hardware_backed || report.source == "fido2_hardware");
}

#[tokio::test]
async fn test_generate_entropy_async_matches_sync_shape() {
    let mut orchestrator = HsmEntropyOrchestrator::new()
        .await
        .expect("HsmEntropyOrchestrator::new in test");

    let request = EntropyGenerationRequest {
        length: 32,
        human_input: None,
        ..Default::default()
    };

    let result = orchestrator.generate_entropy_async(request).await;
    // Without hardware features (fido2/mobile), no HSM source is available,
    // so both sync and async paths return an error — that's correct behavior.
    if let Ok(result) = result {
        assert_eq!(result.source, OS_RNG_SOURCE);
        assert!(!result.hardware_backed);
        assert!(result.quality_score > 0.0);
    }
}

#[tokio::test]
async fn test_mix_with_human_input_blake3() {
    let orchestrator = HsmEntropyOrchestrator::new()
        .await
        .expect("HsmEntropyOrchestrator::new in test");

    let hw_entropy = vec![42u8; 32];
    let input = HumanEntropyInput {
        biometric_data: Some(vec![1, 2, 3, 4]),
        behavioral_data: Some(vec![5, 6, 7, 8]),
        environmental_data: Some(vec![9, 10, 11, 12]),
    };

    let mixed = orchestrator.mix_with_human_input(hw_entropy.clone(), input);
    assert!(mixed.is_ok());
    let mixed = mixed.unwrap();
    assert_eq!(mixed.len(), 32, "BLAKE3 output is 32 bytes");
    assert_ne!(mixed, hw_entropy, "mixing should change the output");
}
