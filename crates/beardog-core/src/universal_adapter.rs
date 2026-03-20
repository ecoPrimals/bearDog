// SPDX-License-Identifier: AGPL-3.0-only

//! Universal Adapter for Primal-to-Primal Communication
//!
//! **Core Principle**: "Primals only know themselves, discover others by capability"
//!
//! This module implements the Universal Adapter pattern - a single, unified interface
//! for all inter-primal communication that eliminates hardcoded primal names, vendor
//! assumptions, and network topology knowledge.
//!
//! # Philosophy: Infant Discovery
//!
//! Like an infant, primals start with zero knowledge:
//! - Know only themselves (via `PrimalSelfKnowledge`)
//! - Discover others by capability (not by name)
//! - Learn infrastructure at runtime (K8s, Consul, bare metal, etc.)
//! - Adapt to what's available (graceful degradation)
//!
//! # Architecture
//!
//! ```text
//!  ┌────────────────────────────────────────────────────────┐
//!  │              Universal Adapter                         │
//!  │  "Single interface for all primal communication"       │
//!  └────────────────────────────────────────────────────────┘
//!                          ↓
//!         ┌────────────────┼───────────────┐
//!         ↓                ↓               ↓
//!   Self-Knowledge    Discovery        Routing
//!   "Who am I?"       "Who provides?"  "How to reach?"
//!         ↓                ↓               ↓
//!   [BearDog]        [Capability]     [HighestTrust]
//!   [v0.9.0]         [AI: Squirrel]   [LeastLoaded]
//!   [8900]           [Storage: ...]   [LowestLatency]
//! ```
//!
//! # Usage Example
//!
//! ```rust,no_run
//! use beardog_core::universal_adapter::UniversalAdapter;
//! use beardog_core::self_knowledge::SimpleCapability;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Create adapter with zero initial knowledge
//! let adapter = UniversalAdapter::new()?;
//!
//! // Find ANY primal providing AI capability (don't care who!)
//! let ai_primal = adapter
//!     .find_primal_by_capability(SimpleCapability::Discovery)
//!     .await?;
//!
//! println!("Using AI primal: {}", ai_primal.name);
//! // Could be: "Squirrel", "FutureAIPrimal", or any AI provider
//! // BearDog discovered it automatically!
//! # Ok(())
//! # }
//! ```

use crate::capability_router::{CapabilityRouter, RequestContext, SelectionStrategy};
use crate::primal_discovery::{DiscoveredPrimal, DiscoveryQuery, PrimalDiscovery};
use crate::self_knowledge::{PrimalSelfKnowledge, SimpleCapability};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Injected options for [`UniversalAdapter`] (cache TTL, etc.). No environment reads in [`Default`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UniversalAdapterEnvInputs {
    /// `UNIVERSAL_ADAPTER_CACHE_TTL_SECS`
    pub cache_ttl_secs: Option<u64>,
}

impl UniversalAdapterEnvInputs {
    /// Read `UNIVERSAL_ADAPTER_CACHE_TTL_SECS` via `std::env::var`.
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            cache_ttl_secs: std::env::var("UNIVERSAL_ADAPTER_CACHE_TTL_SECS")
                .ok()
                .and_then(|s| s.parse().ok()),
        }
    }

    #[must_use]
    fn cache_ttl(&self) -> Duration {
        self.cache_ttl_secs
            .map(Duration::from_secs)
            .unwrap_or_else(|| Duration::from_secs(300))
    }
}

// =============================================================================
// CORE TYPES
// =============================================================================

/// Cached primal information
#[derive(Debug, Clone)]
struct CachedPrimal {
    /// Discovered primal
    primal: DiscoveredPrimal,

    /// When it was cached
    cached_at: Instant,

    /// Cache TTL
    ttl: Duration,
}

impl CachedPrimal {
    /// Check if cache entry is still valid
    fn is_valid(&self) -> bool {
        self.cached_at.elapsed() < self.ttl
    }
}

/// Universal adapter for primal-to-primal communication
///
/// **Philosophy**: Eliminates hardcoded primal names, vendor assumptions, and network topology
///
/// # Example: Complete Zero-Knowledge Flow
///
/// ```rust,no_run
/// use beardog_core::universal_adapter::UniversalAdapter;
/// use beardog_core::self_knowledge::SimpleCapability;
///
/// # async fn example() -> Result<(), beardog_errors::BearDogError> {
/// // 1. Start with zero knowledge
/// let adapter = UniversalAdapter::new()?;
///
/// // 2. Need AI analysis (don't know who provides it)
/// let ai_primals = adapter
///     .discover_capability(SimpleCapability::Discovery)
///     .await?;
///
/// println!("Found {} AI providers", ai_primals.len());
/// // BearDog discovered Squirrel (or whoever) automatically!
///
/// // 3. Get best provider (by trust, load, latency)
/// let best_ai = adapter
///     .find_primal_by_capability(SimpleCapability::Discovery)
///     .await?;
///
/// println!("Using: {} at {:?}", best_ai.name, best_ai.endpoints);
/// # Ok(())
/// # }
/// ```
pub struct UniversalAdapter {
    /// Self-knowledge (who am I?)
    self_knowledge: Arc<PrimalSelfKnowledge>,

