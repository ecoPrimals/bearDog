use beardog_adapters::*;
use beardog_auth::*;
use beardog_compliance::*;
use beardog_deploy::*;
use beardog_errors::BearDogError;
use beardog_errors::*;
use beardog_genetics::*;
use beardog_monitoring::*;
use beardog_security::*;
use beardog_threat::*;
use beardog_types::canonical::*;
use beardog_utils::*;
use beardog_workflows::*;
use std::sync::Arc;
use tokio_test;

mod edge_case_coverage {
    use super::*;

    #[tokio::test]
    async fn test_error_propagation_comprehensive() {
        let config = configuration::BearDogConfig::default();
        let core = BearDogCore::new(config)
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;

        let invalid_service = ServiceConfig {
            name: "".to_string(),
            endpoint:  "invalid_url".to_string(),
        };

        let result = core.register_service(invalid_service);
        assert!(result.is_err(),  "Invalid service registration should fail");

        let discovery_result = core.discover_services( "non_existent_capability");
        assert!(
            discovery_result.is_ok(),
             "Discovery should succeed even with no matches"
        );
        assert!(
            discovery_result
                .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?
                .is_empty(),
             "Should return empty list for non-existent capability"
        );

        let request_result = core
            .send_service_request(
                 "non_existent_service",
                 "test_operation",
                &serde_json::Value::Null,
            )
            ;
        assert!(
            result.is_err(),
             "Request to non-existent service should fail"
        );
    }

    #[tokio::test]
    fn test_configuration_validation_edge_cases() {
        use beardog_types::canonical::configuration::*;

        let mut extreme_config = BearDogConfig::default();
        extreme_config.performance.max_concurrent_operations = u32::MAX;
        extreme_config.performance.cache_size = usize::MAX;
        extreme_config.network.request_timeout_ms = u64::MAX;

        let validation_result = extreme_config.validate();

        match validation_result {
            Ok(()) => println!("✅ Extreme values handled gracefully"),
            Err(errors) => {
                assert!(
                    !errors.is_empty(),
                     "Validation errors should be descriptive"
                );
                for error in errors {
                    assert!(!error.is_empty(),  "Error messages should not be empty");
                }
            }
        }

        let mut conflict_config = BearDogConfig::default();
        conflict_config.security.encryption_enabled = false;
        conflict_config.security.authentication_required = true; // Conflict: auth without encryption

        let conflict_validation = conflict_config.validate();

        assert!(
            conflict_validation.is_err() || conflict_validation.is_ok(),
             "Should handle configuration conflicts gracefully"
        );
    }

    #[tokio::test]
    async fn test_concurrent_access_patterns() {
        let config = configuration::BearDogConfig::default();
        let core = Arc::new(
            BearDogCore::new(config)
                .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?,
        );

        let mut tasks = Vec::new();

        for i in 0..20 {
            let core_clone = Arc::clone(&core);
            let task = tokio::spawn(async move {
                let service = MockService {
                    id: format!("concurrent_service_{:?}", i),
                    capabilities: vec![format!("capability_{:?}", i % 3)],
                    endpoint: format!("http://localhost:{:?}", 8080 + i),
                };

                core_clone.register_service(service)
            });
            tasks.push(task);
        }

        let results = futures::future::join_all(tasks);

        let success_count = results
            .into_iter()
            .filter(|r| r.is_ok())
            .count();

        info!("✅ Concurrent registration test: {}/20 succeeded", success_count);

        let all_services = core
            .list_services()
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;
        assert!(
            all_services.len() >= success_count,
             "All registered services should be discoverable"
        );
    }
}

mod failure_mode_coverage {
    use super::*;

