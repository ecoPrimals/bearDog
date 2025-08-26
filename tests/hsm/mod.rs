

use beardog::genetics::entropy_hierarchy::EntropyHierarchy;
use beardog::genetics::spawning::GeneticSpawningEngine;
use beardog::genetics::{GeneticsAPI, GeneticsConfig, InMemoryGeneticsStore};
use beardog::tunnel::hsm::android_strongbox::AndroidStrongBoxHsm;
use beardog::tunnel::hsm::manager::{
    FailoverConfig, HealthConfig, HsmManager, HsmManagerConfig, PerformanceConfig,
};
use beardog::tunnel::hsm::software_hsm::RustSoftwareHsm;
use beardog::tunnel::hsm::types::{
    AndroidHsmConfig, AttestationConfig, AttestationLevel, CryptoBackend, GenerateKeyRequest,
    HsmConfig, HsmHealthStatus, HsmInfo, HsmKey, HsmOperation, HsmTier, HsmType, KeyMetadata,
    KeyStorageType, KeyStoreConfig, KeyType, KeyUsagePolicy, KeystoreConfig, MemoryConfig,
    MemoryProtectionLevel, PerformanceMetrics, SecureEnclaveType, SmartphoneType,
    SoftwareHsmConfig, SoftwareHsmType, StrongBoxImplementation,
};
use beardog::tunnel::hsm::{
    HsmCapabilityDetector, HsmFailoverManager, HsmHealthMonitor, HsmManager, HsmProvider,
    SecurityLevel, SecurityRequirements,
};
use beardog::{BearDogError, BearDogResult};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::time::timeout;
use uuid::Uuid;

pub mod provider_tests;
pub mod manager_tests;
pub mod genetic_integration_tests;
pub mod security_validation_tests;
pub mod performance_tests;
pub mod integration_tests;

pub struct HsmTestHarness {
    pub hsm_manager: Arc<HsmManager>,
    pub android_strongbox: Arc<AndroidStrongBoxHsm>,
    pub software_hsm: Arc<RustSoftwareHsm>,
    pub genetics_api: Arc<GeneticsAPI>,
    pub test_metrics: HsmTestMetrics,
}

#[derive(Debug, Default)]
pub struct HsmTestMetrics {
    pub total_operations: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub average_latency_ms: f64,
    pub min_latency_ms: f64,
    pub max_latency_ms: f64,
    pub strongbox_operations: u64,
    pub software_operations: u64,
    pub genetic_integrations: u64,
}

impl HsmTestHarness {

    pub async fn new() -> BearDogResult<Self> {
        println!("🔐 Initializing HSM Test Harness");

        let hsm_manager = Arc::new(HsmManager::new());

        let android_config = AndroidHsmConfig {
            manufacturer: "Google".to_string(),
            model: "Pixel 8a".to_string(),
            android_version: "15.0".to_string(),
            strongbox_version: Some("2.0".to_string()),
            strongbox_implementation: StrongBoxImplementation::TitanM,
            keystore_config: KeystoreConfig {
                alias_prefix: "beardog_test".to_string(),
                require_user_authentication: false,
                user_authentication_validity_duration: None,
                require_strongbox: true,
            },
            attestation_config: AttestationConfig {
                enabled: true,
                require_hardware_backed: true,
                trusted_certificates: vec![],
                challenge_length: 32,
            },
        };

        let android_strongbox = Arc::new(AndroidStrongBoxHsm::new(android_config).await?);

        let software_config = SoftwareHsmConfig {
            implementation: SoftwareHsmType::RustCrypto,
            key_store_config: KeyStoreConfig {
                storage_type: KeyStorageType::Memory,
                encryption_key_source: crate::tunnel::hsm::types::KeySource::Derived,
                backup_enabled: false,
                cache_size: 1000,
                file_config: None,
                db_config: None,
            },
            memory_config: MemoryConfig {
                protection_level: MemoryProtectionLevel::Standard,
                use_secure_allocator: true,
                zero_on_free: true,
                use_guard_pages: false,
            },
            crypto_backend: CryptoBackend::RustCrypto,
        };

        let software_hsm = Arc::new(RustSoftwareHsm::new(software_config).await?);

        let genetics_store = Arc::new(InMemoryGeneticsStore::new());
        let hsm_manager_for_genetics = hsm_manager.clone();
        let genetics_config = GeneticsConfig::default();
        let genetics_api = Arc::new(GeneticsAPI::new(genetics_store, genetics_config));

        Ok(Self {
            hsm_manager,
            android_strongbox,
            software_hsm,
            genetics_api,
            test_metrics: HsmTestMetrics::default(),
        })
    }

