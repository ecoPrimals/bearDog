// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;

#[test]
fn test_discoverer_creation() {
    let discoverer = CloudDiscoverer::new();
    assert!(discoverer.is_ok());
}

#[tokio::test]
async fn test_cloud_discovery() {
    let discoverer = CloudDiscoverer::new().expect("cloud discoverer");
    let result = discoverer.discover().await;
    assert!(result.is_ok());
}

#[test]
fn test_aws_kms_capabilities() {
    let discoverer = CloudDiscoverer::new().expect("cloud discoverer");
    let caps = discoverer.create_aws_kms_capabilities();
    
    assert_eq!(caps.security.fips_140_2_level, Some(3));
    assert!(caps.security.tamper_resistance == TamperResistance::Tier1);
    assert!(caps.compliance.fips_140_2);
    assert!(caps.compliance.hipaa);
    assert!(caps.key_management.key_rotation);
    assert_eq!(caps.performance.max_operations_per_second, 100000);
}

#[test]
fn test_azure_capabilities() {
    let discoverer = CloudDiscoverer::new().expect("cloud discoverer");
    let caps = discoverer.create_azure_key_vault_capabilities();
    
    assert_eq!(caps.security.fips_140_2_level, Some(2));
    assert!(caps.key_management.key_recovery);
}

#[test]
fn test_gcp_capabilities() {
    let discoverer = CloudDiscoverer::new().expect("cloud discoverer");
    let caps = discoverer.create_gcp_kms_capabilities();
    
    assert_eq!(caps.security.fips_140_2_level, Some(3));
    assert_eq!(caps.performance.max_operations_per_second, 150000);
}

#[test]
fn test_custom_config() {
    let mut discoverer = CloudDiscoverer::new().expect("cloud discoverer");
    
    let config = CloudHsmConfig {
        provider: CloudProvider::Aws,
        region: "us-west-2".to_string(),
        endpoint: Some("custom.endpoint.com".to_string()),
        service_type: "kms".to_string(),
    };
    
    discoverer.add_custom_config(config);
    assert_eq!(discoverer.custom_configs.len(), 1);
}

#[test]
fn test_provider_types() {
    assert_eq!(CloudProvider::Aws, CloudProvider::Aws);
    assert_ne!(CloudProvider::Aws, CloudProvider::Azure);
}