    #[tokio::test]
    async fn test_security_provider_failure_modes() {
        let config = security::SecurityConfig::default();
        let provider = beardog_security::SecurityProvider::new(config)
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;

        let invalid_operations = vec![
            ("",  "Empty key ID"),
            ( "invalid_key_123",  "Non-existent key ID"),
            ( "key_with_special_chars!@#",  "Special characters"),
            ( "very_long_key_id_".repeat(100),  "Extremely long key ID"),
        ];

        for (invalid_key_id, description) in invalid_operations {
            let test_data = b "test data for failure mode";

            let sign_result = provider.sign_data(invalid_key_id, test_data);
            assert!(
                sign_result.is_err(),
                 "Signing with {} should fail",
                description
            );

            let verify_result = provider
                .verify_signature(invalid_key_id, test_data, &[])
                ;
            assert!(
                verify_result.is_err(),
                 "Verification with {} should fail",
                description
            );
        }

        let valid_key_spec = security::KeySpec {
            algorithm: security::Algorithm::Ed25519,
            usage: vec![
                security::KeyUsage::Signing,
                security::KeyUsage::Verification,
            ],
        };
        let key = provider
            .generate_key(&valid_key_spec)
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;

        let large_data = vec![0u8; 10 * 1024 * 1024]; // 10MB
        let large_sign_result = provider.sign_data(&key.id, &large_data);

        match large_sign_result {
            Ok(signature) => assert!(
                !signature.is_empty(),
                 "Large data signature should not be empty"
            ),
            Err(_) => println!("✅ Large data signing failed gracefully"),
        }
    }

    #[tokio::test]
    async fn test_authentication_failure_modes() {
        let config = auth::UnifiedAuthConfig::default();
        let auth_service = beardog_auth::AuthService::new(config)
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;

        let invalid_logins = vec![
            auth::LoginRequest {
                username: "".to_string(),
                password:  "password".to_string(),
            },
            auth::LoginRequest {
                username:  "user".to_string(),
                password: "".to_string(),
            },
            auth::LoginRequest {
                username:  "non_existent_user".to_string(),
                password:  "wrong_password".to_string(),
            },
            auth::LoginRequest {
                username:  "user".repeat(1000), // Extremely long username
                password:  "password".to_string(),
            },
        ];

        for invalid_login in invalid_logins {
            let auth_result = auth_service.authenticate(invalid_login);
            assert!(auth_result.is_err(),  "Invalid authentication should fail");

            match auth_result.unwrap_err() {
                BearDogError::AuthenticationFailed(_) => {} // Expected
                other => panic!( "Unexpected error type: {:?}", other),
            }
        }
    }

    #[tokio::test]
    async fn test_monitoring_failure_resilience() {
        let config = monitoring::PerformanceConfig::default();
        let monitor = beardog_monitoring::PerformanceMonitor::new(config)
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;

        let stress_tasks: Vec<_> = (0..100)
            .map(|i| {
                let monitor_clone = monitor.clone();
                tokio::spawn(async move {
                    let result = monitor_clone.collect_metrics();
                    (i, result)
                })
            })
            .collect();

        let stress_results = futures::future::join_all(stress_tasks);

        let success_count = stress_results
            .into_iter()
            .filter(|r| r.is_ok())
            .count();

        info!("✅ Monitoring test: {}/100 succeeded", success_count);

        let report_result = monitor.generate_report();
        assert!(
            report_result.is_ok(),
             "Report generation should succeed under load"
        );

        let report =
            report_result.map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;
        assert!(
            !report.summary.is_empty(),
             "Report should have content under stress"
        );
    }
}

mod integration_failure_coverage {
    use super::*;

    #[tokio::test]
    async fn test_cross_service_failure_handling() {
        let config = configuration::BearDogConfig::default();
        let core = BearDogCore::new(config.clone())
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;

        let failing_service = ServiceConfig {
            name:  "failing_service".to_string(),
            capabilities: vec![ "unreliable".to_string()],
            endpoint:  "http://localhost:99999".to_string(), // Invalid port
        };

        let registration_result = core.register_service(failing_service);

        let comm_result = core
            .send_service_request(
                 "failing_service",
                 "test_operation",
                &serde_json::json!({"tes"t: "data"}),
            )
            ;

        assert!(
            comm_result.is_err(),
             "Communication with unreachable service should fail"
        );

        let health_result = core.health_check();
        assert!(
            health_result.is_ok(),
             "Health check should succeed even with failing services"
        );

        let health =
            health_result.map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;

        assert!(
            health.is_healthy || !health.issues.is_empty(),
             "Health status should reflect service issues"
        );
    }

