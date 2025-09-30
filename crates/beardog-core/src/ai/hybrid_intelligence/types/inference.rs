//! Inference configuration types for the hybrid intelligence system
//!
//! This module contains all inference-related configuration types including
//! inference parameters, prediction settings, serving, and caching configurations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::ai::hybrid_intelligence::learning::{PredictionHorizon, PredictionModel};

/// Inference configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceConfig {
    /// Batch size for inference
    pub batch_size: u32,
    /// Maximum inference time
    pub max_inference_time: Duration,
    /// Enable GPU acceleration
    pub use_gpu: bool,
    /// Number of threads for CPU inference
    pub num_threads: u32,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            batch_size: 1,
            max_inference_time: Duration::from_secs(30),
            use_gpu: false,
            num_threads: 4,
        }
    }
}

/// Prediction configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionConfig {
    /// Prediction horizon
    pub horizon: PredictionHorizon,
    /// Prediction model
    pub model: PredictionModel,
    /// Confidence threshold
    pub confidence_threshold: f64,
    /// Enable uncertainty quantification
    pub uncertainty_quantification: bool,
    /// Number of prediction samples for uncertainty
    pub num_samples: u32,
}

impl Default for PredictionConfig {
    fn default() -> Self {
        Self {
            horizon: PredictionHorizon::ShortTerm,
            model: PredictionModel::TimeSeries,
            confidence_threshold: 0.8,
            uncertainty_quantification: true,
            num_samples: 100,
        }
    }
}

/// Model serving configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServingConfig {
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Maximum concurrent requests
    pub max_concurrent_requests: u32,
    /// Request timeout
    pub request_timeout: Duration,
    /// Enable request batching
    pub enable_batching: bool,
    /// Batch timeout
    pub batch_timeout: Duration,
    /// Maximum batch size
    pub max_batch_size: u32,
    /// Enable model warming
    pub enable_warming: bool,
    /// Health check interval
    pub health_check_interval: Duration,
}

impl Default for ServingConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            max_concurrent_requests: 100,
            request_timeout: Duration::from_secs(30),
            enable_batching: true,
            batch_timeout: Duration::from_millis(10),
            max_batch_size: 32,
            enable_warming: true,
            health_check_interval: Duration::from_secs(30),
        }
    }
}

/// Caching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingConfig {
    /// Enable caching
    pub enabled: bool,
    /// Cache size limit (in MB)
    pub size_limit_mb: u64,
    /// Cache TTL (time to live)
    pub ttl: Duration,
    /// Cache type
    pub cache_type: CacheType,
    /// Eviction policy
    pub eviction_policy: EvictionPolicy,
}

impl Default for CachingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            size_limit_mb: 1024, // 1GB
            ttl: Duration::from_secs(3600), // 1 hour
            cache_type: CacheType::Memory,
            eviction_policy: EvictionPolicy::LRU,
        }
    }
}

/// Available cache types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CacheType {
    /// In-memory cache
    Memory,
    /// Redis cache
    Redis,
    /// File-based cache
    File,
    /// Hybrid cache (memory + disk)
    Hybrid,
}

impl Default for CacheType {
    fn default() -> Self {
        Self::Memory
    }
}

/// Cache eviction policies
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EvictionPolicy {
    /// Least Recently Used
    LRU,
    /// Least Frequently Used
    LFU,
    /// First In First Out
    FIFO,
    /// Random
    Random,
    /// Time-based expiration
    TTL,
}

impl Default for EvictionPolicy {
    fn default() -> Self {
        Self::LRU
    }
}

/// Prediction result with confidence and uncertainty
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResult {
    /// Predicted value
    pub value: f64,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Uncertainty estimate
    pub uncertainty: Option<f64>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Prediction timestamp
    pub timestamp: std::time::SystemTime,
}

impl Default for PredictionResult {
    fn default() -> Self {
        Self {
            value: 0.0,
            confidence: 0.0,
            uncertainty: None,
            metadata: HashMap::new(),
            timestamp: std::time::SystemTime::now(),
        }
    }
}

/// Batch prediction request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPredictionRequest {
    /// Input features for batch prediction
    pub inputs: Vec<Vec<f64>>,
    /// Request ID
    pub request_id: String,
    /// Priority level
    pub priority: RequestPriority,
    /// Timeout for the request
    pub timeout: Duration,
}

/// Request priority levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RequestPriority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

impl Default for RequestPriority {
    fn default() -> Self {
        Self::Normal
    }
}

/// Batch prediction response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPredictionResponse {
    /// Prediction results
    pub results: Vec<PredictionResult>,
    /// Request ID
    pub request_id: String,
    /// Processing time
    pub processing_time: Duration,
    /// Status of the request
    pub status: ResponseStatus,
}

/// Response status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ResponseStatus {
    /// Request completed successfully
    Success,
    /// Partial success (some predictions failed)
    PartialSuccess,
    /// Request failed
    Failed,
    /// Request timed out
    Timeout,
}

impl Default for ResponseStatus {
    fn default() -> Self {
        Self::Success
    }
} 