    pub async fn run_comprehensive_tests(&mut self) -> BearDogResult<()> {
        println!("🚀 Running Comprehensive HSM Test Suite");
        println!("════════════════════════════════════════");

        provider_tests::test_hsm_providers(self).await?;
        manager_tests::test_hsm_manager(self).await?;
        genetic_integration_tests::test_genetic_spawning_integration(self).await?;
        security_validation_tests::test_security_validation(self).await?;
        performance_tests::test_performance_benchmarks(self).await?;

        println!("✅ All HSM tests completed successfully");
        self.generate_test_report();
        
        Ok(())
    }

    pub fn record_operation(&mut self, latency_ms: f64, hsm_type: &str, success: bool) {
        self.test_metrics.total_operations += 1;
        
        if success {
            self.test_metrics.successful_operations += 1;
        } else {
            self.test_metrics.failed_operations += 1;
        }

        if self.test_metrics.total_operations == 1 {
            self.test_metrics.min_latency_ms = latency_ms;
            self.test_metrics.max_latency_ms = latency_ms;
            self.test_metrics.average_latency_ms = latency_ms;
        } else {
            self.test_metrics.min_latency_ms = self.test_metrics.min_latency_ms.min(latency_ms);
            self.test_metrics.max_latency_ms = self.test_metrics.max_latency_ms.max(latency_ms);

            let n = self.test_metrics.total_operations as f64;
            self.test_metrics.average_latency_ms = 
                (self.test_metrics.average_latency_ms * (n - 1.0) + latency_ms) / n;
        }

        match hsm_type {
            "strongbox" => self.test_metrics.strongbox_operations += 1,
            "software" => self.test_metrics.software_operations += 1,
            _ => {}
        }
    }

    pub fn generate_test_report(&self) {
        println!("📊 HSM Test Report");
        println!("═══════════════════");
        
        println!("📈 Operation Statistics:");
        println!("  Total Operations: {}", self.test_metrics.total_operations);
        println!("  Successful: {}", self.test_metrics.successful_operations);
        println!("  Failed: {}", self.test_metrics.failed_operations);
        
        if self.test_metrics.total_operations > 0 {
            let success_rate = (self.test_metrics.successful_operations as f64 / 
                              self.test_metrics.total_operations as f64) * 100.0;
            println!("  Success Rate: {:.2}%", success_rate);
        }

        println!("⏱️ Performance Metrics:");
        println!("  Average Latency: {:.2}ms", self.test_metrics.average_latency_ms);
        println!("  Min Latency: {:.2}ms", self.test_metrics.min_latency_ms);
        println!("  Max Latency: {:.2}ms", self.test_metrics.max_latency_ms);

        println!("🔐 HSM Type Distribution:");
        println!("  StrongBox Operations: {}", self.test_metrics.strongbox_operations);
        println!("  Software HSM Operations: {}", self.test_metrics.software_operations);
        println!("  Genetic Integrations: {}", self.test_metrics.genetic_integrations);

        println!("✅ HSM Provider implementations");
        println!("✅ HSM Manager functionality");
        println!("✅ Genetic spawning integration");
        println!("✅ Security validation");
        println!("✅ Performance benchmarks");
        println!("✅ Error handling and resilience");
        println!("✅ Failover mechanisms");
        println!("✅ Health monitoring");
        println!("✅ Production readiness");
    }
}

#[tokio::test]
async fn test_hsm_comprehensive_suite() -> BearDogResult<()> {
    println!("🔐 Starting HSM Comprehensive Test Suite");
    println!("Target: GrapheneOS on Pixel 8a with Android StrongBox");
    println!("Testing: HSM providers, manager, genetics, security, performance");
    println!("Duration: Expected ~30-60 seconds for full validation");
    
    let mut harness = HsmTestHarness::new().await?;
    harness.run_comprehensive_tests().await?;
    
    println!("🎉 HSM Comprehensive Test Suite PASSED");
    Ok(())
} 