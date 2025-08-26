

use crate::common::TestResult;
use beardog_errors::{BearDogError, BearDogResult};
use beardog::{BearDogConfig, BearDogCore};
use serde_json::{json, Value as JsonValue};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;
use std::sync::OnceLock;

pub struct TestFixtures {

    datasets: HashMap<String, JsonValue>,

    configs: HashMap<String, BearDogConfig>,

    test_identities: HashMap<String, TestIdentity>,

    crypto_fixtures: CryptoFixtures,

    network_fixtures: NetworkFixtures,

    genetics_fixtures: GeneticsFixtures,
}

#[derive(Debug, Clone)]
pub struct TestIdentity {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub created_at: SystemTime,
    pub is_active: bool,
    pub metadata: HashMap<String, JsonValue>,
}

#[derive(Debug, Clone)]
pub struct CryptoFixtures {

    pub ed25519_keypairs: Vec<Ed25519KeyPair>,

    pub encrypted_samples: Vec<EncryptedSample>,

    pub signature_samples: Vec<SignatureSample>,

    pub nonce_samples: Vec<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct Ed25519KeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub key_id: String,
    pub created_at: SystemTime,
}

#[derive(Debug, Clone)]
pub struct EncryptedSample {
    pub plaintext: String,
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub algorithm: String,
    pub key_id: String,
}

#[derive(Debug, Clone)]
pub struct SignatureSample {
    pub message: String,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
    pub algorithm: String,
    pub is_valid: bool,
}

#[derive(Debug, Clone)]
pub struct NetworkFixtures {

    pub http_requests: Vec<HttpRequestSample>,

    pub api_responses: Vec<ApiResponseSample>,

    pub network_events: Vec<NetworkEventSample>,

    pub peer_configs: Vec<PeerConfigSample>,
}

#[derive(Debug, Clone)]
pub struct HttpRequestSample {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<JsonValue>,
    pub expected_status: u16,
}

#[derive(Debug, Clone)]
pub struct ApiResponseSample {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: JsonValue,
    pub response_time_ms: u64,
}

#[derive(Debug, Clone)]
pub struct NetworkEventSample {
    pub event_type: String,
    pub source_ip: String,
    pub destination_ip: String,
    pub payload: JsonValue,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone)]
pub struct PeerConfigSample {
    pub peer_id: String,
    pub address: String,
    pub port: u16,
    pub capabilities: Vec<String>,
    pub trust_level: String,
}

#[derive(Debug, Clone)]
pub struct GeneticsFixtures {

    pub genetics_configs: Vec<GeneticsConfigSample>,

    pub spawn_requests: Vec<SpawnRequestSample>,

    pub genetic_lineages: Vec<GeneticLineageSample>,

    pub fitness_samples: Vec<FitnessSample>,
}

#[derive(Debug, Clone)]
pub struct GeneticsConfigSample {
    pub config_id: String,
    pub generation: u32,
    pub capabilities: Vec<String>,
    pub fitness_score: f64,
    pub parent_ids: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SpawnRequestSample {
    pub request_id: String,
    pub parent_genetics: String,
    pub spawn_purpose: String,
    pub desired_capabilities: Vec<String>,
    pub constraints: HashMap<String, JsonValue>,
}

#[derive(Debug, Clone)]
pub struct GeneticLineageSample {
    pub lineage_id: String,
    pub generations: Vec<String>,
    pub branching_points: Vec<u32>,
    pub fitness_progression: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct FitnessSample {
    pub genetics_id: String,
    pub fitness_score: f64,
    pub calculated_at: SystemTime,
    pub factors: HashMap<String, f64>,
}

impl TestFixtures {

    pub fn new() -> Self {
        let mut fixtures = Self {
            datasets: HashMap::with_capacity(16),
            configs: HashMap::with_capacity(16),
            test_identities: HashMap::with_capacity(16),
            crypto_fixtures: CryptoFixtures::default(),
            network_fixtures: NetworkFixtures::default(),
            genetics_fixtures: GeneticsFixtures::default(),
        };

        fixtures.initialize_default_fixtures();
        fixtures
    }

