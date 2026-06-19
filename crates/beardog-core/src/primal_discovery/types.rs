// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::self_knowledge::{Endpoint, SimpleCapability};
use beardog_types::constants::domains::timeouts::HEALTH_CHECK_TIMEOUT;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Discovery method for finding primals
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    /// Environment variables (`PRIMAL_&lt;NAME&gt;_ADDR`)
    Environment,

    /// Universal Primal Authority (UPA) registry
    UniversalPrimalAuthority {
        /// Registry address (Unix socket or TCP endpoint)
        registry_addr: String,
    },

    /// Multicast DNS (mDNS) service discovery
    Mdns {
        /// Service type for mDNS query (e.g., "_primal._tcp.local")
        service_type: String,
    },

    /// DNS Service Discovery (DNS-SD)
    DnsSd {
        /// Domain for DNS-SD lookup
        domain: String,
    },

    /// Multiple methods in priority order
    Multi(Vec<Self>),
}

/// Query for discovering primals
#[derive(Debug, Clone)]
pub struct DiscoveryQuery {
    /// Specific primal name (optional)
    pub name: Option<String>,

    /// Required capabilities
    pub capabilities: Vec<SimpleCapability>,

    /// Discovery timeout
    pub timeout: Duration,
}

/// Discovered primal information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredPrimal {
    /// Primal name
    pub name: String,

    /// Available endpoints
    pub endpoints: Vec<Endpoint>,

    /// Provided capabilities
    pub capabilities: Vec<SimpleCapability>,

    /// Trust score (0.0 - 1.0)
    pub trust_score: Option<f64>,

    /// Discovery timestamp
    pub discovered_at: std::time::SystemTime,
}

/// Primal discovery engine
#[derive(Clone)]
pub struct PrimalDiscovery {
    /// Discovery method
    pub(super) method: DiscoveryMethod,

    /// Cached discoveries (for future use)
    pub(super) _cache: HashMap<String, DiscoveredPrimal>,

    /// Cache TTL (for future use)
    pub(super) _cache_ttl: Duration,

    /// When set, environment-based discovery uses this map instead of reading the process environment.
    pub(super) env_override: Option<HashMap<String, String>>,
}

impl DiscoveryQuery {
    /// Create query for a specific primal by name
    #[must_use]
    pub fn by_name(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            capabilities: Vec::new(),
            timeout: HEALTH_CHECK_TIMEOUT,
        }
    }

    /// Create query for primals by capability
    #[must_use]
    pub fn by_capability(capability: SimpleCapability) -> Self {
        Self {
            name: None,
            capabilities: vec![capability],
            timeout: HEALTH_CHECK_TIMEOUT,
        }
    }

    /// Add required capability
    #[must_use]
    pub fn with_capability(mut self, capability: SimpleCapability) -> Self {
        self.capabilities.push(capability);
        self
    }

    /// Set discovery timeout
    #[must_use]
    pub const fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}