    /// Discovery engine (who provides what?)
    discovery: Arc<RwLock<PrimalDiscovery>>,

    /// Capability router (how to reach them?)
    router: Arc<RwLock<CapabilityRouter>>,

    /// Cached capability map (performance optimization)
    capability_cache: Arc<RwLock<HashMap<SimpleCapability, Vec<CachedPrimal>>>>,

    /// Default cache TTL
    default_cache_ttl: Duration,
}

// =============================================================================
// UNIVERSAL ADAPTER IMPLEMENTATION
// =============================================================================

impl UniversalAdapter {
    /// Create universal adapter with zero initial knowledge
    ///
    /// **Infant Discovery**: Starts knowing only itself, discovers everything else
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use beardog_core::universal_adapter::UniversalAdapter;
    ///
    /// # async fn example() -> Result<(), beardog_errors::BearDogError> {
    /// // Start with zero knowledge about other primals
    /// let adapter = UniversalAdapter::new()?;
    ///
    /// // Now adapter knows:
    /// // - Self (from environment)
    /// // - Discovery method (from environment)
    /// // - Nothing else! (will discover as needed)
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Result<Self, BearDogError> {
        let self_knowledge = PrimalSelfKnowledge::discover()?;
        Self::with_self_knowledge_and_inputs(self_knowledge, UniversalAdapterEnvInputs::from_env())
    }

    /// Initialize adapter with explicit self-knowledge (no environment read for identity/endpoints).
    ///
    /// Use in tests and when configuration is loaded from non-env sources.
    pub fn with_self_knowledge(self_knowledge: PrimalSelfKnowledge) -> Result<Self, BearDogError> {
        Self::with_self_knowledge_and_inputs(self_knowledge, UniversalAdapterEnvInputs::from_env())
    }

    /// Like [`Self::with_self_knowledge`] but uses explicit adapter inputs (e.g. tests without env).
    pub fn with_self_knowledge_and_inputs(
        self_knowledge: PrimalSelfKnowledge,
        adapter_inputs: UniversalAdapterEnvInputs,
    ) -> Result<Self, BearDogError> {
        let discovery = PrimalDiscovery::from_env()?;
        Self::with_self_knowledge_discovery_and_cache(
            self_knowledge,
            discovery,
            adapter_inputs.cache_ttl(),
        )
    }

    /// Full injection: self-knowledge, discovery engine, and cache TTL (no implicit env except optional callers).
    pub fn with_self_knowledge_discovery_and_cache(
        self_knowledge: PrimalSelfKnowledge,
        discovery: PrimalDiscovery,
        default_cache_ttl: Duration,
    ) -> Result<Self, BearDogError> {
        info!("🧒 Initializing Universal Adapter (Infant Discovery Mode)");

        info!("   1️⃣  Self-knowledge provided");
        info!(
            "   ✅ I am: {} v{}",
            self_knowledge.my_name(),
            self_knowledge.my_version().version
        );

        info!("   2️⃣  Discovery engine provided");
        info!("   3️⃣  Initializing capability router...");
        let router = CapabilityRouter::new(discovery.clone());
        info!("   ✅ Routing engine ready");

        info!("✅ Universal Adapter initialized (zero hardcoded knowledge)");

        Ok(Self {
            self_knowledge: Arc::new(self_knowledge),
            discovery: Arc::new(RwLock::new(discovery)),
            router: Arc::new(RwLock::new(router)),
            capability_cache: Arc::new(RwLock::new(HashMap::new())),
            default_cache_ttl,
        })
    }