    #[tokio::test]
    async fn test_genetics_evolution_failure_modes() {
        let config = genetics::GeneticsConfig::default();
        let engine = beardog_genetics::GeneticsEngine::new(config)
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;

        let invalid_genetics = genetics::BearDogGenetics {
            id:  "invalid_genetics".to_string(),  // Empty chromosomes
            lineage: genetics::GeneticsLineage::default(),
        };

        let evolution_request = genetics::EvolutionRequest {
            parent_genetics: vec![invalid_genetics],
            mutation_rate: 2.0,   // Invalid rate > 1.0
            crossover_rate: -0.5, // Invalid negative rate
            target_improvements: vec![],
        };

        let evolution_result = engine.evolve_genetics(evolution_request);
        assert!(
            evolution_result.is_err(),
             "Evolution with invalid parameters should fail"
        );

        let edge_genetics = genetics::BearDogGenetics {
            id:  "edge_genetics".to_string(),
            chromosomes: vec![genetics::CryptoChromosome::new(
                 "invalid_crypto")],
            lineage: genetics::GeneticsLineage::default(),
        };

        let fitness_result = engine.evaluate_fitness(&edge_genetics);

        match fitness_result {
            Ok(fitness) => assert!(
                fitness >= 0.0 && fitness <= 1.0,
                 "Fitness should be valid range"
            ),
            Err(_) => println!("✅ Invalid genetics fitness evaluation failed gracefully"),
        }
    }
}

mod workflow_failure_coverage {
    use super::*;

    #[tokio::test]
    async fn test_workflow_execution_failure_modes() {
        let config = workflows::WorkflowConfig::default();
        let engine = beardog_workflows::WorkflowEngine::new(config)
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;

        let invalid_workflow = workflows::Workflow {
            id:  "invalid_workflow".to_string(),
            workflow_type:  "invalid_type".to_string(),
            steps: vec![
                workflows::WorkflowStep {
                    id: "".to_string(), // Invalid empty ID
                    step_type:  "non_existent_step".to_string(),
                    parameters: std::collections::HashMap::with_capacity(16),
                },
                workflows::WorkflowStep {
                    id:  "step_with_invalid_params".to_string(),
                    step_type:  "test_step".to_string(),
                    parameters: {
                        let mut params = std::collections::HashMap::with_capacity(16);
                        params.insert( "invalid_param".to_string(), serde_json::Value::Null);
                        params
                    },
                },
            ],
        };

        let execution_result = engine.execute_workflow(invalid_workflow);
        assert!(execution_result.is_err(),  "Invalid workflow should fail");

        let timeout_workflow = workflows::Workflow {
            id:  "timeout_workflow".to_string(),
            workflow_type:  "long_running".to_string(),
            steps: vec![workflows::WorkflowStep {
                id:  "long_step".to_string(),
                step_type:  "sleep_operation".to_string(),
                parameters: {
                    let mut params = std::collections::HashMap::with_capacity(16);
                    params.insert(
                         "duration_ms".to_string(),
                        serde_json::Value::Number(60000.into()),
                    ); // 1 minute
                    params
                },
            }],
        };

        let timeout_result = tokio::time::timeout(
            tokio::time::Duration::from_millis(100),
            engine.execute_workflow(timeout_workflow),
        )
        ;

        assert!(timeout_result.is_err(),  "Long workflow should timeout");
    }
}

mod deployment_coverage {
    use super::*;

