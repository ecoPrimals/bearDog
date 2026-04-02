// SPDX-License-Identifier: AGPL-3.0-only
#![allow(missing_docs)] // Phase 2 quantum vocabulary types mirror domain concepts; module docs cover intent.

//! Quantum-inspired capability orchestration (experimental).
//!
//! **Important**: The “quantum” pipeline (superposition, annealing, entanglement graph) is
//! **Phase 2 research** and is not wired to real hardware or to heuristic optimization yet.
//! Capability **resolution** uses the same runtime discovery path as the rest of
//! [`crate::primal_self_knowledge`]: [`PrimalDiscovery`] (mDNS when enabled, registry when
//! configured, cached peers). Callers must supply [`PrimalDiscovery`] via
//! [`QuantumDiscoveryEngine::with_primal_discovery`]; otherwise discovery returns
//! [`BearDogError::requires_capability`](beardog_errors::BearDogError::requires_capability).

use crate::primal_self_knowledge::{DiscoveredPrimal, Endpoint, PrimalDiscovery};
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::{
    AuthConfig, AuthType, CapabilityType, CircuitBreakerConfig, EndpointConfig, HealthStatus,
    PerformanceMetrics, ProviderInfo, SecurityLevel, UniversalCapability,
};
use beardog_types::canonical::providers_unified::core::ProviderType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tracing::{debug, info, warn};
use uuid::Uuid;

type Result<T> = std::result::Result<T, BearDogError>;

/// Quantum-inspired capability discovery engine (orchestration shell over [`PrimalDiscovery`]).
pub struct QuantumDiscoveryEngine {
    /// Engine configuration
    config: QuantumDiscoveryConfig,

    metrics: QuantumMetrics,

    /// When set, [`Self::quantum_discover_capabilities`] uses ecosystem runtime discovery.
    primal_discovery: Option<Arc<PrimalDiscovery>>,
}

/// Quantum capability space representation (Phase 2 state; reserved for future heuristics).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumCapabilitySpace {
    /// Capability qubits in superposition
    capability_qubits: HashMap<CapabilityType, CapabilityQubit>,

    quantum_gates: Vec<QuantumGate>,

    /// Measurement history
    measurement_history: Vec<QuantumMeasurement>,

    coherence_time_ms: u64,
}

/// Individual capability represented as a qubit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityQubit {
    /// Capability type
    capability_type: CapabilityType,

    amplitude_0: f64,

    amplitude_1: f64,

    /// Phase relationship
    phase: f64,

    /// Quality metrics in quantum space
    quantum_quality: QuantumQuality,

    /// Last coherence check (wall clock; serde-safe)
    last_coherence_check: DateTime<Utc>,
}

/// Quantum quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumQuality {
    performance_amplitude: f64,

    /// Reliability probability amplitude
    reliability_amplitude: f64,

    /// Security probability amplitude
    security_amplitude: f64,

    /// Cost efficiency probability amplitude
    cost_amplitude: f64,

    /// Quantum entanglement strength
    entanglement_strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperpositionState {
    /// State ID
    pub id: Uuid,

    /// Capabilities in superposition
    /// Collection of capabilities
    pub capabilities: Vec<CapabilityType>,

    /// Collection of amplitudes
    pub amplitudes: Vec<f64>,

    /// Quantum interference patterns
    /// Collection of interference patterns
    pub interference_patterns: Vec<InterferencePattern>,

    /// Collapse probability
    /// The collapse probability value
    pub collapse_probability: f64,
}

/// Quantum entanglement between capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumEntanglement {
    /// First capability in entanglement
    /// The capability a value
    pub capability_a: CapabilityType,

    /// Second capability in entanglement
    /// The capability b value
    pub capability_b: CapabilityType,

    /// Entanglement strength (0.0 to 1.0)
    /// The strength value
    pub strength: f64,

    /// Entanglement type
    /// The entanglement type value
    pub entanglement_type: EntanglementType,

    /// Bell state classification
    /// The bell state value
    pub bell_state: BellState,
}

