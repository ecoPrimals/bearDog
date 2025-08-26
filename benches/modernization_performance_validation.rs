

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct MockRequest {
    pub id: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct MockResponse {
    pub success: bool,
    pub data: Vec<u8>,
}

fn benchmark_async_patterns(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("native_async_fn_dispatch", |b| {
        b.iter(|| {
            rt.block_on(async {

                let request = MockRequest {
                    id: "test".to_string(),
                    data: vec![1, 2, 3, 4, 5],
                };

                let response = native_async_process(black_box(request)).await;
                black_box(response)
            })
        })
    });
}

async fn native_async_process(request: MockRequest) -> MockResponse {

    tokio::task::yield_now().await;
    
    MockResponse {
        success: true,
        data: request.data,
    }
}

fn benchmark_dispatch_patterns(c: &mut Criterion) {
    c.bench_function("zero_cost_dispatch", |b| {
        let processor = ZeroCostProcessor::new();
        b.iter(|| {
            let data = vec![1u8; 1000];

            let result = processor.process_data(black_box(&data));
            black_box(result)
        })
    });
    
    c.bench_function("trait_object_dispatch", |b| {
        let processor: Box<dyn DataProcessor> = Box::new(TraditionalProcessor::new());
        b.iter(|| {
            let data = vec![1u8; 1000];

            let result = processor.process_data(black_box(&data));
            black_box(result)
        })
    });
}

pub struct ZeroCostProcessor {
    config: ProcessorConfig,
}

impl ZeroCostProcessor {
    pub fn new() -> Self {
        Self {
            config: ProcessorConfig::default(),
        }
    }

    pub fn process_data(&self, data: &[u8]) -> Vec<u8> {

        let multiplier = self.config.multiplier;
        data.iter().map(|&b| b.wrapping_mul(multiplier)).collect()
    }
}

pub trait DataProcessor {
    fn process_data(&self, data: &[u8]) -> Vec<u8>;
}

pub struct TraditionalProcessor {
    config: ProcessorConfig,
}

impl TraditionalProcessor {
    pub fn new() -> Self {
        Self {
            config: ProcessorConfig::default(),
        }
    }
}

impl DataProcessor for TraditionalProcessor {
    fn process_data(&self, data: &[u8]) -> Vec<u8> {

        let multiplier = self.config.multiplier;
        data.iter().map(|&b| b.wrapping_mul(multiplier)).collect()
    }
}

#[derive(Debug, Clone)]
pub struct ProcessorConfig {
    pub multiplier: u8,
    pub buffer_size: usize,
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self {
            multiplier: 2,
            buffer_size: 4096,
        }
    }
}

fn benchmark_compilation_patterns(c: &mut Criterion) {
    c.bench_function("modular_structure_access", |b| {
        let module = ModularStructure::new();
        b.iter(|| {

            let result = module.get_component("core").unwrap();
            let processed = result.process_request("test_data");
            black_box(processed)
        })
    });
}

pub struct ModularStructure {
    components: HashMap<String, ModularComponent>,
}

impl ModularStructure {
    pub fn new() -> Self {
        let mut components = HashMap::with_capacity(16);
        components.insert("core".to_string(), ModularComponent::new("core"));
        components.insert("security".to_string(), ModularComponent::new("security"));
        components.insert("networking".to_string(), ModularComponent::new("networking"));
        
        Self { components }
    }
    
    pub fn get_component(&self, name: &str) -> Option<&ModularComponent> {
        self.components.get(name)
    }
}

pub struct ModularComponent {
    name: String,
    cache: HashMap<String, String>,
}

impl ModularComponent {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            cache: HashMap::with_capacity(16),
        }
    }
    
    pub fn process_request(&self, data: &str) -> String {
        format_args!("{}:{}", self.name, data).to_string()
    }
}

fn benchmark_modernization_impact(c: &mut Criterion) {
    c.bench_function("modernized_system_performance", |b| {
        let system = ModernizedSystem::new();
        b.iter(|| {
            let start = Instant::now();

            let request = MockRequest {
                id: format_args!("req_{}", rand::random::<u32>().to_string()),
                data: vec![rand::random::<u8>(); 100],
            };
            
            let processed = system.process_request(black_box(request));
            let duration = start.elapsed();
            
            black_box((processed, duration))
        })
    });
}

pub struct ModernizedSystem {
    processor: ZeroCostProcessor,
    modules: ModularStructure,
}

impl ModernizedSystem {
    pub fn new() -> Self {
        Self {
            processor: ZeroCostProcessor::new(),
            modules: ModularStructure::new(),
        }
    }
    
    pub fn process_request(&self, request: MockRequest) -> MockResponse {

        let processed_data = self.processor.process_data(&request.data);

        let component = self.modules.get_component("core").unwrap();
        let component_result = component.process_request(&request.id);
        
        MockResponse {
            success: !component_result.is_empty(),
            data: processed_data,
        }
    }
}

criterion_group!(
    modernization_benches,
    benchmark_async_patterns,
    benchmark_dispatch_patterns,
    benchmark_compilation_patterns,
    benchmark_modernization_impact
);

criterion_main!(modernization_benches); 