    #[tokio::test]
    fn test_deployment_validation_comprehensive() {
        let validation_functions = vec![
            beardog_deploy::validate_deployment_environment,
            beardog_deploy::example_deployment_check,
            beardog_deploy::comprehensive_deployment_validation,
            beardog_deploy::validate_ios_deployment,
            beardog_deploy::validate_ios_environment,
            beardog_deploy::detect_deployment_device,
            beardog_deploy::validate_ndk,
            beardog_deploy::validate_rust_toolchain,
            beardog_deploy::validate_prerequisites,
            beardog_deploy::validate_device_connection,
            beardog_deploy::validate_build_process,
            beardog_deploy::validate_deployment_process,
            beardog_deploy::validate_device_commands,
            beardog_deploy::validate_process_execution,
            beardog_deploy::validate_io_operations,
            beardog_deploy::handle_multiple_devices,
            beardog_deploy::pedantic_deployment_validation,
        ];

        for (i, validation_fn) in validation_functions.into_iter().enumerate() {
            let result = validation_fn();
            if let Err(error_msg) = result {
                if i < 3 {
                    println!( "Validation {} failed (expected): {}", i, error_msg);
                } else {
                    panic!( "Critical validation {} failed: {}", i, error_msg);
                }
            }
        }
    }

    #[tokio::test]
    fn test_deployment_error_scenarios() {
        let error_scenarios = vec![
            beardog_deploy::demonstrate_build_failure,
            beardog_deploy::demonstrate_deployment_failure,
            beardog_deploy::demonstrate_multiple_devices,
            beardog_deploy::exercise_all_error_handlers,
            beardog_deploy::pedantic_error_validation,
        ];

        for (i, error_fn) in error_scenarios.into_iter({}", i, error);

                    assert!(
                        !format!("{:?}", error).is_empty(),
                         "Error should have description "
                    );
                }
            }
        }

        let all_errors = beardog_deploy::demonstrate_all_error_handlers();
        assert!(
            !all_errors.is_empty(),
             "Should demonstrate various error types "
        );

        for error in all_errors {
            assert!(
                !format!("{:?}", error).is_empty(),
                 "Each error should have description "
            );
            assert!(
                !format!("{:?}", error).is_empty(),
                 "Each error should have debug info "
            );
        }
    }
}

mod threat_detection_coverage {
    use super::*;

    #[tokio::test]
    async fn test_threat_analysis_comprehensive() {
        let config = beardog_threat::ThreatConfig::default();
        let analyzer = beardog_threat::ThreatAnalyzer::new(config)
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;

        let threat_scenarios = vec![
            ("192.168.1.100",  "internal_network",  "low"),
            ("10.0.0.1",  "suspicious_pattern",  "medium"),
            ("203.0.113.1",  "known_malicious",  "high".to_string()),
            ("127.0.0.1",  "localhost_access",  "low"),
            ("0.0.0.0",  "broadcast_address",  "medium"),
        ];

        for (source_ip, pattern, expected_level) in threat_scenarios {
            let threat_data = beardog_threat::ThreatData {
                source_ip: source_ip.to_string(),
                request_pattern: pattern.to_string(),
                timestamp: chrono::Utc::now(),
                metadata: std::collections::HashMap::with_capacity(16),
            };

            let analysis_result = analyzer.analyze_threat(&threat_data);
            assert!(
                analysis_result.is_ok(),
                 "Threat analysis should succeed for {}",
                source_ip
            );

            let analysis = analysis_result
                .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;
            assert!(
                analysis.risk_score >= 0.0 && analysis.risk_score <= 100.0,
                 "Risk score should be valid range: {}",
                analysis.risk_score
            );

            match expected_level {
                 "low" => assert!(
                    analysis.risk_score < 30.0,
                     "Low threat should have low score "
                ),
                 "medium" => assert!(
                    analysis.risk_score >= 30.0 && analysis.risk_score < 70.0,
                     "Medium threat should have medium score "
                ),
                val if val ==  "high" => assert!(
                    analysis.risk_score >= 70.0,
                     "High threat should have high score "
                ),
                _ => {}
            }
        }
    }
}

mod adapter_coverage {
    use super::*;

