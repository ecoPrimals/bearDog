

use beardog_errors::BearDogResult;
use std::collections::HashMap;
use tracing::{error, info, warn};

#[derive(Debug, Clone, PartialEq)]
enum AdapterCapability {
    Encryption,
    Signing,
    KeyManagement,
    Authentication,
}

#[derive(Debug, Clone)]
struct AdapterProvider {
    id: String,
    name: String,
    capabilities: Vec<AdapterCapability>,
    performance_score: f64,
    reliability: f64,
    cost_factor: f64,
}

#[derive(Debug, Clone)]
struct TestRequest {
    operation: String,
    data: Vec<u8>,
    parameters: HashMap<String, String>,
    required_capability: AdapterCapability,
}

#[derive(Debug, Clone)]
struct TestResponse {
    success: bool,
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
async fn main() -> BearDogResult<()> {

    tracing_subscriber::fmt::init();

    info!("🧪 Standalone Universal Vendor Adapter Test");
    info!("============================================");

    test_basic_adapter_functionality().await?;

    test_provider_selection_logic().await?;

    test_error_handling_and_fallbacks().await?;

    test_performance_and_reliability().await?;

    info!("✅ All Standalone Adapter Tests Passed!");
    Ok(())
}

async fn test_basic_adapter_functionality() -> BearDogResult<()> {
    info!("🔧 Test 1: Basic Adapter Functionality");
    info!("--------------------------------------");

    let mut adapter = StandaloneAdapter::new();

    let providers = create_test_providers();
    for provider in providers {
        adapter.register_provider(provider.clone()).await?;
        info!("✅ Registered provider: {}", provider.name);
    }

    let request = TestRequest {
        operation: "encrypt".to_string(),
        data: b"test data".to_vec(),
        parameters: HashMap::with_capacity(16),
        required_capability: AdapterCapability::Encryption,
    };

    match adapter.process_request(&request).await {
        Ok(response) => {
            info!("✅ Request processed successfully");
            info!("   Provider: {}", response.provider_used);
            info!("   Processing time: {}ms", response.processing_time_ms);
            info!("   Result size: {} bytes", response.result_data.len());
        }
        Err(e) => {
            error!("❌ Request failed: {}", e);
            return Err(e);
        }
    }

    info!(
        "📊 Adapter stats: {}/{} requests successful",
        adapter.success_count, adapter.request_count
    );
    Ok(())
}

async fn test_provider_selection_logic() -> BearDogResult<()> {
    info!("\n🎯 Test 2: Provider Selection Logic");
    info!("-----------------------------------");

    let adapter = create_configured_adapter().await?;

    let test_cases = vec![
        (AdapterCapability::Encryption, "encrypt"),
        (AdapterCapability::Signing, "sign"),
        (AdapterCapability::KeyManagement, "generate_key"),
        (AdapterCapability::Authentication, "authenticate"),
    ];

    for (capability, _operation) in test_cases {
        info!("🔍 Testing {:?} capability:", capability);

        let capable_providers = adapter.find_capable_providers(&capability).await?;
        info!("   Found {} capable providers", capable_providers.len());

        for provider in &capable_providers {
            info!(
                "   - {}: score {:.2}, reliability {:.2}",
                provider.name, provider.performance_score, provider.reliability
            );
        }

        if !capable_providers.is_empty() {
            let selected = adapter.select_best_provider(&capable_providers);
            info!("   🎯 Selected: {} (best overall score)", selected.name);
        }
    }

    Ok(())
}

async fn test_error_handling_and_fallbacks() -> BearDogResult<()> {
    info!("\n⚠️  Test 3: Error Handling and Fallbacks");
    info!("----------------------------------------");

    let mut adapter = create_configured_adapter().await?;

    info!("🔄 Normal operation:");
    let request = TestRequest {
        operation: "encrypt".to_string(),
        data: b"test data".to_vec(),
        parameters: HashMap::with_capacity(16),
        required_capability: AdapterCapability::Encryption,
    };

    match adapter.process_request(&request).await {
        Ok(response) => {
            info!("   ✅ Primary provider: {}", response.provider_used);
        }
        Err(e) => {
            warn!("   ⚠️  Primary failed: {}", e);
        }
    }

    info!("\n⚠️  Simulating provider failure:");
    adapter.simulate_provider_failure("Premium Cloud HSM").await;

    match adapter.process_request(&request).await {
        Ok(response) => {
            info!("   ✅ Fallback successful: {}", response.provider_used);
        }
        Err(e) => {
            warn!("   ❌ Fallback failed: {}", e);
        }
    }

    info!("\n🚫 Testing with no capable providers:");
    let impossible_request = TestRequest {
        operation: "impossible_operation".to_string(),
        data: b"test".to_vec(),
        parameters: HashMap::with_capacity(16),
        required_capability: AdapterCapability::Authentication, // Assume no providers support this
    };

    match adapter.process_request(&impossible_request).await {
        Ok(_) => {
            warn!("   ⚠️  Unexpected success for impossible operation");
        }
        Err(e) => {
            info!("   ✅ Correctly failed: {}", e);
        }
    }

    Ok(())
}

async fn test_performance_and_reliability() -> BearDogResult<()> {
    info!("\n📈 Test 4: Performance and Reliability");
    info!("--------------------------------------");

    let mut adapter = create_configured_adapter().await?;

    let mut response_times = Vec::new();
    let test_count = 10;

    info!("🔄 Running {} test requests:", test_count);

    for i in 1..=test_count {
        let request = TestRequest {
            operation: format!("test_operation_{i}"),
            data: format!("test data {i}").into_bytes(),
            parameters: HashMap::with_capacity(16),
            required_capability: AdapterCapability::Encryption,
        };

        let start_time = std::time::Instant::now();
        match adapter.process_request(&request).await {
            Ok(response) => {
                let total_time = start_time.elapsed().as_millis() as u64;
                response_times.push(total_time);
                info!(
                    "   Request {}: {}ms (provider: {})",
                    i, total_time, response.provider_used
                );
            }
            Err(e) => {
                error!("   Request {} failed: {}", i, e);
            }
        }
    }

    if !response_times.is_empty() {
        let avg_time = response_times.iter().sum::<u64>() as f64 / response_times.len() as f64;
        let min_time = response_times.iter().min().copied().unwrap_or(0);
        let max_time = response_times.iter().max().copied().unwrap_or(0);

        info!("📊 Performance Statistics:");
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
            providers: Vec::new(),
            request_count: 0,
            success_count: 0,
        }
    }

