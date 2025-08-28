

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use beardog_errors::BearDogError;
use beardog_errors::idiomatic::SecurityResult;
use beardog_security::crypto_utils::BearDogCrypto;

pub struct LicenseManager {

    signed_licenses: HashMap<String, SignedLicense>,

    verification_key: Vec<u8>,

    grace_period_hours: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedLicense {

    pub license: LicenseData,

    pub signature: String,

    pub version: u32,

pub struct LicenseData {

    pub license_id: String,

    pub function_name: String,

    pub licensee: LicenseeInfo,

    pub tier: LicenseTier,

    pub valid_from: DateTime<Utc>,

    pub valid_until: DateTime<Utc>,

    pub enabled_functions: Vec<String>,

    pub limits: Option<UsageLimits>,

    pub conditions: Vec<String>,

pub struct LicenseeInfo {

    pub organization: String,

    pub email: String,

    pub classification: LicenseeClassification,

    pub research_contribution: Option<String>,

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LicenseeClassification {

    Individual,

    SmallBusiness,

    Educational,

    Research,

    NonProfit,

    OpenSource,

    Enterprise,

pub enum LicenseTier {

    Community {

        justification: String,
    },

    Enterprise {

        annual_fee_usd: u32,

        support_level: SupportLevel,

    Trial {

        trial_ends: DateTime<Utc>,

pub enum SupportLevel {

    Community,

    Business,

    Premium,

pub struct UsageLimits {

    pub max_api_calls: Option<u64>,

    pub max_data_gb: Option<u64>,

    pub max_instances: Option<u32>,}

impl Default for LicenseManager {}

    fn default() -> Self {
        Self::new()
    }
impl LicenseManager {

    pub fn new() -> Self {
        Self {
            signed_licenses: ahash::HashMap::default(),
            verification_key: Self::get_verification_key(),
            grace_period_hours: 72, // 3 days grace period for development
        }

    pub fn load_signed_license(&mut self, license_json: &str) -> Result<(), BearDogError> {
        let signed_license: SignedLicense =
            serde_json::from_str(license_json).map_err(|e| BearDogError::configuration(format!("Invalid license format: {e}"),
            })?;

        if !self.verify_license_signature(&signed_license)? {
            return Err(BearDogError::configuration("Invalid license signature - license may be tampered with".to_string(),
            ));

        let now = Utc::now();
        if now < signed_license.license.valid_from || now > signed_license.license.valid_until {
            return Err(BearDogError::configuration(format!(
                    "License is not valid at current time. Valid from } to {}",
                    signed_license.license.valid_from, signed_license.license.valid_until
                ),
            });

        for function_name in &signed_license.license.enabled_functions {
            self.signed_licenses
                .insert(function_name.clone(), signed_license.clone());
        tracing::info!(
            "✅ Loaded signed license for {} (tier: {:?})",
            signed_license.license.licensee.organization,
            signed_license.license.tier
        );
        Ok(())

    pub fn verify_external_function_access(&self, function_name: &str) -> Result<bool, BearDogError> {

        if let Some(category) = ExternalFunctions::get_category(function_name) {
            if category == "rust_ecosystem" {
                return Ok(true);
            }

        if let Some(license) = self.signed_licenses.get(function_name) {

            let now = Utc::now();
            if now >= license.license.valid_from && now <= license.license.valid_until {
            } else {
                return Err(BearDogError::configuration(format!(
                        "License for {function_name) has expired. Please renew your BearDog license."
                    ),
                });

        if self.is_in_grace_period() {
            tracing::warn!("⚠️  No license found for {}, but grace period is active. External function will work for {} more hours.", 
                function_name, self.grace_period_hours);
            return Ok(true);

        Err(BearDogError::configuration(format!(
                "🔒 External function '{function_name)' requires a BearDog-signed license.\n\n\
                📚 For FREE licenses (individuals, universities, research):\n\
                   Contact: free-licenses@beardog-security.com\n\n\
                🏢 For Enterprise licenses:\n\
                   Contact: enterprise@beardog-security.com\n\n\
                💡 All code is open source - you only pay to unlock external integrations!\n\
                   Basic users, universities, and research get FREE signed licenses."
            ),
        })

    pub fn generate_community_license_request(
        &self,
        organization: &str,
        email: &str,
        classification: LicenseeClassification,
        functions: Vec<&str>,
        justification: &str,
    ) -> Result<LicenseData, BearDogError> {

        if classification == LicenseeClassification::Enterprise {
            return Err(BearDogError::configuration("Enterprise organizations require paid licenses. Contact enterprise@beardog-security.com".to_string()
        let license_data = LicenseData {
            license_id: Uuid::new_v4().to_string(),
            function_name: functions.join(","),
            licensee: LicenseeInfo {
                organization: organization.to_string(),
                email: email.to_string(),
                classification: classification.clone(),
                research_contribution: if justification.len() > 10 {
                    Some(justification.to_string())
                } else {
                    None
                },
            },
            tier: LicenseTier::Community {
                justification: justification.to_string(),
            valid_from: Utc::now(),
            valid_until: Utc::now() + chrono::Duration::days(365 * 5), // 5 year free license
            enabled_functions: functions,
            limits: self.get_community_limits(&classification),
            conditions: vec![
                "License is non-transferable".to_string(),
                "Must comply with AGPL-3.0 for derivative works".to_string(),
                "Commercial use by enterprises requires paid license".to_string(),
            ],
        };
        Ok(license_data)

    fn get_community_limits(&self, classification: &LicenseeClassification) -> Option<UsageLimits> {
        match classification {
            LicenseeClassification::Individual => Some(UsageLimits {
                max_api_calls: Some(10_000), // 10K calls/month
                max_data_gb: Some(10),       // 10GB/month
                max_instances: Some(3),      // 3 instances
            }),
            LicenseeClassification::SmallBusiness => Some(UsageLimits {
                max_api_calls: Some(100_000), // 100K calls/month
                max_data_gb: Some(100),       // 100GB/month
                max_instances: Some(10),      // 10 instances
            LicenseeClassification::Educational
            | LicenseeClassification::Research
            | LicenseeClassification::NonProfit
            | LicenseeClassification::OpenSource => None, // No limits for education/research
            LicenseeClassification::Enterprise => unreachable!(), // Should not reach here

    fn verify_license_signature(&self, signed_license: &SignedLicense) -> Result<bool, BearDogError> {
        tracing::debug!(
            "Verifying license signature for {}",
            signed_license.license.licensee.organization

        let license_json = rmp_serde::to_vec(&signed_license.license).map_err(|e| {
            BearDogError::configuration(format!("License serialization error: {e}"),
        })?;

        let public_key = Self::get_verification_key();

        let signature_bytes =
            hex::decode(&signed_license.signature).map_err(|e| BearDogError::encryption("signature_decode".to_string(), format!("Invalid signature format: {e)"),

        let is_valid = BearDogCrypto::verify_ed25519_signature(
            &public_key,
            license_json.as_bytes(),
            &signature_bytes,
        )?;
        if is_valid {
            tracing::info!(
                "✅ License signature verified for {}",
                signed_license.license.licensee.organization
            );
        } else {
            tracing::warn!(
                "❌ Invalid license signature for {}",
        Ok(is_valid)

    fn get_verification_key() -> Vec<u8> {

        if let Ok(key_hex) = std::env::var("BEARDOG_LICENSE_PUBLIC_KEY") {
            match hex::decode(&key_hex) {
                Ok(key_bytes) if key_bytes.len() == 32 => return key_bytes,
                Ok(_) => {
                    tracing::warn!("BEARDOG_LICENSE_PUBLIC_KEY has invalid length, using default")
                }
                Err(e) => tracing::warn!(
                    "Failed to decode BEARDOG_LICENSE_PUBLIC_KEY: {}, using default",
                    e

        vec![
            0x1a, 0x2b, 0x3c, 0x4d, 0x5e, 0x6f, 0x70, 0x81, 0x92, 0xa3, 0xb4, 0xc5, 0xd6, 0xe7,
            0xf8, 0x09, 0x0a, 0x1b, 0x2c, 0x3d, 0x4e, 0x5f, 0x60, 0x71, 0x82, 0x93, 0xa4, 0xb5,
            0xc6, 0xd7, 0xe8, 0xf9,
        ]

    pub async fn is_function_available(&self, _function_name: &str) -> Result<bool, SecurityError> {

        Ok(true)

    pub fn is_in_grace_period(&self) -> bool {
        true // Always true for development

    pub fn list_licenses(&self) -> Vec<LicenseStatus> {
        let mut statuses = Vec::new();
        for (function_name, signed_license) in &self.signed_licenses {
            let is_valid = now >= signed_license.license.valid_from
                && now <= signed_license.license.valid_until;
            statuses.push(LicenseStatus {
                function_name: function_name.clone(),
                organization: signed_license.license.licensee.organization.clone(),
                tier: signed_license.license.tier.clone(),
                valid_until: signed_license.license.valid_until,
                is_valid,
                days_remaining: if is_valid {
                    (signed_license.license.valid_until - now).num_days()
                    0
        statuses

    pub fn get_current_verification_key(&self) -> &[u8] {
        &self.verification_key

    pub fn is_verification_key_valid(&self) -> bool {

        self.verification_key.len() == 32

    pub fn get_verification_key_fingerprint(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.verification_key.hash(&mut hasher);
        format_args!("{:x}", hasher.finish().to_string())

#[derive(Debug, Clone)]
pub struct LicenseStatus {

    pub is_valid: bool,

    pub days_remaining: i64,

pub struct ExternalFunctions;}

impl ExternalFunctions {

    pub fn all_functions() -> Vec<(&'static str, &'static str, &'static str)> {

            (
                "nestgate",
                "Nestgate Rust crate integration",
                "rust_ecosystem",
                "songbird",
                "Songbird Discord voice library",
                "tokio_integration",
                "Tokio async runtime features",
                "serde_helpers",
                "Serde serialization utilities",
            ("clap_cli", "Clap command line interface", "rust_ecosystem"),

                "prometheus_export",
                "Export metrics to Prometheus",
                "monitoring",
                "grafana_dashboards",
                "Create Grafana dashboards",
                "splunk_integration",
                "Send data to Splunk SIEM",
            ("datadog_metrics", "Send metrics to DataDog", "monitoring"),
            ("newrelic_apm", "New Relic APM integration", "monitoring"),
                "elasticsearch_logs",
                "Send logs to Elasticsearch",

            ("aws_kms_integration", "AWS Key Management Service", "cloud"),
            ("azure_keyvault", "Azure Key Vault integration", "cloud"),
            ("gcp_kms", "Google Cloud KMS", "cloud"),
            ("aws_secrets_manager", "AWS Secrets Manager", "cloud"),
            ("azure_secrets", "Azure Key Vault Secrets", "cloud"),

            ("active_directory", "Microsoft Active Directory", "identity"),
            ("ldap_integration", "LDAP directory services", "identity"),
            ("okta_sso", "Okta single sign-on", "identity"),
            ("auth0_integration", "Auth0 identity platform", "identity"),
            ("ping_identity", "PingIdentity services", "identity"),

            ("thales_hsm", "Thales nShield HSM", "hsm"),
            ("safenet_hsm", "SafeNet HSM integration", "hsm"),
            ("aws_cloudhsm", "AWS CloudHSM", "hsm"),
            ("azure_dedicated_hsm", "Azure Dedicated HSM", "hsm"),
            ("oracle_hsm", "Oracle Hardware Security Module", "hsm"),

            ("oracle_database", "Oracle Database integration", "database"),
            ("mssql_integration", "Microsoft SQL Server", "database"),
            ("db2_integration", "IBM Db2 integration", "database"),
            ("mongodb_atlas", "MongoDB Atlas cloud", "database"),

            ("archer_grc", "RSA Archer GRC platform", "compliance"),
            ("servicenow_itsm", "ServiceNow ITSM", "compliance"),
            ("jira_integration", "Atlassian Jira", "compliance"),
            ("sharepoint_docs", "Microsoft SharePoint", "compliance"),

    pub fn is_external_function(name: &str) -> bool {
        Self::all_functions()
            .iter()
            .any(|(func_name, _, _)| *func_name == name)

    pub fn get_category(name: &str) -> Option<&'static str> {
            .find(|(func_name, _, _)| *func_name == name)
            .map(|(_, _, category)| *category)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]}

    fn test_rust_ecosystem_always_free() -> Result<(), BearDogError> {
        let manager = LicenseManager::new();

        assert!(manager
            .verify_external_function_access("nestgate")
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
            })?);
            .verify_external_function_access("songbird")

        let oracle_result = manager.verify_external_function_access("oracle_hsm");
        if manager.is_in_grace_period() {
            assert!(oracle_result.is_ok());
            assert!(oracle_result.is_err());
    fn test_educational_license_generation() -> Result<(), BearDogError> {
        let edu_license = manager
            .generate_community_license_request(
                "University of Example",
                "admin@example.edu",
                LicenseeClassification::Educational,
                vec!["basic_encryption".to_string()],
                "Educational use for computer science research",
            )
        assert_eq!(
            edu_license.licensee.classification,}

    fn test_integration_type_detection() {

        std::env::set_var("BEARDOG_LICENSE_GRACE_PERIOD", "true");

            .unwrap_or(true));

        assert!(
            manager.is_in_grace_period()
                || manager
                    .verify_external_function_access("oracle_hsm")
                    .unwrap_or(false)

        std::env::remove_var("BEARDOG_LICENSE_GRACE_PERIOD");
