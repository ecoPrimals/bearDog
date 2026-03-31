// SPDX-License-Identifier: AGPL-3.0-only
#![allow(missing_docs)]

//! Ecosystem integration settings for coordination.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemIntegrationConfig {
    /// Enable integration with ecosystem genetics
    /// Whether `genetics_integration` is enabled
    pub genetics_integration_enabled: bool,
    /// Enable cross-primal coordination
    /// Whether `cross_primal_coordination` is enabled
    pub cross_primal_coordination: bool,
    /// Symbiosis types supported
    /// Collection of supported symbiosis
    pub supported_symbiosis: Vec<String>,
    /// Integration endpoints
    /// Mapping of integration endpoints
    pub integration_endpoints: HashMap<String, String>,
}