/// Types of quantum entanglement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of entanglement
pub enum EntanglementType {
    /// Capabilities that work better together
    Synergistic,

    /// Capabilities that are mutually exclusive
    Exclusive,

    Correlated,

    AntiCorrelated,
}

/// Bell state classifications
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BellState {
    /// |Φ+⟩ = (|00⟩ + |11⟩)/√2
    PhiPlus,

    /// |Φ-⟩ = (|00⟩ - |11⟩)/√2
    PhiMinus,

    /// |Ψ+⟩ = (|01⟩ + |10⟩)/√2
    PsiPlus,

    /// |Ψ-⟩ = (|01⟩ - |10⟩)/√2
    PsiMinus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumGate {
    /// Gate type
    /// The gate type value
    pub gate_type: QuantumGateType,

    /// Target qubits
    /// Collection of target qubits
    pub target_qubits: Vec<CapabilityType>,

    /// Gate parameters
    /// Collection of parameters
    pub parameters: Vec<f64>,

    /// Gate matrix representation
    /// Collection of matrix
    pub matrix: Vec<Vec<f64>>,
}

/// Types of quantum gates
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of quantum gate
pub enum QuantumGateType {
    Hadamard,

    PauliX,

    PauliY,

    PauliZ,

    CNOT,

    Toffoli,

    Custom(String),
}

/// Quantum measurement result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumMeasurement {
    /// Measurement ID
    pub id: Uuid,

    /// Measured capabilities
    /// Collection of capabilities
    pub capabilities: Vec<CapabilityType>,

    /// Measurement results (collapsed states)
    /// Collection of results
    pub results: Vec<bool>,

    /// Measurement probability
    /// The probability value
    pub probability: f64,

    /// Measurement timestamp
    pub timestamp: DateTime<Utc>,

    /// Quantum decoherence after measurement
    pub decoherence_time_ms: u64,
}

/// Interference pattern in quantum space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterferencePattern {
    /// Pattern type
    /// The pattern type value
    pub pattern_type: InterferenceType,

    /// Amplitude modulation
    /// The amplitude value
    pub amplitude: f64,

    /// Phase shift
    /// The phase shift value
    pub phase_shift: f64,

    /// Frequency of oscillation
    /// The frequency value
    pub frequency: f64,
}

/// Types of quantum interference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of interference
pub enum InterferenceType {
    /// Constructive interference (capabilities enhance each other)
    Constructive,

    /// Destructive interference (capabilities interfere with each other)
    Destructive,

    /// Partial interference (mixed enhancement/interference)
    Partial,
}

#[derive(Debug, Clone)]
pub struct QuantumDiscoveryConfig {
    /// Maximum superposition states to maintain.
    pub max_superposition_states: usize,

    /// Quantum coherence time in milliseconds
    pub coherence_time_ms: u64,

    /// The measurement threshold value
    pub measurement_threshold: f64,

    /// Enable quantum error correction.
    pub enable_error_correction: bool,

    /// Quantum annealing temperature
    /// The annealing temperature value
    pub annealing_temperature: f64,

    /// Maximum entanglement distance.
    pub max_entanglement_distance: u32,
}

impl Default for QuantumDiscoveryConfig {
    fn default() -> Self {
        Self {
            max_superposition_states: 100,
            coherence_time_ms: 1000,
            measurement_threshold: 0.5,
            enable_error_correction: false,
            annealing_temperature: 1.0,
            max_entanglement_distance: 10,
        }
    }
}

/// Quantum discovery metrics
#[derive(Debug, Default)]
pub struct QuantumMetrics {
    pub measurements_performed: u64,

    /// Superposition states created.
    pub superposition_states_created: u64,

    /// Entanglements discovered.
    pub entanglements_discovered: u64,

