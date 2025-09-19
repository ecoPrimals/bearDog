// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;

use beardog_types::canonical::capabilities::CapabilityType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

pub use beardog_types::canonical::configuration::production::RetryConfig;

#[derive(Debug, Clone)]
    /// The required capability value
    pub required_capability: CapabilityType,

    /// The operation value
    pub operation: CapabilityOperation,

    /// Mapping of parameters
    pub parameters: HashMap<String, serde_json::Value>,

    /// The quality requirements value
    pub quality_requirements: QualityRequirements,

    /// The routing preferences value
    pub routing_preferences: RoutingPreferences,

    /// The context value
    pub context: RequestContext,


    pub timeout: Option<Duration>,


    pub retry_config: Option<RetryConfig>,

    /// The created at value
    pub created_at: DateTime<Utc>,

    /// The priority value
    pub priority: RequestPriority,

    /// Collection of tags
    pub tags: Vec<String>,
}
impl UniversalVendorRequest {

/// New operation.
    #[must_use] pub fn new(CapabilityType, operation: CapabilityOperation) -> Self {
        Self {
            request_id: Uuid::new_v4(capability,
            operation,
            parameters: HashMap::with_capacity(16),
            quality_requirements: QualityRequirements::default(),
            routing_preferences: RoutingPreferences::default(),
            context: RequestContext::default(),
            timeout: Some(Duration::from_secs(30)),
            retry_config: Some(RetryConfig::default()),
            created_at: Utc::now(RequestPriority::Normal,
            tags: Vec::new(serde::Serialize>(mut self, key: &str, value: T) -> Self {
        self.parameters.insert(
            key.to_string(),
            serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
        );
        self

    #[must_use] pub const fn with_quality_requirements(mut self, requirements: QualityRequirements) -> Self {
        self.quality_requirements = requirements;

/// With Routing Preferences operation.
    #[must_use] pub fn with_routing_preferences(mut self, preferences: RoutingPreferences) -> Self {
        self.routing_preferences = preferences;

    #[must_use] pub const fn with_priority(mut self, priority: RequestPriority) -> Self {
        self.priority = priority;

/// With Tag operation.
    #[must_use] pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(bool,

    /// The data value
    pub data: serde_json::Value,

    /// The handled by capability value
    pub handled_by_capability: CapabilityType,


    pub handler_instance_id: Uuid,


    pub performance: PerformanceMetrics,

    /// The quality value
    pub quality: QualityMetrics,

    /// The cost value
    pub cost: CostMetrics,

    /// Optional error
    pub error: Option<String>,

    /// The processed at value
    pub processed_at: DateTime<Utc>,

    /// The metadata value
    pub metadata: ResponseMetadata,

impl UniversalVendorResponse {

/// Success operation.
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
            cost: CostMetrics::default(None,
            processed_at: Utc::now(),
            metadata: ResponseMetadata::default(beardog_errors::BearDogError,
            success: false,
            data: serde_json::Value::Null,
            error: Some(CryptoOperationType,
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
/// Types of crypto operation
pub enum CryptoOperationType {
    /// Represents generate key variant
    GenerateKey,
    /// Represents encrypt variant
    Encrypt,
    /// Represents decrypt variant
    Decrypt,
    /// Represents sign variant
    Sign,
    /// Represents verify variant
    Verify,
    /// Represents hash variant
    Hash,
    /// Represents derive key variant
    DeriveKey,
    /// Represents export key variant
    ExportKey,
    /// Represents import key variant
    ImportKey,
/// Types of storage operation
pub enum StorageOperationType {
    /// Represents store variant
    Store,
    /// Represents retrieve variant
    Retrieve,
    /// Represents delete variant
    Delete,
    /// Represents list variant
    List,
    /// Represents create container variant
    CreateContainer,
    /// Represents delete container variant
    DeleteContainer,
    /// Represents get metadata variant
    GetMetadata,
    /// Represents set metadata variant
    SetMetadata,
/// Types of a i operation
pub enum AIOperationType {
    /// Represents inference variant
    Inference,
    /// Currently training
    Training,
    /// Currently finetuning
    FineTuning,
    /// Currently embedding
    Embedding,
    /// Represents classification variant
    Classification,
    /// Represents generation variant
    Generation,
    /// Represents analysis variant
    Analysis,
/// Types of hsm operation
pub enum HsmOperationType {
    /// Represents attest variant
    Attest,
    /// Represents get info variant
    GetInfo,
/// Types of network operation
pub enum NetworkOperationType {
    /// Represents http request variant
    HttpRequest,
    /// Represents web socket connect variant
    WebSocketConnect,
    /// Represents grpc call variant
    GrpcCall,
    /// Represents tcp connect variant
    TcpConnect,
    /// Represents udp send variant
    UdpSend,

pub struct UniversalKeySpec {

    /// The algorithm value
    pub algorithm: KeyAlgorithm,

    /// Number of key_size
    pub key_size: u32,

    /// Collection of usage
    pub usage: Vec<KeyUsage>,


    pub key_id: Option<String>,

    /// Whether extractable is enabled
    pub extractable: bool,

pub enum KeyAlgorithm {
    /// Represents rsa variant
    Rsa,
    /// Represents ecdsa variant
    Ecdsa,
    /// Represents ed25519 variant
    Ed25519,
    /// Represents aes variant
    Aes,
    /// Represents cha cha20 variant
    ChaCha20,
    /// Represents hmac variant
    Hmac,

pub enum KeyUsage {
    /// Represents derive variant
    Derive,
    /// Represents wrap variant
    Wrap,
    /// Represents unwrap variant
    Unwrap,

pub struct DataSpec {


    pub format: DataFormat,

    /// Optional compression
    pub compression: Option<CompressionAlgorithm>,

    /// Optional encryption
    pub encryption: Option<EncryptionSpec>,


    pub validation: Option<ValidationSpec>,

pub enum DataFormat {
    /// Represents binary variant
    Binary,
    /// Represents text variant
    Text,
    /// Represents json variant
    Json,
    /// Represents xml variant
    Xml,
    /// Represents yaml variant
    Yaml,
    /// Represents protobuf variant
    Protobuf,
    /// Represents avro variant
    Avro,
    /// Represents parquet variant
    Parquet,

pub enum CompressionAlgorithm {
    /// Represents gzip variant
    Gzip,
    /// Represents zstd variant
    Zstd,
    /// Represents lz4 variant
    Lz4,
    /// Represents brotli variant
    Brotli,
    /// Represents snappy variant
    Snappy,

pub struct EncryptionSpec {

    /// The algorithm value
    pub algorithm: String,

    /// Optional kdf
    pub kdf: Option<String>,

    /// Optional aad
    pub aad: Option<Vec<u8>>,

pub struct ValidationSpec {

    /// Optional checksum
    pub checksum: Option<String>,

    /// Optional schema
    pub schema: Option<String>,

    /// Collection of custom rules
    pub custom_rules: Vec<String>,

pub enum DurabilityLevel {


    /// Represents single variant
    Single,


    /// Represents local variant
    Local,


    /// Represents regional variant
    Regional,


    /// Represents global variant
    Global,

pub enum SecurityLevel {
    /// Represents low variant
    Low,
    /// Represents medium variant
    Medium,
    /// Represents high variant
    High,
    /// Represents critical variant
    Critical,

pub struct ModelRequirements {

    /// The model type value
    pub model_type: String,

    /// Optional min parameters
    pub min_parameters: Option<u64>,

    /// Optional max parameters
    pub max_parameters: Option<u64>,

    /// Collection of capabilities
    pub capabilities: Vec<String>,


    pub performance: ModelPerformanceRequirements,

pub struct ModelPerformanceRequirements {

    /// Number of max_latency_ms
    pub max_latency_ms: u64,

    /// The min throughput rps value
    pub min_throughput_rps: f64,

    /// The min accuracy value
    pub min_accuracy: f64,

pub struct AIContext {

    /// Collection of history
    pub history: Vec<AIMessage>,

    /// Optional system prompt
    pub system_prompt: Option<String>,

    /// Optional temperature
    pub temperature: Option<f64>,

    /// Optional max tokens
    pub max_tokens: Option<u32>,

pub struct AIMessage {

    /// The role value
    pub role: String,

    /// The content value
    pub content: String,


    pub timestamp: DateTime<Utc>,

pub enum NetworkProtocol {
    /// Represents http variant
    Http,
    /// Represents https variant
    Https,
    /// Represents web socket variant
    WebSocket,
    /// Represents grpc variant
    Grpc,
    /// Represents tcp variant
    Tcp,
    /// Represents udp variant
    Udp,

pub struct QualityRequirements {

    /// Optional max latency ms
    pub max_latency_ms: Option<u64>,

    /// Optional min availability percentage
    pub min_availability_percentage: Option<f64>,

    /// Optional consistency level
    pub consistency_level: Option<super::ConsistencyLevel>,

    /// Optional durability level
    pub durability_level: Option<DurabilityLevel>,

    /// Optional security level
    pub security_level: Option<SecurityLevel>,

    /// Optional cost constraints
    pub cost_constraints: Option<CostConstraints>,}
    pub cost_constraints: Option<CostConstraints>,}
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

    /// Optional max cost per operation usd
    pub max_cost_per_operation_usd: Option<f64>,

    /// Optional max monthly budget usd
    pub max_monthly_budget_usd: Option<f64>,

    /// Whether prefer_free_tier is enabled
    pub prefer_free_tier: bool,

pub struct RoutingPreferences {

    /// The strategy value
    pub strategy: RoutingStrategy,

    /// Optional geographic preferences
    pub geographic_preferences: Option<GeographicPreferences>,

    /// Optional vendor preferences
    pub vendor_preferences: Option<VendorPreferences>,

    /// The failover behavior value
    pub failover_behavior: FailoverBehavior,

    /// The load balancing value
    pub load_balancing: LoadBalancingStrategy,}

impl Default for RoutingPreferences {
            strategy: RoutingStrategy::Performance,
            geographic_preferences: None,
            vendor_preferences: None,
            failover_behavior: FailoverBehavior::Automatic,
            load_balancing: LoadBalancingStrategy::RoundRobin,

pub enum RoutingStrategy {


    Performance,


    /// Represents reliability variant
    Reliability,


    /// Represents cost variant
    Cost,


    /// Represents compliance variant
    Compliance,


    /// Represents geographic variant
    Geographic,


    /// Represents adaptive variant
    Adaptive,


    /// Represents multi criteria variant
    MultiCriteria,

pub struct GeographicPreferences {

    /// Collection of preferred regions
    pub preferred_regions: Vec<String>,

    /// Collection of excluded regions
    pub excluded_regions: Vec<String>,


    pub data_residency: Option<String>,

pub struct VendorPreferences {

    /// Collection of preferred vendors
    pub preferred_vendors: Vec<String>,

    /// Collection of excluded vendors
    pub excluded_vendors: Vec<String>,

    /// Whether require_diversity is enabled
    pub require_diversity: bool,

pub enum FailoverBehavior {


    /// Represents automatic variant
    Automatic,


    /// No none specified
    None,

    /// State indicating limited
    Limited(Option<String>,


    pub session_id: Option<String>,


    pub trace_id: Option<String>,

    /// Optional source
    pub source: Option<String>,

    /// Optional client info
    pub client_info: Option<ClientInfo>,

    /// Mapping of custom data
    pub custom_data: HashMap<String, serde_json::Value>,}
    pub custom_data: HashMap<String, serde_json::Value>,}
    pub custom_data: HashMap<String, serde_json::Value>,}

impl Default for RequestContext {
            user_id: None,
            session_id: None,
            trace_id: Some(Uuid::new_v4(None,
            client_info: None,
            custom_data: HashMap::with_capacity(String,

    /// The version value
    pub version: String,

    /// Optional ip address
    pub ip_address: Option<String>,

    /// Optional user agent
    pub user_agent: Option<String>,

pub enum RequestPriority {
    Normal,}
    Normal,}
    Normal,}

impl Default for RequestPriority {
        Self::Normal

pub enum RetryDelayStrategy {

    /// State indicating fixed
    Fixed(Duration,
        increment: Duration,

    /// Represents exponential backoff variant
    ExponentialBackoff {
        max_delay: Duration,
        multiplier: f64,

    /// Represents random jitter variant
    RandomJitter {
        min_delay: Duration,

pub struct PerformanceMetrics {


    pub processing_time_ms: u64,


    pub queue_time_ms: u64,


    pub network_time_ms: u64,

    /// The cpu usage percentage value
    pub cpu_usage_percentage: f64,

    /// Number of memory_usage_mb
    pub memory_usage_mb: u64,

    /// Number of bytes_sent
    pub bytes_sent: u64,

    /// Number of bytes_received
    pub bytes_received: u64,}
    pub bytes_received: u64,}
    pub bytes_received: u64,}

impl Default for PerformanceMetrics {
            processing_time_ms: 0,
            queue_time_ms: 0,
            network_time_ms: 0,
            cpu_usage_percentage: 0.0,
            memory_usage_mb: 0,
            bytes_sent: 0,
            bytes_received: 0,

#[derive(Debug, Clone)]
    pub confidence_score: Option<f64>,

    /// Optional completeness score
    pub completeness_score: Option<f64>,

    /// Optional data freshness seconds
    pub data_freshness_seconds: Option<u64>,


    pub consistency_validated: bool,

pub struct CostMetrics {

    /// The operation cost usd value
    pub operation_cost_usd: f64,

    /// Mapping of cost breakdown
    pub cost_breakdown: HashMap<String, f64>,

    /// The billing model value
    pub billing_model: String,

    /// Collection of optimization suggestions
    pub optimization_suggestions: Vec<String>,}

impl Default for CostMetrics {
            operation_cost_usd: 0.0,
            cost_breakdown: HashMap::with_capacity(16),
            billing_model: "pay-per-use".to_string(),
            optimization_suggestions: Vec::new(String,

    /// Name of the handler
    pub handler_name: String,

    /// The handler version value
    pub handler_version: String,

    /// Optional processing node
    pub processing_node: Option<String>,

    /// Mapping of additional data
    pub additional_data: HashMap<String, serde_json::Value>,}

impl Default for ResponseMetadata {
            format_version: "1.0".to_string(),
            handler_name: "unknown".to_string(),
            handler_version: "1.0.0".to_string(),
            additional_data: HashMap::with_capacity(16),
