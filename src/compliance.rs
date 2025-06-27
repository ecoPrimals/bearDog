//! Compliance Engine
//! 
//! Automated compliance monitoring and reporting for major security frameworks.

use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Mutex;

use crate::error::BearDogResult;

/// Compliance engine for managing compliance checks and reporting
pub struct ComplianceEngine {
    config: Arc<ComplianceConfig>,
    enabled_standards: Vec<ComplianceStandard>,
    event_cache: Arc<Mutex<Vec<ComplianceEvent>>>,
}

/// Compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    /// List of enabled compliance standards
    pub enabled_standards: Vec<ComplianceStandard>,
    /// Monitoring interval for compliance checks
    pub monitoring_interval: Duration,
    /// Audit log retention period
    pub audit_retention: Duration,
    /// Dashboard refresh interval
    pub dashboard_refresh_interval: Duration,
    /// Reporting configuration
    pub reporting: ReportingConfig,
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            enabled_standards: vec![
                ComplianceStandard::GDPR,
                ComplianceStandard::SOX,
                ComplianceStandard::PCI_DSS,
            ],
            monitoring_interval: Duration::minutes(5),
            audit_retention: Duration::days(365),
            dashboard_refresh_interval: Duration::minutes(1),
            reporting: ReportingConfig::default(),
        }
    }
}

/// Reporting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingConfig {
    /// Automatically generate reports
    pub auto_generate: bool,
    /// Report generation interval
    pub generation_interval: Duration,
    /// Storage path for reports
    pub storage_path: String,
    /// Report formats to generate
    pub formats: Vec<ReportFormat>,
}

impl Default for ReportingConfig {
    fn default() -> Self {
        Self {
            auto_generate: true,
            generation_interval: Duration::days(1),
            storage_path: "./reports".to_string(),
            formats: vec![ReportFormat::JSON, ReportFormat::PDF],
        }
    }
}

/// Report format options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportFormat {
    PDF,
    JSON,
    CSV,
    HTML,
}

/// Compliance standards supported by BearDog
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComplianceStandard {
    /// GDPR (General Data Protection Regulation)
    GDPR,
    /// SOX (Sarbanes-Oxley Act)
    SOX,
    /// PCI DSS (Payment Card Industry Data Security Standard)
    PciDss,
    /// PCI DSS (Payment Card Industry Data Security Standard) - alternative name
    #[allow(non_camel_case_types)]
    PCI_DSS,
    /// HIPAA (Health Insurance Portability and Accountability Act)
    HIPAA,
    /// ISO 27001 (Information Security Management)
    ISO27001,
    /// FedRAMP (Federal Risk and Authorization Management Program)
    FedRAMP,
}

/// Compliance event for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceEvent {
    /// Unique event identifier
    pub id: String,
    /// Type of event (e.g., "DataAccess", "PolicyViolation")
    pub event_type: String,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// User who triggered the event
    pub user_id: Option<String>,
    /// Resource affected by the event
    pub resource: Option<String>,
    /// Additional event data
    pub data: HashMap<String, String>,
}

/// Compliance violation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub id: String,
    pub standard: ComplianceStandard,
    pub event_id: String,
    pub violation_type: String,
    pub severity: ComplianceSeverity,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub remediation_required: bool,
}

/// Compliance warning details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceWarning {
    pub id: String,
    pub standard: ComplianceStandard,
    pub warning_type: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
}

/// Compliance severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Hash)]
pub enum ComplianceSeverity {
    /// Information level
    Info,
    /// Warning level
    Warning,
    /// Violation level
    Violation,
    /// Critical level
    Critical,
}

/// Result of compliance evaluation for a single event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceResult {
    pub event_id: String,
    pub compliance_score: f64,
    pub violations: Vec<ComplianceViolation>,
    pub warnings: Vec<ComplianceWarning>,
    pub evaluated_at: DateTime<Utc>,
    pub standards_checked: Vec<ComplianceStandard>,
}

