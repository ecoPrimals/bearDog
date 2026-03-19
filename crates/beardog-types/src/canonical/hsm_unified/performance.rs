// SPDX-License-Identifier: AGPL-3.0-only

// HSM Performance Configuration

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmPerformanceConfig {
    /// Batch operations enabled
    /// Whether `batch_operations` is enabled
    pub batch_operations_enabled: bool,

    /// Connection pooling enabled
    /// Whether `connection_pooling` is enabled
    pub connection_pooling_enabled: bool,

    /// Cache enabled
    /// Whether cache is enabled
    pub cache_enabled: bool,
}