    fn initialize_default_fixtures(&mut self) {

        self.create_default_identities();

        self.create_default_configs();

        self.create_default_datasets();

        self.crypto_fixtures = CryptoFixtures::create_samples();

        self.network_fixtures = NetworkFixtures::create_samples();

        self.genetics_fixtures = GeneticsFixtures::create_samples();
    }

    fn create_default_identities(&mut self) {

        self.test_identities.insert("admin".to_string(), TestIdentity {
            user_id: "admin_001".to_string(),
            username: "admin".to_string(),
            email: "admin@beardog.test".to_string(),
            roles: vec!["admin".to_string(), "user".to_string()],
            permissions: vec!["*".to_string()],
            created_at: UNIX_EPOCH,
            is_active: true,
            metadata: json!({
                "test_user": true,
                "clearance_level": "maximum"
            }).as_object().unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format_args!("Operation failed: {:?}", e).to_string()
).into())
}).clone(),
        });

        self.test_identities.insert("user".to_string(), TestIdentity {
            user_id: "user_001".to_string(),
            username: "testuser".to_string(),
            email: "user@beardog.test".to_string(),
            roles: vec!["user".to_string()],
            permissions: vec!["read".to_string(), "write".to_string()],
            created_at: UNIX_EPOCH,
            is_active: true,
            metadata: json!({
                "test_user": true,
                "clearance_level": "standard"
            }).as_object().unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format_args!("Operation failed: {:?}", e).to_string()
).into())
}).clone(),
        });

        self.test_identities.insert("readonly".to_string(), TestIdentity {
            user_id: "readonly_001".to_string(),
            username: "readonly".to_string(),
            email: "readonly@beardog.test".to_string(),
            roles: vec!["readonly".to_string()],
            permissions: vec!["read".to_string()],
            created_at: UNIX_EPOCH,
            is_active: true,
            metadata: json!({
                "test_user": true,
                "clearance_level": "basic"
            }).as_object().unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format_args!("Operation failed: {:?}", e).to_string()
).into())
}).clone(),
        });
    }

    fn create_default_configs(&mut self) {

        let mut unit_config = BearDogConfig::default();
        unit_config.enable_networking = false;
        unit_config.enable_database = false;
        unit_config.test_mode = true;
        self.configs.insert("unit".to_string(), unit_config);

        let mut integration_config = BearDogConfig::default();
        integration_config.enable_networking = true;
        integration_config.enable_database = true;
        integration_config.test_mode = true;
        self.configs.insert("integration".to_string(), integration_config);

        let mut performance_config = BearDogConfig::default();
        performance_config.enable_performance_monitoring = true;
        performance_config.enable_metrics = true;
        performance_config.test_mode = true;
        self.configs.insert("performance".to_string(), performance_config);
    }

    fn create_default_datasets(&mut self) {

        self.datasets.insert("api_request".to_string(), json!({
            "endpoint": "/api/v1/test",
            "method": "POST",
            "headers": {
                "Content-Type": "application/json",
                "Authorization": "Bearer test_token"
            },
            "body": {
                "action": "test_action",
                "data": {
                    "test_field": "test_value"
                }
            }
        }));

        self.datasets.insert("test_config".to_string(), json!({
            "environment": "test",
            "debug": true,
            "features": {
                "crypto": true,
                "networking": false,
                "genetics": true
            },
            "limits": {
                "max_connections": 100,
                "timeout_seconds": 30
            }
        }));

        self.datasets.insert("error_scenarios".to_string(), json!([
            {
                "name": "network_timeout",
                "error_type": "NetworkError",
                "trigger": "network_delay_5000ms"
            },
            {
                "name": "invalid_signature",
                "error_type": "CryptoError", 
                "trigger": "malformed_signature"
            },
            {
                "name": "resource_exhaustion",
                "error_type": "ResourceError",
                "trigger": "memory_limit_exceeded"
            }
        ]));
    }

    pub fn get_identity(&self, name: &str) -> Option<&TestIdentity> {
        self.test_identities.get(name)
    }

    pub fn get_config(&self, name: &str) -> Option<&BearDogConfig> {
        self.configs.get(name)
    }

    pub fn get_dataset(&self, name: &str) -> Option<&JsonValue> {
        self.datasets.get(name)
    }

    pub fn create_temp_identity(&mut self, username: &str, roles: Vec<&str>) -> String {
        let identity_id = format_args!("temp_{}", Uuid::new_v4().to_string().to_string()[..8].to_string());
        
        self.test_identities.insert(identity_id.clone(), TestIdentity {
            user_id: format_args!("temp_user_{}", Uuid::new_v4().to_string().to_string()[..8].to_string()),
            username: username.to_string(),
            email: format_args!("{}@temp.beardog.test", username).to_string(),
            roles,
            permissions: vec!["read".to_string()],
            created_at: SystemTime::now(),
            is_active: true,
            metadata: json!({
                "test_user": true,
                "temporary": true,
                "created_by": "test_fixtures"
            }).as_object().unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format_args!("Operation failed: {:?}", e).to_string()
).into())
}).clone(),
        });

        identity_id
    }

    pub fn crypto(&self) -> &CryptoFixtures {
        &self.crypto_fixtures
    }

    pub fn network(&self) -> &NetworkFixtures {
        &self.network_fixtures
    }

    pub fn genetics(&self) -> &GeneticsFixtures {
        &self.genetics_fixtures
    }

    pub async fn create_test_core(&self, config_name: &str) -> TestResult<Arc<BearDogCore>> {
        let config = self.get_config(config_name)
            .ok_or_else(|| BearDogError::not_found(&format_args!("Test configuration '{}' not found", config_name).to_string()))?
            .clone();

        let core = BearDogCore::new(config).await
            .map_err(|e| BearDogError::enhanced(
                "TEST_CORE_CREATION",
                format_args!("Failed to create test BearDogCore instance: {}", e).to_string(),
                beardog_errors::ErrorSeverity::High,
                beardog_errors::ErrorCategory::Initialization,
                "test_fixtures",
                "core_creation",
                vec![
                    "Verify test configuration is valid".to_string(),
                    "Check test environment setup".to_string(),
                ],
            ))?;

        Ok(Arc::new(core))
    }
}

