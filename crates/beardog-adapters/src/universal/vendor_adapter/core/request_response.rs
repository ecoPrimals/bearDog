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


/// Universal Request and Response Types
///
/// Defines the standardized request and response formats that work
/// with any vendor capability in a vendor-agnostic way.

use beardog_types::canonical::capabilities::CapabilityType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;
// ✅ CONFIGURATION UNIFICATION - Using canonical RetryConfig
pub use beardog_types::config::RetryConfig;
/// **UNIVERSAL VENDOR REQUEST** - Works with any vendor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalVendorRequest {
    /// Request identifier
    pub request_id: Uuid,
    /// Required capability (not vendor name!)
    pub required_capability: CapabilityType,
    /// Operation to perform
    pub operation: CapabilityOperation,
    /// Request parameters (flexible)
    pub parameters: HashMap<String, serde_json::Value>,
    /// Quality requirements
    pub quality_requirements: QualityRequirements,
    /// Routing preferences
    pub routing_preferences: RoutingPreferences,
    /// Context information
    pub context: RequestContext,
    /// Timeout settings
    pub timeout: Option<Duration>,
    /// Retry configuration - **MIGRATED TO CANONICAL**
    pub retry_config: Option<RetryConfig>,
    /// Request timestamp
    pub created_at: DateTime<Utc>,
    /// Request priority
    pub priority: RequestPriority,
    /// Request tags for categorization
    pub tags: Vec<String>,
}
impl UniversalVendorRequest {
    /// Create a new universal vendor request}


