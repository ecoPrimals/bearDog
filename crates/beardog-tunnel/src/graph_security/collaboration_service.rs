// SPDX-License-Identifier: AGPL-3.0-or-later

//! Collaboration Service - Runtime Capability Discovery
//!
//! **Principle**: "Primals only have self-knowledge"
//!
//! This service discovers collaboration capabilities at runtime.
//! No hardcoded vendor primal names — capability-based discovery only.
//!
//! # Live Collaboration Network Required
//!
//! All endpoints in this module require a live collaboration network reachable via
//! runtime capability discovery (`UniversalPrimalAdapter` + discovery client). When
//! discovery is unavailable, methods return an error rather than synthetic data so
//! security audits cannot be misled by placeholder metrics or trust scores.
//!
//! # Implementation Status
//!
//! **Current**: Discovery client wiring pending (`beardog-adapters` is stable but not
//! yet integrated here).\
//! **Future**: TRUE runtime discovery via capability-based primal lookup (Principle #5).

use beardog_errors::BearDogError;
use tracing::{debug, info};

type Result<T> = std::result::Result<T, BearDogError>;

/// Collaboration Service - discovers collaboration primals at runtime
pub struct CollaborationService {
    // Future: Will hold Arc<UniversalPrimalAdapter> when available
}

impl Default for CollaborationService {
    fn default() -> Self {
        Self::new()
    }
}

impl CollaborationService {
    /// Create new collaboration service
    pub const fn new() -> Self {
        Self {}
    }

    /// Create new collaboration service (for API compatibility)
    pub fn _new_with_adapter<T>(_adapter: T) -> Self {
        Self::new()
    }

    fn discovery_unavailable(operation: &str, capability: &str) -> BearDogError {
        debug!(
            operation,
            capability,
            discovery = "UniversalPrimalAdapter + beardog-adapters capability lookup",
            "collaboration operation blocked — runtime discovery not wired"
        );
        BearDogError::not_yet_available(format!(
            "{operation} requires Collaboration::{capability} via UniversalPrimalAdapter \
             capability discovery (beardog-adapters integration pending); \
             no live collaboration network reachable"
        ))
    }

    /// Get template information (replaces legacy `get_template_info` on a collaboration provider)
    ///
    /// Discovers any primal with `Collaboration::TemplateStorage` capability.
    /// Requires a live collaboration network; returns an error when discovery is unavailable.
    ///
    /// # Errors
    ///
    /// Returns an error if the collaboration network cannot be reached or template metadata
    /// cannot be retrieved.
    pub async fn get_template_info(&self, template_id: &str) -> Result<TemplateInfo> {
        info!("Discovering primal with TemplateStorage capability for template: {template_id}");
        Err(Self::discovery_unavailable(
            "get_template_info",
            "TemplateStorage",
        ))
    }

    /// Get user permissions (replaces legacy collaborator listing on a collaboration provider)
    ///
    /// Discovers any primal with `Collaboration::PermissionManagement` capability.
    /// Requires a live collaboration network; returns an error when discovery is unavailable.
    ///
    /// # Errors
    ///
    /// Returns an error if the collaboration network cannot be reached or user permissions
    /// cannot be retrieved.
    pub async fn get_user_permissions(
        &self,
        _user_id: &str,
        _resource_id: &str,
    ) -> Result<UserPermissions> {
        info!("Discovering primal with PermissionManagement capability");
        Err(Self::discovery_unavailable(
            "get_user_permissions",
            "PermissionManagement",
        ))
    }

    /// Get template lineage (replaces legacy lineage query on a collaboration provider)
    ///
    /// Discovers any primal with `Collaboration::LineageTracking` capability.
    /// Requires a live collaboration network; returns an error when discovery is unavailable.
    ///
    /// # Errors
    ///
    /// Returns an error if the collaboration network cannot be reached or lineage cannot
    /// be retrieved.
    pub async fn get_lineage(&self, _template_id: &str) -> Result<Vec<LineageVersion>> {
        info!("Discovering primal with LineageTracking capability");
        Err(Self::discovery_unavailable(
            "get_lineage",
            "LineageTracking",
        ))
    }

    /// Get community metrics (replaces legacy usage metrics on a collaboration provider)
    ///
    /// Discovers any primal with `Collaboration::CommunityMetrics` capability.
    /// Requires a live collaboration network; returns an error when discovery is unavailable.
    ///
    /// # Errors
    ///
    /// Returns an error if the collaboration network cannot be reached or community metrics
    /// cannot be retrieved.
    pub async fn get_community_metrics(&self, _template_id: &str) -> Result<CommunityMetrics> {
        info!("Discovering primal with CommunityMetrics capability");
        Err(Self::discovery_unavailable(
            "get_community_metrics",
            "CommunityMetrics",
        ))
    }

