// SPDX-License-Identifier: AGPL-3.0-or-later

// Threat Enrichment Handlers
//
// This module provides threat enrichment functionality for the BearDog threat detection system.

use beardog_errors::BearDogError;

/// Optional enrichment stage that can attach [`ExternalIntelligence`] to threats.
pub struct ThreatEnrichmentHandler {
    /// Whether feature is enabled
    pub enabled: bool,
}

impl ThreatEnrichmentHandler {
    /// Create a new enrichment handler
    /// Creates a new instance
    #[must_use]
    pub const fn new() -> Self {
        Self { enabled: true }
    }

    /// Enrich threat with external intelligence
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future enrichment failures.
    pub fn enrich_threat(
        &self,
        _threat_id: &str,
    ) -> Result<Option<ExternalIntelligence>, BearDogError> {
        if !self.enabled {
            return Ok(None);
        }

        // Basic enrichment logic
        Ok(Some(ExternalIntelligence {
            description: "enriched".to_string(),
            confidence: 0.8,
        }))
    }
}

impl Default for ThreatEnrichmentHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Logical network placement used when enriching a threat’s source or target.
#[derive(Debug, Clone)]
pub struct NetworkContextInfo {
    /// The network segment value
    pub network_segment: String,
    /// The security zone value
    pub security_zone: String,
    /// The access level value
    pub access_level: String,
    /// Whether `is_trusted` is enabled
    pub is_trusted: bool,
}

impl Default for NetworkContextInfo {
    fn default() -> Self {
        Self {
            network_segment: "unknown".to_string(),
            security_zone: "default".to_string(),
            access_level: "standard".to_string(),
            is_trusted: true,
        }
    }
}

/// Short free-text enrichment blob returned by [`ThreatEnrichmentHandler::enrich_threat`].
#[derive(Debug, Clone)]
pub struct ExternalIntelligence {
    /// The description value
    pub description: String,
    /// Confidence that the enrichment applies to the queried threat (0.0–1.0).
    pub confidence: f64,
}

impl Default for ExternalIntelligence {
    fn default() -> Self {
        Self {
            description: "none".to_string(),
            confidence: 0.0,
        }
    }
}
