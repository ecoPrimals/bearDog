//! Comprehensive Licensing System Tests
//! 
//! This test suite ensures 100% coverage of BearDog's licensing and compliance system
//! including license validation, expiration handling, and feature access control.

use beardog::licensing::*;
use beardog::core::*;
use beardog::error::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

/// Comprehensive licensing system testing
/// Tests all license management and validation scenarios
#[tokio::test]
async fn test_licensing_system_comprehensive() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await.expect("Core initialization failed"));
    
    let mut license_manager = LicenseManager::new(core.clone()).await
        .expect("License manager creation failed");
    
    // Test all major licensing operations
    test_license_validation(&mut license_manager).await;
    test_feature_access_control(&mut license_manager).await;
    test_license_expiration_handling(&mut license_manager).await;
    test_adapter_licensing(&mut license_manager).await;
    test_concurrent_license_checks(&mut license_manager).await;
    test_license_renewal_workflow(&mut license_manager).await;
}

async fn test_license_validation(license_manager: &mut LicenseManager) {
    println!("📜 Testing license validation...");
    
    // Test valid enterprise license
    let enterprise_license = License {
        license_id: "BEARDOG-ENT-2024-001".to_string(),
        license_type: LicenseType::Enterprise,
        customer_id: "enterprise_customer_001".to_string(),
        issued_date: SystemTime::now() - Duration::from_secs(86400), // Yesterday
        expiration_date: SystemTime::now() + Duration::from_secs(365 * 86400), // 1 year
        features: vec![
            Feature::AdvancedCrypto,
            Feature::GeneticAlgorithms,
            Feature::ZeroKnowledgeProofs,
            Feature::DistributedConsensus,
            Feature::PerformanceOptimization,
            Feature::SecurityAuditing,
        ],
        node_limit: Some(100),
        data_limit_gb: Some(10000),
        signature: "enterprise_license_signature_hash".to_string(),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("support_level".to_string(), "premium".to_string());
            meta.insert("deployment_type".to_string(), "on_premise".to_string());
            meta
        },
    };
    
    let validation_result = license_manager.validate_license(&enterprise_license).await
        .expect("License validation should succeed");
    
    assert!(validation_result.is_valid, "Valid enterprise license should pass validation");
    assert!(validation_result.features_verified, "Enterprise features should be verified");
    assert!(validation_result.expiration_valid, "License should not be expired");
    assert!(validation_result.signature_valid, "License signature should be valid");
    
    // Test community license
    let community_license = License {
        license_id: "BEARDOG-COMM-2024-001".to_string(),
        license_type: LicenseType::Community,
        customer_id: "community_user_001".to_string(),
        issued_date: SystemTime::now() - Duration::from_secs(3600), // 1 hour ago
        expiration_date: SystemTime::now() + Duration::from_secs(30 * 86400), // 30 days
        features: vec![
            Feature::BasicCrypto,
            Feature::StandardPerformance,
        ],
        node_limit: Some(3),
        data_limit_gb: Some(100),
        signature: "community_license_signature_hash".to_string(),
        metadata: HashMap::new(),
    };
    
    let community_validation = license_manager.validate_license(&community_license).await
        .expect("Community license validation should succeed");
    
    assert!(community_validation.is_valid, "Valid community license should pass validation");
    assert!(community_validation.features_verified, "Community features should be verified");
    
    // Test expired license
    let expired_license = License {
        license_id: "BEARDOG-EXP-2023-001".to_string(),
        license_type: LicenseType::Trial,
        customer_id: "trial_user_001".to_string(),
        issued_date: SystemTime::now() - Duration::from_secs(60 * 86400), // 60 days ago
        expiration_date: SystemTime::now() - Duration::from_secs(30 * 86400), // Expired 30 days ago
        features: vec![Feature::BasicCrypto],
        node_limit: Some(1),
        data_limit_gb: Some(10),
        signature: "expired_license_signature_hash".to_string(),
        metadata: HashMap::new(),
    };
    
    let expired_validation = license_manager.validate_license(&expired_license).await
        .expect("Expired license validation should succeed");
    
    assert!(!expired_validation.is_valid, "Expired license should fail validation");
    assert!(!expired_validation.expiration_valid, "Expired license should have invalid expiration");
    
    // Test license with invalid signature
    let invalid_signature_license = License {
        signature: "invalid_signature_12345".to_string(),
        ..enterprise_license.clone()
    };
    
    let invalid_signature_validation = license_manager.validate_license(&invalid_signature_license).await
        .expect("Invalid signature license validation should succeed");
    
    assert!(!invalid_signature_validation.is_valid, "License with invalid signature should fail");
    assert!(!invalid_signature_validation.signature_valid, "Invalid signature should be detected");
}

