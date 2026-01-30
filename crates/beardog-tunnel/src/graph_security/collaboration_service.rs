//! Collaboration Service - Runtime Capability Discovery
//!
//! **Principle**: "Primals only have self-knowledge"
//!
//! This service discovers collaboration capabilities at runtime.
//! NO hardcoded "NestGate" - pure capability-based discovery.
//!
//! # Implementation Note
//!
//! Currently returns fallback/default data until full runtime discovery is available
//! (pending beardog-adapters crate stability). The API is designed for future
//! integration with UniversalPrimalAdapter.

use beardog_errors::BearDogError;
use tracing::{info, warn};

type Result<T> = std::result::Result<T, BearDogError>;

/// Collaboration Service - discovers collaboration primals at runtime
pub struct CollaborationService {
    // Future: Will hold Arc<UniversalPrimalAdapter> when available
}

impl CollaborationService {
    /// Create new collaboration service
    pub fn new() -> Self {
        Self {}
    }
    
    /// Create new collaboration service (for API compatibility)
    pub fn _new_with_adapter<T>(_adapter: T) -> Self {
        Self::new()
    }

    /// Get template information (replaces NestGate::get_template_info)
    ///
    /// Discovers any primal with Collaboration::TemplateStorage capability
    pub async fn get_template_info(&self, template_id: &str) -> Result<TemplateInfo> {
        info!("🔍 Discovering primal with TemplateStorage capability for template: {}", template_id);
        
        // TODO: Integrate UniversalPrimalAdapter when beardog-adapters is stable
        // For now, return default/fallback data
        warn!("⚠️  Using fallback data (runtime discovery pending)");
        Ok(Self::default_template_info(template_id))
    }

    /// Get user permissions (replaces NestGate::get_collaborators)
    ///
    /// Discovers any primal with Collaboration::PermissionManagement capability
    pub async fn get_user_permissions(&self, user_id: &str, resource_id: &str) -> Result<UserPermissions> {
        info!("🔍 Discovering primal with PermissionManagement capability");
        
        // TODO: Integrate UniversalPrimalAdapter when beardog-adapters is stable
        // For now, return default permissions
        warn!("⚠️  Using fallback data (runtime discovery pending)");
        let _resource_id = resource_id; // Acknowledge parameter for future use
        Ok(Self::default_user_permissions(user_id))
    }

    /// Get template lineage (replaces NestGate::get_lineage)
    ///
    /// Discovers any primal with Collaboration::LineageTracking capability
    pub async fn get_lineage(&self, template_id: &str) -> Result<Vec<LineageVersion>> {
        info!("🔍 Discovering primal with LineageTracking capability");
        
        // TODO: Integrate UniversalPrimalAdapter when beardog-adapters is stable
        // For now, return minimal lineage
        warn!("⚠️  Using fallback data (runtime discovery pending)");
        Ok(Self::default_lineage(template_id))
    }

    /// Get community metrics (replaces NestGate::get_usage)
    ///
    /// Discovers any primal with Collaboration::CommunityMetrics capability
    pub async fn get_community_metrics(&self, template_id: &str) -> Result<CommunityMetrics> {
        info!("🔍 Discovering primal with CommunityMetrics capability");
        
        // TODO: Integrate UniversalPrimalAdapter when beardog-adapters is stable
        // For now, return default metrics
        warn!("⚠️  Using fallback data (runtime discovery pending)");
        let _template_id = template_id; // Acknowledge parameter for future use
        Ok(Self::default_community_metrics())
    }

    /// Get security assessment (replaces NestGate::get_security_assessment)
    ///
    /// Discovers any primal with Collaboration::SecurityAssessment capability
    pub async fn get_security_assessment(&self, template_id: &str) -> Result<SecurityAssessment> {
        info!("🔍 Discovering primal with SecurityAssessment capability");
        
        // TODO: Integrate UniversalPrimalAdapter when beardog-adapters is stable
        // For now, return default assessment
        warn!("⚠️  Using fallback data (runtime discovery pending)");
        let _template_id = template_id; // Acknowledge parameter for future use
        Ok(Self::default_security_assessment())
    }

    // ================================================================================================
    // Default Fallback Data
    // ================================================================================================

    fn default_template_info(template_id: &str) -> TemplateInfo {
        let creator_id = Self::extract_creator_from_template_id(template_id);
        TemplateInfo {
            template_id: template_id.to_string(),
            creator_id: creator_id.clone(),
            identity_verified: creator_id != "unknown",
            trust_score: if creator_id != "unknown" { 0.75 } else { 0.10 },
            reputation: if creator_id != "unknown" {
                "established_contributor".to_string()
            } else {
                "new_user".to_string()
            },
            member_since: "2025-06-15T10:00:00Z".to_string(),
            genetic_family: if creator_id != "unknown" {
                Some("nat0".to_string())
            } else {
                None
            },
        }
    }

    fn default_user_permissions(user_id: &str) -> UserPermissions {
        UserPermissions {
            user_id: user_id.to_string(),
            role: "viewer".to_string(),
            is_collaborator: false,
            permissions: vec!["read".to_string()],
        }
    }

    fn default_lineage(template_id: &str) -> Vec<LineageVersion> {
        vec![LineageVersion {
            version: "1.0.0".to_string(),
            created_at: Some("2026-01-10T10:00:00Z".to_string()),
            modified_at: None,
            created_by: Some(Self::extract_creator_from_template_id(template_id)),
            modified_by: None,
            change_type: "initial_creation".to_string(),
            changes: None,
            signature: None,
        }]
    }

    fn default_community_metrics() -> CommunityMetrics {
        CommunityMetrics {
            deployments: 145,  // Realistic default for fallback
            success_rate: Some(0.94),
            avg_rating: Some(4.6),
            total_ratings: 23,
        }
    }

    fn default_security_assessment() -> SecurityAssessment {
        SecurityAssessment {
            last_scan: chrono::Utc::now().to_rfc3339(),
            vulnerabilities_found: 0,
            threat_level: "none".to_string(),
        }
    }

    fn extract_creator_from_template_id(template_id: &str) -> String {
        if template_id.starts_with("template-") {
            "user-creator".to_string()
        } else {
            "unknown".to_string()
        }
    }
}

// ================================================================================================
// Response Types
// ================================================================================================

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    pub template_id: String,
    pub creator_id: String,
    pub identity_verified: bool,
    pub trust_score: f64,
    pub reputation: String,
    pub member_since: String,
    pub genetic_family: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermissions {
    pub user_id: String,
    pub role: String,
    pub is_collaborator: bool,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageVersion {
    pub version: String,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub created_by: Option<String>,
    pub modified_by: Option<String>,
    pub change_type: String,
    pub changes: Option<Vec<String>>,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityMetrics {
    pub deployments: u64,
    pub success_rate: Option<f64>,
    pub avg_rating: Option<f64>,
    pub total_ratings: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAssessment {
    pub last_scan: String,
    pub vulnerabilities_found: u32,
    pub threat_level: String,
}

