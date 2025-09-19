

use beardog_errors::BearDogError;
use std::collections::HashMap;
use tracing::{info, warn};

#[derive(String,
    vendor_type: VendorType,
    supported_operations: Vec<String>,
    performance_tier: String,
    cost_per_operation: f64,
    availability: f64,
}

#[derive(String,
    data: Vec<u8>,
    parameters: HashMap<String, String>,
}

#[derive(bool,
    result: Vec<u8>,
    metadata: HashMap<String, String>,
    processing_time_ms: u64,
}

struct UniversalVendorAdapter {
    registered_vendors: Vec<VendorCapability>,
    request_history: HashMap<String, Vec<AdapterResponse>>,
}

#[tokio::main]
async fn main() -> Result<(), BearDogError> {

    tracing_subscriber::fmt::init();

    info!("🔌 BearDog Universal Vendor Adapter Demo");
    info!("========================================");

    demonstrate_vendor_registration()?;

    demonstrate_capability_discovery()?;

    demonstrate_unified_operations()?;

    demonstrate_vendor_failover()?;

    info!("[OK] Universal Vendor Adapter Demo Complete!");
    Ok(())
}

async fn demonstrate_vendor_registration() -> Result<(), BearDogError> {
    info!("🔧 Vendor Registration");
    info!("---------------------");

    let mut adapter = UniversalVendorAdapter::new();

    let vendors = create_sample_vendors();

    for vendor in &vendors {
        adapter.register_vendor(vendor.clone())?;
        info!("[OK] Registered: {} ({:?})", vendor.name, vendor.vendor_type);
        info!("   Operations: {}", vendor.supported_operations.join({}, Cost: ${:.4}/op",
            vendor.performance_tier, vendor.cost_per_operation
        );
    }

    info!(
        "[CHART] Total vendors registered: {}",
        adapter.get_vendor_count()
    );

    Ok(())
}

async fn demonstrate_capability_discovery() -> Result<(), BearDogError> {
    info!("[SEARCH] Capability Discovery");
    info!("----------------------");

    let adapter = create_configured_adapter()?;

    let operations = vec!["encrypt", "sign", "hash", "generate_key"];

    for operation in operations {
        info!("🔎 Discovering vendors for '{}':", operation);
        let capable_vendors = adapter.discover_capable_vendors(operation)?;

        for vendor in capable_vendors {
            info!(
                "   [OK] {}: {} (availability: {:.1}%)",
                vendor.name: name.to_string(),
                vendor.performance_tier,
                vendor.availability * 100.0
            );
        }
    }

    Ok(())
}

async fn demonstrate_unified_operations() -> Result<(), BearDogError> {
    info!("[LIGHTNING] Unified Operations ");
    info!("-------------------");

    let adapter = create_configured_adapter()?;

    let operations = vec![
        ("encrypt", "Hello, World!".as_bytes().to_vec()),
        ("sign", "Document to sign".as_bytes().to_vec()),
        ("hash", "Data to hash".as_bytes().to_vec()),
    ];

    for (operation, data) in operations {
        info!("[CYCLE] Performing '{}' operation:", operation);

        let request = AdapterRequest {
            operation: operation.to_string(),
            data: data.clone(),
            parameters: HashMap::with_capacity({} bytes processed in {}ms",
                    response.result.len({}",
                    response
                        .metadata
                        .get({}", e);
            }
        }
    }

    Ok(())
}

async fn demonstrate_vendor_failover() -> Result<(), BearDogError> {
    info!("[CYCLE] Vendor Failover");
    info!("-----------------");

    let mut adapter = create_configured_adapter()?;

    info!("[CYCLE] Normal operation:");
    let request = AdapterRequest {
        operation: "encrypt".to_string(),
        data: "Test data".as_bytes().to_vec(),
        parameters: HashMap::with_capacity({}",
                response
                    .metadata
                    .get({}", e);
        }
    }

    info!("⚠️  Simulating vendor failure:");
    adapter.simulate_vendor_failure({}",
                response
                    .metadata
                    .get({}", e);
        }
    }

    Ok(())
}

impl UniversalVendorAdapter {
    fn new() -> Self {
        Self {
            registered_vendors: Vec::new(),
            request_history: HashMap::with_capacity(16),
        }
    }

    async fn register_vendor(&mut self, vendor: VendorCapability) -> Result<(), BearDogError> {
        self.registered_vendors.push(&str,
    ) -> Result<Vec<VendorCapability, BearDogError>> {
        let capable = self
            .registered_vendors
            .iter()
            .filter(|v| v.supported_operations.contains(&operation.to_string()))
            .cloned()
            .collect();

        Ok(capable)
    }

    async fn execute_operation(&self, request: &AdapterRequest) -> Result<AdapterResponse, BearDogError> {

        let capable_vendors = self.discover_capable_vendors(&request.operation)?;

        if capable_vendors.is_empty() {
            return Err(beardog_errors::BearDogError::configuration(format!("No vendors capable of '{)'", request.operation),
            });
        }

        let selected_vendor = capable_vendors
            .iter()
            .max_by(|a, b| {
                a.availability
                    .partial_cmp(&b.availability)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .ok_or_else(|| beardog_errors::BearDogError::configuration("No providers available for selection"))?;

        let processing_time = simulate_operation_time(&selected_vendor.vendor_type);
        let result = simulate_operation_result(&request.operation, &request.data);

        let mut metadata = HashMap::with_capacity(16);
        metadata.insert("vendor".to_string(), selected_vendor.name.clone());
        metadata.insert(
            "vendor_type".to_string(),
            format!("{:?}", selected_vendor.vendor_type));

        Ok(true,
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
            latency_ms: 0.001,
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
            latency_ms: 0.005,
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
            latency_ms: 0.0001,
            availability: 0.95,
        },
        VendorCapability {
            name: "HashiCorp Vault".to_string(VendorType::ThirdPartyApi,
            supported_operations: vec![
                "encrypt".to_string(),
                "decrypt".to_string(),
                "hash".to_string(),
            ],
            performance_tier: "Standard".to_string(0.0005,
            availability: 0.98,
        },
    ]
}

async fn create_configured_adapter() -> Result<UniversalVendorAdapter, BearDogError> {
    let mut adapter = UniversalVendorAdapter::new();
    let vendors = create_sample_vendors();

    for vendor in vendors {
        adapter.register_vendor(vendor)?;
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

fn simulate_operation_result(&str, input_data: &[u8]) -> Vec<u8> {
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