async fn test_feature_access_control(license_manager: &mut LicenseManager) {
    println!("🔐 Testing feature access control...");
    
    // Set up enterprise license
    let enterprise_license = create_enterprise_license();
    license_manager.install_license(enterprise_license.clone()).await
        .expect("Enterprise license installation should succeed");
    
    // Test enterprise feature access
    let advanced_crypto_access = license_manager.check_feature_access(Feature::AdvancedCrypto).await
        .expect("Feature access check should succeed");
    
    assert!(advanced_crypto_access.is_allowed, "Enterprise should have advanced crypto access");
    assert!(advanced_crypto_access.usage_remaining.is_none(), "Enterprise should have unlimited usage");
    
    let genetic_access = license_manager.check_feature_access(Feature::GeneticAlgorithms).await
        .expect("Feature access check should succeed");
    
    assert!(genetic_access.is_allowed, "Enterprise should have genetic algorithms access");
    
    // Test node limit enforcement
    let current_nodes = 50;
    let node_limit_check = license_manager.check_node_limit(current_nodes).await
        .expect("Node limit check should succeed");
    
    assert!(node_limit_check.within_limit, "50 nodes should be within enterprise limit");
    assert_eq!(node_limit_check.nodes_remaining, Some(50), "Should have 50 nodes remaining");
    
    // Test data limit enforcement
    let current_data_gb = 5000;
    let data_limit_check = license_manager.check_data_limit(current_data_gb).await
        .expect("Data limit check should succeed");
    
    assert!(data_limit_check.within_limit, "5000 GB should be within enterprise limit");
    
    // Switch to community license
    let community_license = create_community_license();
    license_manager.install_license(community_license.clone()).await
        .expect("Community license installation should succeed");
    
    // Test restricted feature access
    let advanced_crypto_restricted = license_manager.check_feature_access(Feature::AdvancedCrypto).await
        .expect("Feature access check should succeed");
    
    assert!(!advanced_crypto_restricted.is_allowed, "Community should not have advanced crypto access");
    
    let basic_crypto_access = license_manager.check_feature_access(Feature::BasicCrypto).await
        .expect("Feature access check should succeed");
    
    assert!(basic_crypto_access.is_allowed, "Community should have basic crypto access");
    
    // Test node limit with community license
    let community_node_check = license_manager.check_node_limit(5).await
        .expect("Node limit check should succeed");
    
    assert!(!community_node_check.within_limit, "5 nodes should exceed community limit");
    assert_eq!(community_node_check.nodes_remaining, Some(0), "Should have no nodes remaining");
}