    /// Discover ALL primals providing a capability
    ///
    /// **Zero Assumptions**: Doesn't know or care which primals exist
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use beardog_core::universal_adapter::UniversalAdapter;
    /// use beardog_core::self_knowledge::SimpleCapability;
    ///
    /// # async fn example() -> Result<(), beardog_errors::BearDogError> {
    /// let adapter = UniversalAdapter::new()?;
    ///
    /// // Find ALL primals with AI capability
    /// let ai_primals = adapter
    ///     .discover_capability(SimpleCapability::Discovery)
    ///     .await?;
    ///
    /// for primal in ai_primals {
    ///     println!("AI provider: {} at {:?}", primal.name, primal.endpoints);
    ///     // Could be Squirrel, future AI primals, or anyone!
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn discover_capability(
        &self,
        capability: SimpleCapability,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        debug!("🔍 Discovering primals with capability: {:?}", capability);

        // Check cache first
        if let Some(cached) = self.get_cached_capability(&capability).await {
            debug!("💾 Using cached results for {:?}", capability);
            return Ok(cached);
        }

        // Discover primals providing this capability
        let query = DiscoveryQuery::by_capability(capability.clone());
        let primals = self.discovery.write().await.discover(query).await?;

        info!(
            "✅ Discovered {} primals providing {:?}",
            primals.len(),
            capability
        );

        // Cache results
        self.cache_capability(capability, primals.clone()).await;

        Ok(primals)
    }

    /// Find BEST primal providing a capability
    ///
    /// Uses intelligent routing (trust, load, latency) to select optimal primal
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use beardog_core::universal_adapter::UniversalAdapter;
    /// use beardog_core::self_knowledge::SimpleCapability;
    ///
    /// # async fn example() -> Result<(), beardog_errors::BearDogError> {
    /// let adapter = UniversalAdapter::new()?;
    ///
    /// // Get BEST AI provider (by trust score, load, latency)
    /// let best_ai = adapter
    ///     .find_primal_by_capability(SimpleCapability::Discovery)
    ///     .await?;
    ///
    /// println!("Best AI: {} (reason: highest trust)", best_ai.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn find_primal_by_capability(
        &self,
        capability: SimpleCapability,
    ) -> Result<DiscoveredPrimal, BearDogError> {
        debug!("🎯 Finding best primal for capability: {:?}", capability);

        // Route to best primal
        let decision = self
            .router
            .write()
            .await
            .route(
                capability.clone(),
                RequestContext::new(capability).with_strategy(SelectionStrategy::HighestTrust),
            )
            .await?;

        info!(
            "✅ Selected primal: {} (reason: {})",
            decision.primal.name, decision.reason
        );

        Ok(decision.primal)
    }

    /// Get self-knowledge (who am I?)
    #[must_use]
    pub fn self_knowledge(&self) -> &PrimalSelfKnowledge {
        &self.self_knowledge
    }

    /// Clear capability cache (force re-discovery)
    pub async fn clear_cache(&self) {
        self.capability_cache.write().await.clear();
        info!("🗑️  Capability cache cleared");
    }

    /// Clear cache for specific capability
    pub async fn clear_capability_cache(&self, capability: &SimpleCapability) {
        self.capability_cache.write().await.remove(capability);
        debug!("🗑️  Cleared cache for {:?}", capability);
    }

    // Internal: Get cached capability results
    async fn get_cached_capability(
        &self,
        capability: &SimpleCapability,
    ) -> Option<Vec<DiscoveredPrimal>> {
        let cache = self.capability_cache.read().await;

        if let Some(cached_primals) = cache.get(capability) {
            // Filter out expired entries
            let valid_primals: Vec<DiscoveredPrimal> = cached_primals
                .iter()
                .filter(|cp| cp.is_valid())
                .map(|cp| cp.primal.clone())
                .collect();

            if !valid_primals.is_empty() {
                return Some(valid_primals);
            }
        }

        None
    }

    // Internal: Cache capability results
    async fn cache_capability(&self, capability: SimpleCapability, primals: Vec<DiscoveredPrimal>) {
        let cached_primals: Vec<CachedPrimal> = primals
            .into_iter()
            .map(|primal| CachedPrimal {
                primal,
                cached_at: Instant::now(),
                ttl: self.default_cache_ttl,
            })
            .collect();

        self.capability_cache
            .write()
            .await
            .insert(capability, cached_primals);
    }
}

// =============================================================================
// CONVENIENCE METHODS
// =============================================================================

impl UniversalAdapter {
    /// Check if adapter knows about any primals providing capability
    ///
    /// **Checks cache only**: Doesn't trigger discovery
    #[must_use]
    pub async fn has_capability(&self, capability: &SimpleCapability) -> bool {
        self.get_cached_capability(capability).await.is_some()
    }

    /// Get number of known primals providing capability
    #[must_use]
    pub async fn count_capability_providers(&self, capability: &SimpleCapability) -> usize {
        self.get_cached_capability(capability)
            .await
            .map_or(0, |primals| primals.len())
    }