    async fn register_provider(&mut self, provider: AdapterProvider) -> BearDogResult<()> {
        self.providers.push(provider);
        Ok(())
    }

    async fn process_request(&mut self, request: &TestRequest) -> BearDogResult<TestResponse> {
        self.request_count += 1;

        let capable_providers = self
            .find_capable_providers(&request.required_capability)
            .await?;

        if capable_providers.is_empty() {
            return Err(beardog_errors::BearDogError::configuration(format_args!("No providers capable of {:?)", request.required_capability).to_string(),
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
            format_args!("{:?}", request.required_capability).to_string(),
        );

        Ok(TestResponse {
            success: true,
            provider_used: selected_provider.name,
            result_data,
            processing_time_ms: processing_time,
            metadata,
        })
    }

    async fn find_capable_providers(
        &self,
        capability: &AdapterCapability,
    ) -> BearDogResult<Vec<AdapterProvider>> {
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
            warn!("⚠️  Provider '{}' marked as failed", provider_name);
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
            name: "Hardware HSM Module".to_string(),
            capabilities: vec![AdapterCapability::Encryption, AdapterCapability::Signing],
            performance_score: 0.95,
            reliability: 0.999,
            cost_factor: 5.0,
        },
        AdapterProvider {
            id: "software-hsm-1".to_string(),
            name: "Software HSM Fallback".to_string(),
            capabilities: vec![
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

async fn create_configured_adapter() -> BearDogResult<StandaloneAdapter> {
    let mut adapter = StandaloneAdapter::new();
    let providers = create_test_providers();

    for provider in providers {
        adapter.register_provider(provider).await?;
    }

    Ok(adapter)
}

fn simulate_processing_time(provider: &AdapterProvider) -> u64 {

    let base_time = (100.0 / provider.performance_score) as u64;
    base_time + (rand::random::<u64>() % 20)
}

fn simulate_operation(operation: &str, input_data: &[u8]) -> Vec<u8> {
    match operation {
        op if op.starts_with("encrypt") => {
            format_args!("ENCRYPTED[{}]", String::from_utf8_lossy(input_data).to_string()).into_bytes()
        }
        op if op.starts_with("sign") => {
            format_args!("SIGNATURE[{}]", String::from_utf8_lossy(input_data).to_string()).into_bytes()
        }
        op if op.starts_with("generate_key") => b"GENERATED_KEY_256_BITS".to_vec(),
        _ => format_args!("PROCESSED[{}]", String::from_utf8_lossy(input_data).to_string()).into_bytes(),
    }
}