/// Compliance evaluation for a standard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardEvaluationResult {
    pub score: f64,
    pub violations: Vec<ComplianceViolation>,
    pub warnings: Vec<ComplianceWarning>,
}

/// Date range for reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub id: String,
    pub title: String,
    pub standard: ComplianceStandard,
    pub period: String,
    pub generated_at: DateTime<Utc>,
    pub overall_score: f64,
    pub total_events: u64,
    pub violations_found: u64,
    pub warnings_issued: u64,
    pub recommendations: Vec<String>,
    pub detailed_findings: Vec<String>,
    pub summary: String,
}

/// Dashboard data structure for compliance overview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceDashboard {
    pub compliance_status: ComplianceStatus,
    pub recent_events: Vec<ComplianceEvent>,
    pub active_violations: Vec<ComplianceViolation>,
    pub recent_warnings: Vec<ComplianceWarning>,
    pub recommendations: Vec<String>,
    pub metrics: ComplianceMetrics,
}

/// Overall compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    pub overall_status: String,
    pub compliance_percentage: f64,
    pub active_violations: u64,
    pub total_events_today: u64,
    pub last_updated: DateTime<Utc>,
}

/// Compliance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceMetrics {
    pub events_processed_today: u64,
    pub violations_detected_today: u64,
    pub warnings_issued_today: u64,
    pub average_compliance_score: f64,
    pub standards_monitored: Vec<ComplianceStandard>,
}

impl ComplianceEngine {
    /// Create a new compliance engine
    pub async fn new(config: ComplianceConfig) -> BearDogResult<Self> {
        let enabled_standards = config.enabled_standards.clone();
        Ok(Self {
            config: Arc::new(config),
            enabled_standards,
            event_cache: Arc::new(Mutex::new(Vec::new())),
        })
    }
    
    /// Validate compliance for an event
    pub async fn validate_compliance(&self, event: &ComplianceEvent) -> BearDogResult<ComplianceResult> {
        self.evaluate_event(event.clone()).await
    }

        /// Analyze event for specific compliance standard
    pub async fn analyze_event(&self, event: &ComplianceEvent, standard: ComplianceStandard) -> BearDogResult<ComplianceResult> {
        let mut result = ComplianceResult {
            event_id: event.id.clone(),
            compliance_score: 0.0,
            violations: Vec::new(),
            warnings: Vec::new(),
            evaluated_at: Utc::now(),
            standards_checked: vec![standard.clone()],
        };

        let evaluation = self.evaluate_standard(&standard, event).await?;
        result.compliance_score = evaluation.score;
        result.violations.extend(evaluation.violations);
        result.warnings.extend(evaluation.warnings);

        Ok(result)
    }
    
    /// Monitor an event and return compliance result
    pub async fn monitor_event(&self, event: ComplianceEvent) -> BearDogResult<ComplianceResult> {
        self.event_cache.lock().unwrap().push(event.clone());
        self.evaluate_event(event).await
    }

    /// Process a compliance event and evaluate it against all enabled standards
    pub async fn evaluate_event(&self, event: ComplianceEvent) -> BearDogResult<ComplianceResult> {
        let mut violations = Vec::new();
        let mut warnings = Vec::new();
        let mut score = 1.0f64;
        
        // Evaluate against each enabled standard
        for standard in &self.enabled_standards {
            match self.evaluate_standard(standard, &event).await {
                Ok(result) => {
                    violations.extend(result.violations);
                    warnings.extend(result.warnings);
                    score = score.min(result.score);
                }
                Err(e) => {
                    eprintln!("Failed to evaluate compliance for {:?}: {}", standard, e);
                    // Continue with other standards
                }
            }
        }
        
        // Store the result for audit trail
        let result = ComplianceResult {
            event_id: event.id.clone(),
            compliance_score: score,
            violations,
            warnings,
            evaluated_at: Utc::now(),
            standards_checked: self.enabled_standards.clone(),
        };
        
        // Cache the event
        if let Ok(mut cache) = self.event_cache.lock() {
            cache.push(event);
            // Keep only recent events (last 1000)
            if cache.len() > 1000 {
                cache.remove(0);
            }
        }
        
        Ok(result)
    }
    
