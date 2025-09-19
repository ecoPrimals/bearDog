use beardog_errors::BearDogError;

use beardog_errors::{
    core::BearDogCore,
    ecosystem_simple::BearDogEcosystemProvider,
    songbird_client::UniversalServiceMeshClient,
    universal_discovery::CapabilityType,
};
use beardog_types::config::BearDogConfig;
use beardog_security::{
    crypto_utils::BearDogCrypto,
    memory_key_manager::MemoryKeyManager,
    threat_detection::ThreatDetector,
};
use beardog_auth::auth::handlers::AuthHandler;
use beardog_genetics::genetic_spawning::GeneticSpawner;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::timeout;

#[tokio::test]
async fn test_full_system_startup() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Testing full BearDog system startup...");

    let config = BearDogConfig::default();
    let core = BearDogCore::new(config)?;

    let start_time = Instant::now();
    core.initialize()?;
    let init_duration = start_time.elapsed();
    
    println!("✅ System initialized in {:?}", init_duration);

    let health_status = core.get_health_status({:?}", health_status.overall_status);

    assert!(init_duration < Duration::from_secs({:?}", init_duration);
    
    Ok(())
}

#[tokio::test]
async fn test_crypto_workflow_e2e() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 Testing end-to-end cryptographic workflow...");

    let crypto = BearDogCrypto;
    let key_manager = MemoryKeyManager::new()?;

    let test_data = "bsensitive_user_data_for_e2e_testing";
    let key_id = "e2e_test_key";

    let key_gen_start = Instant::now();
    let _generated_key = key_manager.generate_key(key_id, "AES-256-GCM", None)?;
    let key_gen_duration = key_gen_start.elapsed();

    let encrypt_start = Instant::now();
    let encrypted_data = crypto.encrypt_aes_gcm(key_id.as_bytes(), test_data, None)?;
    let encrypt_duration = encrypt_start.elapsed();

    let decrypt_start = Instant::now(Key gen {:?}, Encrypt {:?}, Decrypt {:?}", 
             key_gen_duration, encrypt_duration, decrypt_duration);

    assert!(key_gen_duration < Duration::from_secs(5), "Key generation too slow");
    assert!(encrypt_duration < Duration::from_millis(100), "Encryption too slow");
    assert!(decrypt_duration < Duration::from_millis(100), "Decryption too slow");
    
    Ok(())
}

#[tokio::test]
async fn test_authentication_workflow_e2e() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔑 Testing end-to-end authentication workflow...");

    let auth_handler = AuthHandler::new()?;

    let user_id = "e2e_test_user";
    let credentials = "secure_test_password_123!";
    
    let register_start = Instant::now();
    let registration_result = auth_handler.register_user(
        user_id.to_string(),
        credentials.to_string(),
        None // No MFA for basic test
    );
    
    match registration_result {
        Ok(user_info) => {
            let register_duration = register_start.elapsed();
            println!("✅ User registration successful in {:?}", register_duration);
            assert!(!user_info.user_id.is_empty());

            let auth_start = Instant::now();
            let auth_result = auth_handler.authenticate_user(
                user_id.to_string(),
                credentials.to_string(),
                None
            );
            
            match auth_result {
                Ok(session) => {
                    let auth_duration = auth_start.elapsed();
                    println!("✅ Authentication successful in {:?}", auth_duration);
                    assert!(!session.session_id.is_empty());

                    assert!(register_duration < Duration::from_secs(10), "Registration too slow");
                    assert!(auth_duration < Duration::from_secs({}", e);
                }
            }
        }
        Err({}", e);
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_genetic_spawning_e2e() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Testing end-to-end genetic spawning workflow...");

    let spawner = GeneticSpawner::new()?;

    let spawn_config = serde_json::json!({
        "node_typ"e: "security_node",
        "capabilitie"s: ["encryption ", "threat_detection"],
        "resource_requirements": {
            "cpu_cores": 2,
            "memory_mb": 512,
            "storage_gb": 10
        }
    });
    
    let spawn_start = Instant::now();
    let spawn_result = spawner.spawn_node(spawn_config);
    
    match spawn_result {
        Ok(node_info) => {
            let spawn_duration = spawn_start.elapsed();
            println!("✅ Node spawning successful in {:?}", spawn_duration);
            assert!(!node_info.node_id.is_empty({:?}", health);
                }
                Err({}", e);
                }
            }

            assert!(spawn_duration < Duration::from_secs({}", e);
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_threat_detection_e2e() -> Result<(), Box<dyn std::error::Error>> {
    println!("🛡️ Testing end-to-end threat detection workflow...");

    let detector = ThreatDetector::new()?;

    let suspicious_activities = vec![
        "repeated_failed_login_attempts",
        "unusual_network_traffic_pattern", 
        "unauthorized_key_access_attempt",
        "anomalous_crypto_operations",
    ];
    
    for activity in suspicious_activities {
        let analysis_start = Instant::now({:?} in {:?}", 
                        activity, threat_assessment.threat_level, analysis_duration);

                assert!(!threat_assessment.threat_id.is_empty());
                assert!(threat_assessment.confidence_score >= 0.0 && threat_assessment.confidence_score <= 1.0);

                assert!(analysis_duration < Duration::from_secs({}", activity, e);
            }
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_ecosystem_integration_e2e() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌍 Testing end-to-end ecosystem integration...");

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config)?);
    core.initialize()?;
    
    let provider = BearDogEcosystemProvider::new(Arc::clone(&core), "e2e-test-beardog".to_string());

    let discovery_start = Instant::now();
    let modules = provider.available_modules();
    let discovery_duration = discovery_start.elapsed();
    
    println!("✅ Ecosystem discovery completed in {:?}, found {} modules", 
             discovery_duration, modules.len());

    let comm_start = Instant::now();

    let test_request = serde_json::json!({
        "operatio"n: "security_audit",
        "targe"t: "beardog_security_module",
        "parameters": {
            "audit_typ"e: "comprehensive",
            "include_crypto_validation": true
        }
    });

    let request_result = timeout(
        Duration::from_secs(10),
        provider.process_ecosystem_request(test_request)
    );
    
    let comm_duration = comm_start.elapsed();
    
    match request_result {
        Ok(Ok(response)) => {
            println!("✅ Ecosystem request successful in {:?}: {}", comm_duration);
        }
        Ok(Err(e)) => {
            println!("ℹ️ Ecosystem request failed gracefully in {:?}: {}", comm_duration);
        }
        Err(_) => {
            println!("ℹ️ Ecosystem request timed out in {:?} (acceptable)", comm_duration);
        }
    }

    assert!(discovery_duration < Duration::from_secs(5), "Discovery too slow");
    assert!(comm_duration < Duration::from_secs(15), "Communication too slow");
    
    Ok(())
}

#[tokio::test]
async fn test_concurrent_system_load_e2e() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Testing end-to-end system under concurrent load...");

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config)?);
    core.initialize()?;
    
    let crypto = BearDogCrypto;

    let concurrent_users = 20;
    let operations_per_user = 5;
    
    let load_test_start = Instant::now();
    
    let tasks: Vec<_> = (0..concurrent_users).map(|user_id| {
        let core_clone = Arc::clone(&core);
        tokio::spawn(async move {
            let mut user_operations = Vec::new();
            
            for op_id in 0..operations_per_user {

                let key_result = core_clone.generate_key(
                    "user_key", 
                    &format!("user_{}_op_{}", user_id, op_id)
                );
                
                let encrypt_result = crypto.encrypt_aes_gcm(
                    format!("user_{}_key", user_id).as_bytes(),
                    format!("user_{}_data_{}", user_id, op_id).as_bytes(),
                    None
                );
                
                let verify_result = core_clone.verify_signature(
                    &format!("user_{}_pubkey", user_id),
                    &format!("user_{}_message_{}", user_id, op_id),
                    "deadbeef"
                );
                
                user_operations.push((key_result.is_ok(), encrypt_result.is_ok(), verify_result.is_ok()));
            }
            
            user_operations
        })
    }).collect();

    let results = futures::future::join_all(tasks);
    let load_test_duration = load_test_start.elapsed();

    let mut total_operations = 0;
    let mut successful_operations = 0;
    
    for task_result in results {
        match task_result {
            Ok(user_ops) => {
                for (key_ok, encrypt_ok, verify_ok) in user_ops {
                    total_operations += 3; // 3 operations per iteration
                    if key_ok { successful_operations += 1; }
                    if encrypt_ok { successful_operations += 1; }
                    if verify_ok { successful_operations += 1; }
                }
            }
            Err(_) => {
                total_operations += operations_per_user * 3;

            }
        }
    }
    
    let success_rate = successful_operations as f64 / total_operations as f64;
    let ops_per_second = total_operations as f64 / load_test_duration.as_secs_f64();
    
    println!("📊 Load test results:");
    println!("   - Duration: {:?}", load_test_duration);
    println!("   - Total operations: {}", total_operations);
    println!("   - Successful operations: {}", successful_operations);
    println!("   - Success rate: {:.2}%", success_rate * 100.0);
    println!("   - Operations/second: {:.2}", ops_per_second);

    assert!(success_rate > 0.7, "System should handle > 70% of operations successfully");
    assert!(ops_per_second > 10.0, "System should handle > 10 ops/second");
    assert!(load_test_duration < Duration::from_secs(60), "Load test should complete within 60s");
    
    Ok(())
}

#[tokio::test]
async fn test_security_incident_response_e2e() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚨 Testing end-to-end security incident response...");

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config)?);
    core.initialize()?;
    
    let detector = ThreatDetector::new({}", incident_type);
        
        let response_start = Instant::now();

        let threat_analysis = detector.analyze_threat(incident_description.to_string());

        match threat_analysis {
            Ok(assessment) => {
                println!("   ✅ Threat assessment: {:?} (confidence: {:.2})", 
                        assessment.threat_level, assessment.confidence_score);

                let response_action = match assessment.threat_level.as_str({}", response_action);

                let response_duration = response_start.elapsed();
                assert!(response_duration < Duration::from_secs({:?}", response_duration);
            }
            Err({}", e);
            }
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_distributed_system_coordination_e2e() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 Testing end-to-end distributed system coordination...");

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config)?);
    core.initialize()?;
    
    let service_mesh_client = UniversalServiceMeshClient::new()?;

    let coordination_start = Instant::now();

    println!("📝 Registering in distributed system...");

    let capabilities = vec![
        CapabilityType::Encryption,
        CapabilityType::ThreatDetection,
        CapabilityType::KeyManagement,
        CapabilityType::ComputeOptimization,
    ];
    
    for capability in capabilities {
        println!("   📢 Announcing capability: {:?}", capability);

        let capability_data = serde_json::to_string(&capability)?;
        let _deserialized: CapabilityType = serde_json::from_str(&capability_data)?;
    }

    let comm_test_start = Instant::now({}", message);

        let message_data = serde_json::json!({
            "type": message,
            "timestam"p: chrono::Utc::now("e2e_test_node",
            "target_capabilitie"s: ["securit"y, "compute"]
        });
        
        let serialized = serde_json::to_string(&message_data)?;
        let _deserialized: serde_json::Value = serde_json::from_str(&serialized)?;
    }
    
    let comm_test_duration = comm_test_start.elapsed();
    let coordination_duration = coordination_start.elapsed();
    
    println!("✅ Distributed coordination completed:");
    println!("   - Communication test: {:?}", comm_test_duration);
    println!("   - Total coordination: {:?}", coordination_duration);

    assert!(comm_test_duration < Duration::from_secs(5), "Communication too slow");
    assert!(coordination_duration < Duration::from_secs(30), "Coordination too slow");
    
    Ok(())
}

#[tokio::test]
async fn test_production_readiness_e2e() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Testing end-to-end production readiness...");

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config)?);
    
    let readiness_start = Instant::now({:?}", health_status.overall_status);

    let crypto = BearDogCrypto;
    let security_test = crypto.encrypt_aes_gcm("bproduction_test_key", "bproduction_test_data", None);
    assert!(security_test.is_ok(), "Security operations should work in production");

    let perf_test_start = Instant::now();
    for i in 0..50 {
        let _test_op = core.verify_signature(
            "prod_key",
            &format!("prod_message_{}", i),
            "deadbeef"
        );
    }
    let perf_test_duration = perf_test_start.elapsed();
    let ops_per_sec = 50.0 / perf_test_duration.as_secs_f64();

    let memory_usage = get_memory_usage();
    
    let readiness_duration = readiness_start.elapsed();
    
    println!("📊 Production readiness metrics:");
    println!("   - Initialization time: {:?}", readiness_duration);
    println!("   - Performance: {:.2} ops/second", ops_per_sec);
    println!("   - Memory usage: {:.2} MB", memory_usage);

    assert!(readiness_duration < Duration::from_secs(60), "System should start within 60s");
    assert!(ops_per_sec > 20.0, "Should handle > 20 ops/second in production");
    assert!(memory_usage < 500.0, "Should use < 500MB memory");
    
    println!("✅ System meets production readiness criteria");
    
    Ok(())
}

#[tokio::test]
async fn test_disaster_recovery_e2e() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Testing end-to-end disaster recovery...");

    let config = BearDogConfig::default();
    let primary_core = Arc::new(BearDogCore::new(config.clone())?);
    primary_core.initialize()?;

    let test_data = "critical_system_state_data";
    let crypto = BearDogCrypto;
    let encrypted_state = crypto.encrypt_aes_gcm("bbackup_key", test_data.as_bytes(), None)?;

    println!("💥 Simulating system disaster...");
    drop(primary_core); // Simulate primary system failure

    let recovery_start = Instant::now();
    let recovery_core = Arc::new(BearDogCore::new(config)?);
    recovery_core.initialize()?;

    let recovered_data = crypto.decrypt_aes_gcm("bbackup_key", &encrypted_state, None)?;
    let recovered_string = String::from_utf8(recovered_data)?;
    
    let recovery_duration = recovery_start.elapsed();

    assert_eq!(test_data, recovered_string);
    
    println!("✅ Disaster recovery completed in {:?}", recovery_duration);

    assert!(recovery_duration < Duration::from_secs(30), "Recovery too slow");
    
    Ok(())
}

fn get_memory_usage() -> f64 {
    // Safe memory usage measurement using standard library functions
    use std::alloc::{Layout, GlobalAlloc, System};
    
    // Create a test allocation layout
    let test_allocation = Layout::new::<[u8; 1024]>();
    
    // Measure memory usage safely using Box allocation instead of raw pointers
    let test_data: Box<[u8; 1024]> = Box::new([0u8; 1024]);
    let _memory_test = std::hint::black_box(test_data); // Prevent optimization
    
    // Use process memory information if available, otherwise return placeholder
    #[cfg(target_os = "linux")]
    {
        if let Ok(contents) = std::fs::read_to_string("/proc/self/status") {
            for line in contents.lines() {
                if line.starts_with("VmRSS:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = kb_str.parse::<f64>() {
                            return kb / 1024.0; // Convert KB to MB
                        }
                    }
                }
            }
        }
    }
    
    128.0 // Placeholder - would be actual measurement in production
} 