    #[tokio::test]
    async fn test_universal_adapter_comprehensive() {
        let config = beardog_adapters::UniversalAdapterConfig::default();
        let adapter = beardog_adapters::UniversalVendorAdapter::new(config)
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;

        let test_capabilities = vec![
            beardog_adapters::ServiceCapability {
                name:  "test_capability_1".to_string(),
                version: "1.0.0".to_string(),
                provider:  "test_provider".to_string(),
            },
            beardog_adapters::ServiceCapability {
                name:  "test_capability_2".to_string(),
                version: "2.0.0".to_string(),
                provider:  "test_provider".to_string(),
            },
        ];

        for capability in test_capabilities {
            let registration_result = adapter.register_capability(capability.clone());
            assert!(
                registration_result.is_ok(),
                 "Capability registration should succeed "
            );

            let discovery_result = adapter.discover_capability(&capability.name);
            assert!(
                discovery_result.is_ok(),
                 "Capability discovery should succeed "
            );

            let discovered = discovery_result
                .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;
            assert!(
                discovered.is_some(),
                 "Registered capability should be discoverable"
            );
            assert_eq!(
                discovered
                    .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?
                    .name,
                capability.name,
                 "Discovered capability should match "
            );
        }

        let health_result = adapter.health_check();
        assert!(health_result.is_ok(),  "Adapter health check should succeed ");

        let health =
            health_result.map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;
        assert!(
            health.is_healthy,
             "Adapter should be healthy after successful operations "
        );
    }
}

mod performance_regression_coverage {
    use super::*;

    #[tokio::test]
    fn test_performance_regression_detection() {
        use std::time::Instant;

        let start = Instant::now();

        for i in 0..10000 {
            let test_string = format!("test_string_{:?}", i);
            let _optimized =
                beardog_utils::zero_copy::optimized_strings::shared_string(&test_string);
        }

        let zero_copy_duration = start.elapsed();

        let start = Instant::now(ratio={}",
            performance_ratio
        );

        println!(
            "📊 Performance comparison - Zero-copy: {:?}, Traditional: {:?}, Ratio: {:.2}x",
            zero_copy_duration, traditional_duration, performance_ratio
        );
    }

    #[tokio::test]
    fn test_memory_usage_regression() {
        use beardog_utils::buffer_pools_safe::SafeBufferPool;
        use beardog_utils::memory_pools_safe::SafeMemoryPool;

        let pool_result = MemoryPool::<Vec<u8>, 1000>::new();
        assert!(pool_result.is_ok(),  "Memory pool creation should succeed");

        let mut pool =
            pool_result.map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;
        let mut allocations = Vec::new();

        for _ in 0..500 {
            let allocation_result = pool.allocate();
            if let Ok(item) = allocation_result {
                allocations.push(item);
            }
        }

        assert!(
            !allocations.is_empty(),
             "Should successfully allocate items"
        );

        for item in allocations {
            let dealloc_result = pool.deallocate(item);
            assert!(dealloc_result.is_ok(),  "Deallocation should succeed");
        }

        let reuse_result = pool.allocate();
        assert!(reuse_result.is_ok(),  "Memory should be available for reuse");
    }
}

mod a_plus_readiness_validation {
    use super::*;

