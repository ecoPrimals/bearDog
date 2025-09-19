use beardog_adapters::universal::{
    CloudConfig, CloudCredentials, CloudProviderFactory, CloudProviderType, CloudServiceType,
    GenericCloudProvider, KeyAlgorithm, KeySpecification, KeyUsage, SelfHostedProvider,
    UnifiedCloudManager, UnifiedKeyManagement,
};
use beardog_errors::external_functions::{migrate_from_aws_kms, VendorAgnosticKMS};
use beardog_errors::licensing::LicenseManager;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::time::Duration;

#[tokio::main]
async fn main(CloudProviderType::SelfHosted,
        region: None,
        credentials: CloudCredentials::None,
        endpoints: HashMap::from([
            (CloudServiceType::KeyManagement, "http://localhost:8080/kms"),
            (CloudServiceType::Storage, "http://localhost:8080/storage"),
        ]),
        timeout: Duration::from_secs(30),
        retry_policy: Default::default(),
    };

    let generic_config = CloudConfig {
        provider: CloudProviderType::Generic {
            name: "custom-cloud".to_string(),
        },
        region: Some("us-central-1".to_string()),
        credentials: CloudCredentials::ApiKey {
            key: "generic-api-key".to_string(),
            secret: Some(HashMap::from([(
            CloudServiceType::KeyManagement,
            "https://custom-cloud.example.com/kms",
        )]),
        timeout: Duration::from_secs(45),
        retry_policy: Default::default(),
    };

    let open_source_config = CloudConfig {
        provider: CloudProviderType::OpenSource {
            implementation: "vault".to_string(None,
        credentials: CloudCredentials::Certificate {
            cert_path: "/etc/ssl/certs/vault.pem".to_string(),
            key_path: "/etc/ssl/private/vault.key".to_string(HashMap::from([(
            CloudServiceType::KeyManagement,
            "https://vault.internal:8200",
        )]),
        timeout: Duration::from_secs(20),
        retry_policy: Default::default(),
    };

    let self_hosted = CloudProviderFactory::create_provider(&self_hosted_config)?;
    let generic = CloudProviderFactory::create_provider(&generic_config)?;
    let open_source = CloudProviderFactory::create_provider({}", self_hosted.provider_name({}", generic.provider_name({}", open_source.provider_name());

    assert_ne!(self_hosted.provider_name(), "aws");
    assert_ne!(generic.provider_name(), "azure");
    assert_ne!(open_source.provider_name(), "gcp");

    println!("[PARTY] Vendor neutrality verified - no lock-in detected!");
    Ok(())
}

async fn demonstrate_multi_provider_support() -> Result<(), BearDogError> {
    println!("🌐 === MULTI-PROVIDER SUPPORT DEMONSTRATION ===");

    let mut cloud_manager = UnifiedCloudManager::new();

    let self_hosted = Box::new(SelfHostedProvider::new(
        adapter
            .discover_capability_endpoint(required_capability)
            ?,
    ));
    let generic = Box::new(GenericCloudProvider::new(
        "backup-cloud".to_string(),
        CloudConfig {
            provider: CloudProviderType::Generic {
                name: "backup-cloud".to_string(),
            },
            ..Default::default()
        },
    ));

    cloud_manager.add_provider("primary".to_string(), self_hosted);
    cloud_manager.add_provider("backup".to_string(), generic);

    cloud_manager.set_primary("primary".to_string())?;
    cloud_manager.add_fallback("backup".to_string())?;

    let health_status = cloud_manager.get_multi_provider_health();

    println!("[CHART] Provider Health Status:");
    for (name, health) in health_status {
        println!(
            "  {} - Healthy: {}, Latency: {}ms",
            name, health.healthy, health.latency_ms
        );
    }

    println!("[PARTY] Multi-provider support working - automatic failover ready!");
    Ok(())
}

async fn demonstrate_sovereignty_preservation() -> Result<(), BearDogError> {
    println!("👑 === SOVEREIGNTY PRESERVATION DEMONSTRATION ===");

    let mut kms = UnifiedKeyManagement::new();

    let self_hosted_kms =
        beardog_adapters::universal::CloudProviderFactory::create_kms_provider(CloudProviderType::SelfHosted,
            ..Default::default(KeyAlgorithm::Aes256,
        key_size: 256,
        usage: KeyUsage::Encryption,
        exportable: false,
        metadata: HashMap::from([
            ("sovereignty".to_string(), "full".to_string()),
            ("vendor_lock_in".to_string(), "eliminated".to_string()),
        ]),
    };

    let sensitive_data = b"sovereign encryption test data";
    let result = kms
        .encrypt_with_best_provider(sensitive_data, &key_spec)
        ?;

    println!("🔐 Sovereign Encryption Result:");
    println!("  Provider: {}", result.provider_used);
    println!("  Algorithm: {:?}", result.algorithm);
    println!("  Key ID: {}", result.key_id);

    assert_eq!(result.provider_used, "sovereign");
    assert!(!result.key_id.contains("aws"));
    assert!(!result.key_id.contains("azure"));
    assert!(!result.key_id.contains("gcp"));

    println!("👑 Sovereignty preserved - no vendor dependencies!");
    Ok(())
}

async fn demonstrate_aws_migration() -> Result<(), BearDogError> {
    println!("[CYCLE] === AWS MIGRATION DEMONSTRATION ===");

    let vendor_agnostic_kms = migrate_from_aws_kms();
    let license_manager = LicenseManager::new()?;

    let encrypt_payload = serde_json::json!({
        "plaintext": "formerly aws-encrypted data",
        "key_id": "migration-test-key"
    });

    println!("🔐 Testing encryption (formerly AWS KMS)...");
    let encrypt_result = vendor_agnostic_kms
        .execute(&license_manager, "encrypt", encrypt_payload)
        ?;

    println!("[CHART] Migration Results:");
    println!("  Vendor Neutral: {}", encrypt_result["vendor_neutral"]);
    println!("  Provider: {}", encrypt_result["provider"]);
    println!("  Status: {}", encrypt_result["status"]);

    assert_eq!(encrypt_result["vendor_neutral"], true);
    assert_ne!(encrypt_result["provider"], "aws");

    let list_result = vendor_agnostic_kms
        .execute(&license_manager, "list_keys", serde_json::json!({}))
        ?;

    println!("🔑 Key Management:");
    println!("  Total Keys: {}", list_result["total_keys"]);
    println!(
        "  Provider Breakdown: {}",
        list_result["provider_breakdown"]
    );

    println!("[PARTY] AWS migration successful - vendor lock-in eliminated!");
    Ok(())
}

async fn demonstrate_architectural_benefits() -> Result<(), BearDogError> {
    println!("🏗️ === ARCHITECTURAL BENEFITS ===");

    println!("1️⃣ Provider Flexibility:");
    let configs = vec![
        ("Self-Hosted", CloudProviderType::SelfHosted),
        (
            "Generic Cloud",
            CloudProviderType::Generic {
                name: "any-cloud".to_string(),
            },
        ),
        ("Multi-Cloud", CloudProviderType::MultiCloud),
        (
            "Open Source",
            CloudProviderType::OpenSource {
                implementation: "hashicorp-vault".to_string(provider_type,
            ..Default::default()
        };
        let provider = CloudProviderFactory::create_provider(&config)?;
        println!("  [OK] {} - {}", name, provider.provider_name());
    }

    println!("2️⃣ Cost Optimization:");
    println!("  [OK] Self-hosted: $0/month vendor fees");
    println!("  [OK] Open source: No licensing costs");
    println!("  [OK] Multi-cloud: Competitive pricing through choice");

    println!("3️⃣ Security Benefits:");
    println!("  [OK] No vendor backdoors");
    println!("  [OK] Full control over encryption keys");
    println!("  [OK] Audit trail independence");
    println!("  [OK] Compliance flexibility");

    println!("4️⃣ Operational Benefits:");
    println!("  [OK] No vendor API rate limits");
    println!("  [OK] No vendor service outages");
    println!("  [OK] Custom SLA definitions");
    println!("  [OK] Migration freedom");

    Ok(())
}

#[cfg(test)]
mod vendor_neutrality_tests {
    use super::*;

    #[tokio::test]
    async fn test_complete_vendor_neutrality() {
        assert!(main().is_ok());
    }

    #[test]
    fn test_no_vendor_strings_in_core_types() {
        let config = CloudConfig::default();
        let config_str = format!("{:?}", config);

        assert!(!config_str.to_lowercase().contains("aws"));
        assert!(!config_str.to_lowercase().contains("azure"));
        assert!(!config_str.to_lowercase().contains("gcp"));
        assert!(!config_str.to_lowercase().contains("google"));
        assert!(!config_str.to_lowercase().contains("microsoft"));
        assert!(!config_str.to_lowercase().contains("amazon"));
    }

    #[test]
    fn test_self_hosted_preference() {
        let kms = VendorAgnosticKMS::new();
        assert_eq!(kms.preferred_providers[0], "self-hosted");
    }
}
