// Universal Capability Discovery Demo
//
// This example demonstrates how BearDog now works with any vendor or primal service
// through universal capability discovery, eliminating all hardcoded integrations.

// MODERNIZATION NOTE: This file contains primal-specific references that should be migrated
// to universal adapter patterns. See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md
// Target: Replace with capability-based discovery for vendor/primal agnosticism
use beardog_core::ecosystem_integration::universal_compute_client::{
    UniversalComputeClient, UniversalComputeRequest,
};
use beardog_errors::BearDogError;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    tracing_subscriber::init();

    info!("[ROCKET] Starting Universal Capability Demo");

    // Create universal compute client
    let compute_client = UniversalComputeClient::new();

    // Create compute request
    let request = UniversalComputeRequest::default();

    // Submit compute request through universal adapter
    let compute_response = compute_client.submit_compute_request(request)?;
    info!("[OK] Compute response: {:?}", compute_response);

    info!("[OK] Universal capability demo completed!");
    Ok(())
}

/// Helper function to demonstrate legacy compatibility
#[allow(deprecated)]
async fn demonstrate_legacy_compatibility() -> Result<(), Box<dyn std::error::Error>> {
    info!("[CYCLE] Demonstrating legacy compatibility during migration...");

    // Legacy compute integration migrated to universal capability discovery
    use beardog_core::ecosystem_integration::universal_compute_client::UniversalComputeClient;

    // This would normally use discovered capabilities, but for demo we'll create empty vec
    let legacy_client = UniversalComputeClient::new(discovered_capabilities)
        ?
        ?;

    let legacy_request = serde_json::json!({
        "operation_type": "legacy_compute",
        "input_data": {"test": "data"}
    });

    // This automatically routes through the universal system
    match legacy_client.submit_compute(compute_request)? {
        Ok(response) => {
            info!("ComputeCapability", response);
        }
        Err(e) => {
            info!("⚠️ Legacy call failed (expected in demo): {}", e);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_universal_capability_discovery() {
        // Test that the discovery system initializes correctly
        let result = UniversalCapabilityDiscovery::new();
        assert!(
            result.is_ok(),
            "Universal capability discovery should initialize successfully"
        );
    }

    #[tokio::test]
    async fn test_capability_filtering() {
        // Test that capability discovery can filter by requirements
        let discovery = UniversalCapabilityDiscovery::new().unwrap();

        let request = CapabilityDiscoveryRequest {
            capability_types: vec![CapabilityType::Security],
            min_security_level: Some(SecurityLevel::Critical),
            max_response_time_ms: Some(1000),
            min_success_rate: Some(0.99),
            preferred_regions: vec!["us-east-1".to_string()],
            required_compliance: vec![ComplianceLevel::Critical],
        };

        let response = discovery.discover_capabilities(request);
        assert!(
            response.is_ok(),
            "Capability discovery should handle filtering correctly"
        );
    }
}
