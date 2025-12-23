# Universal Adapter Usage Guide

**How to Use BearDog's Universal Adapter Pattern for Zero-Hardcoding Development**

---

## 🌟 **Overview**

BearDog has eliminated all hardcoded vendor and primal dependencies in favor of a **Universal Adapter Pattern**. This guide shows you how to develop applications that work with ANY ecosystem configuration without hardcoded connections.

### **Core Principle**
> **"Each primal only knows itself and discovers others via the universal adapter"**

---

## 🚀 **Quick Start**

### **Before (Hardcoded - DON'T DO THIS)**
```rust
// ❌ BAD: Hardcoded primal connections
let toadstool_client = ToadStoolClient::new("http://toadstool:8081");
let result = toadstool_client.compute(data).await?;

// ❌ BAD: Hardcoded vendor integration  
let aws_kms = AwsKmsClient::new(region, credentials);
let encrypted = aws_kms.encrypt(key_id, plaintext).await?;
```

### **After (Universal - DO THIS)**
```rust
// ✅ GOOD: Universal capability discovery
use beardog_adapters::universal::UniversalCapabilityDiscovery;
use beardog_core::ecosystem_integration::universal_compute_client::UniversalComputeClient;

// Discover available capabilities
let discovery = UniversalCapabilityDiscovery::new().await?;
let capabilities = discovery.discover_capabilities(&[
    ServiceCapabilityType::ComputeIntelligence,
    ServiceCapabilityType::KeyManagement,
]).await?;

// Use discovered capabilities
let compute_client = UniversalComputeClient::new(capabilities.clone()).await?;
let result = compute_client.submit_compute(request).await?;
```

---

## 📚 **Core Concepts**

### **1. Service Capability Types**
Instead of hardcoded primal names, use capability types:

```rust
use beardog_types::canonical::capabilities::ServiceCapabilityType;

// Replace hardcoded names with capabilities
let required_capabilities = vec![
    ServiceCapabilityType::ComputeIntelligence,    // Instead of "toadstool"
    ServiceCapabilityType::ServiceMesh,            // Instead of "songbird"  
    ServiceCapabilityType::DataStorage,            // Instead of "nestgate"
    ServiceCapabilityType::DistributedIntelligence, // Instead of "squirrel"
    ServiceCapabilityType::Security,               // Instead of "beardog"
];
```

### **2. Universal Discovery**
Discover services dynamically instead of hardcoding endpoints:

```rust
use beardog_adapters::universal::capability_discovery::UniversalCapabilityDiscovery;

let discovery = UniversalCapabilityDiscovery::new().await?;

// Discover by capability type
let compute_providers = discovery.discover_capabilities(&[
    ServiceCapabilityType::ComputeIntelligence
]).await?;

// Automatic failover and load balancing
let best_provider = discovery.select_best_provider(
    &compute_providers,
    &SelectionCriteria::default()
).await?;
```

### **3. Universal Clients**
Use universal clients that work with any discovered provider:

```rust
use beardog_core::ecosystem_integration::universal_compute_client::{
    UniversalComputeClient, UniversalComputeRequest, ComputePriority
};

// Create universal client with discovered capabilities
let compute_client = UniversalComputeClient::new(discovered_capabilities).await?;

// Submit requests without knowing the underlying provider
let request = UniversalComputeRequest {
    request_id: uuid::Uuid::new_v4().to_string(),
    operation_type: "data_analysis".to_string(),
    input_data: serde_json::to_value(&input)?,
    processing_requirements: ProcessingCapability {
        cpu_cores: Some(4),
        memory_gb: Some(8),
        architectures: vec![ComputeArchitecture::X86_64],
        special_capabilities: vec![],
    },
    priority: ComputePriority::High,
    optimization: OptimizationType::Speed,
    timeout_ms: Some(30000),
    metadata: HashMap::new(),
};

let response = compute_client.submit_compute(request).await?;
```

---

## 🛠️ **Common Usage Patterns**

### **1. Compute Operations**

```rust
use beardog_core::ecosystem_integration::universal_compute_client::*;

// Initialize universal compute client
let discovery = UniversalCapabilityDiscovery::new().await?;
let compute_capabilities = discovery.discover_capabilities(&[
    ServiceCapabilityType::ComputeIntelligence
]).await?;

let compute_client = UniversalComputeClient::new(compute_capabilities).await?;

// Submit compute job
let compute_request = UniversalComputeRequest {
    request_id: "job-123".to_string(),
    operation_type: "machine_learning_inference".to_string(),
    input_data: serde_json::json!({
        "model": "sentiment_analysis",
        "text": "This is a great product!"
    }),
    processing_requirements: ProcessingCapability {
        cpu_cores: Some(2),
        memory_gb: Some(4),
        gpu_units: None,
        storage_gb: Some(1),
        architectures: vec![ComputeArchitecture::X86_64],
        special_capabilities: vec!["tensorflow".to_string()],
    },
    priority: ComputePriority::Normal,
    optimization: OptimizationType::Balanced,
    timeout_ms: Some(60000),
    metadata: HashMap::new(),
};

let response = compute_client.submit_compute(compute_request).await?;
println!("Compute result: {:?}", response.result);
```