    #[must_use] pub fn new(capability: CapabilityType, operation: CapabilityOperation) -> Self {
        Self {
            request_id: Uuid::new_v4(),
            required_capability: capability,
            operation,
            parameters: HashMap::new(),
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
    /// Add a parameter to the request
    pub fn with_parameter<T: serde::Serialize>(mut self, key: &str, value: T) -> Self {
        self.parameters.insert(
            key.to_string(),
            serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
        );
        self
    /// Set quality requirements
    #[must_use] pub const fn with_quality_requirements(mut self, requirements: QualityRequirements) -> Self {
        self.quality_requirements = requirements;
    /// Set routing preferences
    #[must_use] pub fn with_routing_preferences(mut self, preferences: RoutingPreferences) -> Self {
        self.routing_preferences = preferences;
    /// Set request priority
    #[must_use] pub const fn with_priority(mut self, priority: RequestPriority) -> Self {
        self.priority = priority;
    /// Add a tag
    #[must_use] pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
/// **UNIVERSAL VENDOR RESPONSE** - Standardized across all vendors
pub struct UniversalVendorResponse {
    /// Original request ID
    /// Operation success
    pub success: bool,
    /// Response data (vendor-agnostic format)
    pub data: serde_json::Value,
    /// Capability that handled this
    pub handled_by_capability: CapabilityType,
    /// Handler instance that processed this
    pub handler_instance_id: Uuid,
    /// Performance metrics
    pub performance: PerformanceMetrics,
    /// Quality metrics
    pub quality: QualityMetrics,
    /// Cost metrics
    pub cost: CostMetrics,
    /// Error information (if any)
    pub error: Option<String>,
    /// Processing timestamp
    pub processed_at: DateTime<Utc>,
    /// Response metadata
    pub metadata: ResponseMetadata,
    /// Response tags}


impl UniversalVendorResponse {
    /// Create a successful response}


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
    /// Create an error response
    #[must_use] pub fn error(
        error: beardog_errors::BearDogError,
            success: false,
            data: serde_json::Value::Null,
            error: Some(error.to_string()),
/// **CAPABILITY OPERATION** - Vendor-agnostic operations
pub enum CapabilityOperation {
    /// Cryptographic operations
    Crypto {
        operation_type: CryptoOperationType,
        algorithm: Option<String>,
        key_spec: Option<UniversalKeySpec>,
        data: Vec<u8>,
    },
    /// Storage operations  
    Storage {
        operation_type: StorageOperationType,
        data_spec: Option<DataSpec>,
        durability: Option<DurabilityLevel>,
        path: String,
        data: Option<Vec<u8>>,
    /// AI/ML operations
    Intelligence {
        operation_type: AIOperationType,
        model_requirements: Option<ModelRequirements>,
        context: Option<AIContext>,
        input_data: serde_json::Value,
    /// HSM operations
    HardwareSecurityModule {
        operation_type: HsmOperationType,
        security_level: SecurityLevel,
    /// Network operations
    Network {
        operation_type: NetworkOperationType,
        endpoint: String,
        protocol: NetworkProtocol,
    /// Custom operations (extensible)
    Custom {
        operation_name: String,
        operation_data: serde_json::Value,
/// **CRYPTO OPERATION TYPES**
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
/// **STORAGE OPERATION TYPES**}


pub enum StorageOperationType {
    Store,
    Retrieve,
    Delete,
    List,
    CreateContainer,
    DeleteContainer,
    GetMetadata,
    SetMetadata,
/// **AI OPERATION TYPES**
pub enum AIOperationType {
    Inference,
    Training,
    FineTuning,
    Embedding,
    Classification,
    Generation,
    Analysis,
/// **HSM OPERATION TYPES**}


pub enum HsmOperationType {
    Attest,
    GetInfo,
/// **NETWORK OPERATION TYPES**
pub enum NetworkOperationType {
    HttpRequest,
    WebSocketConnect,
    GrpcCall,
    TcpConnect,
    UdpSend,
/// **UNIVERSAL KEY SPEC** - Vendor-agnostic key specification}


pub struct UniversalKeySpec {
    /// Key algorithm
    pub algorithm: KeyAlgorithm,
    /// Key size in bits
    pub key_size: u32,
    /// Key usage permissions
    pub usage: Vec<KeyUsage>,
    /// Key identifier
    pub key_id: Option<String>,
    /// Extractable flag
    pub extractable: bool,
/// **KEY ALGORITHM**
pub enum KeyAlgorithm {
    Rsa,
    Ecdsa,
    Ed25519,
    Aes,
    ChaCha20,
    Hmac,
/// **KEY USAGE**}


pub enum KeyUsage {
    Derive,
    Wrap,
    Unwrap,
/// **DATA SPEC** - Specification for data operations
pub struct DataSpec {
    /// Data format
    pub format: DataFormat,
    /// Compression algorithm
    pub compression: Option<CompressionAlgorithm>,
    /// Encryption requirements
    pub encryption: Option<EncryptionSpec>,
    /// Data validation requirements
    pub validation: Option<ValidationSpec>,
/// **DATA FORMAT**
pub enum DataFormat {
    Binary,
    Text,
    Json,
    Xml,
    Yaml,
    Protobuf,
    Avro,
    Parquet,
/// **COMPRESSION ALGORITHM**}


pub enum CompressionAlgorithm {
    Gzip,
    Zstd,
    Lz4,
    Brotli,
    Snappy,
/// **ENCRYPTION SPEC** - Encryption requirements
pub struct EncryptionSpec {
    /// Encryption algorithm
    pub algorithm: String,
    /// Key derivation function
    pub kdf: Option<String>,
    /// Additional authenticated data
    pub aad: Option<Vec<u8>>,
/// **VALIDATION SPEC** - Data validation requirements
pub struct ValidationSpec {
    /// Checksum algorithm
    pub checksum: Option<String>,
    /// Schema validation
    pub schema: Option<String>,
    /// Custom validation rules
    pub custom_rules: Vec<String>,
/// **DURABILITY LEVEL** - Data durability requirements
pub enum DurabilityLevel {
    /// Single copy, no redundancy
    Single,
    /// Multiple copies within single location
    Local,
    /// Multiple copies across locations
    Regional,
    /// Maximum durability across multiple regions
    Global,
/// **SECURITY LEVEL** - Security requirements}


pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Critical,
/// **MODEL REQUIREMENTS** - AI model requirements
pub struct ModelRequirements {
    /// Model type
    pub model_type: String,
    /// Minimum model size
    pub min_parameters: Option<u64>,
    /// Maximum model size
    pub max_parameters: Option<u64>,
    /// Required capabilities
    pub capabilities: Vec<String>,
    /// Performance requirements
    pub performance: ModelPerformanceRequirements,
/// **MODEL PERFORMANCE REQUIREMENTS**
pub struct ModelPerformanceRequirements {
    /// Maximum latency in milliseconds
    pub max_latency_ms: u64,
    /// Minimum throughput (requests per second)
    pub min_throughput_rps: f64,
    /// Required accuracy (0.0 - 1.0)
    pub min_accuracy: f64,
/// **AI CONTEXT** - Context for AI operations
pub struct AIContext {
    /// Conversation history
    pub history: Vec<AIMessage>,
    /// System prompt
    pub system_prompt: Option<String>,
    /// Temperature setting
    pub temperature: Option<f64>,
    /// Maximum tokens
    pub max_tokens: Option<u32>,
/// **AI MESSAGE**
pub struct AIMessage {
    /// Message role
    pub role: String,
    /// Message content
    pub content: String,
    /// Message timestamp
    pub timestamp: DateTime<Utc>,
/// **NETWORK PROTOCOL**
pub enum NetworkProtocol {
    Http,
    Https,
    WebSocket,
    Grpc,
    Tcp,
    Udp,
/// **QUALITY REQUIREMENTS** - Quality of service requirements}


pub struct QualityRequirements {
    /// Maximum acceptable latency
    pub max_latency_ms: Option<u64>,
    /// Minimum required availability
    pub min_availability_percentage: Option<f64>,
    /// Required consistency level
    pub consistency_level: Option<super::ConsistencyLevel>,
    /// Durability requirements
    pub durability_level: Option<DurabilityLevel>,
    /// Security requirements
    pub security_level: Option<SecurityLevel>,
    /// Cost constraints
    pub cost_constraints: Option<CostConstraints>,}


impl Default for QualityRequirements {}


    fn default() -> Self {
            max_latency_ms: Some(5000), // 5 seconds
            min_availability_percentage: Some(99.0),
            consistency_level: Some(super::ConsistencyLevel::Eventual),
            durability_level: Some(DurabilityLevel::Local),
            security_level: Some(SecurityLevel::Medium),
            cost_constraints: None,
/// **COST CONSTRAINTS** - Cost limitations
pub struct CostConstraints {
    /// Maximum cost per operation
    pub max_cost_per_operation_usd: Option<f64>,
    /// Maximum monthly budget
    pub max_monthly_budget_usd: Option<f64>,
    /// Prefer free tier when available
    pub prefer_free_tier: bool,
/// **ROUTING PREFERENCES** - How to route the request
pub struct RoutingPreferences {
    /// Preferred routing strategy
    pub strategy: RoutingStrategy,
    /// Geographic preferences
    pub geographic_preferences: Option<GeographicPreferences>,
    /// Vendor preferences
    pub vendor_preferences: Option<VendorPreferences>,
    /// Failover behavior
    pub failover_behavior: FailoverBehavior,
    /// Load balancing preferences
    pub load_balancing: LoadBalancingStrategy,}


impl Default for RoutingPreferences {
            strategy: RoutingStrategy::Performance,
            geographic_preferences: None,
            vendor_preferences: None,
            failover_behavior: FailoverBehavior::Automatic,
            load_balancing: LoadBalancingStrategy::RoundRobin,
/// **ROUTING STRATEGY**}


pub enum RoutingStrategy {
    /// Route to fastest handler
    Performance,
    /// Route to most reliable handler
    Reliability,
    /// Route to cheapest handler
    Cost,
    /// Route to most compliant handler
    Compliance,
    /// Route to geographically closest handler
    Geographic,
    /// Use adaptive learning
    Adaptive,
    /// Use custom multi-criteria
    MultiCriteria,
/// **GEOGRAPHIC PREFERENCES**}


pub struct GeographicPreferences {
    /// Preferred regions
    pub preferred_regions: Vec<String>,
    /// Excluded regions
    pub excluded_regions: Vec<String>,
    /// Data residency requirements
    pub data_residency: Option<String>,
/// **VENDOR PREFERENCES**
pub struct VendorPreferences {
    /// Preferred vendor tags
    pub preferred_vendors: Vec<String>,
    /// Excluded vendor tags
    pub excluded_vendors: Vec<String>,
    /// Vendor diversity requirements
    pub require_diversity: bool,
/// **FAILOVER BEHAVIOR**
pub enum FailoverBehavior {
    /// Automatically failover to next best handler
    Automatic,
    /// Fail immediately without trying other handlers
    None,
    /// Try specific number of handlers
    Limited(u32),
    /// Custom failover logic
    Custom(String),
/// **LOAD BALANCING STRATEGY**}


pub enum LoadBalancingStrategy {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    Random,
    ConsistentHash,
/// **REQUEST CONTEXT** - Additional context for the request
pub struct RequestContext {
    /// User ID (if applicable)
    pub user_id: Option<String>,
    /// Session ID
    pub session_id: Option<String>,
    /// Trace ID for distributed tracing
    pub trace_id: Option<String>,
    /// Request source
    pub source: Option<String>,
    /// Client information
    pub client_info: Option<ClientInfo>,
    /// Custom context data
    pub custom_data: HashMap<String, serde_json::Value>,}


impl Default for RequestContext {
            user_id: None,
            session_id: None,
            trace_id: Some(Uuid::new_v4().to_string()),
            source: None,
            client_info: None,
            custom_data: HashMap::new(),
/// **CLIENT INFO** - Information about the client making the request}


pub struct ClientInfo {
    /// Client name
    pub name: String,
    /// Client version
    pub version: String,
    /// Client IP address
    pub ip_address: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
/// **REQUEST PRIORITY**
pub enum RequestPriority {
    Normal,}


impl Default for RequestPriority {
        Self::Normal
/// **RETRY DELAY STRATEGY** - For backward compatibility with existing code}


pub enum RetryDelayStrategy {
    /// Fixed delay between retries
    Fixed(Duration),
    /// Linear backoff
    Linear {
        initial_delay: Duration,
        increment: Duration,
    /// Exponential backoff
    ExponentialBackoff {
        max_delay: Duration,
        multiplier: f64,
    /// Random jitter
    RandomJitter {
        min_delay: Duration,
/// **PERFORMANCE METRICS** - Performance data for the response}


pub struct PerformanceMetrics {
    /// Total processing time in milliseconds
    pub processing_time_ms: u64,
    /// Queue time in milliseconds
    pub queue_time_ms: u64,
    /// Network time in milliseconds
    pub network_time_ms: u64,
    /// CPU usage during processing
    pub cpu_usage_percentage: f64,
    /// Memory usage during processing in MB
    pub memory_usage_mb: u64,
    /// Network bytes sent
    pub bytes_sent: u64,
    /// Network bytes received
    pub bytes_received: u64,}


impl Default for PerformanceMetrics {
            processing_time_ms: 0,
            queue_time_ms: 0,
            network_time_ms: 0,
            cpu_usage_percentage: 0.0,
            memory_usage_mb: 0,
            bytes_sent: 0,
            bytes_received: 0,
/// **QUALITY METRICS** - Quality measurements for the response}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QualityMetrics {
    /// Accuracy score (0.0 - 1.0)
    pub accuracy_score: Option<f64>,
    /// Confidence score (0.0 - 1.0)
    pub confidence_score: Option<f64>,
    /// Completeness score (0.0 - 1.0)
    pub completeness_score: Option<f64>,
    /// Freshness of data (age in seconds)
    pub data_freshness_seconds: Option<u64>,
    /// Consistency validation passed
    pub consistency_validated: bool,
/// **COST METRICS** - Cost information for the response
pub struct CostMetrics {
    /// Actual cost for this operation in USD
    pub operation_cost_usd: f64,
    /// Cost breakdown by component
    pub cost_breakdown: HashMap<String, f64>,
    /// Billing model used
    pub billing_model: String,
    /// Cost optimization suggestions
    pub optimization_suggestions: Vec<String>,}


impl Default for CostMetrics {
            operation_cost_usd: 0.0,
            cost_breakdown: HashMap::new(),
            billing_model: "pay-per-use".to_string(),
            optimization_suggestions: Vec::new(),
/// **RESPONSE METADATA** - Additional metadata about the response}


pub struct ResponseMetadata {
    /// Response format version
    pub format_version: String,
    /// Handler that processed the request
    pub handler_name: String,
    /// Handler version
    pub handler_version: String,
    /// Processing node/instance
    pub processing_node: Option<String>,
    /// Additional metadata
    pub additional_data: HashMap<String, serde_json::Value>,}


impl Default for ResponseMetadata {
            format_version: "1.0".to_string(),
            handler_name: "unknown".to_string(),
            handler_version: "1.0.0".to_string(),
            processing_node: None,
            additional_data: HashMap::new(),
