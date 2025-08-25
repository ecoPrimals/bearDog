// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! Universal Vendor Adapter Demo
//!
//! This example demonstrates BearDog's universal adapter system that allows
//! to working with different providers through a unified interface.

use beardog_errors::BearDogResult;
use std::collections::HashMap;
use tracing::{info, warn};

#[derive(Debug, Clone)]
enum VendorType {
    CloudHsm,
    HardwareHsm,
    SoftwareHsm,
    ThirdPartyApi,
}

#[derive(Debug, Clone)]
struct VendorCapability {
    name: String,
    vendor_type: VendorType,
    supported_operations: Vec<String>,
    performance_tier: String,
    cost_per_operation: f64,
    availability: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct AdapterRequest {
    operation: String,
    data: Vec<u8>,
    parameters: HashMap<String, String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct AdapterResponse {
    success: bool,
    result: Vec<u8>,
    metadata: HashMap<String, String>,
    processing_time_ms: u64,
}

#[allow(dead_code)]
struct UniversalVendorAdapter {
    registered_vendors: Vec<VendorCapability>,
    request_history: HashMap<String, Vec<AdapterResponse>>,
}

#[tokio::main]
async fn main() -> BearDogResult<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🔌 BearDog Universal Vendor Adapter Demo");
    info!("========================================");

    // Demonstrate vendor registration
    demonstrate_vendor_registration().await?;

    // Show capability discovery
    demonstrate_capability_discovery().await?;

    // Demonstrate unified operations
    demonstrate_unified_operations().await?;

    // Show vendor failover
    demonstrate_vendor_failover().await?;

    info!("✅ Universal Vendor Adapter Demo Complete!");
    Ok(())
}

async fn demonstrate_vendor_registration() -> BearDogResult<()> {
    info!("🔧 Vendor Registration");
    info!("---------------------");

    let mut adapter = UniversalVendorAdapter::new();

    // Register different types of vendors
    let vendors = create_sample_vendors();

    for vendor in &vendors {
        adapter.register_vendor(vendor.clone()).await?;
        info!("✅ Registered: {} ({:?})", vendor.name, vendor.vendor_type);
        info!("   Operations: {}", vendor.supported_operations.join(", "));
        info!(
            "   Performance: {}, Cost: ${:.4}/op",
            vendor.performance_tier, vendor.cost_per_operation
        );
    }

    info!(
        "📊 Total vendors registered: {}",
        adapter.get_vendor_count()
    );

    Ok(())
}

async fn demonstrate_capability_discovery() -> BearDogResult<()> {
    info!("\n🔍 Capability Discovery");
    info!("----------------------");

    let adapter = create_configured_adapter().await?;

    // Discover capabilities for different operations
    let operations = vec!["encrypt", "sign", "hash", "generate_key"];

    for operation in operations {
        info!("🔎 Discovering vendors for '{}':", operation);
        let capable_vendors = adapter.discover_capable_vendors(operation).await?;

        for vendor in capable_vendors {
            info!(
                "   ✅ {}: {} (availability: {:.1}%)",
                vendor.name,
                vendor.performance_tier,
                vendor.availability * 100.0
            );
        }
    }

    Ok(())
}

async fn demonstrate_unified_operations() -> BearDogResult<()> {
    info!("\n⚡ Unified Operations");
    info!("-------------------");

    let adapter = create_configured_adapter().await?;

    // Demonstrate how the same operation can be performed across different vendors
    let operations = vec![
        ("encrypt", "Hello, World!".as_bytes().to_vec()),
        ("sign", "Document to sign".as_bytes().to_vec()),
        ("hash", "Data to hash".as_bytes().to_vec()),
    ];

    for (operation, data) in operations {
        info!("🔄 Performing '{}' operation:", operation);

        let request = AdapterRequest {
            operation: operation.to_string(),
            data: data.clone(),
            parameters: HashMap::new(),
        };

        match adapter.execute_operation(&request).await {
            Ok(response) => {
                info!(
                    "   ✅ Success: {} bytes processed in {}ms",
                    response.result.len(),
                    response.processing_time_ms
                );
                info!(
                    "   📊 Vendor: {}",
                    response
                        .metadata
                        .get("vendor")
                        .unwrap_or(&"Unknown".to_string())
                );
            }
            Err(e) => {
                warn!("   ❌ Failed: {}", e);
            }
        }
    }

    Ok(())
}

async fn demonstrate_vendor_failover() -> BearDogResult<()> {
    info!("\n🔄 Vendor Failover");
    info!("-----------------");

    let mut adapter = create_configured_adapter().await?;

    info!("🔄 Normal operation:");
    let request = AdapterRequest {
        operation: "encrypt".to_string(),
        data: "Test data".as_bytes().to_vec(),
        parameters: HashMap::new(),
    };

    match adapter.execute_operation(&request).await {
        Ok(response) => {
            info!(
                "   ✅ Primary vendor: {}",
                response
                    .metadata
                    .get("vendor")
                    .unwrap_or(&"Unknown".to_string())
            );
        }
        Err(e) => {
            warn!("   ❌ Primary failed: {}", e);
        }
    }

    // Simulate vendor failure
    info!("\n⚠️  Simulating vendor failure:");
    adapter.simulate_vendor_failure("AWS CloudHSM").await;

    match adapter.execute_operation(&request).await {
        Ok(response) => {
            info!(
                "   ✅ Failover successful: {}",
                response
                    .metadata
                    .get("vendor")
                    .unwrap_or(&"Unknown".to_string())
            );
        }
        Err(e) => {
            warn!("   ❌ Failover failed: {}", e);
        }
    }

    Ok(())
}

impl UniversalVendorAdapter {
    fn new() -> Self {
        Self {
            registered_vendors: Vec::new(),
            request_history: HashMap::new(),
        }
    }