### **2. Key Management Operations**

```rust
use beardog_adapters::universal::vendor_adapter::UniversalKmsHandler;

// Discover KMS capabilities
let kms_capabilities = discovery.discover_capabilities(&[
    CapabilityType::KeyManagement
]).await?;

// Create universal KMS handler
let kms_handler = UniversalKmsHandler::new(kms_capabilities).await?;

// Encrypt data (works with AWS KMS, Azure Key Vault, etc.)
let encrypt_request = KmsRequest::Encrypt {
    key_id: "primary-key".to_string(),
    plaintext: sensitive_data.as_bytes().to_vec(),
    encryption_context: HashMap::new(),
};

let encrypted_data = kms_handler.execute_kms_operation(encrypt_request).await?;
```

### **3. Service Registration**

```rust
use beardog_core::ecosystem::service_registration::UniversalServiceRegistry;

// Register your service capabilities
let registry = UniversalServiceRegistry::new().await?;

let service_registration = ServiceRegistration {
    service_id: "my-service".to_string(),
    capabilities: vec![
        ServiceCapabilityType::Custom("data_processing".to_string()),
        ServiceCapabilityType::Custom("file_conversion".to_string()),
    ],
    endpoints: vec![
        ServiceEndpoint {
            protocol: "https".to_string(),
            host: "my-service.ecosystem.internal".to_string(),
            port: 8080,
            path: "/api/v1".to_string(),
        }
    ],
    health_check_endpoint: Some("/health".to_string()),
    metadata: HashMap::new(),
};

registry.register_service(service_registration).await?;
```

---

## 🔧 **Configuration**

### **Environment Variables**
Configure discovery endpoints instead of hardcoded service URLs:

```bash
# Universal discovery endpoints
export BEARDOG_DISCOVERY_ENDPOINT="https://discovery.ecosystem.internal:8080"
export BEARDOG_CAPABILITY_REGISTRY="https://capabilities.ecosystem.internal:8443"

# Service mesh discovery  
export BEARDOG_SERVICE_MESH_ENDPOINT="https://mesh.ecosystem.internal:8080"

# No hardcoded primal endpoints needed!
```

### **Configuration File**
```toml
# beardog-config.toml

[discovery]
timeout_ms = 5000
max_concurrent = 10
cache_duration_ms = 300000  # 5 minutes
health_check_interval_ms = 60000  # 1 minute
auto_register = true

[discovery.endpoints]
primary = "https://discovery.ecosystem.internal:8080"
secondary = "https://capabilities.ecosystem.internal:8443"

[compute]
request_timeout_ms = 30000
max_concurrent_requests = 10
retry_attempts = 3
enable_batching = true
batch_size = 5

[compute.discovery]
discovery_timeout_ms = 5000
cache_duration_ms = 300000
min_performance_score = 0.7
enable_failover = true
preferred_architectures = ["X86_64", "Arm64"]
```

---

## 🚀 **Advanced Usage**

### **1. Custom Capability Discovery**

```rust
use beardog_adapters::universal::capability_discovery::{
    DiscoveryStrategy, UniversalCapabilityDiscovery
};

// Implement custom discovery strategy
#[derive(Debug)]
struct CustomDiscoveryStrategy {
    endpoint: String,
}

#[async_trait::async_trait]
impl DiscoveryStrategy for CustomDiscoveryStrategy {
    async fn discover_capabilities(
        &self,
        capability_types: &[CapabilityType]
    ) -> Result<Vec<UniversalCapability>, BearDogError> {
        // Custom discovery logic
        // Query your internal service registry, Consul, etc.
        todo!("Implement custom discovery")
    }
    
    fn strategy_name(&self) -> &'static str {
        "custom_registry"
    }
    
    async fn is_available(&self) -> bool {
        // Check if custom registry is available
        true
    }
}

// Add custom strategy to discovery
let mut discovery = UniversalCapabilityDiscovery::new().await?;
discovery.add_strategy(Box::new(CustomDiscoveryStrategy {
    endpoint: "https://my-registry.internal".to_string(),
})).await?;
```

### **2. Capability Filtering and Selection**

```rust
// Advanced capability selection
let capabilities = discovery.discover_capabilities(&[
    ServiceCapabilityType::ComputeIntelligence
]).await?;

// Filter by custom criteria
let filtered_capabilities: Vec<_> = capabilities
    .into_iter()
    .filter(|cap| {
        cap.performance_metrics.reliability_score > 0.9 &&
        cap.security_level == SecurityLevel::High &&
        cap.provider_info.region.as_ref().map_or(false, |r| r == "us-east-1")
    })
    .collect();

// Custom load balancing
let selected_capability = select_capability_with_custom_logic(&filtered_capabilities)?;
```