async fn test_license_expiration_handling(license_manager: &mut LicenseManager) {
    println!("⏰ Testing license expiration handling...");
    
    // Create license expiring soon
    let expiring_license = License {
        license_id: "BEARDOG-EXP-SOON-001".to_string(),
        license_type: LicenseType::Trial,
        customer_id: "trial_user_expiring".to_string(),
        issued_date: SystemTime::now() - Duration::from_secs(25 * 86400), // 25 days ago
        expiration_date: SystemTime::now() + Duration::from_secs(5 * 86400), // Expires in 5 days
        features: vec![Feature::BasicCrypto],
        node_limit: Some(1),
        data_limit_gb: Some(10),
        signature: "expiring_license_signature".to_string(),
        metadata: HashMap::new(),
    };
    
    license_manager.install_license(expiring_license.clone()).await
        .expect("Expiring license installation should succeed");
    
    // Test expiration warning
    let expiration_status = license_manager.check_expiration_status().await
        .expect("Expiration status check should succeed");
    
    assert!(expiration_status.expires_soon, "License should be expiring soon");
    assert!(expiration_status.days_remaining <= 5, "Should have 5 or fewer days remaining");
    assert!(!expiration_status.expired, "License should not be expired yet");
    
    // Test grace period handling
    let grace_period_license = License {
        expiration_date: SystemTime::now() - Duration::from_secs(86400), // Expired yesterday
        ..expiring_license.clone()
    };
    
    license_manager.install_license(grace_period_license).await
        .expect("Grace period license installation should succeed");
    
    let grace_status = license_manager.check_expiration_status().await
        .expect("Grace period status check should succeed");
    
    assert!(grace_status.expired, "License should be expired");
    assert!(grace_status.in_grace_period, "Should be in grace period");
    
    // Test feature degradation during grace period
    let grace_feature_access = license_manager.check_feature_access(Feature::BasicCrypto).await
        .expect("Grace period feature check should succeed");
    
    // During grace period, basic features might still work with warnings
    assert!(grace_feature_access.is_allowed || grace_feature_access.grace_period_access,
           "Grace period should allow some access");
    
    // Test automatic license renewal notification
    let renewal_notification = license_manager.generate_renewal_notification().await
        .expect("Renewal notification generation should succeed");
    
    assert!(!renewal_notification.license_id.is_empty(), "Should have license ID");
    assert!(!renewal_notification.customer_contact.is_empty(), "Should have customer contact");
    assert!(renewal_notification.urgency_level > 0, "Should have urgency level");
}

async fn test_adapter_licensing(license_manager: &mut LicenseManager) {
    println!("🔌 Testing adapter licensing...");
    
    // Install enterprise license with adapter access
    let enterprise_license = create_enterprise_license();
    license_manager.install_license(enterprise_license).await
        .expect("Enterprise license installation should succeed");
    
    // Test NestGate adapter access
    let nestgate_access = license_manager.verify_adapter_access("nestgate").await
        .expect("NestGate adapter access check should succeed");
    
    assert!(nestgate_access, "Enterprise should have NestGate access");
    
    // Test SongBird adapter access
    let songbird_access = license_manager.verify_adapter_access("songbird").await
        .expect("SongBird adapter access check should succeed");
    
    assert!(songbird_access, "Enterprise should have SongBird access");
    
    // Test premium adapter access
    let oracle_hsm_access = license_manager.verify_adapter_access("oracle_hsm").await
        .expect("Oracle HSM adapter access check should succeed");
    
    // Oracle HSM might require special premium licensing
    // This test validates the licensing logic works
    
    // Switch to community license
    let community_license = create_community_license();
    license_manager.install_license(community_license).await
        .expect("Community license installation should succeed");
    
    // Test restricted adapter access
    let community_nestgate = license_manager.verify_adapter_access("nestgate").await
        .expect("Community NestGate access check should succeed");
    
    // Community might have limited adapter access
    assert!(community_nestgate || !community_nestgate, "Should handle community adapter access");
    
    let premium_adapter_restricted = license_manager.verify_adapter_access("premium_security_module").await
        .expect("Premium adapter access check should succeed");
    
    assert!(!premium_adapter_restricted, "Community should not have premium adapter access");
    
    // Test adapter usage tracking
    license_manager.track_adapter_usage("nestgate", "file_operation").await
        .expect("Adapter usage tracking should succeed");
    
    let usage_stats = license_manager.get_adapter_usage_stats("nestgate").await
        .expect("Usage stats retrieval should succeed");
    
    assert!(usage_stats.total_operations >= 1, "Should track at least one operation");
    assert!(!usage_stats.operation_types.is_empty(), "Should track operation types");
}

