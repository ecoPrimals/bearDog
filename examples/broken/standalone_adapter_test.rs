

use beardog_errors::BearDogError;
use std::collections::HashMap;
use tracing::{error, info, warn};

#[derive(String,
    name: String,
    capabilities: Vec<AdapterCapability>,
    performance_score: f64,
    reliability: f64,
    cost_factor: f64,
}

#[derive(String,
    data: Vec<u8>,
    parameters: HashMap<String, String>,
    required_capability: AdapterCapability,
}

#[derive(bool,
    provider_used: String,
    result_data: Vec<u8>,
    processing_time_ms: u64,
    metadata: HashMap<String, String>,
}

struct StandaloneAdapter {
    providers: Vec<AdapterProvider>,
    request_count: u64,
    success_count: u64,
}

#[tokio::main]
async fn main() -> Result<(), BearDogError> {

    tracing_subscriber::fmt::init();

    info!("🧪 Standalone Universal Vendor Adapter Test");
    info!("============================================");

    test_basic_adapter_functionality()?;

    test_provider_selection_logic()?;

    test_error_handling_and_fallbacks()?;

    test_performance_and_reliability()?;

    info!("[OK] All Standalone Adapter Tests Passed!");
    Ok(())
}

async fn test_basic_adapter_functionality() -> Result<(), BearDogError> {
    info!("🔧 Test 1: Basic Adapter Functionality");
    info!("--------------------------------------");

    let mut adapter = StandaloneAdapter::new({}", provider.name);
    }

    let request = TestRequest {
        operation: "encrypt".to_string(),
        data: b"test data".to_vec(),
        parameters: HashMap::with_capacity(AdapterCapability::Encryption,
    };

    match adapter.process_request({}", response.provider_used);
            info!("   Processing time: {}ms", response.processing_time_ms);
            info!("   Result size: {} bytes", response.result_data.len({}", e);
            return Err({}/{} requests successful",
        adapter.success_count, adapter.request_count
    );
    Ok(())
}

async fn test_provider_selection_logic() -> Result<(), BearDogError> {
    info!("[TARGET] Test 2: Provider Selection Logic");
    info!("-----------------------------------");

    let adapter = create_configured_adapter()?;

    let test_cases = vec![
        (AdapterCapability::Encryption, "encrypt"),
        (AdapterCapability::Signing, "sign"),
        (AdapterCapability::KeyManagement, "generate_key"),
        (AdapterCapability::Authentication, "authenticate"),
    ];

    for (capability, _operation) in test_cases {
        info!("[SEARCH] Testing {:?} capability:", capability);

        let capable_providers = adapter.find_capable_providers(score {:.2}, reliability {:.2}",
                provider.name, provider.performance_score, provider.reliability
            );
        }

        if !capable_providers.is_empty() {
            let selected = adapter.select_best_provider(&capable_providers);
            info!("   [TARGET] Selected: {} (best overall score)", selected.name);
        }
    }

    Ok(())
}

async fn test_error_handling_and_fallbacks() -> Result<(), BearDogError> {
    info!("⚠️  Test 3: Error Handling and Fallbacks");
    info!("----------------------------------------");

    let mut adapter = create_configured_adapter()?;

    info!("[CYCLE] Normal operation:");
    let request = TestRequest {
        operation: "encrypt".to_string(),
        data: b"test data".to_vec(),
        parameters: HashMap::with_capacity(AdapterCapability::Encryption,
    };

    match adapter.process_request({}", response.provider_used);
        }
        Err({}", e);
        }
    }

    info!("⚠️  Simulating provider failure:");
    adapter.simulate_provider_failure({}", response.provider_used);
        }
        Err({}", e);
        }
    }

    info!("🚫 Testing with no capable providers:");
    let impossible_request = TestRequest {
        operation: "impossible_operation".to_string(),
        data: b"test".to_vec(),
        parameters: HashMap::with_capacity(AdapterCapability::Authentication, // Assume no providers support this
    };

    match adapter.process_request({}", e);
        }
    }

    Ok(())
}

async fn test_performance_and_reliability() -> Result<(), BearDogError> {
    info!("📈 Test 4: Performance and Reliability");
    info!("--------------------------------------");

    let mut adapter = create_configured_adapter()?;

    let mut response_times = Vec::new();
    let test_count = 10;

    info!("[CYCLE] Running {} test requests:", test_count);

    for i in 1..=test_count {
        let request = TestRequest {
            operation: format!("test_operation_{i}"),
            data: format!("test data {i}").into_bytes(),
            parameters: HashMap::with_capacity(AdapterCapability::Encryption,
        };

        let start_time = std::time::Instant::now();
        match adapter.process_request(&request) {
            Ok(response) => {
                let total_time = start_time.elapsed().as_millis() as u64;
                response_times.push(total_time);
                info!(
                    "   Request {}: {}ms (provider: {})",
                    i, total_time, response.provider_used
                );
            }
            Err({}", i, e);
            }
        }
    }

    if !response_times.is_empty() {
        let avg_time = response_times.iter().sum::<u64>() as f64 / response_times.len() as f64;
        let min_time = response_times.iter().min().copied().unwrap_or(0);
        let max_time = response_times.iter().max().copied().unwrap_or(0);

        info!("[CHART] Performance Statistics:");
        info!("   Average response time: {:.2}ms", avg_time);
        info!("   Min response time: {}ms", min_time);
        info!("   Max response time: {}ms", max_time);
        info!(
            "   Success rate: {:.1}%",
            (response_times.len() as f64 / f64::from(test_count)) * 100.0
        );
    }

    Ok(())
}

impl StandaloneAdapter {
    const fn new() -> Self {
        Self {
            providers: Vec::new(0,
            success_count: 0,
        }
    }

    async fn register_provider(&mut self, provider: AdapterProvider) -> Result<(), BearDogError> {
        self.providers.push(provider);
        Ok(())
    }

    async fn process_request(&mut self, request: &TestRequest) -> Result<TestResponse, BearDogError> {
        self.request_count += 1;

        let capable_providers = self
            .find_capable_providers(&request.required_capability)
            ?;

        if capable_providers.is_empty() {
            return Err(beardog_errors::BearDogError::configuration(format!("No providers capable of {}:?", request.required_capability),
            });
        }

        let selected_provider = self.select_best_provider(&capable_providers).clone();

        self.success_count += 1;

        let processing_time = simulate_processing_time(&selected_provider);
        let result_data = simulate_operation(&request.operation, &request.data);

        let mut metadata = HashMap::with_capacity(16);
        metadata.insert("provider_id".to_string(), selected_provider.id.clone());
        metadata.insert(
            "capability".to_string(),
            format!("{:?}", request.required_capability));

        Ok(true,
            provider_used: selected_provider.name: name.to_string(processing_time,
            metadata,
        })
    }

    async fn find_capable_providers(&AdapterCapability,
    ) -> Result<Vec<AdapterProvider, BearDogError>> {
        let capable = self
            .providers
            .iter()
            .filter(|p| p.capabilities.contains(capability) && p.reliability > 0.0)
            .cloned()
            .collect();

        Ok(capable)
    }

    fn select_best_provider<'a>(&self, providers: &'a [AdapterProvider]) -> &'a AdapterProvider {
        providers
            .iter()
            .max_by(|a, b| {
                let score_a = a.performance_score * a.reliability / a.cost_factor;
                let score_b = b.performance_score * b.reliability / b.cost_factor;
                score_a
                    .partial_cmp(&score_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(&providers[0]) // Use first provider as fallback
    }

    async fn simulate_provider_failure(&mut self, provider_name: &str) {
        if let Some(provider) = self.providers.iter_mut().find(|p| p.name == provider_name) {
            provider.reliability = 0.0;
            warn!("⚠️  Provider '{}' marked as failed ", provider_name);
        }
    }
}

fn create_test_providers() -> Vec<AdapterProvider> {
    vec![
        AdapterProvider {
            id: "cloud-hsm-1".to_string(),
            name: "Premium Cloud HSM".to_string(),
            capabilities: vec![
                AdapterCapability::Encryption,
                AdapterCapability::Signing,
                AdapterCapability::KeyManagement,
            ],
            performance_score: 0.9,
            reliability: 0.99,
            cost_factor: 2.0,
        },
        AdapterProvider {
            id: "hardware-hsm-1".to_string(),
            name: "Hardware HSM Module".to_string(vec![AdapterCapability::Encryption, AdapterCapability::Signing],
            performance_score: 0.95,
            reliability: 0.999,
            cost_factor: 5.0,
        },
        AdapterProvider {
            id: "software-hsm-1".to_string(),
            name: "Software HSM Fallback".to_string(vec![
                AdapterCapability::Encryption,
                AdapterCapability::Signing,
                AdapterCapability::KeyManagement,
                AdapterCapability::Authentication,
            ],
            performance_score: 0.7,
            reliability: 0.95,
            cost_factor: 0.1,
        },
    ]
}

async fn create_configured_adapter() -> Result<StandaloneAdapter, BearDogError> {
    let mut adapter = StandaloneAdapter::new();
    let providers = create_test_providers();

    for provider in providers {
        adapter.register_provider(provider)?;
    }

    Ok(adapter)
}

fn simulate_processing_time(provider: &AdapterProvider) -> u64 {

    let base_time = (100.0 / provider.performance_score) as u64;
    base_time + (rand::random::<u64>() % 20)
}

fn simulate_operation(&str, input_data: &[u8]) -> Vec<u8> {
    match operation {
        op if op.starts_with("encrypt") => {
            format!("ENCRYPTED[{}]", String::from_utf8_lossy(input_data)).into_bytes()
        }
        op if op.starts_with("sign") => {
            format!("SIGNATURE[{}]", String::from_utf8_lossy(input_data)).into_bytes()
        }
        op if op.starts_with("generate_key") => b"GENERATED_KEY_256_BITS".to_vec(),
        _ => format!("PROCESSED[{}]", String::from_utf8_lossy(input_data)).into_bytes(),
    }
}