    /// Average quantum coherence time
    pub avg_coherence_time_ms: f64,

    /// Quantum speedup factor over classical discovery
    /// The quantum speedup factor value
    pub quantum_speedup_factor: f64,
}

impl QuantumDiscoveryEngine {
    /// Create a new engine without [`PrimalDiscovery`]. [`Self::quantum_discover_capabilities`]
    /// will return `BearDogError::requires_capability` until [`Self::with_primal_discovery`] is used.
    #[must_use]
    pub fn new(config: QuantumDiscoveryConfig) -> Self {
        info!(
            "Initializing quantum-inspired discovery orchestration (PrimalDiscovery not attached)"
        );

        Self {
            config,
            metrics: QuantumMetrics::default(),
            primal_discovery: None,
        }
    }

    /// Attach runtime [`PrimalDiscovery`] (same pattern as [`crate::primal_self_knowledge::PrimalSelfKnowledge`]).
    #[must_use]
    pub fn with_primal_discovery(
        config: QuantumDiscoveryConfig,
        primal_discovery: Arc<PrimalDiscovery>,
    ) -> Self {
        info!("Quantum-inspired discovery orchestration bound to PrimalDiscovery");
        Self {
            config,
            metrics: QuantumMetrics::default(),
            primal_discovery: Some(primal_discovery),
        }
    }

    /// Resolve requested capabilities using [`PrimalDiscovery`] (mDNS / registry / cache).
    ///
    /// Does not fabricate endpoints or provider identities. Phase 2 quantum heuristics
    /// (superposition collapse, annealing) are not applied to the result set yet.
    ///
    /// # Errors
    ///
    /// Returns an error if `PrimalDiscovery` was not provided via
    /// [`with_primal_discovery`](Self::with_primal_discovery), or if the underlying
    /// discovery mechanism fails.
    pub async fn quantum_discover_capabilities(
        &mut self,
        request_capabilities: Vec<CapabilityType>,
    ) -> Result<Vec<UniversalCapability>> {
        info!("Quantum discovery: resolving capabilities via PrimalDiscovery");
        debug!("Requested capabilities: {:?}", request_capabilities);

        let Some(discovery) = self.primal_discovery.as_ref() else {
            return Err(BearDogError::requires_capability(
                "primal-discovery",
                "Quantum discovery orchestration requires PrimalDiscovery; use QuantumDiscoveryEngine::with_primal_discovery",
            ));
        };

        if request_capabilities.len() > self.config.max_superposition_states {
            warn!(
                "Requested {} capabilities exceeds max_superposition_states {}; truncating",
                request_capabilities.len(),
                self.config.max_superposition_states
            );
        }

        let mut out: Vec<UniversalCapability> = Vec::new();
        let mut seen: HashSet<(String, String)> = HashSet::new();

        for capability_type in request_capabilities
            .into_iter()
            .take(self.config.max_superposition_states)
        {
            let token = capability_type.discovery_env_token();
            let primals = discovery.discover_by_capability(&token).await?;
            for primal in primals {
                if primal.endpoints.is_empty() {
                    warn!(
                        "Discovered primal {} advertises no endpoints; skipping",
                        primal.name
                    );
                    continue;
                }
                for ep in &primal.endpoints {
                    let uc = universal_capability_from_discovered_primal(
                        &primal,
                        capability_type.clone(),
                        ep,
                    )?;
                    let key = (
                        uc.provider.provider_id.clone(),
                        uc.endpoint.base_url.clone(),
                    );
                    if seen.insert(key) {
                        out.push(uc);
                    }
                }
            }
        }

        self.metrics.measurements_performed += 1;
        info!(
            "Quantum discovery complete: {} capability advertisements collected",
            out.len()
        );
        Ok(out)
    }