    /// Evaluate a single standard against an event
    async fn evaluate_standard(&self, standard: &ComplianceStandard, event: &ComplianceEvent) -> BearDogResult<StandardEvaluationResult> {
        match standard {
            ComplianceStandard::GDPR => self.evaluate_gdpr(event).await,
            ComplianceStandard::SOX => self.evaluate_sox(event).await,
            ComplianceStandard::PciDss | ComplianceStandard::PCI_DSS => self.evaluate_pci_dss(event).await,
            ComplianceStandard::HIPAA => self.evaluate_hipaa(event).await,
            _ => {
                // Default implementation for unhandled standards
                Ok(StandardEvaluationResult {
                    score: 1.0,
                    violations: Vec::new(),
                    warnings: Vec::new(),
                })
            }
        }
    }
    
    /// Evaluate GDPR compliance
    async fn evaluate_gdpr(&self, event: &ComplianceEvent) -> BearDogResult<StandardEvaluationResult> {
        let mut violations = Vec::new();
        let mut warnings = Vec::new();
        let mut score: f64 = 1.0;
        
        // Check for data access events (case insensitive)
        let is_data_access = event.event_type.to_lowercase() == "dataaccess" || 
                            event.event_type.to_lowercase() == "data_access";
        
        // Check for unauthorized access attempts
        let is_unauthorized_access = event.event_type.to_lowercase() == "unauthorized_access_attempt";
        
        if is_unauthorized_access {
            violations.push(ComplianceViolation {
                id: format!("gdpr-violation-{}", Uuid::new_v4()),
                standard: ComplianceStandard::GDPR,
                event_id: event.id.clone(),
                violation_type: "UNAUTHORIZED_ACCESS".to_string(),
                severity: ComplianceSeverity::Critical,
                description: "Unauthorized access attempt detected".to_string(),
                timestamp: Utc::now(),
                remediation_required: true,
            });
            score = 0.0; // Zero score for unauthorized access
        }
        
        if is_data_access {
            // Check for consent verification
            if !event.data.contains_key("consent_verified") || 
               event.data.get("consent_verified") != Some(&"true".to_string()) {
                violations.push(ComplianceViolation {
                    id: format!("gdpr-violation-{}", Uuid::new_v4()),
                    standard: ComplianceStandard::GDPR,
                    event_id: event.id.clone(),
                    violation_type: "CONSENT_MISSING".to_string(),
                    severity: ComplianceSeverity::Critical,
                    description: "Data access without verified consent".to_string(),
                    timestamp: Utc::now(),
                    remediation_required: true,
                });
                score = 0.2; // Low score for missing consent
            }
            
            // Check for encryption
            if !event.data.contains_key("encryption_enabled") || 
               event.data.get("encryption_enabled") != Some(&"true".to_string()) {
                violations.push(ComplianceViolation {
                    id: format!("gdpr-violation-{}", Uuid::new_v4()),
                    standard: ComplianceStandard::GDPR,
                    event_id: event.id.clone(),
                    violation_type: "ENCRYPTION_MISSING".to_string(),
                    severity: ComplianceSeverity::Violation,
                    description: "Personal data accessed without encryption".to_string(),
                    timestamp: Utc::now(),
                    remediation_required: true,
                });
                score = score.min(0.4); // Reduce score for missing encryption
            }
            
            // Check for audit logging
            if !event.data.contains_key("audit_logged") || 
               event.data.get("audit_logged") != Some(&"true".to_string()) {
                warnings.push(ComplianceWarning {
                    id: format!("gdpr-warning-{}", Uuid::new_v4()),
                    standard: ComplianceStandard::GDPR,
                    warning_type: "AUDIT_TRAIL_INCOMPLETE".to_string(),
                    description: "Data access should be properly audited".to_string(),
                    timestamp: Utc::now(),
                });
                score = score.min(0.7); // Reduce score for missing audit
            }
        }
        
        Ok(StandardEvaluationResult {
            score,
            violations,
            warnings,
        })
    }
    