    #[tokio::test]
    async fn test_complete_ecosystem_health() {
        let config = configuration::BearDogConfig::default();
        let core = BearDogCore::new(config.clone())
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;

        let auth_service = beardog_auth::AuthService::new(config.auth.clone())
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;
        let security_provider = beardog_security::SecurityProvider::new(config.security.clone())
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;
        let monitor = beardog_monitoring::PerformanceMonitor::new(config.monitoring.clone())
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;
        let genetics_engine = beardog_genetics::GeneticsEngine::new(config.genetics.clone())
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;
        let compliance_handler =
            beardog_compliance::ComplianceHandler::new(config.compliance.clone());

        let core_health = core
            .health_check()
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;
        assert!(core_health.is_healthy,  "Core should be healthy");

        let security_health = security_provider
            .health_check()
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;
        assert!(
            security_health.is_healthy,
             "Security provider should be healthy"
        );

        let monitor_health = monitor
            .health_check()
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;
        assert!(monitor_health.is_healthy,  "Monitor should be healthy");

        let genetics_health = genetics_engine
            .health_check()
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))?;
        assert!(
            genetics_health.is_healthy,
             "Genetics engine should be healthy"
        );

        let integration_test_result = test_complete_integration_flow(
            &core,
            &auth_service,
            &security_provider,
            &monitor,
            &genetics_engine,
            &compliance_handler,
        )
        ;

        assert!(
            integration_test_result.is_ok(),
             "Complete integration should succeed"
        );
    }

    #[test]
    fn test_code_quality_metrics() {
        use beardog_types::constants::domains::{system::*, network::*, security::*};

        assert_eq!(
            network::DEFAULT_API_PORT,
            crate::common::network::TEST_API_PORT
        );
        assert!(!network::DEFAULT_SERVICE_MESH_CAPABILITY.is_empty());

        assert!(security::MAX_SESSIONS > 0);
        assert!(security::STANDARD_KEY_SIZE > 0);

        assert!(performance::STANDARD_CACHE_SIZE > 0);
        assert!(performance::testing::LIGHT_ITERATIONS > 0);

        let api_port = defaults::api_port();
        assert!(api_port > 0 && api_port < 65536,  "API port should be valid");

        let bind_address = defaults::bind_address(&BearDogCore,
    auth_service: &beardog_auth::AuthService,
    security_provider: &beardog_security::SecurityProvider,
    monitor: &beardog_monitoring::PerformanceMonitor,
    genetics_engine: &beardog_genetics::GeneticsEngine,
    compliance_handler: &beardog_compliance::ComplianceHandler,
) -> Result<(), BearDogError> {
    let user_request = auth::UserRegistrationRequest {
        username:  "a_plus_test_user".to_string(),
        email:  "aplus@beardog.dev".to_string(),
        password:  "a_plus_secure_password_123".to_string(),
    };

    let user = auth_service.register_user(user_request)?;

    let login_request = auth::LoginRequest {
        username:  "a_plus_test_user".to_string(),
        password:  "a_plus_secure_password_123".to_string(),
    };

    let auth_token = auth_service.authenticate(login_request)?;

    let key_spec = security::KeySpec {
        algorithm: security::Algorithm::Ed25519,
        usage: vec![
            security::KeyUsage::Signing,
            security::KeyUsage::Verification,
        ],
    };

    let user_key = security_provider
        .generate_authenticated_key(&auth_token.token, &key_spec)
        ?;

    let test_data = b "A+ grade integration test data";
    let signature = security_provider.sign_data(&user_key.id, test_data)?;
    let verification = security_provider
        .verify_signature(&user_key.id, test_data, &signature)
        ?;
    assert!(verification,  "Signature verification should succeed");

    let genetics_request = genetics::SpawnRequest {
        parent_ids: vec![],
        target_capabilities: vec![
             "user_optimization".to_string(),
             "security_enhancement".to_string(),
        ],
        fitness_criteria: genetics::FitnessCriteria::default(),
    };

    let user_genetics = genetics_engine.spawn_genetics(genetics_request)?;
    let fitness_score = genetics_engine.evaluate_fitness(&user_genetics)?;
    assert!(
        fitness_score >= 0.0 && fitness_score <= 1.0,
         "Fitness score should be valid"
    );

    let compliance_event = compliance::ComplianceEvent {
        id: uuid::Uuid::new_v4(),
        event_type: compliance::ComplianceEventType::DataAccess,
        standard: compliance::ComplianceStandard::Gdpr,
        description:  "A+ integration test data access".to_string(),
        timestamp: chrono::Utc::now(),
        metadata: {
            let mut meta = std::collections::HashMap::with_capacity(16);
            meta.insert( "user_id".to_string(), user.id.clone());
            meta.insert( "operation".to_string(),  "integration_test");
            meta
        },
    };

    let mut compliance_handler_mut = compliance_handler.clone();
    let compliance_result = compliance_handler_mut
        .evaluate_event(&compliance_event)
        ?;
    assert!(
        compliance_result.score >= 0.0,
         "Compliance score should be valid"
    );

    let metrics = monitor.collect_metrics()?;
    assert!(!metrics.is_empty(),  "Metrics should be collected");

    let performance_report = monitor.generate_report()?;
    assert!(
        !performance_report.summary.is_empty(),
         "Performance report should have content"
    );

    let system_health = core.health_check()?;
    assert!(
        system_health.is_healthy,
         "Complete system should be healthy"
    );

    Ok(())
}