    /// Get all capabilities currently in cache
    #[must_use]
    pub async fn cached_capabilities(&self) -> Vec<SimpleCapability> {
        self.capability_cache.read().await.keys().cloned().collect()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primal_discovery::DiscoveryMethod;
    use crate::self_knowledge::{IdentityInputs, SelfKnowledgeInputs};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_universal_adapter_creation() {
        let sk = PrimalSelfKnowledge::discover_from_inputs(&SelfKnowledgeInputs {
            identity: IdentityInputs {
                primal_name: Some("BearDog".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .unwrap();
        let discovery = PrimalDiscovery::new(DiscoveryMethod::Environment);
        let adapter = UniversalAdapter::with_self_knowledge_discovery_and_cache(
            sk,
            discovery,
            Duration::from_secs(300),
        )
        .unwrap();

        assert_eq!(adapter.self_knowledge().my_name(), "BearDog");
        assert_eq!(adapter.cached_capabilities().await.len(), 0); // Empty cache initially
    }

    #[tokio::test]
    async fn test_discover_capability_from_environment() {
        let dir = tempfile::tempdir().unwrap();
        let sock = dir.path().join("testprimal.sock");
        std::fs::File::create(&sock).unwrap();
        let uri = format!("unix://{}", sock.display());

        let mut env = HashMap::new();
        env.insert("PRIMAL_TESTPRIMAL_ADDR".to_string(), uri);
        env.insert(
            "PRIMAL_TESTPRIMAL_CAPABILITIES".to_string(),
            "Discovery".to_string(),
        );

        let sk = PrimalSelfKnowledge::discover_from_inputs(&SelfKnowledgeInputs {
            identity: IdentityInputs {
                primal_name: Some("BearDog".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .unwrap();

        let discovery = PrimalDiscovery::new(DiscoveryMethod::Environment).with_env_override(env);
        let adapter = UniversalAdapter::with_self_knowledge_discovery_and_cache(
            sk,
            discovery,
            Duration::from_secs(300),
        )
        .unwrap();

        let primals = adapter
            .discover_capability(SimpleCapability::Discovery)
            .await
            .unwrap();

        assert!(
            primals.iter().any(|p| p.name == "testprimal"),
            "expected testprimal from env + capability filter, got {:?}",
            primals
        );
    }

    #[tokio::test]
    async fn test_cache_behavior() {
        let dir = tempfile::tempdir().unwrap();
        let sock = dir.path().join("testprimal.sock");
        std::fs::File::create(&sock).unwrap();
        let uri = format!("unix://{}", sock.display());

        let mut env = HashMap::new();
        env.insert("PRIMAL_TESTPRIMAL_ADDR".to_string(), uri);
        env.insert(
            "PRIMAL_TESTPRIMAL_CAPABILITIES".to_string(),
            "Discovery,SecureTunneling".to_string(),
        );

        let sk = PrimalSelfKnowledge::discover_from_inputs(&SelfKnowledgeInputs {
            identity: IdentityInputs {
                primal_name: Some("BearDog".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .unwrap();

        let discovery = PrimalDiscovery::new(DiscoveryMethod::Environment).with_env_override(env);
        let adapter = UniversalAdapter::with_self_knowledge_discovery_and_cache(
            sk,
            discovery,
            Duration::from_secs(60),
        )
        .unwrap();

        assert_eq!(adapter.cached_capabilities().await.len(), 0);

        let result = adapter
            .discover_capability(SimpleCapability::Discovery)
            .await;
        assert!(result.is_ok(), "Discovery should succeed");
        let primals = result.unwrap();

        assert_eq!(
            primals.len(),
            1,
            "Should discover 1 primal with Discovery capability, got {:?}",
            primals
        );

        assert_eq!(adapter.cached_capabilities().await.len(), 1);
        assert!(adapter.has_capability(&SimpleCapability::Discovery).await);

        adapter
            .clear_capability_cache(&SimpleCapability::Discovery)
            .await;
        assert!(!adapter.has_capability(&SimpleCapability::Discovery).await);
    }

    #[test]
    fn test_self_knowledge_access() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let sk_in = PrimalSelfKnowledge::discover_from_inputs(&SelfKnowledgeInputs {
                identity: IdentityInputs {
                    primal_name: Some("beardog".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            })
            .unwrap();
            let discovery = PrimalDiscovery::new(DiscoveryMethod::Environment);
            let adapter = UniversalAdapter::with_self_knowledge_discovery_and_cache(
                sk_in,
                discovery,
                Duration::from_secs(300),
            )
            .unwrap();

            let sk = adapter.self_knowledge();
            assert_eq!(sk.my_name(), "beardog");
        });
    }
}