    /// Get security assessment (replaces legacy security assessment on a collaboration provider)
    ///
    /// Discovers any primal with `Collaboration::SecurityAssessment` capability.
    /// Requires a live collaboration network; returns an error when discovery is unavailable.
    ///
    /// # Errors
    ///
    /// Returns an error if the collaboration network cannot be reached or the security
    /// assessment cannot be retrieved.
    pub async fn get_security_assessment(&self, template_id: &str) -> Result<SecurityAssessment> {
        info!("Discovering primal with SecurityAssessment capability for template: {template_id}");
        Err(Self::discovery_unavailable(
            "get_security_assessment",
            "SecurityAssessment",
        ))
    }
}

// ================================================================================================
// Response Types
// ================================================================================================

use serde::{Deserialize, Serialize};

/// Template information from collaboration capability
///
/// Contains creator identity, trust metrics, and genetic family information
/// for a template discovered via runtime capability-based discovery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    /// Unique template identifier
    pub template_id: String,
    /// Creator's user ID
    pub creator_id: String,
    /// Whether creator's identity has been verified
    pub identity_verified: bool,
    /// Trust score (0.0-1.0)
    pub trust_score: f64,
    /// Reputation level (e.g., "trusted", "new", "verified")
    pub reputation: String,
    /// ISO 8601 timestamp of when creator joined
    pub member_since: String,
    /// Optional genetic family identifier
    pub genetic_family: Option<String>,
}

/// User permissions for a resource
///
/// Returned from collaboration capability to determine user roles
/// and permissions for graph security authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermissions {
    /// User identifier
    pub user_id: String,
    /// User's role (e.g., "owner", "collaborator", "viewer")
    pub role: String,
    /// Whether user is a collaborator
    pub is_collaborator: bool,
    /// List of specific permissions granted
    pub permissions: Vec<String>,
}

/// Template lineage version information
///
/// Represents a single version in the template's evolution history,
/// including change metadata and optional cryptographic signatures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageVersion {
    /// Version identifier (e.g., "1.0.0", "v2")
    pub version: String,
    /// ISO 8601 timestamp of creation
    pub created_at: Option<String>,
    /// ISO 8601 timestamp of last modification
    pub modified_at: Option<String>,
    /// User ID of creator
    pub created_by: Option<String>,
    /// User ID of last modifier
    pub modified_by: Option<String>,
    /// Type of change (e.g., "create", "update", "fork")
    pub change_type: String,
    /// Optional list of specific changes made
    pub changes: Option<Vec<String>>,
    /// Optional Ed25519 signature of version
    pub signature: Option<String>,
}

/// Community usage metrics for a template
///
/// Aggregated metrics from the collaboration capability showing
/// deployment success rates and community ratings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityMetrics {
    /// Total number of deployments
    pub deployments: u64,
    /// Success rate as a ratio (0.0-1.0)
    pub success_rate: Option<f64>,
    /// Average rating (0.0-5.0)
    pub avg_rating: Option<f64>,
    /// Total number of ratings received
    pub total_ratings: u64,
}

/// Security assessment results
///
/// Summary of recent security scans and threat analysis
/// from the collaboration capability's security validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAssessment {
    /// ISO 8601 timestamp of last security scan
    pub last_scan: String,
    /// Number of vulnerabilities found
    pub vulnerabilities_found: u32,
    /// Threat level assessment (e.g., "none", "low", "medium", "high")
    pub threat_level: String,
}

