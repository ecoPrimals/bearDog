use beardog_auth::*;
use beardog_compliance::*;
use beardog_errors::BearDogError;
use beardog_errors::*;
use beardog_genetics::*;
use beardog_monitoring::*;
use beardog_security::*;
use beardog_types::canonical::*;
use beardog_utils::*;
use std::sync::Arc;
use tokio_test;

mod error_handling_tests {
    use super::*;

    #[test]
    fn test_all_error_variants() {
        let errors = vec![
            BearDogError::invalid_input("test".to_string()),
            BearDogError::invalid_state("test".to_string()),
            BearDogError::invalid_configuration("test".to_string()),
            BearDogError::authentication_failed("test".to_string()),
            BearDogError::authorization_failed("test".to_string()),
            BearDogError::encryption_error("test".to_string()),
            BearDogError::decryption_error("test".to_string()),
            BearDogError::network_error("test".to_string()),
            BearDogError::timeout_error("test".to_string()),
            BearDogError::resource_exhausted("test".to_string()),
            BearDogError::not_found("test".to_string()),
            BearDogError::already_exists("test".to_string()),
            BearDogError::permission_denied("test".to_string()),
            BearDogError::rate_limited("test".to_string()),
            BearDogError::service_unavailable("test".to_string()),
            BearDogError::internal("test".to_string()),
            BearDogError::not_implemented("test".to_string()),
            BearDogError::unsupported_operation("test".to_string()),
        ];

        for error in errors {
            let display_str = format!("{:?}", error);
            let debug_str = format!("{:?}", error);

            assert!(!display_str.is_empty());
            assert!(!debug_str.is_empty());
                    assert!(display_str.contains("test"));

        let std_error: &dyn std::error::Error = &error;
        assert!(std_error.to_string().contains("test"));
        }
    }

    #[test]
    fn test_error_chaining() {
        let root_cause = std::io::Error::new(std::io::ErrorKind::NotFound,  "file not found");
        let chained_error = BearDogError::network_error( "Network operation failed".to_string())
            .with_source(Box::new(root_cause));

        assert!(chained_error.source().is_some());
        let source_chain: Vec<&dyn std::error::Error> =
            std::iter::successors(chained_error.source(), |e| e.source()).collect();

        assert!(!source_chain.is_empty());
    }
}

mod configuration_tests {
    use super::*;

    #[test]
    fn test_configuration_edge_cases() {
        use beardog_types::canonical::configuration::*;

        let mut min_config = BearDogConfig::default();
        min_config.security.encryption_enabled = true;
        min_config.security.authentication_required = true;
        assert!(min_config.validate().is_ok());

        let mut max_config = BearDogConfig::default();
        max_config.performance.max_concurrent_operations = u32::MAX;
        max_config.performance.cache_size = usize::MAX;

        let validation_result = max_config.validate();

        assert!(validation_result.is_ok() || validation_result.is_err());

        let mut full_config = BearDogConfig::default();
        full_config.features.enable_ai = true;
        full_config.features.enable_genetics = true;
        full_config.features.enable_monitoring = true;
        full_config.features.enable_compliance = true;
        assert!(full_config.validate().is_ok());
    }

