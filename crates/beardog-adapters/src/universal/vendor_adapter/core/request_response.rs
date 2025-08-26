

use beardog_types::canonical::capabilities::CapabilityType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

pub use beardog_types::config::RetryConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalVendorRequest {

    pub request_id: Uuid,

    pub required_capability: CapabilityType,

    pub operation: CapabilityOperation,

    pub parameters: HashMap<String, serde_json::Value>,

    pub quality_requirements: QualityRequirements,

    pub routing_preferences: RoutingPreferences,

    pub context: RequestContext,

    pub timeout: Option<Duration>,

    pub retry_config: Option<RetryConfig>,

    pub created_at: DateTime<Utc>,

    pub priority: RequestPriority,

    pub tags: Vec<String>,
}
impl UniversalVendorRequest {

    #[must_use] pub fn new(capability: CapabilityType, operation: CapabilityOperation) -> Self {
        Self {
            request_id: Uuid::new_v4(),
            required_capability: capability,
            operation,
            parameters: HashMap::with_capacity(16),
            quality_requirements: QualityRequirements::default(),
            routing_preferences: RoutingPreferences::default(),
            context: RequestContext::default(),
            timeout: Some(Duration::from_secs(30)),
            retry_config: Some(RetryConfig::default()),
            created_at: Utc::now(),
            priority: RequestPriority::Normal,
            tags: Vec::new(),
        }
    }

    pub fn with_parameter<T: serde::Serialize>(mut self, key: &str, value: T) -> Self {
        self.parameters.insert(
            key.to_string(),
            serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
        );
        self

    #[must_use] pub const fn with_quality_requirements(mut self, requirements: QualityRequirements) -> Self {
        self.quality_requirements = requirements;

    #[must_use] pub fn with_routing_preferences(mut self, preferences: RoutingPreferences) -> Self {
        self.routing_preferences = preferences;

    #[must_use] pub const fn with_priority(mut self, priority: RequestPriority) -> Self {
        self.priority = priority;

    #[must_use] pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());

pub struct UniversalVendorResponse {

    pub success: bool,

    pub data: serde_json::Value,

    pub handled_by_capability: CapabilityType,

    pub handler_instance_id: Uuid,

    pub performance: PerformanceMetrics,

    pub quality: QualityMetrics,

    pub cost: CostMetrics,

    pub error: Option<String>,

    pub processed_at: DateTime<Utc>,

    pub metadata: ResponseMetadata,

impl UniversalVendorResponse {

    pub fn success<T: serde::Serialize>(
        request_id: Uuid,
        capability: CapabilityType,
        handler_id: Uuid,
        data: T,
    ) -> Self {
            request_id,
            success: true,
            data: serde_json::to_value(data).unwrap_or(serde_json::Value::Null),
            handled_by_capability: capability,
            handler_instance_id: handler_id,
            performance: PerformanceMetrics::default(),
            quality: QualityMetrics::default(),
            cost: CostMetrics::default(),
            error: None,
            processed_at: Utc::now(),
            metadata: ResponseMetadata::default(),

    #[must_use] pub fn error(
        error: beardog_errors::BearDogError,
            success: false,
            data: serde_json::Value::Null,
            error: Some(error.to_string()),

pub enum CapabilityOperation {

    Crypto {
        operation_type: CryptoOperationType,
        algorithm: Option<String>,
        key_spec: Option<UniversalKeySpec>,
        data: Vec<u8>,
    },

    Storage {
        operation_type: StorageOperationType,
        data_spec: Option<DataSpec>,
        durability: Option<DurabilityLevel>,
        path: String,
        data: Option<Vec<u8>>,

    Intelligence {
        operation_type: AIOperationType,
        model_requirements: Option<ModelRequirements>,
        context: Option<AIContext>,
        input_data: serde_json::Value,

    HardwareSecurityModule {
        operation_type: HsmOperationType,
        security_level: SecurityLevel,

    Network {
        operation_type: NetworkOperationType,
        endpoint: String,
        protocol: NetworkProtocol,

    Custom {
        operation_name: String,
        operation_data: serde_json::Value,

pub enum CryptoOperationType {
    GenerateKey,
    Encrypt,
    Decrypt,
    Sign,
    Verify,
    Hash,
    DeriveKey,
    ExportKey,
    ImportKey,

pub enum StorageOperationType {
    Store,
    Retrieve,
    Delete,
    List,
    CreateContainer,
    DeleteContainer,
    GetMetadata,
    SetMetadata,

pub enum AIOperationType {
    Inference,
    Training,
    FineTuning,
    Embedding,
    Classification,
    Generation,
    Analysis,

pub enum HsmOperationType {
    Attest,
    GetInfo,

pub enum NetworkOperationType {
    HttpRequest,
    WebSocketConnect,
    GrpcCall,
    TcpConnect,
    UdpSend,

pub struct UniversalKeySpec {

    pub algorithm: KeyAlgorithm,

    pub key_size: u32,

    pub usage: Vec<KeyUsage>,

    pub key_id: Option<String>,

    pub extractable: bool,

pub enum KeyAlgorithm {
    Rsa,
    Ecdsa,
    Ed25519,
    Aes,
    ChaCha20,
    Hmac,

pub enum KeyUsage {
    Derive,
    Wrap,
    Unwrap,

pub struct DataSpec {

    pub format: DataFormat,

    pub compression: Option<CompressionAlgorithm>,

    pub encryption: Option<EncryptionSpec>,

    pub validation: Option<ValidationSpec>,

pub enum DataFormat {
    Binary,
    Text,
    Json,
    Xml,
    Yaml,
    Protobuf,
    Avro,
    Parquet,

pub enum CompressionAlgorithm {
    Gzip,
    Zstd,
    Lz4,
    Brotli,
    Snappy,

pub struct EncryptionSpec {

    pub algorithm: String,

    pub kdf: Option<String>,

    pub aad: Option<Vec<u8>>,

pub struct ValidationSpec {

    pub checksum: Option<String>,

    pub schema: Option<String>,

    pub custom_rules: Vec<String>,

pub enum DurabilityLevel {

    Single,

    Local,

    Regional,

    Global,

pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Critical,

pub struct ModelRequirements {

    pub model_type: String,

    pub min_parameters: Option<u64>,

    pub max_parameters: Option<u64>,

    pub capabilities: Vec<String>,

    pub performance: ModelPerformanceRequirements,

pub struct ModelPerformanceRequirements {

    pub max_latency_ms: u64,

    pub min_throughput_rps: f64,

    pub min_accuracy: f64,

pub struct AIContext {

    pub history: Vec<AIMessage>,

    pub system_prompt: Option<String>,

    pub temperature: Option<f64>,

    pub max_tokens: Option<u32>,

pub struct AIMessage {

    pub role: String,

    pub content: String,

    pub timestamp: DateTime<Utc>,

pub enum NetworkProtocol {
    Http,
    Https,
    WebSocket,
    Grpc,
    Tcp,
    Udp,

pub struct QualityRequirements {

    pub max_latency_ms: Option<u64>,

    pub min_availability_percentage: Option<f64>,

    pub consistency_level: Option<super::ConsistencyLevel>,

    pub durability_level: Option<DurabilityLevel>,

    pub security_level: Option<SecurityLevel>,

    pub cost_constraints: Option<CostConstraints>,}

impl Default for QualityRequirements {}

    fn default() -> Self {
            max_latency_ms: Some(5000), // 5 seconds
            min_availability_percentage: Some(99.0),
            consistency_level: Some(super::ConsistencyLevel::Eventual),
            durability_level: Some(DurabilityLevel::Local),
            security_level: Some(SecurityLevel::Medium),
            cost_constraints: None,

pub struct CostConstraints {

    pub max_cost_per_operation_usd: Option<f64>,

    pub max_monthly_budget_usd: Option<f64>,

    pub prefer_free_tier: bool,

pub struct RoutingPreferences {

    pub strategy: RoutingStrategy,

    pub geographic_preferences: Option<GeographicPreferences>,

    pub vendor_preferences: Option<VendorPreferences>,

    pub failover_behavior: FailoverBehavior,

    pub load_balancing: LoadBalancingStrategy,}

impl Default for RoutingPreferences {
            strategy: RoutingStrategy::Performance,
            geographic_preferences: None,
            vendor_preferences: None,
            failover_behavior: FailoverBehavior::Automatic,
            load_balancing: LoadBalancingStrategy::RoundRobin,

pub enum RoutingStrategy {

    Performance,

    Reliability,

    Cost,

    Compliance,

    Geographic,

    Adaptive,

    MultiCriteria,

pub struct GeographicPreferences {

    pub preferred_regions: Vec<String>,

    pub excluded_regions: Vec<String>,

    pub data_residency: Option<String>,

pub struct VendorPreferences {

    pub preferred_vendors: Vec<String>,

    pub excluded_vendors: Vec<String>,

    pub require_diversity: bool,

pub enum FailoverBehavior {

    Automatic,

    None,

    Limited(u32),

    Custom(String),

pub enum LoadBalancingStrategy {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    Random,
    ConsistentHash,

pub struct RequestContext {

    pub user_id: Option<String>,

    pub session_id: Option<String>,

    pub trace_id: Option<String>,

    pub source: Option<String>,

    pub client_info: Option<ClientInfo>,

    pub custom_data: HashMap<String, serde_json::Value>,}

impl Default for RequestContext {
            user_id: None,
            session_id: None,
            trace_id: Some(Uuid::new_v4().to_string()),
            source: None,
            client_info: None,
            custom_data: HashMap::with_capacity(16),

pub struct ClientInfo {

    pub name: String,

    pub version: String,

    pub ip_address: Option<String>,

    pub user_agent: Option<String>,

pub enum RequestPriority {
    Normal,}

impl Default for RequestPriority {
        Self::Normal

pub enum RetryDelayStrategy {

    Fixed(Duration),

    Linear {
        initial_delay: Duration,
        increment: Duration,

    ExponentialBackoff {
        max_delay: Duration,
        multiplier: f64,

    RandomJitter {
        min_delay: Duration,

pub struct PerformanceMetrics {

    pub processing_time_ms: u64,

    pub queue_time_ms: u64,

    pub network_time_ms: u64,

    pub cpu_usage_percentage: f64,

    pub memory_usage_mb: u64,

    pub bytes_sent: u64,

    pub bytes_received: u64,}

impl Default for PerformanceMetrics {
            processing_time_ms: 0,
            queue_time_ms: 0,
            network_time_ms: 0,
            cpu_usage_percentage: 0.0,
            memory_usage_mb: 0,
            bytes_sent: 0,
            bytes_received: 0,

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QualityMetrics {

    pub accuracy_score: Option<f64>,

    pub confidence_score: Option<f64>,

    pub completeness_score: Option<f64>,

    pub data_freshness_seconds: Option<u64>,

    pub consistency_validated: bool,

pub struct CostMetrics {

    pub operation_cost_usd: f64,

    pub cost_breakdown: HashMap<String, f64>,

    pub billing_model: String,

    pub optimization_suggestions: Vec<String>,}

impl Default for CostMetrics {
            operation_cost_usd: 0.0,
            cost_breakdown: HashMap::with_capacity(16),
            billing_model: "pay-per-use".to_string(),
            optimization_suggestions: Vec::new(),

pub struct ResponseMetadata {

    pub format_version: String,

    pub handler_name: String,

    pub handler_version: String,

    pub processing_node: Option<String>,

    pub additional_data: HashMap<String, serde_json::Value>,}

impl Default for ResponseMetadata {
            format_version: "1.0".to_string(),
            handler_name: "unknown".to_string(),
            handler_version: "1.0.0".to_string(),
            processing_node: None,
            additional_data: HashMap::with_capacity(16),