impl CryptoFixtures {
    fn create_samples() -> Self {
        Self {
            ed25519_keypairs: vec![
                Ed25519KeyPair {
                    private_key: hex::decode("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef").unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format_args!("Operation failed: {:?}", e).to_string()
).into())
}),
                    public_key: hex::decode("fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210").unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format_args!("Operation failed: {:?}", e).to_string()
).into())
}),
                    key_id: "test_key_001".to_string(),
                    created_at: UNIX_EPOCH,
                },
            ],
            encrypted_samples: vec![
                EncryptedSample {
                    plaintext: "Hello, BearDog!".to_string(),
                    ciphertext: b"encrypted_test_data".to_vec(),
                    nonce: b"test_nonce_12".to_vec(),
                    algorithm: "AES-256-GCM".to_string(),
                    key_id: "test_key_001".to_string(),
                },
            ],
            signature_samples: vec![
                SignatureSample {
                    message: "Test message for signing".to_string(),
                    signature: b"test_signature_bytes".to_vec(),
                    public_key: hex::decode("fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210").unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format_args!("Operation failed: {:?}", e).to_string()
).into())
}),
                    algorithm: "Ed25519".to_string(),
                    is_valid: true,
                },
            ],
            nonce_samples: vec![
                b"test_nonce_01".to_vec(),
                b"test_nonce_02".to_vec(),
                b"test_nonce_03".to_vec(),
            ],
        }
    }

    pub fn get_ed25519_keypair(&self, index: usize) -> Option<&Ed25519KeyPair> {
        self.ed25519_keypairs.get(index)
    }

    pub fn get_encrypted_sample(&self, index: usize) -> Option<&EncryptedSample> {
        self.encrypted_samples.get(index)
    }
}