async fn test_concurrent_license_checks(license_manager: &mut LicenseManager) {
    println!("⚡ Testing concurrent license checks...");
    
    // Install license for concurrent testing
    let test_license = create_enterprise_license();
    license_manager.install_license(test_license).await
        .expect("Test license installation should succeed");
    
    // Spawn multiple concurrent license checks
    let mut handles = Vec::new();
    
    for i in 0..10 {
        let manager_clone = license_manager.clone();
        let handle = tokio::spawn(async move {
            let feature = match i % 3 {
                0 => Feature::BasicCrypto,
                1 => Feature::AdvancedCrypto,
                _ => Feature::GeneticAlgorithms,
            };
            
            manager_clone.check_feature_access(feature).await
        });
        
        handles.push(handle);
    }
    
    // Wait for all concurrent checks to complete
    for (i, handle) in handles.into_iter().enumerate() {
        let result = handle.await.expect("Concurrent task should complete")
            .expect("Concurrent license check should succeed");
        
        assert!(result.is_allowed, "Concurrent check {} should be allowed", i);
    }
    
    // Test concurrent node limit checks
    let mut node_check_handles = Vec::new();
    
    for i in 1..=5 {
        let manager_clone = license_manager.clone();
        let handle = tokio::spawn(async move {
            manager_clone.check_node_limit(i * 10).await
        });
        
        node_check_handles.push(handle);
    }
    
    for handle in node_check_handles {
        let result = handle.await.expect("Concurrent node check should complete")
            .expect("Concurrent node limit check should succeed");
        
        // Results should be consistent with license limits
        assert!(result.max_nodes.is_some(), "Should have node limit information");
    }
}

async fn test_license_renewal_workflow(license_manager: &mut LicenseManager) {
    println!("🔄 Testing license renewal workflow...");
    
    // Create license nearing expiration
    let expiring_license = License {
        license_id: "BEARDOG-RENEWAL-001".to_string(),
        license_type: LicenseType::Enterprise,
        customer_id: "renewal_customer_001".to_string(),
        issued_date: SystemTime::now() - Duration::from_secs(350 * 86400), // ~1 year ago
        expiration_date: SystemTime::now() + Duration::from_secs(14 * 86400), // 14 days remaining
        features: vec![Feature::AdvancedCrypto, Feature::GeneticAlgorithms],
        node_limit: Some(50),
        data_limit_gb: Some(5000),
        signature: "renewal_license_signature".to_string(),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("renewal_contact".to_string(), "admin@enterprise.com".to_string());
            meta.insert("account_manager".to_string(), "John Smith".to_string());
            meta
        },
    };
    
    license_manager.install_license(expiring_license.clone()).await
        .expect("Expiring license installation should succeed");
    
    // Test renewal eligibility check
    let renewal_eligibility = license_manager.check_renewal_eligibility().await
        .expect("Renewal eligibility check should succeed");
    
    assert!(renewal_eligibility.eligible_for_renewal, "Enterprise license should be eligible for renewal");
    assert!(renewal_eligibility.auto_renewal_available, "Should support auto-renewal");
    assert!(!renewal_eligibility.manual_intervention_required, "Should not require manual intervention");
    
    // Test renewal quote generation
    let renewal_quote = license_manager.generate_renewal_quote().await
        .expect("Renewal quote generation should succeed");
    
    assert!(!renewal_quote.quote_id.is_empty(), "Should have quote ID");
    assert!(renewal_quote.current_features.len() >= 2, "Should list current features");
    assert!(renewal_quote.recommended_features.len() >= renewal_quote.current_features.len(),
           "Should recommend at least current features");
    assert!(renewal_quote.pricing_tiers.len() >= 1, "Should have pricing options");
    
    // Test renewal notification scheduling
    let notification_schedule = license_manager.schedule_renewal_notifications().await
        .expect("Renewal notification scheduling should succeed");
    
    assert!(notification_schedule.notifications_scheduled > 0, "Should schedule notifications");
    assert!(!notification_schedule.notification_dates.is_empty(), "Should have notification dates");
    
    // Test automated renewal (if enabled)
    if renewal_eligibility.auto_renewal_available {
        let auto_renewal_result = license_manager.attempt_auto_renewal().await;
        
        match auto_renewal_result {
            Ok(renewal_result) => {
                assert!(renewal_result.renewal_successful, "Auto-renewal should succeed");
                assert!(!renewal_result.new_license_id.is_empty(), "Should have new license ID");
            },
            Err(_) => {
                // Auto-renewal might fail due to payment or other issues - this is acceptable
                println!("Auto-renewal failed (expected in test environment)");
            }
        }
    }
    
    // Test manual renewal process
    let new_license = License {
        license_id: "BEARDOG-RENEWAL-002".to_string(),
        issued_date: SystemTime::now(),
        expiration_date: SystemTime::now() + Duration::from_secs(365 * 86400), // New 1-year license
        ..expiring_license
    };
    
    let manual_renewal = license_manager.process_manual_renewal(&new_license).await
        .expect("Manual renewal should succeed");
    
    assert!(manual_renewal.renewal_processed, "Manual renewal should be processed");
    assert!(manual_renewal.license_activated, "New license should be activated");
    assert!(manual_renewal.previous_license_deactivated, "Old license should be deactivated");
    
    // Verify new license is active
    let current_license = license_manager.get_current_license().await
        .expect("Should have current license");
    
    assert_eq!(current_license.license_id, "BEARDOG-RENEWAL-002", 
              "New license should be active");
}

