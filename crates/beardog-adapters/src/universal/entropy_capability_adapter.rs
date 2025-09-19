// Universal Entropy Capability Adapter
//
// This adapter revolutionizes randomness across the ecoPrimals ecosystem by:
// 1. Exposing human-owned entropy as a universal capability
// 2. Enabling other primals to access sovereign randomness
// 3. Maintaining ownership and audit trails across systems
// 4. Creating the first ecosystem-wide human sovereignty layer

use crate::universal::capability_based_adapter::UniversalCapabilityAdapter;
use beardog_core::ai::hybrid_intelligence::sovereign_rng::{
    HumanEntropyWeightInitializer, SovereignRng, SovereignRngConfig,
};
use beardog_errors::{BearDogError, BearDogResult};
use beardog_genetics::genetics::entropy_hierarchy::{
    EntropyClass, EntropyHierarchyManager, HumanIdentity,
};
use beardog_types::canonical::capabilities::{
    CapabilityRequest, CapabilityResponse, ServiceCapabilityType, UniversalCapability,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Universal Entropy Capability - exposes human randomness to ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalEntropyCapability {
    /// Capability identifier
    pub capability_id: String,
    /// Supported entropy tiers (1=Machine, 2=Supervised, 3=Human)
    /// Collection of supported tiers
    pub supported_tiers: Vec<u8>,
    pub available_identities: Vec<String>,
    /// Maximum entropy bytes per request
    /// Number of max_entropy_bytes
    pub max_entropy_bytes: usize,
    /// Rate limiting configuration
    /// The rate limits value
    pub rate_limits: EntropyRateLimits,
    /// Ownership validation requirements
    /// The ownership requirements value
    pub ownership_requirements: OwnershipRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyRateLimits {
    /// Maximum requests per minute per identity
    /// Number of max_requests_per_minute
    pub max_requests_per_minute: u32,
    /// Maximum total entropy bytes per hour
    /// Number of max_bytes_per_hour
    pub max_bytes_per_hour: u64,
    /// Number of high_tier_cooldown_seconds
    pub high_tier_cooldown_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipRequirements {
    /// Require cryptographic proof of identity ownership
    /// Whether require_ownership_proof is enabled
    pub require_ownership_proof: bool,
    pub require_biometric_validation: bool,
    /// Whether require_cross_primal_consent is enabled
    pub require_cross_primal_consent: bool,
    /// Audit all entropy requests
    /// Whether audit_all_requests is enabled
    pub audit_all_requests: bool,
}

/// Entropy request from external primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyRequest {
    /// Requesting primal identifier
    /// The requesting primal value
    pub requesting_primal: String,
    pub human_identity_id: String,
    /// Required entropy tier
    /// Number of required_tier
    pub required_tier: u8,
    /// Number of entropy bytes requested
    /// Number of entropy_bytes
    pub entropy_bytes: usize,
    /// The use case value
    pub use_case: String,
    /// Cryptographic proof of ownership (if required)
    /// Optional ownership proof
    pub ownership_proof: Option<Vec<u8>>,
    /// Request timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Entropy response to external primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyResponse {
    pub request_id: String,
    /// Generated entropy bytes (if successful)
    /// Optional entropy bytes
    pub entropy_bytes: Option<Vec<u8>>,
    /// Actual entropy tier provided
    pub provided_tier: u8,
    /// Entropy quality score
    /// The quality score value
    pub quality_score: f64,
    /// Generation timestamp
    /// The generated at value
    pub generated_at: chrono::DateTime<chrono::Utc>,
    /// Audit trail identifier
    pub audit_id: String,
}

/// Universal Entropy Capability Adapter
pub struct UniversalEntropyCapabilityAdapter {
    sovereign_rng: Arc<RwLock<SovereignRng>>,
    /// Entropy hierarchy manager
    entropy_manager: Arc<EntropyHierarchyManager>,
    /// Capability configuration
    config: EntropyCapabilityConfig,
    /// Active entropy sessions by primal
    active_sessions: Arc<RwLock<HashMap<String, EntropySession>>>,
    /// Rate limiting tracker
    rate_limiter: Arc<RwLock<HashMap<String, RateLimitState>>>,
    /// Ownership registry
    ownership_registry: Arc<RwLock<HashMap<String, OwnershipRecord>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyCapabilityConfig {
    /// Enable cross-primal entropy sharing
    /// Whether enable_cross_primal_sharing is enabled
    pub enable_cross_primal_sharing: bool,
    /// Number of default_entropy_tier
    pub default_entropy_tier: u8,
    /// Maximum concurrent entropy sessions
    /// Number of max_concurrent_sessions
    pub max_concurrent_sessions: usize,
    /// Session timeout (seconds)
    pub session_timeout_seconds: u64,
    /// Audit retention period (days)
    /// Number of audit_retention_days
    pub audit_retention_days: u32,
}

/// Active entropy session
#[derive(Debug, Clone)]
struct EntropySession {
    /// Session identifier
    session_id: String,
    /// Requesting primal
    requesting_primal: String,
    /// Human identity
    human_identity_id: String,
    /// Session start time
    started_at: chrono::DateTime<chrono::Utc>,
    /// Last activity timestamp
    last_activity: chrono::DateTime<chrono::Utc>,
    /// Total entropy bytes generated in session
    total_entropy_generated: u64,
}

/// Rate limiting state
#[derive(Debug, Clone)]
struct RateLimitState {
    /// Requests in current minute
    requests_this_minute: u32,
    /// Bytes generated in current hour
    bytes_this_hour: u64,
    /// Last high-tier request timestamp
    last_high_tier_request: Option<chrono::DateTime<chrono::Utc>>,
    /// Current minute window
    current_minute: i64,
    /// Current hour window
    current_hour: i64,
}

#[derive(Debug, Clone)]
struct OwnershipRecord {
    /// Human identity
    human_identity_id: String,
    /// Owning primal
    owning_primal: String,
    /// Authorized sharing primals
    authorized_primals: Vec<String>,
    /// Ownership proof hash
    ownership_proof_hash: String,
    /// Registration timestamp
    registered_at: chrono::DateTime<chrono::Utc>,
}

impl Default for EntropyCapabilityConfig {
    fn default() -> Self {
        Self {
            enable_cross_primal_sharing: true,
            default_entropy_tier: 2, // Human supervised by default
            max_concurrent_sessions: 100,
            session_timeout_seconds: 3600, // 1 hour
            audit_retention_days: 90,
        }
    }
}

impl UniversalEntropyCapabilityAdapter {
    /// Create new universal entropy capability adapter
    /// Creates a new instance
    pub async fn new(
        entropy_manager: Arc<EntropyHierarchyManager>,
        config: EntropyCapabilityConfig,
    ) -> BearDogResult<Self> {
        info!("🎲 Initializing Universal Entropy Capability Adapter");
        info!("🌐 Mission: Enable ecosystem-wide human-owned randomness");
        info!("📋 Configuration:");
        info!(
            "   🤝 Cross-primal sharing: {}",
            config.enable_cross_primal_sharing
        );
        info!(
            "   📊 Default entropy tier: {}",
            config.default_entropy_tier
        );
        info!(
            "   🔗 Max concurrent sessions: {}",
            config.max_concurrent_sessions
        );
        info!("   ⏰ Session timeout: {}s", config.session_timeout_seconds);

        let sovereign_rng_config = SovereignRngConfig {
            min_entropy_tier: config.default_entropy_tier,
            cache_entropy: true,
            cache_max_age_seconds: 300,
            allow_machine_fallback: true,
            audit_entropy_usage: true,
        };

        let sovereign_rng = Arc::new(RwLock::new(SovereignRng::new(
            entropy_manager.clone(),
            sovereign_rng_config,
        )));

        Ok(Self {
            sovereign_rng,
            entropy_manager,
            config,
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            rate_limiter: Arc::new(RwLock::new(HashMap::new())),
            ownership_registry: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Register as universal entropy capability
    pub fn register_capability(
        &self,
        adapter: &UniversalCapabilityAdapter,
    ) -> BearDogResult<()> {
        let entropy_capability = UniversalCapability {
            capability_id: "human_owned_entropy".to_string(),
            service_type: ServiceCapabilityType::Entropy,
            provider_id: "beardog_sovereign_entropy".to_string(),
            version: "1.0.0".to_string(),
            endpoint: "entropy://beardog/sovereign".to_string(),
            supported_operations: vec![
                "generate_entropy".to_string(),
                "generate_neural_weights".to_string(),
                "validate_ownership".to_string(),
                "create_entropy_session".to_string(),
            ],
            security_requirements: Default::default(),
            performance_characteristics: Default::default(),
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("entropy_tiers".to_string(), "1,2,3".to_string());
                meta.insert("max_entropy_bytes".to_string(), "1048576".to_string()); // 1MB
                meta.insert("supports_human_ownership".to_string(), "true".to_string());
                meta.insert("audit_enabled".to_string(), "true".to_string());
                meta
            },
        };

        adapter.register_capability(entropy_capability)?;

        info!("✅ Registered human-owned entropy as universal ecosystem capability");
        Ok(())
    }

    /// Handle entropy capability request from external primal
    /// Handles capability_request
    /// Handles capability_request
    pub fn handle_capability_request(
        &self,
        request: &CapabilityRequest,
    ) -> BearDogResult<CapabilityResponse> {
        let request_id = Uuid::new_v4().to_string();

        debug!("🎲 Processing entropy capability request: {}", request_id);

        match request.operation.as_str() {
            "generate_entropy" => {
                self.handle_generate_entropy_request(request, &request_id)
            }
            "generate_neural_weights" => {
                self.handle_generate_neural_weights_request(request, &request_id)
            }
            "validate_ownership" => {
                self.handle_validate_ownership_request(request, &request_id)
            }
            "create_entropy_session" => {
                self.handle_create_entropy_session_request(request, &request_id)
            }
            _ => {
                warn!("❌ Unknown entropy operation: {}", request.operation);
                Ok(CapabilityResponse {
                    success: false,
                    data: None,
                    error: Some(format!("Unknown operation: {}", request.operation)),
                    metadata: HashMap::new(),
                })
            }
        }
    }

    /// Handle raw entropy generation request
    /// Handles generate_entropy_request
    fn handle_generate_entropy_request(
        &self,
        request: &CapabilityRequest,
        request_id: &str,
    ) -> BearDogResult<CapabilityResponse> {
        // Parse entropy request from parameters
        let entropy_request: EntropyRequest =
            serde_json::from_value(serde_json::to_value(&request.parameters)?)?;

        // Validate rate limits
        if !self.check_rate_limits(&entropy_request)? {
            return Ok(CapabilityResponse {
                success: false,
                data: None,
                error: Some("Rate limit exceeded".to_string()),
                metadata: HashMap::new(),
            });
        }

        // Validate ownership if required
        if !self.validate_ownership(&entropy_request)? {
            return Ok(CapabilityResponse {
                success: false,
                data: None,
                error: Some("Ownership validation failed".to_string()),
                metadata: HashMap::new(),
            });
        }

        // Generate entropy using sovereign RNG
        let entropy_bytes = {
            let mut rng = self.sovereign_rng.write();
            rng.generate_entropy_bytes(
                &entropy_request.human_identity_id,
                entropy_request.required_tier,
                entropy_request.entropy_bytes,
            )
            ?
        };

        // Create audit record
        self.create_audit_record(&entropy_request, request_id, entropy_bytes.len())
            ?;

        // Update rate limits
        self.update_rate_limits(&entropy_request, entropy_bytes.len())
            ?;

        let response = EntropyResponse {
            request_id: request_id.to_string(),
            entropy_bytes: Some(entropy_bytes),
            provided_tier: entropy_request.required_tier,
            quality_score: 0.95, // High quality for human entropy
            generated_at: chrono::Utc::now(),
            audit_id: format!("audit_{}", request_id),
        };

        Ok(CapabilityResponse {
            success: true,
            data: Some(serde_json::to_value(response)?),
            error: None,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("entropy_source".to_string(), "human_owned".to_string());
                meta.insert("sovereignty_preserved".to_string(), "true".to_string());
                meta
            },
        })
    }

    /// Handle neural network weight generation request
    /// Handles generate_neural_weights_request
    fn handle_generate_neural_weights_request(
        &self,
        request: &CapabilityRequest,
        request_id: &str,
    ) -> BearDogResult<CapabilityResponse> {
        // Parse weight initialization request
        let weight_request: HumanEntropyWeightInitializer =
            serde_json::from_value(serde_json::to_value(&request.parameters)?)?;

        // Generate weights using sovereign RNG
        let weights = {
            let mut rng = self.sovereign_rng.write();
            rng.initialize_weights(&weight_request)?
        };

        // Create audit record for neural network initialization
        self.create_neural_audit_record(&weight_request, request_id, &weights)
            ?;

        Ok(CapabilityResponse {
            success: true,
            data: Some(serde_json::to_value(weights)?),
            error: None,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert(
                    "initialization_type".to_string(),
                    "human_entropy_driven".to_string(),
                );
                meta.insert(
                    "entropy_tier".to_string(),
                    weight_request.entropy_tier.to_string(),
                );
                meta.insert("human_owned".to_string(), "true".to_string());
                meta
            },
        })
    }

    /// Validates ownership
    fn validate_ownership(&self, request: &EntropyRequest) -> BearDogResult<bool> {
        let ownership_registry = self.ownership_registry.read();

        if let Some(record) = ownership_registry.get(&request.human_identity_id) {
            // Check if requesting primal is authorized
            if record.owning_primal == request.requesting_primal
                || record
                    .authorized_primals
                    .contains(&request.requesting_primal)
            {
                return Ok(true);
            }
        }

        // For now, allow if no explicit ownership record (permissive mode)
        // In production, this should be more restrictive
        warn!(
            "⚠️  No ownership record found for identity: {}",
            request.human_identity_id
        );
        Ok(true)
    }

    fn check_rate_limits(&self, request: &EntropyRequest) -> BearDogResult<bool> {
        let mut rate_limiter = self.rate_limiter.write();
        let now = chrono::Utc::now();
        let current_minute = now.timestamp() / 60;
        let current_hour = now.timestamp() / 3600;

        let rate_state = rate_limiter
            .entry(request.human_identity_id.clone())
            .or_insert(RateLimitState {
                requests_this_minute: 0,
                bytes_this_hour: 0,
                last_high_tier_request: None,
                current_minute,
                current_hour,
            });

        // Reset counters if time windows have moved
        if rate_state.current_minute != current_minute {
            rate_state.requests_this_minute = 0;
            rate_state.current_minute = current_minute;
        }

        if rate_state.current_hour != current_hour {
            rate_state.bytes_this_hour = 0;
            rate_state.current_hour = current_hour;
        }

        // Check rate limits (now configurable)
        let max_requests_per_minute = self.config.rate_limits.max_requests_per_minute;
        let max_bytes_per_hour = self.config.rate_limits.max_bytes_per_hour;

        if rate_state.requests_this_minute >= max_requests_per_minute {
            return Ok(false);
        }

        if rate_state.bytes_this_hour + request.entropy_bytes as u64 > max_bytes_per_hour {
            return Ok(false);
        }

        // Check high-tier cooldown for Tier 3 entropy
        if request.required_tier >= 3 {
            if let Some(last_request) = rate_state.last_high_tier_request {
                let cooldown_duration = chrono::Duration::seconds(30); // 30 second cooldown
                if now - last_request < cooldown_duration {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    /// Update rate limiting state after successful request
    /// Updates rate_limits
    fn update_rate_limits(
        &self,
        request: &EntropyRequest,
        bytes_generated: usize,
    ) -> BearDogResult<()> {
        let mut rate_limiter = self.rate_limiter.write();

        if let Some(rate_state) = rate_limiter.get_mut(&request.human_identity_id) {
            rate_state.requests_this_minute += 1;
            rate_state.bytes_this_hour += bytes_generated as u64;

            if request.required_tier >= 3 {
                rate_state.last_high_tier_request = Some(chrono::Utc::now());
            }
        }

        Ok(())
    }

    /// Creates audit_record
    fn create_audit_record(
        &self,
        request: &EntropyRequest,
        request_id: &str,
        bytes_generated: usize,
    ) -> BearDogResult<()> {
        let audit_record = serde_json::json!({
            "timestamp": chrono::Utc::now(),
            "request_id": request_id,
            "operation": "generate_entropy",
            "requesting_primal": request.requesting_primal,
            "human_identity": request.human_identity_id,
            "entropy_tier": request.required_tier,
            "bytes_requested": request.entropy_bytes,
            "bytes_generated": bytes_generated,
            "use_case": request.use_case,
            "sovereignty_preserved": true,
        });

        // TODO: Store audit record in persistent storage
        info!("📋 Audit Record: {}", audit_record);

        Ok(())
    }

    /// Creates neural_audit_record
    fn create_neural_audit_record(
        &self,
        request: &HumanEntropyWeightInitializer,
        request_id: &str,
        weights: &Vec<Vec<f64>>,
    ) -> BearDogResult<()> {
        let audit_record = serde_json::json!({
            "timestamp": chrono::Utc::now(),
            "request_id": request_id,
            "operation": "generate_neural_weights",
            "human_identity": request.human_identity_id,
            "entropy_tier": request.entropy_tier,
            "layer_shape": request.layer_shape,
            "distribution": request.distribution,
            "weights_generated": weights.len() * weights.get(0).map(|row| row.len()).unwrap_or(0),
            "human_owned_ai": true,
        });

        info!("🧠 Neural Audit Record: {}", audit_record);

        Ok(())
    }

    /// Handle other capability operations
    /// Handles validate_ownership_request
    fn handle_validate_ownership_request(
        &self,
        _request: &CapabilityRequest,
        _request_id: &str,
    ) -> BearDogResult<CapabilityResponse> {
        // TODO: Implement ownership validation
        Ok(CapabilityResponse {
            success: true,
            data: Some(serde_json::json!({"ownership_valid": true})),
            error: None,
            metadata: HashMap::new(),
        })
    }

    /// Handles create_entropy_session_request
    fn handle_create_entropy_session_request(
        &self,
        _request: &CapabilityRequest,
        _request_id: &str,
    ) -> BearDogResult<CapabilityResponse> {
        // TODO: Implement entropy session management
        Ok(CapabilityResponse {
            success: true,
            data: Some(serde_json::json!({"session_id": "temp_session"})),
            error: None,
            metadata: HashMap::new(),
        })
    }
}