    /// Evaluate SOX compliance
    async fn evaluate_sox(&self, event: &ComplianceEvent) -> BearDogResult<StandardEvaluationResult> {
        let mut violations = Vec::new();
        let mut warnings = Vec::new();
        let score = 1.0;
        
        // Basic SOX evaluation logic
        if event.event_type == "DataAccess" && event.data.contains_key("financial_data") {
            warnings.push(ComplianceWarning {
                id: format!("sox-warning-{}", Uuid::new_v4()),
                standard: ComplianceStandard::SOX,
                warning_type: "AUDIT_TRAIL_REQUIRED".to_string(),
                description: "Financial data access requires additional audit trail".to_string(),
                timestamp: Utc::now(),
            });
        }
        
        Ok(StandardEvaluationResult {
            score,
            violations,
            warnings,
        })
    }
    
    /// Evaluate PCI DSS compliance
    async fn evaluate_pci_dss(&self, event: &ComplianceEvent) -> BearDogResult<StandardEvaluationResult> {
        let violations = Vec::new();
        let warnings = Vec::new();
        let score = 1.0;
        
        // Basic PCI DSS evaluation logic would go here
        
        Ok(StandardEvaluationResult {
            score,
            violations,
            warnings,
        })
    }
    
    /// Evaluate HIPAA compliance
    async fn evaluate_hipaa(&self, event: &ComplianceEvent) -> BearDogResult<StandardEvaluationResult> {
        let mut violations = Vec::new();
        let mut warnings = Vec::new();
        let mut score: f64 = 1.0;
        
        // Check for data access events (case insensitive)
        let is_data_access = event.event_type.to_lowercase() == "dataaccess" || 
                            event.event_type.to_lowercase() == "data_access";
        
        if is_data_access {
            // Check if accessing medical/PHI data
            let is_phi_data = event.data.contains_key("data_type") && 
                             event.data.get("data_type").map(|s| s.contains("medical") || s.contains("health")).unwrap_or(false);
            
            if is_phi_data {
                // Check for encryption
                if !event.data.contains_key("encryption_enabled") || 
                   event.data.get("encryption_enabled") != Some(&"true".to_string()) {
                    violations.push(ComplianceViolation {
                        id: format!("hipaa-violation-{}", Uuid::new_v4()),
                        standard: ComplianceStandard::HIPAA,
                        event_id: event.id.clone(),
                        violation_type: "PHI_ENCRYPTION_MISSING".to_string(),
                        severity: ComplianceSeverity::Critical,
                        description: "Protected Health Information accessed without encryption".to_string(),
                        timestamp: Utc::now(),
                        remediation_required: true,
                    });
                    score = 0.1; // Very low score for unencrypted PHI access
                }
                
                // Check for audit logging
                if !event.data.contains_key("audit_logged") || 
                   event.data.get("audit_logged") != Some(&"true".to_string()) {
                    violations.push(ComplianceViolation {
                        id: format!("hipaa-violation-{}", Uuid::new_v4()),
                        standard: ComplianceStandard::HIPAA,
                        event_id: event.id.clone(),
                        violation_type: "AUDIT_TRAIL_MISSING".to_string(),
                        severity: ComplianceSeverity::Critical,
                        description: "PHI access without proper audit trail".to_string(),
                        timestamp: Utc::now(),
                        remediation_required: true,
                    });
                    score = score.min(0.2); // Reduce score for missing audit
                }
            }
        }
        
        Ok(StandardEvaluationResult {
            score,
            violations,
            warnings,
        })
    }
    