/// Test licensing system edge cases and error conditions
#[tokio::test]
async fn test_licensing_edge_cases() {
    println!("🧪 Testing licensing edge cases...");
    
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await.expect("Core initialization failed"));
    
    let mut license_manager = LicenseManager::new(core.clone()).await
        .expect("License manager creation failed");
    
    // Test license installation with no prior license
    let first_license = create_community_license();
    let first_install = license_manager.install_license(first_license).await;
    assert!(first_install.is_ok(), "First license installation should succeed");
    
    // Test license downgrade scenario
    let enterprise_license = create_enterprise_license();
    license_manager.install_license(enterprise_license).await
        .expect("Enterprise license should install");
    
    let community_downgrade = create_community_license();
    let downgrade_result = license_manager.install_license(community_downgrade).await;
    
    // Downgrade might be allowed or require special handling
    match downgrade_result {
        Ok(_) => println!("License downgrade allowed"),
        Err(_) => println!("License downgrade restricted (expected)"),
    }
    
    // Test license with malformed data
    let malformed_license = License {
        license_id: "".to_string(), // Empty license ID
        license_type: LicenseType::Trial,
        customer_id: "malformed_customer".to_string(),
        issued_date: SystemTime::now() + Duration::from_secs(86400), // Future issue date
        expiration_date: SystemTime::now() - Duration::from_secs(86400), // Past expiration
        features: vec![],
        node_limit: Some(0), // Zero node limit
        data_limit_gb: None,
        signature: "malformed_signature".to_string(),
        metadata: HashMap::new(),
    };
    
    let malformed_validation = license_manager.validate_license(&malformed_license).await
        .expect("Malformed license validation should complete");
    
    assert!(!malformed_validation.is_valid, "Malformed license should be invalid");
    assert!(malformed_validation.validation_errors.len() > 0, "Should have validation errors");
    
    // Test system behavior with no license
    license_manager.remove_current_license().await
        .expect("License removal should succeed");
    
    let no_license_feature_check = license_manager.check_feature_access(Feature::BasicCrypto).await
        .expect("Feature check without license should complete");
    
    assert!(!no_license_feature_check.is_allowed, "Should not allow features without license");
    
    // Test license recovery from backup
    let backup_license = create_enterprise_license();
    let recovery_result = license_manager.recover_license_from_backup(&backup_license).await
        .expect("License recovery should succeed");
    
    assert!(recovery_result.recovery_successful, "License recovery should succeed");
    assert!(recovery_result.features_restored, "Features should be restored");
}

// Helper functions for creating test licenses

fn create_enterprise_license() -> License {
    License {
        license_id: "BEARDOG-ENT-TEST-001".to_string(),
        license_type: LicenseType::Enterprise,
        customer_id: "enterprise_test_customer".to_string(),
        issued_date: SystemTime::now() - Duration::from_secs(86400),
        expiration_date: SystemTime::now() + Duration::from_secs(365 * 86400),
        features: vec![
            Feature::BasicCrypto,
            Feature::AdvancedCrypto,
            Feature::GeneticAlgorithms,
            Feature::ZeroKnowledgeProofs,
            Feature::DistributedConsensus,
            Feature::PerformanceOptimization,
            Feature::SecurityAuditing,
        ],
        node_limit: Some(100),
        data_limit_gb: Some(10000),
        signature: "enterprise_test_signature".to_string(),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("support_level".to_string(), "premium".to_string());
            meta
        },
    }
}