    /// Quantum entanglement graph over capability relationships — not implemented.
    ///
    /// # Errors
    ///
    /// Always returns a not-implemented error.
    pub fn create_quantum_entanglement(
        &mut self,
        _capability_a: CapabilityType,
        _capability_b: CapabilityType,
        _entanglement_type: EntanglementType,
    ) -> Result<QuantumEntanglement> {
        Err(BearDogError::not_implemented(
            "Quantum entanglement graph (historical correlations / dependency analysis)",
        ))
    }

    /// Quantum annealing over candidate capabilities — not implemented.
    ///
    /// # Errors
    ///
    /// Always returns a not-implemented error.
    pub fn quantum_anneal_selection(
        &self,
        _candidates: Vec<UniversalCapability>,
        _optimization_criteria: Vec<OptimizationCriterion>,
    ) -> Result<Vec<UniversalCapability>> {
        Err(BearDogError::not_implemented(
            "Quantum annealing selection over discovered capabilities",
        ))
    }
}

/// Map a discovered primal endpoint into a canonical [`UniversalCapability`].
fn universal_capability_from_discovered_primal(
    primal: &DiscoveredPrimal,
    capability_type: CapabilityType,
    endpoint: &Endpoint,
) -> Result<UniversalCapability> {
    let base_url = endpoint.url();
    let version = "0".to_string();

    Ok(UniversalCapability {
        capability_type,
        provider: ProviderInfo {
            provider_id: primal.name.clone(),
            provider_name: primal.name.clone(),
            provider_type: ProviderType::Generic,
            version,
            region: None,
        },
        endpoint: EndpointConfig {
            base_url,
            api_version: None,
            timeout_ms: 30_000,
            max_retries: 3,
            circuit_breaker: CircuitBreakerConfig::default(),
        },
        auth_config: AuthConfig {
            auth_type: AuthType::None,
            api_key: None,
            bearer_token: None,
            cert_path: None,
            custom_params: HashMap::new(),
        },
        health_status: HealthStatus::Unknown,
        performance: PerformanceMetrics::default(),
        security_level: SecurityLevel::Standard,
        metadata: HashMap::new(),
    })
}

impl QuantumCapabilitySpace {
    /// Create new quantum capability space
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            capability_qubits: HashMap::new(),
            quantum_gates: Vec::new(),
            measurement_history: Vec::new(),
            coherence_time_ms: 1000, // 1 second default coherence
        }
    }
}

impl Default for QuantumCapabilitySpace {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum OptimizationCriterion {
    /// Minimize response latency
    MinimizeLatency,

    /// Maximize throughput
    MaximizeThroughput,

    /// Minimize cost
    MinimizeCost,

    /// Maximize reliability
    MaximizeReliability,

    MaximizeSecurity,

    /// Custom optimization function
    Custom(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primal_self_knowledge::{PrimalIdentity, PrimalIdentityEnvInputs};

    #[tokio::test]
    async fn discover_without_primal_discovery_is_not_implemented() {
        let mut engine = QuantumDiscoveryEngine::new(QuantumDiscoveryConfig::default());
        let err = engine
            .quantum_discover_capabilities(vec![CapabilityType::Security])
            .await
            .expect_err("expected not implemented");
        let msg = format!("{err}");
        assert!(
            msg.contains("primal-discovery")
                || msg.contains("PrimalDiscovery")
                || msg.contains("Requires capability"),
            "{msg}"
        );
    }

    #[tokio::test]
    async fn discover_with_primal_discovery_returns_empty_without_peers() {
        let identity =
            PrimalIdentity::from_inputs(&PrimalIdentityEnvInputs::default()).expect("identity");
        let discovery = Arc::new(PrimalDiscovery::new(identity));
        let mut engine = QuantumDiscoveryEngine::with_primal_discovery(
            QuantumDiscoveryConfig::default(),
            discovery,
        );
        let caps = engine
            .quantum_discover_capabilities(vec![CapabilityType::Security])
            .await
            .expect("discovery");
        assert!(caps.is_empty());
    }
}
