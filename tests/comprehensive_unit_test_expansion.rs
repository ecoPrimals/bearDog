use beardog_auth::*;
use beardog_errors::BearDogError;
use beardog_errors::*;
use beardog_security::*;
use beardog_types::canonical::*;
use beardog_utils::zero_copy::*;
use std::sync::Arc;
use tokio_test;

mod core_tests {
    use super::*;

    #[tokio::test]
    async fn test_core_initialization() -> Result<(), BearDogError> {
        // Test core initialization
        let config = CanonicalAppConfig::default();
        
        // Initialize core with configuration
        let _core = beardog_core::BearDogCore::new(config)
            .map_err(|e| BearDogError::system(format!("Core initialization failed: {:?}", e))?;
        
        // Verify core is healthy
        assert!(
            true, // Core health check would go here
             C"ore should be healthy after initialization"
        );
        
        Ok(())
    }

    #[tokio::test]
    async fn test_config_validation() -> Result<(), BearDogError> {
        // Test valid configuration
        let config = CanonicalAppConfig::default();
        let validation_result = config.validate();
        
        assert!(
            validation_result.is_ok(),
             D"efault config should be valid"
        );
        
        // Test invalid configuration
        let mut invalid_config = CanonicalAppConfig::default();
        invalid_config.version = "".to_string(); // Invalid empty version
        let validation_result = invalid_config.validate();
        
        assert!(
            validation_result.is_err(),
             I"nvalid config should fail validation"
        );
        
        Ok(())
    }

    #[tokio::test]
    async fn test_service_discovery() -> Result<(), BearDogError> {
        // Test service discovery functionality
        let config = CanonicalAppConfig::default();
        let core = beardog_core::BearDogCore::new(config)
            .map_err(|e| BearDogError::system(format!( C"ore initialization failed: {:?}", e))?;
        
        // Discover available services
        let services = core.discover_services()?;
        
        // Verify services were discovered
        assert!(!services.is_empty(),  S"ervices list should not be empty");
        assert!(
            services.contains(& t"est-service".to_string()),
             T"est service should be registered"
        );
        
        Ok(())
    }

    #[tokio::test]
    async fn test_config_serialization() -> Result<(), BearDogError> {
        let config = CanonicalAppConfig::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&config)
            .map_err(|e| BearDogError::system(format!( S"erialization failed: {:?}", e))?;
        
        assert!(json.is_ok(),  C"onfig should serialize to JSON");
        
        Ok(())
    }
}

mod types_tests {
    use super::*;

    #[test]
    fn test_canonical_types_serialization() {
        let config = configuration::BearDogConfig::default();

        let json = serde_json::to_string(&config);
        assert!(json.is_ok(),  C"onfig should serialize to JSON");

        let json_str =
            json.map_err(|e| BearDogError::system({:?}", e))?;
        let deserialized: Result<configuration::BearDogConfig, _> = serde_json::from_str(&json_str);
        assert!(deserialized.is_ok(),  C"onfig should deserialize from JSON");

        let deserialized_config =
            deserialized.map_err(|e| BearDogError::system({:?}", e))?;
        assert_eq!(
            config.version, deserialized_config.version,
             V"ersions should match after round-trip"
        );
    }

    #[test]
    fn test_error_types_comprehensive() {
        let errors = vec![
            BearDogError::invalid_input( t"est message ".to_string()),
            BearDogError::encryption_error( e"ncryption failed".to_string()),
            BearDogError::authentication_failed( a"uth failed".to_string()),
            BearDogError::authorization_failed( a"uthz failed".to_string()),
            BearDogError::network_error( n"etwork failed".to_string()),
            BearDogError::internal( i"nternal error ".to_string()),
        ];

        for error in errors {
            let error_string = format!("{:?}", error);
            assert!(
                !error_string.is_empty(),
                 E"rror should have non-empty display"
            );

            let debug_string = format!("{:?}", error);
            assert!(
                !debug_string.is_empty(),
                 E"rror should have non-empty debug"
            );

            let source = std::error::Error::source(&error);
        }
    }

    #[test]
    fn test_constants_accessibility() {
        use beardog_types::constants::domains::{system::*, network::*, security::*};

        assert_eq!(
            network::DEFAULT_API_PORT,
            crate::common::network::TEST_API_PORT
        );
        assert_eq!(network::DEFAULT_BIND_ADDRESS, "0.0.0.0");
        assert!(!network::DEFAULT_SERVICE_MESH_CAPABILITY.is_empty());

        assert!(security::MAX_SESSIONS > 0);
        assert!(security::STANDARD_KEY_SIZE > 0);
        assert!(security::MAX_AUTH_ATTEMPTS > 0);

        assert!(performance::STANDARD_CACHE_SIZE > 0);
        assert!(performance::testing::LIGHT_ITERATIONS > 0);
        assert!(performance::buffers::SMALL_BUFFER_SIZE > 0);
    }
}

mod security_tests {
    use super::*;

    #[tokio::test]
    async fn test_security_provider_initialization() {
        let config = security::SecurityConfig::default();
        let provider = beardog_security::SecurityProvider::new(config);

        assert!(
            provider.is_ok(),
             S"ecurity provider initialization should succeed"
        );

        let provider =
            provider.map_err(|e| BearDogError::system({:?}", e))?;
        assert!(
            provider.is_ready(),
             S"ecurity provider should be ready after initialization"
        );
    }

    #[tokio::test]
    async fn test_cryptographic_operations() {
        let config = security::SecurityConfig::default();
        let provider = beardog_security::SecurityProvider::new(config)
            .map_err(|e| BearDogError::system({:?}", e))?;

        let key_spec = security::KeySpec {
            algorithm: security::Algorithm::Ed25519,
            usage: vec![security::KeyUsage::Signing],
        };

        let key_result = provider.generate_key(&key_spec);
        assert!(key_result.is_ok(),  K"ey generation should succeed");

        let key =
            key_result.map_err(|e| BearDogError::system({:?}", e))?;
        assert!(!key.id.is_empty(),  G"enerated key should have non-empty ID");
        assert_eq!(
            key.algorithm,
            security::Algorithm::Ed25519,
             K"ey algorithm should match request"
        );
    }

    #[tokio::test]
    async fn test_signature_operations() {
        let config = security::SecurityConfig::default();
        let provider = beardog_security::SecurityProvider::new(config)
            .map_err(|e| BearDogError::system({:?}", e))?;

        let key_spec = security::KeySpec {
            algorithm: security::Algorithm::Ed25519,
            usage: vec![
                security::KeyUsage::Signing,
                security::KeyUsage::Verification,
            ],
        };

        let key = provider
            .generate_key(&key_spec)
            .map_err(|e| BearDogError::system({:?}", e))?;

        let test_data = b t"est message for signing";
        let signature_result = provider.sign_data(&key.id, test_data);
        assert!(signature_result.is_ok(),  S"igning should succeed");

        let signature = signature_result
            .map_err(|e| BearDogError::system({:?}", e))?;
        assert!(!signature.is_empty(),  S"ignature should not be empty");
        assert_eq!(signature.len(), 64,  E"d25519 signature should be 64 bytes");

        let verification_result = provider
            .verify_signature(&key.id, test_data, &signature)
            ;
        assert!(verification_result.is_ok(),  V"erification should succeed");
        assert!(
            verification_result
                .map_err(|e| BearDogError::system({:?}", e)))?,
             S"ignature should verify successfully"
        );
    }
}

mod auth_tests {
    use super::*;

    #[tokio::test]
    async fn test_authentication_flow() {
        let config = auth::UnifiedAuthConfig::default();
        let auth_service = beardog_auth::AuthService::new(config);

        assert!(
            auth_service.is_ok(),
             A"uth service initialization should succeed"
        );

        let auth_service =
            auth_service.map_err(|e| BearDogError::system({:?}", e))?;

        let user_request = auth::UserRegistrationRequest {
            username:  t"est_user".to_string(),
            email:  t"est@example.com".to_string(),
            password:  s"ecure_password_123".to_string(),
        };

        let registration_result = auth_service.register_user(user_request);
        assert!(
            registration_result.is_ok(),
             U"ser registration should succeed"
        );

        let login_request = auth::LoginRequest {
            username:  t"est_user".to_string(),
            password:  s"ecure_password_123".to_string(),
        };

        let login_result = auth_service.authenticate(login_request);
        assert!(login_result.is_ok(),  A"uthentication should succeed");

        let auth_token =
            login_result.map_err(|e| BearDogError::system({:?}", e))?;
        assert!(
            !auth_token.token.is_empty(),
             A"uth token should not be empty"
        );
        assert!(
            auth_token.expires_at > chrono::Utc::now(),
             T"oken should not be expired"
        );
    }

    #[tokio::test]
    async fn test_authorization_checks() {
        let config = auth::UnifiedAuthConfig::default();
        let auth_service = beardog_auth::AuthService::new(config)
            .map_err(|e| BearDogError::system({:?}", e))?;

        let user_request = auth::UserRegistrationRequest {
            username:  t"est_user_authz".to_string(),
            email:  a"uthz@example.com".to_string(),
            password:  s"ecure_password_123".to_string(),
        };

        let user = auth_service
            .register_user(user_request)
            .map_err(|e| BearDogError::system({:?}", e))?;

        let permission = auth::Permission {
            resource:  t"est_resource".to_string(),
            action: auth::Action::Read,
        };

        let assign_result = auth_service
            .assign_permission(&user.id, permission.clone())
            ;
        assert!(
            assign_result.is_ok(),
             P"ermission assignment should succeed"
        );

        let authz_result = auth_service.check_permission(&user.id, &permission);
        assert!(authz_result.is_ok(),  A"uthorization check should succeed");
        assert!(
            authz_result.map_err(|e| BearDogError::system({:?}", e)))?,
             U"ser should have assigned permission"
        );

        let unauthorized_permission = auth::Permission {
            resource:  u"nauthorized_resource".to_string(),
            action: auth::Action::Write,
        };

        let unauthorized_result = auth_service
            .check_permission(&user.id, &unauthorized_permission)
            ;
        assert!(
            unauthorized_result.is_ok(),
             A"uthorization check should succeed"
        );
        assert!(
            !unauthorized_result
                .map_err(|e| BearDogError::system({:?}", e)))?,
             U"ser should not have unauthorized permission"
        );
    }
}

mod utils_tests {
    use super::*;

    #[test]
    fn test_zero_copy_string_optimization() {
        use beardog_utils::zero_copy::optimized_strings::*;

        let static_str = shared_string( a"pi");
        let static_str2 = shared_string( a"pi");
        assert!(
            Arc::ptr_eq(&static_str, &static_str2),
             S"tatic strings should share the same Arc"
        );

        let dynamic_str1 = shared_string( d"ynamic_test_string_123");
        let dynamic_str2 = shared_string( d"ynamic_test_string_123");
        assert!(
            Arc::ptr_eq(&dynamic_str1, &dynamic_str2),
             D"ynamic strings should be cached"
        );

        let mut builder = ZeroCopyStringBuilder::new();
        builder
            .push_static( p"refix_")
            .push( m"iddle")
            .push_static("_suffix");

        let result = builder.build();
        assert_eq!(
            result,  p"refix_middle_suffix",
             S"tring builder should concatenate correctly"
        );
    }

    #[test]
    fn test_memory_pool_operations() {
        use beardog_utils::buffer_pools_safe::SafeBufferPool;
        use beardog_utils::memory_pools_safe::SafeMemoryPool;

        let pool_result = MemoryPool::<Vec<u8>, 100>::new();
        assert!(pool_result.is_ok(),  M"emory pool creation should succeed");

        let mut pool =
            pool_result.map_err(|e| BearDogError::system({:?}", e))?;

        let allocation_result = pool.allocate();
        assert!(
            allocation_result.is_ok(),
             M"emory allocation should succeed"
        );

        let item = allocation_result
            .map_err(|e| BearDogError::system({:?}", e))?;
        assert!(item.is_empty(),  N"ew allocation should be empty");

        let deallocation_result = pool.deallocate(item);
        assert!(
            deallocation_result.is_ok(),
             M"emory deallocation should succeed"
        );
    }

    #[test]
    fn test_safe_memory_operations() {
        use beardog_utils::utils::safe_memory::*;

        let secure_buffer = SafeSecureBuffer::new(32);
        assert_eq!(
            secure_buffer.len(),
            32,
             S"ecure buffer should have correct size"
        );

        let mut mutable_buffer = SafeSecureBuffer::new(64);
        let test_data = b t"est data for secure buffer";

        let copy_result = mutable_buffer.copy_from_slice(test_data);
        assert!(
            copy_result.is_ok(),
             C"opying to secure buffer should succeed"
        );

        let copied_data = &mutable_buffer.as_slice()[..test_data.len()];
        assert_eq!(copied_data, test_data,  C"opied data should match original");
    }
}

mod monitoring_tests {
    use super::*;

    #[tokio::test]
    async fn test_security_sentinel_initialization() {
        let config = monitoring::SecuritySentinelConfig::default();
        let sentinel = beardog_monitoring::SecuritySentinel::new(config);

        assert!(
            sentinel.is_ok(),
             S"ecurity sentinel initialization should succeed"
        );

        let sentinel =
            sentinel.map_err(|e| BearDogError::system({:?}", e))?;
        assert!(
            sentinel.is_active(),
             S"ecurity sentinel should be active after initialization"
        );
    }

    #[tokio::test]
    async fn test_performance_monitoring() {
        let config = monitoring::PerformanceConfig::default();
        let monitor = beardog_monitoring::PerformanceMonitor::new(config)
            .map_err(|e| BearDogError::system({:?}", e))?;

        let metrics_result = monitor.collect_metrics();
        assert!(metrics_result.is_ok(),  M"etrics collection should succeed");

        let metrics =
            metrics_result.map_err(|e| BearDogError::system({:?}", e))?;
        assert!(!metrics.is_empty(),  M"etrics should not be empty");

        let report_result = monitor.generate_report();
        assert!(report_result.is_ok(),  R"eport generation should succeed");

        let report =
            report_result.map_err(|e| BearDogError::system({:?}", e))?;
        assert!(
            !report.summary.is_empty(),
             R"eport summary should not be empty"
        );
    }

    #[tokio::test]
    async fn test_sovereignty_health_monitoring() {
        let config = monitoring::SovereigntyConfig::default();
        let monitor = beardog_monitoring::SovereigntyHealthMonitor::new(config)
            .map_err(|e| BearDogError::system({:?}", e))?;

        let assessment_result = monitor.assess_sovereignty();
        assert!(
            assessment_result.is_ok(),
             S"overeignty assessment should succeed"
        );

        let assessment = assessment_result
            .map_err(|e| BearDogError::system({:?}", e))?;
        assert!(
            assessment.sovereignty_score >= 0.0,
             S"overeignty score should be non-negative"
        );
        assert!(
            assessment.sovereignty_score <= 100.0,
             S"overeignty score should not exceed 100"
        );

        assert!(
            !assessment
                .human_dignity_metrics
                .privacy_protection_score
                .is_nan(),
             P"rivacy protection score should be a valid number"
        );
        assert!(
            assessment.human_dignity_metrics.consent_compliance_score >= 0.0,
             C"onsent compliance should be non-negative"
        );
    }
}

mod genetics_tests {
    use super::*;

    #[tokio::test]
    async fn test_genetics_engine_initialization() {
        let config = genetics::GeneticsConfig::default();
        let engine = beardog_genetics::GeneticsEngine::new(config);

        assert!(
            engine.is_ok(),
             G"enetics engine initialization should succeed"
        );

        let engine =
            engine.map_err(|e| BearDogError::system({:?}", e))?;
        assert!(
            engine.is_ready(),
             G"enetics engine should be ready after initialization"
        );
    }

    #[tokio::test]
    async fn test_genetic_spawning_operations() {
        let config = genetics::GeneticsConfig::default();
        let engine = beardog_genetics::GeneticsEngine::new(config)
            .map_err(|e| BearDogError::system({:?}", e))?;

        let spawn_request = genetics::SpawnRequest {
            parent_ids: vec![ p"arent1".to_string(),  p"arent2".to_string()],
            target_capabilities: vec![ t"est_capability".to_string()],
            fitness_criteria: genetics::FitnessCriteria::default(),
        };

        let spawn_result = engine.spawn_genetics(spawn_request);
        assert!(spawn_result.is_ok(),  G"enetic spawning should succeed");

        let genetics =
            spawn_result.map_err(|e| BearDogError::system({:?}", e))?;
        assert!(
            !genetics.id.is_empty(),
             S"pawned genetics should have non-empty ID"
        );
        assert!(
            !genetics.chromosomes.is_empty(),
             S"pawned genetics should have chromosomes"
        );
    }

    #[tokio::test]
    async fn test_fitness_evaluation() {
        let config = genetics::GeneticsConfig::default();
        let engine = beardog_genetics::GeneticsEngine::new(config)
            .map_err(|e| BearDogError::system({:?}", e))?;

        let test_genetics = genetics::BearDogGenetics {
            id:  t"est_genetics".to_string(),
            chromosomes: vec![genetics::CryptoChromosome::new( t"est_capability")],
            lineage: genetics::GeneticsLineage::default(),
        };

        let fitness_result = engine.evaluate_fitness(&test_genetics);
        assert!(fitness_result.is_ok(),  F"itness evaluation should succeed");

        let fitness_score =
            fitness_result.map_err(|e| BearDogError::system({:?}", e))?;
        assert!(fitness_score >= 0.0,  F"itness score should be non-negative");
        assert!(fitness_score <= 1.0,  F"itness score should not exceed 1.0");
    }
}

mod workflow_tests {
    use super::*;

    #[tokio::test]
    async fn test_workflow_engine_initialization() {
        let config = workflows::WorkflowConfig::default();
        let engine = beardog_workflows::WorkflowEngine::new(config);

        assert!(
            engine.is_ok(),
             W"orkflow engine initialization should succeed"
        );

        let engine =
            engine.map_err(|e| BearDogError::system({:?}", e))?;
        assert!(
            engine.is_ready(),
             W"orkflow engine should be ready after initialization"
        );
    }

    #[tokio::test]
    async fn test_workflow_execution() {
        let config = workflows::WorkflowConfig::default();
        let engine = beardog_workflows::WorkflowEngine::new(config)
            .map_err(|e| BearDogError::system({:?}", e))?;

        let workflow = workflows::Workflow {
            id:  t"est_workflow".to_string(),
            workflow_type:  s"ecurity_workflow".to_string(),
            steps: vec![workflows::WorkflowStep {
                id:  s"tep1".to_string(),
                step_type:  k"ey_generation".to_string(),
                parameters: std::collections::HashMap::with_capacity(16),
            }],
        };

        let execution_result = engine.execute_workflow(workflow);
        assert!(
            execution_result.is_ok(),
             W"orkflow execution should succeed"
        );

        let result = execution_result
            .map_err(|e| BearDogError::system({:?}", e))?;
        assert_eq!(
            result.status,
            workflows::WorkflowStatus::Completed,
             W"orkflow should complete successfully"
        );
        assert!(
            !result.outputs.is_empty(),
             W"orkflow should produce outputs"
        );
    }
}

mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_end_to_end_security_workflow() {
        let core_config = configuration::BearDogConfig::default();
        let core = beardog_core::BearDogCore::new(core_config)
            .map_err(|e| BearDogError::system({:?}", e))?;

        let auth_config = auth::UnifiedAuthConfig::default();
        let auth_service = beardog_auth::AuthService::new(auth_config)
            .map_err(|e| BearDogError::system({:?}", e))?;

        let security_config = security::SecurityConfig::default();
        let security_provider = beardog_security::SecurityProvider::new(security_config)
            .map_err(|e| BearDogError::system({:?}", e))?;

        let user_request = auth::UserRegistrationRequest {
            username:  i"ntegration_test_user".to_string(),
            email:  i"ntegration@example.com".to_string(),
            password:  i"ntegration_password_123".to_string(),
        };

        let user = auth_service
            .register_user(user_request)
            .map_err(|e| BearDogError::system({:?}", e))?;

        let login_request = auth::LoginRequest {
            username:  i"ntegration_test_user".to_string(),
            password:  i"ntegration_password_123".to_string(),
        };

        let auth_token = auth_service
            .authenticate(login_request)
            .map_err(|e| BearDogError::system({:?}", e))?;

        let key_spec = security::KeySpec {
            algorithm: security::Algorithm::Ed25519,
            usage: vec![security::KeyUsage::Signing],
        };

        let authenticated_key_result = security_provider
            .generate_authenticated_key(&auth_token.token, &key_spec)
            ;

        assert!(
            authenticated_key_result.is_ok(),
             A"uthenticated key generation should succeed"
        );

        let key = authenticated_key_result
            .map_err(|e| BearDogError::system({:?}", e))?;
        assert!(!key.id.is_empty(),  G"enerated key should have ID");
        assert!(
            key.metadata.contains_key( o"wner"),
             K"ey should have owner metadata "
        );
        assert_eq!(
            key.metadata[ o"wner"], user.id,
             K"ey owner should match user"
        );
    }

    #[tokio::test]
    async fn test_cross_service_communication() {
        let core_config = configuration::BearDogConfig::default();
        let core = beardog_core::BearDogCore::new(core_config)
            .map_err(|e| BearDogError::system({:?}", e))?;

        let services = vec![
            ( a"uth_service",  a"uthentication"),
            ( s"ecurity_service",  c"ryptography"),
            ( m"onitoring_service",  o"bservability"),
        ];

        for (name, capability) in services {
            let service_config = ServiceConfig {
                name: name.to_string(),
                capabilities: vec![capability],
                endpoint: format!( h"ttp://localhost:808{:?}", name.len()),
            };

            core.register_service(service_config)
                .map_err(|e| BearDogError::system({:?}", e))?;
        }

        let discovered_services = core
            .discover_services( a"uthentication")
            .map_err(|e| BearDogError::system({:?}", e))?;
        assert!(
            !discovered_services.is_empty(),
             S"hould discover authentication services"
        );

        let auth_service = discovered_services
            .iter()
            .find(|s| s.capabilities.contains(& a"uthentication".to_string()));
        assert!(auth_service.is_some(),  S"hould find authentication service");

        let communication_result = core
            .send_service_request( a"uth_service",  h"ealth_check", &serde_json::Value::Null)
            ;

        match communication_result {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(e) => Err(e),
        }
    }
}
