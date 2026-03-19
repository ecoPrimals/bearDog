// SPDX-License-Identifier: AGPL-3.0-only

// Canonical constants - Single source of truth
// **MODERNIZED**: All constants consolidated using domain-organized system

// MODERNIZED: Use domain-organized constants instead of deprecated unified system
pub use crate::constants::domains::system::timeouts::default_timeout_ms;
pub use crate::constants::domains::system::ConstantRegistry as UnifiedConstantRegistry;

// **MODERNIZED MODULE ACCESS** - Domain-organized constants
/// Api module
pub mod api {
    pub use crate::constants::domains::network::api::*;
}

/// Network module
pub mod network {
    pub use crate::constants::domains::network::*;
}

/// Security module
pub mod security {
    pub use crate::constants::domains::security::*;
}

pub mod performance {
    pub use crate::constants::domains::system::performance::*;
}

/// Hsm module
pub mod hsm {
    pub use crate::constants::domains::security::hsm::*;
}

/// Cache module
pub mod cache {
    pub use crate::constants::domains::system::cache::*;
}

/// Nodes module
pub mod nodes {
    pub use crate::constants::domains::network::nodes::*;
}

/// Compliance module
pub mod compliance {
    pub use crate::constants::domains::security::compliance::*;
}

/// System module
pub mod system {
    pub use crate::constants::domains::system::*;
}

/// Workflow module
pub mod workflow {
    pub use crate::constants::domains::system::workflow::*;
}

/// Genetics module
pub mod genetics {
    pub use crate::constants::domains::system::genetics::*;
}

/// Testing module
pub mod testing {
    pub use crate::constants::domains::system::testing::*;
}

/// Versions module
pub mod versions {
    pub use crate::constants::domains::system::versions::*;
}

/// Http module
pub mod http {
    pub use crate::constants::domains::network::http::*;
}

/// Services module
pub mod services {
    pub use crate::constants::domains::network::services::*;
}

/// States module
pub mod states {
    pub use crate::constants::domains::system::states::*;
}

// **CANONICAL CONSTANT REGISTRY** - Modernized alias
pub use crate::constants::domains::system::ConstantRegistry as CanonicalConstantRegistry;