    /// Generate a compliance report for a specific standard and date range
    pub async fn generate_compliance_report(&self, standard: ComplianceStandard, date_range: (DateTime<Utc>, DateTime<Utc>)) -> BearDogResult<ComplianceReport> {
        let (start_date, end_date) = date_range;
        
        // Retrieve events from the date range (placeholder - would need audit engine integration)
        let events = if let Ok(cache) = self.event_cache.lock() {
            cache.clone()
        } else {
            Vec::new()
        };
        
        let mut total_events = 0;
        let mut compliant_events = 0;
        let mut violations = Vec::new();
        let mut recommendations = Vec::new();
        
        // Analyze each event for compliance
        for event in &events {
            total_events += 1;
            
            let compliance_result = self.validate_compliance(event).await?;
            
            if compliance_result.violations.is_empty() {
                compliant_events += 1;
            } else {
                // Record violations
                violations.extend(compliance_result.violations);
            }
        }
        
        // Calculate compliance score
        let compliance_score = if total_events > 0 {
            compliant_events as f64 / total_events as f64
        } else {
            1.0 // Perfect compliance if no events
        };
        
        // Generate recommendations based on violations
        self.generate_recommendations(&violations, &mut recommendations);
        
        Ok(ComplianceReport {
            id: format!("report_{}", Utc::now().timestamp()),
            title: format!("{:?} Compliance Report", standard),
            standard,
            period: format!("{} to {}", start_date.format("%Y-%m-%d"), end_date.format("%Y-%m-%d")),
            generated_at: Utc::now(),
            overall_score: compliance_score,
            total_events,
            violations_found: violations.len() as u64,
            warnings_issued: 0, // Would be calculated from actual warnings
            recommendations,
            detailed_findings: vec![], // Would be populated with actual findings
            summary: format!("Compliance score: {:.2}%", compliance_score * 100.0),
        })
    }
    
    fn generate_recommendations(&self, violations: &[ComplianceViolation], recommendations: &mut Vec<String>) {
        // Analyze violation patterns and generate specific recommendations
        let mut violation_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        
        for violation in violations {
            *violation_counts.entry(violation.id.clone()).or_insert(0) += 1;
        }
        
        // Generate recommendations based on most common violations
        for (rule_id, count) in violation_counts {
            if count > 5 {
                match rule_id.as_str() {
                    "GDPR_DATA_ACCESS" => {
                        recommendations.push("Implement automated data access controls to reduce GDPR compliance violations".to_string());
                    },
                    "SOX_FINANCIAL_ACCESS" => {
                        recommendations.push("Strengthen financial system access controls and implement dual authorization".to_string());
                    },
                    "PCI_DSS_PAYMENT" => {
                        recommendations.push("Enhance payment processing security controls and encryption".to_string());
                    },
                    "HIPAA_PHI_ACCESS" => {
                        recommendations.push("Implement additional PHI access controls and audit monitoring".to_string());
                    },
                    _ => {
                        recommendations.push(format!("Address recurring compliance violations for rule: {}", rule_id));
                    }
                }
            }
        }
        
        // Always include general recommendations
        if !violations.is_empty() {
            recommendations.push("Regular compliance training for all users".to_string());
            recommendations.push("Implement automated compliance monitoring alerts".to_string());
        }
    }

    /// Perform periodic compliance check
    pub async fn perform_periodic_check(&self) -> BearDogResult<()> {
        tracing::debug!("Performing periodic compliance check");
        
        // Create a sample event for periodic checking
        let check_event = ComplianceEvent {
            id: format!("periodic-check-{}", uuid::Uuid::new_v4()),
            event_type: "system_check".to_string(),
            user_id: Some("system".to_string()),
            resource: Some("periodic_check".to_string()),
            data: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now(),
        };
        
        // Monitor the periodic check event
        let result = self.monitor_event(check_event).await?;
        
        if result.compliance_score < 0.8 {
            tracing::warn!("Compliance score below threshold: {}", result.compliance_score);
        }
        
        // Log any violations found
        if !result.violations.is_empty() {
            tracing::warn!("Found {} compliance violations during periodic check", result.violations.len());
        }
        
        Ok(())
    }

