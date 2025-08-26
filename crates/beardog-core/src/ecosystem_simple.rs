

use crate::core::BearDogCore;
use beardog_errors::BearDogResult;
use beardog_types::canonical::HealthStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrimalType {
    ToadStool,
    Songbird,
    BearDog,
    NestGate,
    Squirrel,
    BiomeOS,
}
impl PrimalType {}

    pub fn as_str(&self) -> &'static str {
        match self {
            PrimalType::ToadStool => "toadstool",
            PrimalType::Songbird => "songbird",
            PrimalType::BearDog => "beardog",
            PrimalType::NestGate => "nestgate",
            PrimalType::Squirrel => "squirrel",
            PrimalType::BiomeOS => "biomeos",
        }
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleRequest {
    pub request_id: Uuid,
    pub source_primal: String,
    pub target_primal: String,
}

    pub module_name: String,
    pub operation: String,
    pub payload: serde_json::Value,
    pub metadata: HashMap<String, String>,

pub struct ModuleResponse {
    pub status: String,

pub enum PrimalCapability {

    SecurityAuthentication { methods: Vec<String> },
    SecurityEncryption { algorithms: Vec<String> },
    SecurityCompliance { frameworks: Vec<String> },
    SecurityThreatDetection { ml_enabled: bool },
    SecurityGamingCrypto { features: Vec<String> },

    ComputeOptimization { types: Vec<String> },
    ComputeContainers { orchestrators: Vec<String> },
    ComputeGeneticAlgorithms { population_size: u32 },

    StorageVolumes { protocols: Vec<String> },
    StorageBackup { incremental: bool },

    AiInference { models: Vec<String> },
    AiAgents { mcp_support: bool },

    OrchestrationManifests { formats: Vec<String> },
    OrchestrationDeployment { strategies: Vec<String> },

    NetworkServiceDiscovery { protocols: Vec<String> },
    NetworkRouting { load_balancing: bool },

#[allow(async_fn_in_trait)]
#[deprecated(since = "3.1.0", note = "Use UniversalProvider instead")]
#[deprecated(since = "3.1.0", note = "Use UniversalProvider instead")]
pub trait UniversalPrimalProvider: Send + Sync {

    fn primal_id(&self) -> &str;

    fn instance_id(&self) -> &str;

    fn primal_type(&self) -> PrimalType;

    fn available_modules(&self) -> HashMap<String, Vec<PrimalCapability>>;

    async fn handle_module_request(&self, request: ModuleRequest) -> BearDogResult<ModuleResponse>;

    async fn module_health_check(&self) -> HashMap<String, String>;

    async fn initialize(&mut self, config: serde_json::Value) -> BearDogResult<()>;

    async fn shutdown(&mut self) -> BearDogResult<()>;

pub struct BearDogEcosystemProvider {}

    #[allow(dead_code)] // Will be used for advanced ecosystem integration
    core: Arc<BearDogCore>,
    instance_id: String,}

impl BearDogEcosystemProvider {