impl NetworkFixtures {
    fn create_samples() -> Self {
        Self {
            http_requests: vec![
                HttpRequestSample {
                    method: "GET".to_string(),
                    path: "/api/v1/health".to_string(),
                    headers: [("Accept".to_string(), "application/json".to_string())].into(),
                    body: None,
                    expected_status: 200,
                },
                HttpRequestSample {
                    method: "POST".to_string(),
                    path: "/api/v1/auth/login".to_string(),
                    headers: [("Content-Type".to_string(), "application/json".to_string())].into(),
                    body: Some(json!({
                        "username": "testuser",
                        "password": "testpass123"
                    })),
                    expected_status: 200,
                },
            ],
            api_responses: vec![
                ApiResponseSample {
                    status_code: 200,
                    headers: [("Content-Type".to_string(), "application/json".to_string())].into(),
                    body: json!({
                        "status": "success",
                        "message": "API is healthy"
                    }),
                    response_time_ms: 45,
                },
            ],
            network_events: vec![
                NetworkEventSample {
                    event_type: "connection_established".to_string(),
                    source_ip: "192.168.1.100".to_string(),
                    destination_ip: "192.168.1.200".to_string(),
                    payload: json!({
                        "protocol": "TCP",
                        "port": 8080
                    }),
                    timestamp: UNIX_EPOCH,
                },
            ],
            peer_configs: vec![
                PeerConfigSample {
                    peer_id: "peer_001".to_string(),
                    address: "192.168.1.50".to_string(),
                    port: 8080,
                    capabilities: vec!["crypto".to_string(), "genetics".to_string()],
                    trust_level: "high".to_string(),
                },
            ],
        }
    }
}

impl GeneticsFixtures {
    fn create_samples() -> Self {
        Self {
            genetics_configs: vec![
                GeneticsConfigSample {
                    config_id: "genetics_001".to_string(),
                    generation: 1,
                    capabilities: vec!["crypto".to_string(), "networking".to_string()],
                    fitness_score: 0.85,
                    parent_ids: vec![],
                },
            ],
            spawn_requests: vec![
                SpawnRequestSample {
                    request_id: "spawn_001".to_string(),
                    parent_genetics: "genetics_001".to_string(),
                    spawn_purpose: "security_enhancement".to_string(),
                    desired_capabilities: vec!["advanced_crypto".to_string()],
                    constraints: [("max_generation".to_string(), json!(5))].into(),
                },
            ],
            genetic_lineages: vec![
                GeneticLineageSample {
                    lineage_id: "lineage_001".to_string(),
                    generations: vec!["gen_1".to_string(), "gen_2".to_string()],
                    branching_points: vec![1, 2],
                    fitness_progression: vec![0.7, 0.85],
                },
            ],
            fitness_samples: vec![
                FitnessSample {
                    genetics_id: "genetics_001".to_string(),
                    fitness_score: 0.85,
                    calculated_at: UNIX_EPOCH,
                    factors: [
                        ("crypto_strength".to_string(), 0.9),
                        ("network_efficiency".to_string(), 0.8),
                    ].into(),
                },
            ],
        }
    }
}

impl Default for CryptoFixtures {
    fn default() -> Self {
        Self::create_samples()
    }
}

impl Default for NetworkFixtures {
    fn default() -> Self {
        Self::create_samples()
    }
}

impl Default for GeneticsFixtures {
    fn default() -> Self {
        Self::create_samples()
    }
}

impl Default for TestFixtures {
    fn default() -> Self {
        Self::new()
    }
}

static GLOBAL_FIXTURES: OnceLock<TestFixtures> = OnceLock::new();

pub fn global_fixtures() -> &'static TestFixtures {
    GLOBAL_FIXTURES.get_or_init(|| TestFixtures::new())
}

#[macro_export]
macro_rules! test_fixtures {
    () => {
        $crate::common::fixtures::global_fixtures()
    };
    ($name:expr) => {
        $crate::common::fixtures::global_fixtures().get_dataset($name)
    };
} 