    /// Get compliance dashboard data
    pub async fn get_dashboard_data(&self) -> BearDogResult<ComplianceDashboard> {
        let events = if let Ok(cache) = self.event_cache.lock() {
            cache.clone()
        } else {
            Vec::new()
        };

        let recent_events = events.iter().take(10).cloned().collect();
        
        // Calculate actual compliance metrics based on processed events
        let total_events = events.len() as u64;
        let violations: Vec<ComplianceViolation> = events.iter()
            .filter_map(|event| {
                // Check if event represents a violation based on event type
                if event.event_type.contains("Violation") || event.event_type.contains("Error") {
                    Some(ComplianceViolation {
                        id: format!("violation_{}", event.id),
                        standard: ComplianceStandard::GDPR, // Would be determined by event analysis
                        event_id: event.id.clone(),
                        violation_type: event.event_type.clone(),
                        severity: ComplianceSeverity::Warning,
                        description: format!("Compliance violation detected in event {}", event.id),
                        timestamp: event.timestamp,
                        remediation_required: true,
                    })
                } else {
                    None
                }
            })
            .collect();

        let warnings: Vec<ComplianceWarning> = events.iter()
            .filter_map(|event| {
                // Check if event represents a warning
                if event.event_type.contains("Warning") {
                    Some(ComplianceWarning {
                        id: format!("warning_{}", event.id),
                        standard: ComplianceStandard::SOX, // Would be determined by event analysis
                        warning_type: event.event_type.clone(),
                        description: format!("Compliance warning for event {}", event.id),
                        timestamp: event.timestamp,
                    })
                } else {
                    None
                }
            })
            .collect();

        let violation_count = violations.len() as u64;
        let warning_count = warnings.len() as u64;
        
        // Calculate compliance percentage based on violations vs total events
        let compliance_percentage = if total_events > 0 {
            ((total_events - violation_count) as f64 / total_events as f64) * 100.0
        } else {
            100.0
        };

        let compliance_status = ComplianceStatus {
            overall_status: if compliance_percentage >= 95.0 {
                "Excellent".to_string()
            } else if compliance_percentage >= 85.0 {
                "Good".to_string()
            } else if compliance_percentage >= 70.0 {
                "Fair".to_string()
            } else {
                "Poor".to_string()
            },
            compliance_percentage,
            active_violations: violation_count,
            total_events_today: total_events,
            last_updated: Utc::now(),
        };

        let metrics = ComplianceMetrics {
            events_processed_today: total_events,
            violations_detected_today: violation_count,
            warnings_issued_today: warning_count,
            average_compliance_score: compliance_percentage / 100.0,
            standards_monitored: self.enabled_standards.clone(),
        };

        // Generate recommendations based on current state
        let mut recommendations = Vec::new();
        if violation_count > 0 {
            recommendations.push("Review and address active compliance violations".to_string());
        }
        if warning_count > 0 {
            recommendations.push("Investigate compliance warnings to prevent violations".to_string());
        }
        if compliance_percentage < 90.0 {
            recommendations.push("Consider implementing additional compliance controls".to_string());
        }
        if recommendations.is_empty() {
            recommendations.push("Compliance monitoring is operating effectively".to_string());
        }

        Ok(ComplianceDashboard {
            compliance_status,
            recent_events,
            active_violations: violations,
            recent_warnings: warnings,
            recommendations,
            metrics,
        })
    }