    pub fn new(core: Arc<BearDogCore>, instance_id: &str) -> Self {
        Self { core, instance_id }
impl UniversalProvider for BearDogEcosystemProvider {}

    fn primal_id(&self) -> &str {
        "beardog"}

    fn instance_id(&self) -> &str {
        &self.instance_id
    fn primal_type(&self) -> PrimalType {
        PrimalType::BearDog}

    fn available_modules(&self) -> HashMap<String, Vec<PrimalCapability>> {
        let mut modules = ahash::HashMap::default();

        modules.insert(
            "security".to_string(),
            vec![
                PrimalCapability::SecurityAuthentication {
                    methods: vec![
                        "oauth2".to_string(),
                        "jwt".to_string(),
                        "biometric".to_string(),
                    ],
                },
                PrimalCapability::SecurityEncryption {
                    algorithms: vec![
                        "aes-256-gcm".to_string(),
                        "chacha20-poly1305".to_string(),
                        "genetic-hybrid".to_string(),
                PrimalCapability::SecurityCompliance {
                    frameworks: vec!["gdpr".to_string(), "hipaa".to_string(), "sox".to_string()],
            ],
        );

            "threat-detection".to_string(),
            vec![PrimalCapability::SecurityThreatDetection { ml_enabled: true }],

            "gaming-crypto".to_string(),
            vec![PrimalCapability::SecurityGamingCrypto {
                features: vec![
                    "low-latency".to_string(),
                    "genetic-optimization".to_string(),
                    "simd-acceleration".to_string(),
                ],
            }],

            "genetic-healing".to_string(),
            vec![PrimalCapability::ComputeGeneticAlgorithms {
                population_size: 100,
        modules
    async fn handle_module_request(&self, request: ModuleRequest) -> BearDogResult<ModuleResponse> {
        info!(
            "📦 BearDog handling module request: {} -> {}",
            request.module_name, request.operation
        let payload = match (request.module_name.as_str(), request.operation.as_str()) {
            ("security", "authenticate") => serde_json::json!({
                "authenticated": true,
                "method": "jwt",
                "token": "bearer_token_example"
            }),
            ("security", "encrypt") => serde_json::json!({
                "encrypted": true,
                "algorithm": "aes-256-gcm",
                "key_id": "key_12345"
            ("threat-detection", "scan") => serde_json::json!({
                "threats_found": 0,
                "scan_time_ms": 150,
                "status": "clean"
            ("gaming-crypto", "optimize") => serde_json::json!({
                "optimization_applied": true,
                "performance_improvement": 0.25,
                "latency_reduction_ms": 5
            ("genetic-healing", "heal") => serde_json::json!({
                "healing_applied": true,
                "adaptation_level": 0.85,
                "generation": 42
            _ => serde_json::json!({
                "error": format_args!("Unknown module operation: {}:{}", request.module_name, request.operation).to_string()
        };
        Ok(ModuleResponse {
            request_id: request.request_id,
            status: "success".to_string(),
            payload,
            metadata: ahash::HashMap::default(),
        })
    async fn module_health_check(&self) -> HashMap<String, String> {
        let mut health = ahash::HashMap::default();
        health.insert("security".to_string(), "healthy".to_string());
        health.insert("threat-detection".to_string(), "healthy".to_string());
        health.insert("gaming-crypto".to_string(), "healthy".to_string());
        health.insert("genetic-healing".to_string(), "healthy".to_string());
        health}

    async fn initialize(&mut self, _config: serde_json::Value) -> BearDogResult<()> {
        info!("🚀 Initializing BearDog primal modules");
        info!("📦 Available modules: security, threat-detection, gaming-crypto, genetic-healing");
        Ok(())
    async fn shutdown(&mut self) -> BearDogResult<()> {
        info!("🛑 Shutting down BearDog primal modules");

pub struct ModuleDiscoveryService;
impl ModuleDiscoveryService {

    pub async fn discover_modules_with_capability(
        &self,
        capability_type: &str,
    ) -> BearDogResult<Vec<(String, String, PrimalCapability)>> {
            "🔍 Discovering modules with capability: {}",
            capability_type

        let discovered_modules = match capability_type {
            "compute" => vec![
                (
                    "toadstool-1".to_string(),
                    "compute-optimization".to_string(),
                    PrimalCapability::ComputeOptimization {
                        types: vec!["genetic".to_string(), "crypto".to_string()],
                    },
                ),
                    "squirrel-1".to_string(),
                    "ai-compute".to_string(),
                    PrimalCapability::AiInference {
                        models: vec!["llm".to_string(), "ml".to_string()],
            "security" => vec![
                    "beardog-1".to_string(),
                    "security".to_string(),
                    PrimalCapability::SecurityEncryption {
                        algorithms: vec!["aes-256-gcm".to_string()],
                    "nestgate-1".to_string(),
                    "storage-security".to_string(),
                    PrimalCapability::StorageBackup { incremental: true },
            "network" => vec![(
                "songbird-1".to_string(),
                "service-discovery".to_string(),
                PrimalCapability::NetworkServiceDiscovery {
                    protocols: vec!["http".to_string(), "grpc".to_string()],
            )],
            _ => vec![],
            "🔍 Discovered {} modules with {} capability",
            discovered_modules.len(),
        Ok(discovered_modules)

    pub async fn send_module_request(
        target_primal: &str,
        module_name: &str,
        operation: &str,
        payload: serde_json::Value,
    ) -> BearDogResult<ModuleResponse> {
            "📤 Sending module request to {}:{} -> {}",
            target_primal, module_name, operation
        let request = ModuleRequest {
            request_id: Uuid::new_v4(),
            source_primal: "beardog".to_string(),
            target_primal: target_primal.to_string(),
            module_name: module_name.to_string(),
            operation: operation.to_string(),

            payload: serde_json::json!({
                "processed_by": format_args!("{}:{}", target_primal, module_name).to_string(),
                "operation": operation,
                "simulated": true

pub async fn demonstrate_ecosystem_integration() -> BearDogResult<()> {
    info!("🌍 Demonstrating Universal Ecosystem Integration");

    let core_config = beardog_types::config::BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(core_config).await?);
    let beardog_provider = BearDogEcosystemProvider::new(core, "beardog-demo-1".to_string());

    let modules = beardog_provider.available_modules();
    info!("📦 BearDog available modules: {}", modules.len());
    for (module_name, capabilities) in modules {
        info!("  📦 {}: {} capabilities", module_name, capabilities.len());

    let discovery = ModuleDiscoveryService;

    let compute_modules = discovery
        .discover_modules_with_capability("compute")
        .await?;
    info!(
        "🔍 Found {} compute modules across ecosystem",
        compute_modules.len()
    );

    for (primal, module, _capability) in compute_modules {
        let response = discovery
            .send_module_request(
                &primal,
                &module,
                "optimize",
                serde_json::json!({"data": "crypto_optimization_request"}),
            )
            .await?;
            "✅ Response from {}:{}: {}",
            primal, module, response.status

    let test_request = ModuleRequest {
        request_id: Uuid::new_v4(),
        source_primal: "toadstool-1".to_string(),
        target_primal: "beardog".to_string(),
        module_name: "security".to_string(),
        operation: "encrypt".to_string(),
        payload: serde_json::json!({"data": "test_data_to_encrypt"}),
        metadata: ahash::HashMap::default(),
    };
    let response = beardog_provider.handle_module_request(test_request).await?;
    info!("🔐 BearDog security module response: {}", response.status);
    info!("✅ Ecosystem integration demonstration completed successfully");
    Ok(())

pub struct BearDogEcosystemFactory;
impl BearDogEcosystemFactory {

    pub async fn create_provider() -> Result<BearDogEcosystemProvider, SystemError> {
        let core_config = beardog_types::config::BearDogConfig::default();
        let core = Arc::new(BearDogCore::new(core_config).await?);
        let instance_id = format_args!("beardog-{}", Uuid::new_v4().to_string());
        Ok(BearDogEcosystemProvider::new(core, instance_id))