// ================================================================================================
// Tests - Comprehensive coverage for security-critical CollaborationService
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_discovery_unavailable(err: BearDogError) {
        let message = err.to_string();
        assert!(
            message.contains("UniversalPrimalAdapter")
                || message.contains("collaboration network")
                || message.contains("Not yet available"),
            "unexpected error: {message}"
        );
    }

    // ========================================================================================
    // Service Creation Tests
    // ========================================================================================

    #[test]
    fn test_collaboration_service_new() {
        let _service = CollaborationService::new();
    }

    #[test]
    fn test_collaboration_service_default() {
        let _service = CollaborationService::default();
    }

    #[test]
    fn new_with_adapter_compatibility() {
        let _svc = CollaborationService::_new_with_adapter(());
        let _svc2 = CollaborationService::_new_with_adapter("any-adapter");
    }

    // ========================================================================================
    // Discovery Failure Tests
    // ========================================================================================

    #[tokio::test]
    async fn test_get_template_info_requires_live_network() {
        let service = CollaborationService::new();
        let err = service
            .get_template_info("template-abc123")
            .await
            .expect_err("template info should fail without collaboration network");
        assert_discovery_unavailable(err);
    }

    #[tokio::test]
    async fn test_get_user_permissions_requires_live_network() {
        let service = CollaborationService::new();
        let err = service
            .get_user_permissions("user-123", "resource-456")
            .await
            .expect_err("permissions should fail without collaboration network");
        assert_discovery_unavailable(err);
    }

    #[tokio::test]
    async fn test_get_lineage_requires_live_network() {
        let service = CollaborationService::new();
        let err = service
            .get_lineage("template-abc123")
            .await
            .expect_err("lineage should fail without collaboration network");
        assert_discovery_unavailable(err);
    }

    #[tokio::test]
    async fn test_get_community_metrics_requires_live_network() {
        let service = CollaborationService::new();
        let err = service
            .get_community_metrics("template-abc")
            .await
            .expect_err("community metrics should fail without collaboration network");
        assert_discovery_unavailable(err);
    }

    #[tokio::test]
    async fn test_get_security_assessment_requires_live_network() {
        let service = CollaborationService::new();
        let err = service
            .get_security_assessment("template-secure")
            .await
            .expect_err("security assessment should fail without collaboration network");
        assert_discovery_unavailable(err);
    }

    #[tokio::test]
    async fn test_concurrent_discovery_failures() {
        use std::sync::Arc;

        let service = Arc::new(CollaborationService::new());

        let handles: Vec<_> = (0..10)
            .map(|i| {
                let svc = Arc::clone(&service);
                tokio::spawn(async move {
                    let template_id = format!("template-{i}");
                    svc.get_template_info(&template_id).await
                })
            })
            .collect();

        for handle in handles {
            let result = handle
                .await
                .expect("concurrent get_template_info task join");
            assert_discovery_unavailable(result.expect_err("expected discovery failure"));
        }
    }

    // ========================================================================================
    // Serialization Tests
    // ========================================================================================

    #[test]
    fn test_template_info_serialization() {
        let info = TemplateInfo {
            template_id: "test".to_string(),
            creator_id: "creator".to_string(),
            identity_verified: true,
            trust_score: 0.9,
            reputation: "trusted".to_string(),
            member_since: "2025-01-01T00:00:00Z".to_string(),
            genetic_family: Some("nat0".to_string()),
        };

        let json = serde_json::to_string(&info).expect("serialize TemplateInfo");
        let deserialized: TemplateInfo =
            serde_json::from_str(&json).expect("deserialize TemplateInfo");

        assert_eq!(deserialized.template_id, info.template_id);
        assert_eq!(deserialized.trust_score, info.trust_score);
        assert_eq!(deserialized.genetic_family, info.genetic_family);
    }

    #[test]
    fn test_user_permissions_serialization() {
        let perms = UserPermissions {
            user_id: "user-1".to_string(),
            role: "admin".to_string(),
            is_collaborator: true,
            permissions: vec!["read".to_string(), "write".to_string(), "admin".to_string()],
        };

        let json = serde_json::to_string(&perms).expect("serialize UserPermissions");
        let deserialized: UserPermissions =
            serde_json::from_str(&json).expect("deserialize UserPermissions");

        assert_eq!(deserialized.user_id, perms.user_id);
        assert_eq!(deserialized.permissions.len(), 3);
    }

    #[test]
    fn test_lineage_version_serialization() {
        let version = LineageVersion {
            version: "2.0.0".to_string(),
            created_at: Some("2026-01-01T00:00:00Z".to_string()),
            modified_at: Some("2026-02-01T00:00:00Z".to_string()),
            created_by: Some("user-1".to_string()),
            modified_by: Some("user-2".to_string()),
            change_type: "update".to_string(),
            changes: Some(vec!["Added feature".to_string()]),
            signature: Some("sig-abc".to_string()),
        };

        let json = serde_json::to_string(&version).expect("serialize LineageVersion");
        let deserialized: LineageVersion =
            serde_json::from_str(&json).expect("deserialize LineageVersion");

        assert_eq!(deserialized.version, "2.0.0");
        assert!(deserialized.changes.is_some());
    }

    #[test]
    fn test_community_metrics_serialization() {
        let metrics = CommunityMetrics {
            deployments: 1000,
            success_rate: Some(0.99),
            avg_rating: Some(4.8),
            total_ratings: 500,
        };

        let json = serde_json::to_string(&metrics).expect("serialize CommunityMetrics");
        let deserialized: CommunityMetrics =
            serde_json::from_str(&json).expect("deserialize CommunityMetrics");

        assert_eq!(deserialized.deployments, 1000);
        assert_eq!(deserialized.success_rate, Some(0.99));
    }

    #[test]
    fn test_security_assessment_serialization() {
        let assessment = SecurityAssessment {
            last_scan: "2026-02-04T12:00:00Z".to_string(),
            vulnerabilities_found: 2,
            threat_level: "low".to_string(),
        };

        let json = serde_json::to_string(&assessment).expect("serialize SecurityAssessment");
        let deserialized: SecurityAssessment =
            serde_json::from_str(&json).expect("deserialize SecurityAssessment");

        assert_eq!(deserialized.vulnerabilities_found, 2);
        assert_eq!(deserialized.threat_level, "low");
    }
}