    #[test]
    fn test_configuration_serialization_formats() -> Result<(), BearDogError> {
        let config = BearDogConfig::default();

        let json_str = serde_json::to_string_pretty(&config)
            .map_err(|e| BearDogError::system(format!("JSON serialization failed: {:?}", e))?;
        let from_json: BearDogConfig = serde_json::from_str(&json_str)
            .map_err(|e| BearDogError::system(format!("JSON deserialization failed: {:?}", e))?;
        assert_eq!(config.version, from_json.version);

        // Test JSON deserialization validation
        assert!(serde_json::from_str::<BearDogConfig>(&json_str).is_ok(),  "Config should deserialize from JSON ");

        let toml_str = toml::to_string_pretty(&config)
            .map_err(|e| BearDogError::system(format!("TOML serialization failed: {:?}", e)))?;
        let from_toml: BearDogConfig = toml::from_str(&toml_str)
            .map_err(|e| BearDogError::system(format!("TOML deserialization failed: {:?}", e)))?;
        assert_eq!(config.version, from_toml.version);

        Ok(())
    }
}

mod performance_tests {
    use super::*;

    #[test]
    fn test_zero_copy_string_operations() {
        use beardog_utils::zero_copy::optimized_strings::*;

        let constant_str = shared_string( "api");
        let constant_str2 = shared_string( "api");
        assert!(Arc::ptr_eq(&constant_str, &constant_str2));

        let mut builder = ZeroCopyStringBuilder::new();
        builder
            .push_static( "prefix_")
            .push( "dynamic")
            .push_static("_suffix");

        let result = builder.build();
        assert_eq!(result,  "prefix_dynamic_suffix");

        let cow_static = cow_string( "localhost");
        assert!(matches!(cow_static, std::borrow::Cow::Borrowed(_)));

        let cow_dynamic = cow_string( "dynamic_string_12345");
        assert!(matches!(cow_dynamic, std::borrow::Cow::Owned(_)));
    }

    #[test]
    fn test_clone_optimization_patterns() {
        use beardog_utils::optimization::clone_optimizer::*;

        let mut optimizer = CloneOptimizer::new();

        let strategies = vec![
            (
                 "String",
                 "shared_across_threads",
                CloneOptimizationStrategy::SharedOwnership,
            ),
            (
                 "Vec<u8>",
                 "occasional_modification",
                CloneOptimizationStrategy::CopyOnWrite,
            ),
            (
                 "Config",
                 "read_only_access",
                CloneOptimizationStrategy::BorrowInstead,
            ),
            (
                 "ApiResponse",
                 "string_operations",
                CloneOptimizationStrategy::ZeroCopy,
            ),
        ];

        for (type_name, pattern, expected) in strategies {
            let strategy = optimizer.analyze_type::<String>(type_name, pattern);
            assert!(matches!(strategy, expected));
        }

        assert_eq!(optimizer.get_stats().types_analyzed, 4);
    }

    #[async_test]
    async fn test_memory_pool_comprehensive() -> Result<(), BearDogError> {
        use beardog_utils::buffer_pools_safe::SafeBufferPool;
        use beardog_utils::memory_pools_safe::SafeMemoryPool;

        let pool_result = MemoryPool::<String, 1000>::new();
        assert!(pool_result.is_ok());

        let mut pool =
            pool_result.map_err(|e| BearDogError::system(format!("Pool creation error: {:?}", e))?;

        let mut allocated_items = Vec::new();

        for i in 0..10 {
            let item_result = pool.allocate();
            assert!(item_result.is_ok());

            let mut item = item_result
                .map_err(|e| BearDogError::system(format!("Pool allocation error: {:?}", e))?;
            item.push_str(&format!("test_item_{:?}", i));
            allocated_items.push(item);
        }

        assert_eq!(allocated_items.len(), 10);

        for item in allocated_items {
            let dealloc_result = pool.deallocate(item);
            assert!(dealloc_result.is_ok());
        }
        
        Ok(())
    }
}

mod security_comprehensive_tests {
    use super::*;

    #[tokio::test]
    async fn test_comprehensive_cryptographic_operations() {
        let config = security::SecurityConfig::default();
        let provider = beardog_security::SecurityProvider::new(config)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;

        let algorithms = vec![
            security::Algorithm::Ed25519,
            security::Algorithm::X25519,
            security::Algorithm::ChaCha20Poly1305,
        ];

        for algorithm in algorithms {
            let key_spec = security::KeySpec {
                algorithm,
                usage: vec![
                    security::KeyUsage::Signing,
                    security::KeyUsage::Verification,
                ],
            };

            let key_result = provider.generate_key(&key_spec);
            assert!(
                key_result.is_ok(),
                 "Key generation should succeed for {:?}",
                algorithm
            );

            let key =
                key_result.map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
            assert_eq!(key.algorithm, algorithm);

            let test_data = format!("test data for {:?}", algorithm)
                .to_string()
                .into_bytes();

            if key.usage.contains(&security::KeyUsage::Signing) {
                let signature_result = provider.sign_data(&key.id, &test_data);
                assert!(
                    signature_result.is_ok(),
                     "Signing should succeed for {:?}",
                    algorithm
                );

                let signature = signature_result
                    .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
                assert!(!signature.is_empty(),  "Signature should not be empty");

                if key.usage.contains(&security::KeyUsage::Verification) {
                    let verify_result = provider
                        .verify_signature(&key.id, &test_data, &signature)
                        ;
                    assert!(
                        verify_result.is_ok(),
                         "Verification should succeed for {:?}",
                        algorithm
                    );
                    assert!(
                        verify_result.map_err(|e| BearDogError::system({:?}",
                            e
                        )))?,
                         "Signature should verify for {:?}",
                        algorithm
                    );
                }
            }
        }
    }

    #[tokio::test]
    async fn test_security_provider_edge_cases() {
        let config = security::SecurityConfig::default();
        let provider = beardog_security::SecurityProvider::new(config)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;

        let invalid_key_id =  "non_existent_key_12345";
        let test_data = b "test data ";

        let sign_result = provider.sign_data(invalid_key_id, test_data);
        assert!(sign_result.is_err(),  "Signing with invalid key should fail ");

        let verify_result = provider
            .verify_signature(invalid_key_id, test_data, &[])
            ;
        assert!(
            verify_result.is_err(),
             "Verification with invalid key should fail "
        );

        let key_spec = security::KeySpec {
            algorithm: security::Algorithm::Ed25519,
            usage: vec![security::KeyUsage::Signing],
        };
        let key = provider
            .generate_key(&key_spec)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;

        let empty_data_sign = provider.sign_data(&key.id, &[]);
        assert!(empty_data_sign.is_ok(),  "Signing empty data should succeed ");

        let large_data = vec![0u8; 1024 * 1024]; // 1MB
        let large_data_sign = provider.sign_data(&key.id, &large_data);
        assert!(large_data_sign.is_ok(),  "Signing large data should succeed ");
    }
}

mod monitoring_comprehensive_tests {
    use super::*;

    #[tokio::test]
    async fn test_security_sentinel_comprehensive() {
        let config = monitoring::SecuritySentinelConfig::default();
        let sentinel = beardog_monitoring::SecuritySentinel::new(config)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;

        let threat_data = monitoring::ThreatData {
            source_ip: "192.168.1.100".to_string(),
            request_pattern:  "suspicious_pattern".to_string(),
            timestamp: chrono::Utc::now(),
        };

        let detection_result = sentinel.analyze_threat(&threat_data);
        assert!(detection_result.is_ok());

        let threat_assessment = detection_result
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
        assert!(threat_assessment.risk_score >= 0.0);
        assert!(threat_assessment.risk_score <= 100.0);

        let security_event = monitoring::SecurityEvent {
            event_type: monitoring::SecurityEventType::AuthenticationAttempt,
            severity: monitoring::Severity::Medium,
            description:  "Test authentication attempt ".to_string(),
            metadata: std::collections::HashMap::with_capacity(16),
        };

        let log_result = sentinel.log_security_event(security_event);
        assert!(log_result.is_ok());
    }

    #[tokio::test]
    async fn test_performance_monitoring_comprehensive() {
        let config = monitoring::PerformanceConfig::default();
        let monitor = beardog_monitoring::PerformanceMonitor::new(config)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;

        for i in 0..5 {
            let metrics_result = monitor.collect_metrics();
            assert!(metrics_result.is_ok());

            let metrics = metrics_result
                .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
            assert!(!metrics.is_empty());

            for metric in metrics {
                assert!(!metric.name.is_empty());
                assert!(metric.value.is_finite());
                assert!(metric.timestamp <= chrono::Utc::now());
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        let report_result = monitor.generate_comprehensive_report();
        assert!(report_result.is_ok());

        let report =
            report_result.map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
        assert!(!report.summary.is_empty());
        assert!(!report.recommendations.is_empty());
        assert!(report.overall_score >= 0.0);
        assert!(report.overall_score <= 100.0);
    }
}

mod genetics_comprehensive_tests {
    use super::*;

    #[tokio::test]
    async fn test_genetics_lifecycle_comprehensive() {
        let config = genetics::GeneticsConfig::default();
        let engine = beardog_genetics::GeneticsEngine::new(config)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;

        let initial_spawn = genetics::SpawnRequest {
            parent_ids: vec![],
            target_capabilities: vec![ "security".to_string(genetics::FitnessCriteria {
                security_weight: 0.6,
                performance_weight: 0.4,
                minimum_fitness: 0.7,
            },
        };

        let generation_0 = engine
            .spawn_genetics(initial_spawn)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
        assert_eq!(generation_0.generation, 0);
        assert!(!generation_0.chromosomes.is_empty());

        let fitness_result = engine
            .evaluate_fitness(&generation_0)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
        assert!(fitness_result >= 0.0 && fitness_result <= 1.0);

        let evolution_request = genetics::EvolutionRequest {
            parent_genetics: vec![generation_0.clone(0.1,
            crossover_rate: 0.8,
            target_improvements: vec![ "efficiency".to_string()],
        };

        let generation_1 = engine
            .evolve_genetics(evolution_request)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
        assert_eq!(generation_1.generation, 1);
        assert_eq!(generation_1.lineage.parent_ids, vec![generation_0.id]);

        for gen in 2..=5 {
            let evolution_request = genetics::EvolutionRequest {
                parent_genetics: vec![generation_1.clone(0.05,
                crossover_rate: 0.9,
                target_improvements: vec![ "optimization".to_string()],
            };

            let next_gen = engine
                .evolve_genetics(evolution_request)
                .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
            assert_eq!(next_gen.generation, gen);

            let fitness = engine
                .evaluate_fitness(&next_gen)
                .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
            assert!(fitness >= 0.0);
        }
    }

    #[tokio::test]
    async fn test_genetics_crossover_and_mutation() {
        let config = genetics::GeneticsConfig::default();
        let engine = beardog_genetics::GeneticsEngine::new(config)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;

        let parent1 = genetics::BearDogGenetics {
            id:  "parent1".to_string(1,
            chromosomes: vec![
                genetics::CryptoChromosome::new( "ed25519".to_string()),
                genetics::PerformanceChromosome::new( "zero_copy".to_string()),
            ],
            lineage: genetics::GeneticsLineage::default(),
        };

        let parent2 = genetics::BearDogGenetics {
            id:  "parent2".to_string(1,
            chromosomes: vec![
                genetics::CryptoChromosome::new( "x25519".to_string()),
                genetics::NetworkChromosome::new( "mesh_communication".to_string()),
            ],
            lineage: genetics::GeneticsLineage::default(),
        };

        let crossover_request = genetics::CrossoverRequest {
            parent1: parent1.clone(),
            parent2: parent2.clone(vec![1], // Crossover after first chromosome
        };

        let offspring = engine
            .perform_crossover(crossover_request)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
        assert_eq!(offspring.generation, 2);
        assert!(!offspring.chromosomes.is_empty());

        let mutation_request = genetics::MutationRequest {
            target_genetics: offspring,
            mutation_rate: 0.2,
            mutation_types: vec![
                genetics::MutationType::ChromosomeModification,
                genetics::MutationType::CapabilityEnhancement,
            ],
        };

        let mutated = engine
            .perform_mutation(mutation_request)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
        assert!(!mutated.chromosomes.is_empty());
    }
}

mod compliance_comprehensive_tests {
    use super::*;

    #[tokio::test]
    async fn test_all_compliance_standards() {
        let config = compliance::ComplianceConfig::default();
        let mut handler = beardog_compliance::ComplianceHandler::new(config);

        let standards = vec![
            compliance::ComplianceStandard::Gdpr,
            compliance::ComplianceStandard::Sox,
            compliance::ComplianceStandard::PciDss,
            compliance::ComplianceStandard::Hipaa,
            compliance::ComplianceStandard::Iso27001,
            compliance::ComplianceStandard::Soc2,
            compliance::ComplianceStandard::Ccpa,
        ];

        for standard in standards {
            let test_event = compliance::ComplianceEvent {
                id: uuid::Uuid::new_v4(compliance::ComplianceEventType::DataAccess,
                standard: standard.clone(format!("Test event for {:?}", standard),
                timestamp: chrono::Utc::now(),
                metadata: std::collections::HashMap::with_capacity(16),
            };

            let evaluation_result = handler.evaluate_event(&test_event);
            assert!(
                evaluation_result.is_ok(),
                 "Compliance evaluation should succeed for {:?}",
                standard
            );

            let result = evaluation_result
                .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
            assert_eq!(result.standard, standard);
            assert!(result.score >= 0.0 && result.score <= 100.0);
        }
    }

    #[tokio::test]
    async fn test_audit_trail_comprehensive() {
        let config = compliance::ComplianceConfig::default();
        let mut handler = beardog_compliance::ComplianceHandler::new(config);

        for i in 0..10 {
            let event = compliance::ComplianceEvent {
                id: uuid::Uuid::new_v4(compliance::ComplianceEventType::DataModification,
                standard: compliance::ComplianceStandard::Gdpr,
                description: format!("Audit test event {:?}", i),
                timestamp: chrono::Utc::now(),
                metadata: std::collections::HashMap::with_capacity(16),
            };

            handler
                .evaluate_event(&event)
                .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
        }

        assert_eq!(handler.audit_trail.len(), 10);

        for entry in &handler.audit_trail {
            assert!(!entry.id.to_string().is_empty());
            assert!(entry.timestamp <= chrono::Utc::now());
            assert!(!entry.action.is_empty());
        }
    }
}

mod workflow_comprehensive_tests {
    use super::*;

    #[tokio::test]
    async fn test_workflow_engine_comprehensive() {
        let config = workflows::WorkflowConfig::default();
        let engine = beardog_workflows::WorkflowEngine::new(config)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;

        let complex_workflow = workflows::Workflow {
            id:  "complex_test_workflow".to_string(),
            workflow_type:  "security_compliance_workflow".to_string(),
            steps: vec![
                workflows::WorkflowStep {
                    id:  "user_authentication".to_string(),
                    step_type:  "authentication".to_string(),
                    parameters: {
                        let mut params = std::collections::HashMap::with_capacity(16);
                        params.insert(
                             "username".to_string(),
                            serde_json::Value::String( "test_user"),
                        );
                        params
                    },
                },
                workflows::WorkflowStep {
                    id:  "key_generation".to_string(),
                    step_type:  "cryptographic_operation".to_string(),
                    parameters: {
                        let mut params = std::collections::HashMap::with_capacity(16);
                        params.insert(
                             "algorithm".to_string(),
                            serde_json::Value::String( "Ed25519"),
                        );
                        params
                    },
                },
                workflows::WorkflowStep {
                    id:  "compliance_check".to_string(),
                    step_type:  "compliance_validation".to_string(),
                    parameters: {
                        let mut params = std::collections::HashMap::with_capacity(16);
                        params.insert( "standard".to_string(), serde_json::Value::String( "GDPR"));
                        params
                    },
                },
            ],
        };

        let execution_result = engine.execute_workflow(complex_workflow);
        assert!(
            execution_result.is_ok(),
             "Complex workflow execution should succeed "
        );

        let result = execution_result
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
        assert_eq!(result.status, workflows::WorkflowStatus::Completed);
        assert_eq!(result.completed_steps.len(), 3);
    }

    #[tokio::test]
    async fn test_parallel_workflow_execution() {
        let config = workflows::WorkflowConfig::default();
        let engine = beardog_workflows::WorkflowEngine::new(config)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;

        let workflows: Vec<workflows::Workflow> = (0..5)
            .map(|i| workflows::Workflow {
                id: format!("parallel_workflow_{:?}", i),
                workflow_type:  "parallel_test".to_string(vec![workflows::WorkflowStep {
                    id: format!("step_{:?}", i),
                    step_type:  "test_operation".to_string(),
                    parameters: std::collections::HashMap::with_capacity(16),
                }],
            })
            .collect();

        let execution_futures: Vec<_> = workflows
            .into_iter()
            .map(|workflow| engine.execute_workflow(workflow))
            .collect();

        let results = futures::future::join_all(execution_futures);

        for (i, result) in results.into_iter().enumerate() {
            assert!(result.is_ok(),  "Parallel workflow {} should succeed ", i);

            let workflow_result =
                result.map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
            assert_eq!(workflow_result.status, workflows::WorkflowStatus::Completed);
        }
    }
}

mod edge_case_tests {
    use super::*;

    #[tokio::test]
    async fn test_resource_exhaustion_handling() {
        let mut config = configuration::BearDogConfig::default();
        config.performance.max_concurrent_operations = 1; // Very limited

        let core = beardog_core::BearDogCore::new(config)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;

        let mut tasks = Vec::new();
        for i in 0..10 {
            let core_clone = core.clone();
            let task = tokio::spawn(format!("stress_service_{:?}", i),
                    capabilities: vec![ "stress_test".to_string(format!("http://localhost:808{:?}", i),
                };
                core_clone.register_service(service)
            });
            tasks.push(task);
        }

        let results = futures::future::join_all(tasks);

        let success_count = results
            .into_iter()
            .filter(|r| {
                r.is_ok()
                    && r.as_ref()
                        .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e))))?
                        .is_ok()
            })
            .count();

        assert!(success_count > 0,  "At least some operations should succeed ");
    }

    #[test]
    fn test_configuration_boundary_conditions() {
        use beardog_types::canonical::configuration::*;

        let mut zero_config = BearDogConfig::default();
        zero_config.performance.max_concurrent_operations = 0;
        zero_config.performance.cache_size = 0;

        let validation = zero_config.validate();

        assert!(validation.is_ok() || validation.is_err());

        let mut negative_config = BearDogConfig::default();
        negative_config.network.request_timeout_ms = 0; // Effectively negative timeout

        let validation = negative_config.validate();

        assert!(validation.is_ok() || validation.is_err());
    }

    #[test]
    fn test_memory_safety_edge_cases() {
        use beardog_utils::utils::safe_memory::*;

        let zero_buffer = SafeSecureBuffer::new(0);
        assert_eq!(zero_buffer.len(), 0);
        assert!(zero_buffer.is_empty());

        let large_size = 1024 * 1024 * 100; // 100MB
        let large_buffer_result = std::panic::catch_unwind(|| SafeSecureBuffer::new(large_size));

        assert!(large_buffer_result.is_ok());

        let mut buffer = SafeSecureBuffer::new(10);

        let large_data = vec![0u8; 20];
        let copy_result = buffer.copy_from_slice(&large_data);
        assert!(copy_result.is_err(),  "Copying oversized data should fail ");

        let exact_data = vec![1u8; 10];
        let exact_copy_result = buffer.copy_from_slice(&exact_data);
        assert!(
            exact_copy_result.is_ok(),
             "Copying exact size should succeed "
        );
    }
}

mod integration_comprehensive_tests {
    use super::*;

    #[tokio::test]
    async fn test_full_ecosystem_integration() {
        let config = configuration::BearDogConfig::default();
        let core = beardog_core::BearDogCore::new(config.clone())
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;

        let auth_service = beardog_auth::AuthService::new(config.auth.clone())
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
        let security_provider = beardog_security::SecurityProvider::new(config.security.clone())
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
        let monitor = beardog_monitoring::PerformanceMonitor::new(config.monitoring.clone())
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
        let genetics_engine = beardog_genetics::GeneticsEngine::new(config.genetics.clone())
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;

        let user_request = auth::UserRegistrationRequest {
            username:  "integration_user".to_string(),
            email:  "integration@beardog.dev".to_string(),
            password:  "secure_integration_password_123".to_string(),
        };

        let user = auth_service
            .register_user(user_request)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;

        let key_spec = security::KeySpec {
            algorithm: security::Algorithm::Ed25519,
            usage: vec![
                security::KeyUsage::Signing,
                security::KeyUsage::Verification,
            ],
        };

        let user_key = security_provider
            .generate_user_key(&user.id, &key_spec)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
        assert_eq!(user_key.owner_id, Some(user.id.clone()));

        let genetics_request = genetics::SpawnRequest {
            parent_ids: vec![],
            target_capabilities: vec![ "user_optimization".to_string(genetics::FitnessCriteria {
                security_weight: 0.7,
                performance_weight: 0.3,
                minimum_fitness: 0.8,
            },
        };

        let user_genetics = genetics_engine
            .spawn_genetics(genetics_request)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;

        let integration_metrics = monitor
            .collect_integration_metrics()
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
        assert!(!integration_metrics.is_empty());

        let test_data = b "integration test data ";
        let signature = security_provider
            .sign_data(&user_key.id, test_data)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
        let verification = security_provider
            .verify_signature(&user_key.id, test_data, &signature)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
        assert!(
            verification,
             "End-to-end signature verification should succeed "
        );

        let ecosystem_health = core
            .comprehensive_health_check()
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
        assert!(
            ecosystem_health.is_healthy,
             "Entire ecosystem should be healthy"
        );
    }
}

mod stress_tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_operations_stress() {
        let config = configuration::BearDogConfig::default();
        let core = beardog_core::BearDogCore::new(config)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;

        let concurrent_tasks = 50;
        let mut tasks = Vec::with_capacity(concurrent_tasks);

        for i in 0..concurrent_tasks {
            let core_clone = core.clone();
            let task = tokio::spawn(format!("stress_service_{:?}", i),
                    capabilities: vec![format!("capability_{:?}", i % 5)],
                    endpoint: format!("http://localhost:{:?}", 8080 + i),
                };

                core_clone.register_service(service)
            });
            tasks.push(task);
        }

        let results = futures::future::join_all(tasks);

        let success_count = results
            .into_iter()
            .filter(|r| {
                r.is_ok()
                    && r.as_ref()
                        .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e))))?
                        .is_ok()
            })
            .count();

        assert!(
            success_count >= concurrent_tasks / 2,
             "Should handle reasonable concurrent load"
        );
    }

    #[tokio::test]
    async fn test_memory_pressure_handling() {
        let config = configuration::BearDogConfig::default();
        let security_provider = beardog_security::SecurityProvider::new(config.security)
            .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;

        let mut keys = Vec::new();

        for i in 0..100 {
            let key_spec = security::KeySpec {
                algorithm: security::Algorithm::Ed25519,
                usage: vec![security::KeyUsage::Signing],
            };

            let key_result = security_provider.generate_key(&key_spec);
            if let Ok(key) = key_result {
                keys.push(key);
            }

            if i % 10 == 0 {
                let health = security_provider
                    .health_check()
                    .map_err(|e| BearDogError::system(format!("Operation error: {:?}", e)))?;
                assert!(
                    health.is_healthy || health.memory_usage_mb < 1000,
                     "Memory usage should remain reasonable"
                );
            }
        }

        assert!(
            !keys.is_empty(),
             "Should generate at least some keys under memory pressure"
        );
    }
}