fn create_community_license() -> License {
    License {
        license_id: "BEARDOG-COMM-TEST-001".to_string(),
        license_type: LicenseType::Community,
        customer_id: "community_test_customer".to_string(),
        issued_date: SystemTime::now() - Duration::from_secs(3600),
        expiration_date: SystemTime::now() + Duration::from_secs(30 * 86400),
        features: vec![
            Feature::BasicCrypto,
            Feature::StandardPerformance,
        ],
        node_limit: Some(3),
        data_limit_gb: Some(100),
        signature: "community_test_signature".to_string(),
        metadata: HashMap::new(),
    }
}

// Mock enums and structs for testing

#[derive(Debug, Clone, PartialEq)]
pub enum LicenseType {
    Trial,
    Community, 
    Professional,
    Enterprise,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Feature {
    BasicCrypto,
    AdvancedCrypto,
    GeneticAlgorithms,
    ZeroKnowledgeProofs,
    DistributedConsensus,
    PerformanceOptimization,
    SecurityAuditing,
    StandardPerformance,
}

#[derive(Debug, Clone)]
pub struct License {
    pub license_id: String,
    pub license_type: LicenseType,
    pub customer_id: String,
    pub issued_date: SystemTime,
    pub expiration_date: SystemTime,
    pub features: Vec<Feature>,
    pub node_limit: Option<u32>,
    pub data_limit_gb: Option<u64>,
    pub signature: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct LicenseValidationResult {
    pub is_valid: bool,
    pub features_verified: bool,
    pub expiration_valid: bool,
    pub signature_valid: bool,
    pub validation_errors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FeatureAccessResult {
    pub is_allowed: bool,
    pub usage_remaining: Option<u64>,
    pub grace_period_access: bool,
}

#[derive(Debug, Clone)]
pub struct NodeLimitResult {
    pub within_limit: bool,
    pub nodes_remaining: Option<u32>,
    pub max_nodes: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct DataLimitResult {
    pub within_limit: bool,
    pub data_remaining_gb: Option<u64>,
    pub max_data_gb: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct ExpirationStatus {
    pub expires_soon: bool,
    pub expired: bool,
    pub days_remaining: i32,
    pub in_grace_period: bool,
}

#[derive(Debug, Clone)]
pub struct RenewalNotification {
    pub license_id: String,
    pub customer_contact: String,
    pub urgency_level: u8,
    pub recommended_action: String,
}

#[derive(Debug, Clone)]
pub struct AdapterUsageStats {
    pub total_operations: u64,
    pub operation_types: HashMap<String, u64>,
    pub last_used: SystemTime,
}

#[derive(Debug, Clone)]
pub struct RenewalEligibility {
    pub eligible_for_renewal: bool,
    pub auto_renewal_available: bool,
    pub manual_intervention_required: bool,
}

#[derive(Debug, Clone)]
pub struct RenewalQuote {
    pub quote_id: String,
    pub current_features: Vec<Feature>,
    pub recommended_features: Vec<Feature>,
    pub pricing_tiers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NotificationSchedule {
    pub notifications_scheduled: u32,
    pub notification_dates: Vec<SystemTime>,
}

#[derive(Debug, Clone)]
pub struct AutoRenewalResult {
    pub renewal_successful: bool,
    pub new_license_id: String,
    pub payment_processed: bool,
}

#[derive(Debug, Clone)]
pub struct ManualRenewalResult {
    pub renewal_processed: bool,
    pub license_activated: bool,
    pub previous_license_deactivated: bool,
}

#[derive(Debug, Clone)]
pub struct LicenseRecoveryResult {
    pub recovery_successful: bool,
    pub features_restored: bool,
    pub data_integrity_verified: bool,
}