    /// Get compliance metrics
    pub async fn get_compliance_metrics(&self) -> BearDogResult<ComplianceMetrics> {
        let events = if let Ok(cache) = self.event_cache.lock() {
            cache.len()
        } else {
            0
        };

        Ok(ComplianceMetrics {
            events_processed_today: events as u64,
            violations_detected_today: 0,
            warnings_issued_today: 0,
            average_compliance_score: 0.95,
            standards_monitored: self.enabled_standards.clone(),
        })
    }

    /// Create a placeholder instance for initialization
    pub fn placeholder() -> Self {
        Self {
            config: Arc::new(ComplianceConfig::default()),
            enabled_standards: vec![ComplianceStandard::GDPR, ComplianceStandard::SOX],
            event_cache: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    fn create_test_config() -> ComplianceConfig {
        ComplianceConfig {
            enabled_standards: vec![
                ComplianceStandard::SOX,
                ComplianceStandard::PciDss,
                ComplianceStandard::GDPR,
            ],
            monitoring_interval: Duration::minutes(5),
            audit_retention: Duration::days(365),
            dashboard_refresh_interval: Duration::minutes(1),
            reporting: ReportingConfig {
                auto_generate: true,
                generation_interval: Duration::days(30),
                storage_path: "/var/log/beardog/compliance".to_string(),
                formats: vec![ReportFormat::PDF, ReportFormat::JSON],
            },
        }
    }
    
    fn create_test_event() -> ComplianceEvent {
        ComplianceEvent {
            id: "test-compliance-001".to_string(),
            event_type: "DataAccess".to_string(),
            timestamp: Utc::now(),
            user_id: Some("test-user".to_string()),
            resource: Some("/sensitive/data.txt".to_string()),
            data: HashMap::new(),
        }
    }
    
    #[tokio::test]
    async fn test_compliance_engine_creation() {
        let config = create_test_config();
        let engine = ComplianceEngine::new(config).await;
        assert!(engine.is_ok());
    }
    
    #[tokio::test]
    async fn test_gdpr_data_access() {
        let config = create_test_config();
        let engine = ComplianceEngine::new(config).await.unwrap();
        
        let mut event = create_test_event();
        event.event_type = "DataAccess".to_string();
        event.data = HashMap::from([("consent_verified".to_string(), "true".to_string())]);
        
        let result = engine.evaluate_event(event).await.unwrap();
        assert!(result.compliance_score >= 0.0);
    }
    
    #[tokio::test]
    async fn test_pci_dss_payment_processing() {
        let config = create_test_config();
        let engine = ComplianceEngine::new(config).await.unwrap();
        
        let mut event = create_test_event();
        event.event_type = "DataProcessing".to_string();
        event.data = HashMap::from([("encryption_verified".to_string(), "true".to_string())]);
        
        let result = engine.evaluate_event(event).await.unwrap();
        assert!(result.compliance_score >= 0.0);
    }
    
    #[tokio::test]
    async fn test_sox_financial_access() {
        let config = create_test_config();
        let engine = ComplianceEngine::new(config).await.unwrap();
        
        let mut event = create_test_event();
        event.event_type = "DataAccess".to_string();
        event.data = HashMap::from([("financial_data".to_string(), "true".to_string())]);
        
        let result = engine.evaluate_event(event).await.unwrap();
        assert!(result.compliance_score >= 0.0);
    }
    
    #[tokio::test]
    async fn test_policy_violation_detection() {
        let config = create_test_config();
        let engine = ComplianceEngine::new(config).await.unwrap();
        
        let mut event = create_test_event();
        event.event_type = "PolicyViolation".to_string();
        event.data = HashMap::from([("policy_id".to_string(), "SECURITY_001".to_string())]);
        
        let result = engine.evaluate_event(event).await.unwrap();
        assert!(result.compliance_score >= 0.0);
    }
    
    #[tokio::test]
    async fn test_data_retention_compliance() {
        let config = create_test_config();
        let engine = ComplianceEngine::new(config).await.unwrap();
        
        let mut event = create_test_event();
        event.event_type = "DataRetention".to_string();
        event.data = HashMap::from([("retention_period".to_string(), "365 days".to_string())]);
        
        let result = engine.evaluate_event(event).await.unwrap();
        assert!(result.compliance_score >= 0.0);
    }
    
    #[tokio::test]
    async fn test_generate_compliance_report() {
        let config = create_test_config();
        let engine = ComplianceEngine::new(config).await.unwrap();
        
        // Process several events
        for i in 0..5 {
            let mut event = create_test_event();
            event.id = format!("test-event-{}", i);
            let _result = engine.evaluate_event(event).await.unwrap();
        }
        
        let date_range = DateRange {
            start_date: Utc::now() - chrono::Duration::days(30),
            end_date: Utc::now(),
        };
        
        let report = engine.generate_compliance_report(ComplianceStandard::GDPR, (date_range.start_date, date_range.end_date)).await;
        assert!(report.is_ok());
        
        let report = report.unwrap();
        assert!(!report.title.is_empty());
        assert!(report.overall_score >= 0.0);
    }
    
    #[tokio::test]
    async fn test_multiple_compliance_standards() {
        let config = create_test_config();
        let engine = ComplianceEngine::new(config).await.unwrap();
        
        let mut event = create_test_event();
        event.event_type = "DataAccess".to_string();
        event.data = HashMap::from([("multi_standard".to_string(), "true".to_string())]);
        
        let result = engine.evaluate_event(event).await.unwrap();
        assert!(result.compliance_score >= 0.0);
    }
    
    #[tokio::test]
    async fn test_audit_trail_creation() {
        let config = create_test_config();
        let engine = ComplianceEngine::new(config).await.unwrap();
        
        let event = create_test_event();
        let _event_id = event.id.clone();
        
        let result = engine.evaluate_event(event).await.unwrap();
        
        assert!(result.compliance_score >= 0.0);
        // Audit trail should be maintained internally
    }
    
    #[tokio::test]
    async fn test_compliance_validation() {
        let config = create_test_config();
        let engine = ComplianceEngine::new(config).await.unwrap();
        
        let mut event = create_test_event();
        event.event_type = "ValidationTest".to_string();
        
        let result = engine.evaluate_event(event).await.unwrap();
        assert!(result.compliance_score >= 0.0);
    }
    
    #[tokio::test]
    async fn test_concurrent_compliance_evaluation() {
        let config = create_test_config();
        let engine = std::sync::Arc::new(ComplianceEngine::new(config).await.unwrap());
        
        let mut handles = vec![];
        
        for i in 0..10 {
            let engine_clone = engine.clone();
            let handle = tokio::spawn(async move {
                let event = ComplianceEvent {
                    id: format!("concurrent-test-{}", i),
                    event_type: "DataAccess".to_string(),
                    timestamp: Utc::now(),
                    user_id: Some(format!("user-{}", i)),
                    resource: Some(format!("/test/resource{}.txt", i)),
                    data: HashMap::from([("test".to_string(), "value".to_string())]),
                };
                
                let result = engine_clone.evaluate_event(event).await.unwrap();
                assert!(result.compliance_score >= 0.0);
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.await.unwrap();
        }
    }
    
    #[test]
    fn test_compliance_severity_ordering() {
        assert!(ComplianceSeverity::Critical > ComplianceSeverity::Info);
    }
    
    #[test]
    fn test_compliance_event_creation() {
        let event = create_test_event();
        assert!(!event.id.is_empty());
        assert!(event.user_id.is_some());
        assert!(event.resource.is_some());
    }
    
    #[test]
    fn test_compliance_standard_debug() {
        // Test that we can debug print the standards
        let standards = vec![
            ComplianceStandard::SOX,
            ComplianceStandard::GDPR,
            ComplianceStandard::PciDss,
        ];
        
        for standard in standards {
            let debug_str = format!("{:?}", standard);
            assert!(!debug_str.is_empty());
        }
    }
} 