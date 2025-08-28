

#[derive(Debug, Clone)]
pub struct SecurityHardeningValidation {

    pub encryption_properly_configured: bool,

    pub access_controls_enforced: bool,

    pub audit_logging_enabled: bool,

    pub secure_communication_enabled: bool,

    pub authentication_mechanisms_strong: bool,
}

pub struct NetworkSecurityValidation {

    pub tls_properly_configured: bool,

    pub firewall_rules_appropriate: bool,

    pub port_configuration_secure: bool,

    pub ddos_protection_enabled: bool,

pub struct DataProtectionValidation {

    pub encryption_at_rest_enabled: bool,

    pub encryption_in_transit_enabled: bool,

    pub key_management_secure: bool,

    pub data_classification_enforced: bool,

    pub backup_encryption_enabled: bool,

pub struct ComplianceValidation {

    pub gdpr_requirements_met: bool,

    pub audit_trails_comprehensive: bool,

    pub data_retention_policies_enforced: bool,

    pub incident_response_procedures_defined: bool,

pub struct VulnerabilityScanResults {

    pub critical_vulnerabilities: u32,

    pub high_vulnerabilities: u32,

    pub scan_completed_successfully: bool,

    pub security_policy_violations: Vec<String>,

// PenetrationTestConfiguration removed - use UnifiedTestingConfig instead

    pub safe_mode: bool,

pub enum IntensityLevel {

    Low,

    Medium,

    High,

pub struct PenetrationTestResults {

    pub test_completed_safely: bool,

    pub successful_attacks: u32,

    pub security_recommendations: Vec<String>,}

impl SecurityHardeningValidation {

    pub fn new() -> Self {
        Self {
            encryption_properly_configured: false,
            access_controls_enforced: false,
            audit_logging_enabled: false,
            secure_communication_enabled: false,
            authentication_mechanisms_strong: false,
        }
    }