    async fn register_vendor(&mut self, vendor: VendorCapability) -> BearDogResult<()> {
        self.registered_vendors.push(vendor);
        Ok(())
    }

    const fn get_vendor_count(&self) -> usize {
        self.registered_vendors.len()
    }

    async fn discover_capable_vendors(
        &self,
        operation: &str,
    ) -> BearDogResult<Vec<VendorCapability>> {
        let capable = self
            .registered_vendors
            .iter()
            .filter(|v| v.supported_operations.contains(&operation.to_string()))
            .cloned()
            .collect();

        Ok(capable)
    }

    async fn execute_operation(&self, request: &AdapterRequest) -> BearDogResult<AdapterResponse> {
        // Find the best vendor for this operation
        let capable_vendors = self.discover_capable_vendors(&request.operation).await?;

        if capable_vendors.is_empty() {
            return Err(beardog_errors::BearDogError::configuration(format!("No vendors capable of '{)'", request.operation),
            });
        }

        // Select the best vendor (highest availability)
        let selected_vendor = capable_vendors
            .iter()
            .max_by(|a, b| {
                a.availability
                    .partial_cmp(&b.availability)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .ok_or_else(|| beardog_errors::BearDogError::configuration("No providers available for selection".to_string(),
            ))?;

        // Simulate operation execution
        let processing_time = simulate_operation_time(&selected_vendor.vendor_type);
        let result = simulate_operation_result(&request.operation, &request.data);

        let mut metadata = HashMap::new();
        metadata.insert("vendor".to_string(), selected_vendor.name.clone());
        metadata.insert(
            "vendor_type".to_string(),
            format!("{:?}", selected_vendor.vendor_type),
        );

        Ok(AdapterResponse {
            success: true,
            result,
            metadata,
            processing_time_ms: processing_time,
        })
    }

    async fn simulate_vendor_failure(&mut self, vendor_name: &str) {
        if let Some(vendor) = self
            .registered_vendors
            .iter_mut()
            .find(|v| v.name == vendor_name)
        {
            vendor.availability = 0.0;
            warn!("⚠️  Vendor '{}' marked as unavailable", vendor_name);
        }
    }
}

fn create_sample_vendors() -> Vec<VendorCapability> {
    vec![
        VendorCapability {
            name: "AWS CloudHSM".to_string(),
            vendor_type: VendorType::CloudHsm,
            supported_operations: vec![
                "encrypt".to_string(),
                "decrypt".to_string(),
                "sign".to_string(),
                "verify".to_string(),
            ],
            performance_tier: "High".to_string(),
            cost_per_operation: 0.001,
            availability: 0.99,
        },
        VendorCapability {
            name: "Thales Hardware HSM".to_string(),
            vendor_type: VendorType::HardwareHsm,
            supported_operations: vec![
                "encrypt".to_string(),
                "decrypt".to_string(),
                "sign".to_string(),
                "verify".to_string(),
                "generate_key".to_string(),
            ],
            performance_tier: "Premium".to_string(),
            cost_per_operation: 0.005,
            availability: 0.999,
        },
        VendorCapability {
            name: "BearDog Software HSM".to_string(),
            vendor_type: VendorType::SoftwareHsm,
            supported_operations: vec![
                "encrypt".to_string(),
                "decrypt".to_string(),
                "sign".to_string(),
                "verify".to_string(),
                "hash".to_string(),
                "generate_key".to_string(),
            ],
            performance_tier: "Standard".to_string(),
            cost_per_operation: 0.0001,
            availability: 0.95,
        },
        VendorCapability {
            name: "HashiCorp Vault".to_string(),
            vendor_type: VendorType::ThirdPartyApi,
            supported_operations: vec![
                "encrypt".to_string(),
                "decrypt".to_string(),
                "hash".to_string(),
            ],
            performance_tier: "Standard".to_string(),
            cost_per_operation: 0.0005,
            availability: 0.98,
        },
    ]
}

async fn create_configured_adapter() -> BearDogResult<UniversalVendorAdapter> {
    let mut adapter = UniversalVendorAdapter::new();
    let vendors = create_sample_vendors();

    for vendor in vendors {
        adapter.register_vendor(vendor).await?;
    }

    Ok(adapter)
}

fn simulate_operation_time(vendor_type: &VendorType) -> u64 {
    match vendor_type {
        VendorType::CloudHsm => 50 + (rand::random::<u64>() % 20),
        VendorType::HardwareHsm => 10 + (rand::random::<u64>() % 5),
        VendorType::SoftwareHsm => 5 + (rand::random::<u64>() % 3),
        VendorType::ThirdPartyApi => 100 + (rand::random::<u64>() % 50),
    }
}

fn simulate_operation_result(operation: &str, input_data: &[u8]) -> Vec<u8> {
    match operation {
        "encrypt" => {
            let mut result = input_data.to_vec();
            result.reverse(); // Simple "encryption"
            result
        }
        "sign" => format!("SIGNATURE_OF_{}", String::from_utf8_lossy(input_data)).into_bytes(),
        "hash" => format!("HASH_{}", input_data.len()).into_bytes(),
        "generate_key" => b"GENERATED_KEY_256_BITS".to_vec(),
        _ => input_data.to_vec(),
    }
}