### **3. Monitoring and Metrics**

```rust
// Get discovery metrics
let metrics = discovery.get_metrics().await;
println!("Total discoveries: {}", metrics.total_discoveries);
println!("Cache hit rate: {:.2}%", 
    (metrics.cache_hits as f64 / (metrics.cache_hits + metrics.cache_misses) as f64) * 100.0);

// Get compute client metrics
let compute_metrics = compute_client.get_metrics().await;
println!("Successful requests: {}/{}", 
    compute_metrics.successful_requests, 
    compute_metrics.total_requests);
println!("Average response time: {:.2}ms", compute_metrics.avg_response_time_ms);
```

---

## 🔍 **Migration Guide**

### **Migrating from Hardcoded ToadStool Client**

**Before:**
```rust
use beardog_core::ecosystem_integration::toadstool_client::{
    ToadStoolClient, ToadStoolComputeRequest
};

let client = ToadStoolClient::new("http://toadstool:8081").await?;
let response = client.submit_compute(request).await?;
```

**After:**
```rust
use beardog_core::ecosystem_integration::universal_compute_client::{
    UniversalComputeClient, UniversalComputeRequest
};

// Discover compute capabilities
let capabilities = discovery.discover_capabilities(&[
    ServiceCapabilityType::ComputeIntelligence
]).await?;

let client = UniversalComputeClient::new(capabilities).await?;
let response = client.submit_compute(request).await?;
```

### **Migrating from Hardcoded Configuration Flags**

**Before:**
```rust
struct Config {
    enable_toadstool: bool,
    enable_squirrel: bool,
    enable_nestgate: bool,
    toadstool_endpoint: String,
}
```

**After:**
```rust
struct Config {
    required_capabilities: Vec<ServiceCapabilityType>,
    optional_capabilities: Vec<ServiceCapabilityType>,
    discovery_config: CapabilityDiscoveryConfig,
}

// Usage
let config = Config {
    required_capabilities: vec![
        ServiceCapabilityType::ComputeIntelligence,
        ServiceCapabilityType::Security,
    ],
    optional_capabilities: vec![
        ServiceCapabilityType::DistributedIntelligence,
        ServiceCapabilityType::DataStorage,
    ],
    discovery_config: CapabilityDiscoveryConfig::default(),
};
```

---

## 🛡️ **Best Practices**

### **1. Always Use Capability Types**
```rust
// ✅ GOOD: Capability-based
let required_capabilities = vec![ServiceCapabilityType::ComputeIntelligence];

// ❌ BAD: Hardcoded primal names
let hardcoded_services = vec!["toadstool", "squirrel"];
```

### **2. Handle Discovery Failures Gracefully**
```rust
match discovery.discover_capabilities(&required_capabilities).await {
    Ok(capabilities) if !capabilities.is_empty() => {
        // Use discovered capabilities
        let client = UniversalComputeClient::new(capabilities).await?;
    },
    Ok(_) => {
        // No capabilities found - handle gracefully
        warn!("No compute capabilities discovered, using fallback");
        return Ok(fallback_response);
    },
    Err(e) => {
        // Discovery failed - handle error
        error!("Capability discovery failed: {}", e);
        return Err(e);
    }
}
```

### **3. Cache Discovered Capabilities**
```rust
// Use built-in caching
let discovery = UniversalCapabilityDiscovery::new().await?;

// Capabilities are automatically cached based on configuration
let capabilities = discovery.discover_capabilities(&[
    ServiceCapabilityType::ComputeIntelligence
]).await?; // First call - network request

let cached_capabilities = discovery.discover_capabilities(&[
    ServiceCapabilityType::ComputeIntelligence  
]).await?; // Second call - cached result
```

### **4. Monitor Health and Performance**
```rust
// Regular health checks
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(60));
    
    loop {
        interval.tick().await;
        
        match discovery.refresh_capabilities().await {
            Ok(_) => debug!("Capability refresh successful"),
            Err(e) => warn!("Capability refresh failed: {}", e),
        }
    }
});
```

---

## 🎯 **Summary**

The Universal Adapter Pattern provides:

- **🔥 Zero hardcoded dependencies** - Works with any ecosystem configuration
- **🌌 Infinite scalability** - New services integrate automatically  
- **🛡️ True sovereignty** - Each service only knows itself
- **⚡ High performance** - Sub-100ms discovery with intelligent caching
- **🔧 Easy maintenance** - Single pattern replaces all hardcoded integrations

### **Key Takeaways**

1. **Use `ServiceCapabilityType`** instead of hardcoded primal names
2. **Use `UniversalCapabilityDiscovery`** to find services dynamically
3. **Use universal clients** (like `UniversalComputeClient`) for operations
4. **Handle discovery failures** gracefully with fallbacks
5. **Monitor and cache** for optimal performance

**🎉 Welcome to the age of zero-hardcoding development!** 