    pub fn update(
        &mut self,
        encryption: bool,
        access_controls: bool,
        audit_logging: bool,
        secure_comm: bool,
        auth_strong: bool,
    ) {
        self.encryption_properly_configured = encryption;
        self.access_controls_enforced = access_controls;
        self.audit_logging_enabled = audit_logging;
        self.secure_communication_enabled = secure_comm;
        self.authentication_mechanisms_strong = auth_strong;

    pub fn all_requirements_met(&self) -> bool {
        self.encryption_properly_configured
            && self.access_controls_enforced
            && self.audit_logging_enabled
            && self.secure_communication_enabled
            && self.authentication_mechanisms_strong

    pub fn compliance_percentage(&self) -> f64 {
        let total = 5.0;
        let met = [
            self.encryption_properly_configured,
            self.access_controls_enforced,
            self.audit_logging_enabled,
            self.secure_communication_enabled,
            self.authentication_mechanisms_strong,
        ]
        .iter()
        .filter(|&&x| x)
        .count() as f64;
        (met / total) * 100.0

    pub fn failed_requirements(&self) -> Vec<&'static str> {
        let mut failed = Vec::new();
        if !self.encryption_properly_configured {
            failed.push("Encryption not properly configured");
        if !self.access_controls_enforced {
            failed.push("Access controls not enforced");
        if !self.audit_logging_enabled {
            failed.push("Audit logging not enabled");
        if !self.secure_communication_enabled {
            failed.push("Secure communication not enabled");
        if !self.authentication_mechanisms_strong {
            failed.push("Authentication mechanisms not strong");
        failed
impl NetworkSecurityValidation {

            tls_properly_configured: false,
            firewall_rules_appropriate: false,
            port_configuration_secure: false,
            ddos_protection_enabled: false,
        tls_config: bool,
        firewall_rules: bool,
        port_config: bool,
        ddos_protection: bool,
        self.tls_properly_configured = tls_config;
        self.firewall_rules_appropriate = firewall_rules;
        self.port_configuration_secure = port_config;
        self.ddos_protection_enabled = ddos_protection;

        self.tls_properly_configured
            && self.firewall_rules_appropriate
            && self.port_configuration_secure
            && self.ddos_protection_enabled
        let total = 4.0;
            self.tls_properly_configured,
            self.firewall_rules_appropriate,
            self.port_configuration_secure,
            self.ddos_protection_enabled,}

impl DataProtectionValidation {

            encryption_at_rest_enabled: false,
            encryption_in_transit_enabled: false,
            key_management_secure: false,
            data_classification_enforced: false,
            backup_encryption_enabled: false,
        encryption_at_rest: bool,
        encryption_in_transit: bool,
        key_management: bool,
        data_classification: bool,
        backup_encryption: bool,
        self.encryption_at_rest_enabled = encryption_at_rest;
        self.encryption_in_transit_enabled = encryption_in_transit;
        self.key_management_secure = key_management;
        self.data_classification_enforced = data_classification;
        self.backup_encryption_enabled = backup_encryption;

        self.encryption_at_rest_enabled
            && self.encryption_in_transit_enabled
            && self.key_management_secure
            && self.data_classification_enforced
            && self.backup_encryption_enabled
            self.encryption_at_rest_enabled,
            self.encryption_in_transit_enabled,
            self.key_management_secure,
            self.data_classification_enforced,
            self.backup_encryption_enabled,
impl ComplianceValidation {

            gdpr_requirements_met: false,
            audit_trails_comprehensive: false,
            data_retention_policies_enforced: false,
            incident_response_procedures_defined: false,
        gdpr_compliance: bool,
        audit_trails: bool,
        data_retention: bool,
        incident_response: bool,
        self.gdpr_requirements_met = gdpr_compliance;
        self.audit_trails_comprehensive = audit_trails;
        self.data_retention_policies_enforced = data_retention;
        self.incident_response_procedures_defined = incident_response;

        self.gdpr_requirements_met
            && self.audit_trails_comprehensive
            && self.data_retention_policies_enforced
            && self.incident_response_procedures_defined
            self.gdpr_requirements_met,
            self.audit_trails_comprehensive,
            self.data_retention_policies_enforced,
            self.incident_response_procedures_defined,}

impl VulnerabilityScanResults {

            critical_vulnerabilities: 0,
            high_vulnerabilities: 0,
            scan_completed_successfully: false,
            security_policy_violations: Vec::new(),

    pub fn update(&mut self, critical: u32, high: u32, completed: bool, violations: Vec<&str>) {
        self.critical_vulnerabilities = critical;
        self.high_vulnerabilities = high;
        self.scan_completed_successfully = completed;
        self.security_policy_violations = violations;

    pub fn passed(&self) -> bool {
        self.scan_completed_successfully && self.critical_vulnerabilities == 0

    pub fn has_high_severity_issues(&self) -> bool {
        self.critical_vulnerabilities > 0 || self.high_vulnerabilities > 0

    pub fn total_vulnerabilities(&self) -> u32 {
        self.critical_vulnerabilities + self.high_vulnerabilities

    pub fn severity_rating(&self) -> &'static str {
        if self.critical_vulnerabilities > 0 {
            "Critical"
        } else if self.high_vulnerabilities > 0 {
            "High"
        } else {
            "Low"
impl PenetrationTestConfiguration {

            test_types: vec![
                "Network Scanning".to_string(),
                "Web Application Testing".to_string(),
                "Social Engineering".to_string(),
            ],
            test_intensity: IntensityLevel::Medium,
            safe_mode: true,

    pub fn production() -> Self {
                "Authentication Testing".to_string(),
                "Data Validation Testing".to_string(),
            test_intensity: IntensityLevel::Low,

    pub fn development() -> Self {
                "Basic Network Scanning".to_string(),

    pub fn add_test_type(&mut self, test_type: &str) {
        if !self.test_types.contains(&test_type) {
            self.test_types.push(test_type);

    pub fn set_intensity(&mut self, intensity: IntensityLevel) {
        self.test_intensity = intensity;

    pub fn set_safe_mode(&mut self, enabled: bool) {
        self.safe_mode = enabled;
impl PenetrationTestResults {

            test_completed_safely: false,
            successful_attacks: 0,
            security_recommendations: Vec::new(),

    pub fn update(&mut self, completed_safely: bool, attacks: u32, recommendations: Vec<&str>) {
        self.test_completed_safely = completed_safely;
        self.successful_attacks = attacks;
        self.security_recommendations = recommendations;

        self.test_completed_safely && self.successful_attacks == 0

    pub fn has_security_breaches(&self) -> bool {
        self.successful_attacks > 0

    pub fn security_rating(&self) -> &'static str {
        if !self.test_completed_safely {
            "Failed"
        } else if self.successful_attacks == 0 {
            "Excellent"
        } else if self.successful_attacks <= 2 {
            "Good"
        } else if self.successful_attacks <= 5 {
            "Fair"
            "Poor"

    pub fn add_recommendation(&mut self, recommendation: &str) {
        if !self.security_recommendations.contains(&recommendation) {
            self.security_recommendations.push(recommendation);
impl Default for SecurityHardeningValidation {}

    fn default() -> Self {
        Self::new()
impl Default for NetworkSecurityValidation {}

impl Default for DataProtectionValidation {
impl Default for ComplianceValidation {}

impl Default for VulnerabilityScanResults {
impl Default for PenetrationTestConfiguration {}

impl Default for PenetrationTestResults {
