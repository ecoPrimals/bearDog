//! Load Testing Operations
//!
//! This module defines various operation types that can be used
//! in load testing scenarios.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Database operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseOperation {
    /// Operation name
    pub name: String,
    /// Operation type
    pub operation_type: DatabaseOperationType,
    /// Query parameters
    pub parameters: HashMap<String, String>,
    /// Expected response time
    pub expected_response_time: Duration,
}

/// Database operation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseOperationType {
    /// Select operation
    Select {
        table: String,
        conditions: Vec<String>,
    },
    /// Insert operation
    Insert {
        table: String,
        values: HashMap<String, String>,
    },
    /// Update operation
    Update {
        table: String,
        values: HashMap<String, String>,
        conditions: Vec<String>,
    },
    /// Delete operation
    Delete {
        table: String,
        conditions: Vec<String>,
    },
    /// Custom query
    Custom { query: String },
}

/// Memory operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryOperation {
    /// Operation name
    pub name: String,
    /// Operation type
    pub operation_type: MemoryOperationType,
    /// Memory size
    pub memory_size: usize,
    /// Operation count
    pub operation_count: usize,
}

/// Memory operation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryOperationType {
    /// Allocate memory
    Allocate,
    /// Deallocate memory
    Deallocate,
    /// Read memory
    Read,
    /// Write memory
    Write,
}

/// Cache operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheOperation {
    /// Operation name
    pub name: String,
    /// Operation type
    pub operation_type: CacheOperationType,
    /// Cache key
    pub cache_key: String,
    /// Cache value size
    pub value_size: usize,
}

/// Cache operation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheOperationType {
    /// Get from cache
    Get,
    /// Put to cache
    Put,
    /// Delete from cache
    Delete,
    /// Clear cache
    Clear,
}

/// Crypto operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoOperation {
    /// Operation name
    pub name: String,
    /// Operation type
    pub operation_type: CryptoOperationType,
    /// Data size
    pub data_size: usize,
    /// Algorithm
    pub algorithm: String,
}

/// Crypto operation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CryptoOperationType {
    /// Encrypt data
    Encrypt,
    /// Decrypt data
    Decrypt,
    /// Sign data
    Sign,
    /// Verify signature
    Verify,
}

/// Genetic operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticOperation {
    /// Operation name
    pub name: String,
    /// Operation type
    pub operation_type: GeneticOperationType,
    /// Population size
    pub population_size: usize,
    /// Generation count
    pub generation_count: usize,
}

/// Genetic operation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeneticOperationType {
    /// Spawn new individuals
    Spawn,
    /// Evolve population
    Evolve,
    /// Crossover operation
    Crossover,
    /// Mutation operation
    Mutate,
}

/// System operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemOperation {
    /// Operation name
    pub name: String,
    /// Operation type
    pub operation_type: SystemOperationType,
    /// Operation parameters
    pub parameters: HashMap<String, String>,
}

/// System operation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemOperationType {
    /// HTTP request
    HttpRequest { method: String, url: String },
    /// File operation
    FileOperation { operation: String, path: String },
    /// Network operation
    NetworkOperation { operation: String, target: String },
    /// Process operation
    ProcessOperation { operation: String, command: String },
}

/// Stress operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressOperation {
    /// Operation name
    pub name: String,
    /// Operation type
    pub operation_type: StressOperationType,
    /// Stress intensity
    pub intensity: f64,
    /// Stress duration
    pub duration: Duration,
}

/// Stress operation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StressOperationType {
    /// CPU stress
    CPU,
    /// Memory stress
    Memory,
    /// Disk stress
    Disk,
    /// Network stress
    Network,
}

/// Scalability operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalabilityOperation {
    /// Operation name
    pub name: String,
    /// Operation type
    pub operation_type: ScalabilityOperationType,
    /// Scaling factor
    pub scaling_factor: f64,
    /// Resource type
    pub resource_type: String,
}

/// Scalability operation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScalabilityOperationType {
    /// Scale up
    ScaleUp,
    /// Scale down
    ScaleDown,
    /// Scale out
    ScaleOut,
    /// Scale in
    ScaleIn,
}

impl Default for DatabaseOperation {
    fn default() -> Self {
        Self {
            name: "Default DB Operation".to_string(),
            operation_type: DatabaseOperationType::Select {
                table: "test_table".to_string(),
                conditions: Vec::new(),
            },
            parameters: HashMap::new(),
            expected_response_time: Duration::from_millis(100),
        }
    }
}

impl Default for MemoryOperation {
    fn default() -> Self {
        Self {
            name: "Default Memory Operation".to_string(),
            operation_type: MemoryOperationType::Allocate,
            memory_size: 1024,
            operation_count: 1,
        }
    }
}

impl Default for CacheOperation {
    fn default() -> Self {
        Self {
            name: "Default Cache Operation".to_string(),
            operation_type: CacheOperationType::Get,
            cache_key: "test_key".to_string(),
            value_size: 256,
        }
    }
}

impl Default for CryptoOperation {
    fn default() -> Self {
        Self {
            name: "Default Crypto Operation".to_string(),
            operation_type: CryptoOperationType::Encrypt,
            data_size: 1024,
            algorithm: "AES-256-GCM".to_string(),
        }
    }
}

impl Default for GeneticOperation {
    fn default() -> Self {
        Self {
            name: "Default Genetic Operation".to_string(),
            operation_type: GeneticOperationType::Spawn,
            population_size: 100,
            generation_count: 10,
        }
    }
}

impl Default for SystemOperation {
    fn default() -> Self {
        Self {
            name: "Default System Operation".to_string(),
            operation_type: SystemOperationType::HttpRequest {
                method: "GET".to_string(),
                url: "http://localhost:8080".to_string(),
            },
            parameters: HashMap::new(),
        }
    }
}

impl Default for StressOperation {
    fn default() -> Self {
        Self {
            name: "Default Stress Operation".to_string(),
            operation_type: StressOperationType::CPU,
            intensity: 0.5,
            duration: Duration::from_secs(60),
        }
    }
}

impl Default for ScalabilityOperation {
    fn default() -> Self {
        Self {
            name: "Default Scalability Operation".to_string(),
            operation_type: ScalabilityOperationType::ScaleUp,
            scaling_factor: 2.0,
            resource_type: "CPU".to_string(),
        }
    }
}
