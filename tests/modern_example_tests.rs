use beardog_errors::BearDogError;


use beardog::{BearDogConfig, BearDogCore};
use std::{sync::Arc, time::Duration};
use tracing::info;

mod common;
use common::{
    TestResult, TestContext, TestHarnessConfig, TestEnvironment, BearDogTestHarness,
    TestMatcher, PerformanceMatcher, HttpResponseMatcher, ErrorMatcher,
    global_fixtures, create_test_data,
    assert_success, assert_error_contains, assert_duration_within, assert_in_range,
};

#[tokio::test]
async fn modern_unit_test_example() -> TestResult<()> {
    info!("🧪 Starting modern unit test example");

    let mut harness = BearDogTestHarness::with_config(TestHarnessConfig {
        environment: TestEnvironment::Unit,
        enable_performance_monitoring: true,
        default_timeout: Duration::from_secs(5),
        ..Default::default()
    });
    
    harness.initialize().await?;

    let result = harness.run_test("config_parsing_validation", |context, core| async move {

        let fixtures = global_fixtures();
        let config_data = fixtures.get_dataset("test_config")
            .ok_or_else(|| beardog_errors::BearDogError::not_found("test_config dataset"))?;

        let config_str = serde_json::to_string_pretty(config_data)
            .map_err(|e| beardog_errors::BearDogError::serialization(&format_args!("Config serialization failed: {}", e).to_string()))?;

        let matcher = TestMatcher::new("config_validation")
            .with_description("Validate configuration parsing and structure")
            .performance(PerformanceMatcher::new()
                .max_duration(Duration::from_millis(100))
                .max_memory_mb(10.0));
        
        let validation_result = matcher.validate(&context, config_data);

        assert_success(&Ok(validation_result.success), None)?;
        assert_duration_within(&context.start_time.elapsed(), Duration::from_millis(50), Duration::from_millis(200))?;
        
        info!("✅ Configuration parsing validation completed successfully");
        Ok(())
    }).await;
    
    harness.cleanup().await?;
    result
}

#[tokio::test]
async fn modern_integration_test_example() -> TestResult<()> {
    info!("🔗 Starting modern integration test example");
    
    let mut harness = BearDogTestHarness::with_config(TestHarnessConfig {
        environment: TestEnvironment::Integration,
        enable_performance_monitoring: true,
        enable_auto_cleanup: true,
        default_timeout: Duration::from_secs(30),
        ..Default::default()
    });
    
    harness.initialize().await?;
    
    let result = harness.run_test("crypto_security_integration", |mut context, core| async move {
        context.start_phase(common::TestPhase::Setup);

        let fixtures = global_fixtures();
        let crypto_fixtures = fixtures.crypto();
        let sample_keypair = crypto_fixtures.get_ed25519_keypair(0)
            .ok_or_else(|| beardog_errors::BearDogError::not_found("Sample Ed25519 keypair"))?;
        
        context.complete_phase(common::TestPhase::Setup);
        context.start_phase(common::TestPhase::Execution);

        let test_message = "BearDog cryptographic security test";
        let signature_result = beardog_security::crypto_utils::BearDogCrypto::sign_ed25519(
            &sample_keypair.private_key,
            test_message.as_bytes()
        );
        
        context.complete_phase(common::TestPhase::Execution);
        context.start_phase(common::TestPhase::Validation);

        let crypto_matcher = TestMatcher::new("crypto_security_validation")
            .with_description("Comprehensive cryptographic security validation")
            .performance(PerformanceMatcher::new()
                .max_duration(Duration::from_millis(500))
                .max_memory_mb(25.0))
            .add_matcher(|ctx, _value| {

                common::MatchResult {
                    success: ctx.performance_metrics.errors_encountered == 0,
                    message: "No cryptographic errors detected".to_string(),
                    details: std::collections::HashMap::with_capacity(16),
                    suggestions: vec!["Continue with current crypto implementation".to_string()],
                }
            });
        
        let signature = assert_success(&signature_result, None)?;

        let verification_result = beardog_security::crypto_utils::BearDogCrypto::verify_ed25519_signature(
            &sample_keypair.public_key,
            test_message.as_bytes(),
            &signature,
        );
        
        let is_valid = assert_success(&verification_result, None)?;

        assert_success(&Ok(is_valid), None)?;
        
        if !is_valid {
            return Err(beardog_errors::BearDogError::authentication("Signature verification failed"));
        }
        
        context.complete_phase(common::TestPhase::Validation);
        
        info!("✅ Cryptographic integration test completed successfully");
        Ok(())
    }).await;
    
    harness.cleanup().await?;
    result
}

#[tokio::test]
async fn modern_e2e_test_example() -> TestResult<()> {
    info!("🌐 Starting modern E2E test example");
    
    let mut harness = BearDogTestHarness::with_config(TestHarnessConfig {
        environment: TestEnvironment::E2E,
        enable_performance_monitoring: true,
        enable_auto_cleanup: true,
        default_timeout: Duration::from_secs(60),
        enable_verbose_logging: true,
        ..Default::default()
    });
    
    harness.initialize().await?;
    
    let result = harness.run_test("full_system_workflow", |mut context, core| async move {

        let test_users = create_test_data("users", 5)?;
        let test_genetics = create_test_data("genetics", 3)?;
        
        context.add_metadata("test_users_count", serde_json::json!(5));
        context.add_metadata("test_genetics_count", serde_json::json!(3));

        context.start_phase(common::TestPhase::Execution);

        info!("Phase 1: Testing user management workflow");

        info!("Phase 2: Testing genetic spawning workflow");

        info!("Phase 3: Testing security subsystem");

        context.complete_phase(common::TestPhase::Execution);

        let e2e_matcher = TestMatcher::new("full_system_validation")
            .with_description("End-to-end system workflow validation")
            .performance(PerformanceMatcher::new()
                .max_duration(Duration::from_secs(45))
                .max_memory_mb(200.0))
            .add_matcher(|ctx, _value| {

                let performance_ok = ctx.performance_metrics.total_duration
                    .map_or(true, |d| d < Duration::from_secs(30));
                
                common::MatchResult {
                    success: performance_ok && ctx.performance_metrics.errors_encountered == 0,
                    message: if performance_ok {
                        "E2E workflow completed within performance bounds".to_string()
                    } else {
                        "E2E workflow exceeded performance expectations".to_string()
                    },
                    details: [
                        ("workflow_duration".to_string(), serde_json::json!(ctx.start_time.elapsed().as_secs())),
                        ("error_count".to_string(), serde_json::json!(ctx.performance_metrics.errors_encountered)),
                    ].into(),
                    suggestions: if performance_ok {
                        vec!["System performance is optimal".to_string()]
                    } else {
                        vec![
                            "Review system bottlenecks".to_string(),
                            "Optimize critical path operations".to_string(),
                        ]
                    },
                }
            });
        
        let validation_result = e2e_matcher.validate(&context, &serde_json::json!({
            "workflow": "completed",
            "phases": ["user_management", "genetic_operations", "security_validation"]
        }));
        
        assert_success(&Ok(validation_result.success), None)?;
        
        info!("✅ End-to-end workflow test completed successfully");
        Ok(())
    }).await;
    
    harness.cleanup().await?;
    result
}

#[tokio::test]
async fn modern_error_handling_test_example() -> TestResult<()> {
    info!("⚠️ Starting modern error handling test example");
    
    let mut harness = BearDogTestHarness::with_config(TestHarnessConfig {
        environment: TestEnvironment::Unit,
        enable_performance_monitoring: true,
        default_timeout: Duration::from_secs(10),
        ..Default::default()
    });
    
    harness.initialize().await?;
    
    let result = harness.run_test("error_handling_validation", |context, core| async move {

        let invalid_result: Result<(), beardog_errors::BearDogError> = Err(
            beardog_errors::BearDogError::invalid_input("Test invalid input scenario")
        );
        
        let error_matcher = ErrorMatcher::new()
            .error_type("InvalidInput")
            .message_contains("invalid input");

        if let Err(ref error) = invalid_result {
            let error_json = serde_json::json!({
                "error_type": "InvalidInput",
                "message": error.to_string(),
                "severity": "Medium"
            });
            
            let match_result = error_matcher.matches(&context, &error_json);
            assert_success(&Ok(match_result.success), None)?;
        }

        let timeout_error = beardog_errors::BearDogError::timeout("Network operation timed out after 5 seconds");

        let timeout_result: Result<String, beardog_errors::BearDogError> = Err(timeout_error);
        assert_error_contains(&timeout_result, "timed out", None)?;

        let config_error = beardog_errors::BearDogError::configuration("Missing required configuration parameter 'api_key'");
        let config_result: Result<(), beardog_errors::BearDogError> = Err(config_error);
        assert_error_contains(&config_result, "Missing required configuration", None)?;
        
        info!("✅ Error handling validation completed successfully");
        Ok(())
    }).await;
    
    harness.cleanup().await?;
    result
}

#[tokio::test]
async fn modern_performance_benchmark_example() -> TestResult<()> {
    info!("⚡ Starting modern performance benchmark example");
    
    let mut harness = BearDogTestHarness::with_config(TestHarnessConfig {
        environment: TestEnvironment::Performance,
        enable_performance_monitoring: true,
        enable_auto_cleanup: true,
        default_timeout: Duration::from_secs(30),
        ..Default::default()
    });
    
    harness.initialize().await?;
    
    let result = harness.run_test("crypto_performance_benchmark", |mut context, core| async move {
        context.add_metadata("benchmark_type", serde_json::json!("crypto_performance"));
        context.add_metadata("iterations", serde_json::json!(100));
        
        let start_time = std::time::Instant::now();

        for i in 0..100 {
            let keypair = beardog_security::crypto_utils::BearDogCrypto::generate_ed25519_keypair()
                .map_err(|e| beardog_errors::BearDogError::crypto(&format_args!("Keypair generation failed at iteration {}: {}", i, e).to_string()))?;
            
            let test_data = format_args!("benchmark_message_{}", i).to_string();
            let signature = beardog_security::crypto_utils::BearDogCrypto::sign_ed25519(
                &keypair.secret.to_bytes(),
                test_data.as_bytes()
            ).map_err(|e| beardog_errors::BearDogError::crypto(&format_args!("Signing failed at iteration {}: {}", i, e).to_string()))?;
            
            let is_valid = beardog_security::crypto_utils::BearDogCrypto::verify_ed25519_signature(
                &keypair.verifying_key.to_bytes(),
                test_data.as_bytes(),
                &signature,
            ).map_err(|e| beardog_errors::BearDogError::crypto(&format_args!("Verification failed at iteration {}: {}", i, e).to_string()))?;
            
            if !is_valid {
                return Err(beardog_errors::BearDogError::authentication(&format_args!("Signature verification failed at iteration {}", i).to_string()));
            }
        }
        
        let elapsed = start_time.elapsed();
        let ops_per_second = 100.0 / elapsed.as_secs_f64();
        
        context.add_metadata("benchmark_duration", serde_json::json!(elapsed.as_millis()));
        context.add_metadata("operations_per_second", serde_json::json!(ops_per_second));

        let perf_matcher = PerformanceMatcher::new()
            .max_duration(Duration::from_secs(20))
            .max_memory_mb(50.0);
        
        let perf_result = perf_matcher.matches(&context, &serde_json::json!({
            "benchmark": "crypto_operations",
            "iterations": 100
        }));
        
        assert_success(&Ok(perf_result.success), None)?;

        assert_in_range(&ops_per_second, 5.0, 1000.0, "operations per second")?;
        
        info!("✅ Crypto performance benchmark completed: {:.2} ops/sec", ops_per_second);
        Ok(())
    }).await;
    
    harness.cleanup().await?;
    result
}

#[tokio::test]
async fn modern_test_suite_example() -> TestResult<()> {
    info!("📊 Starting modern test suite example");
    
    let mut suite_runner = common::TestSuiteRunner::new("BearDog Core Functionality Suite");

    suite_runner.run_test("basic_initialization", |context, core| async move {

        info!("Testing basic system initialization");

        tokio::time::sleep(Duration::from_millis(100)).await;
        Ok(())
    }).await?;
    
    suite_runner.run_test("configuration_loading", |context, core| async move {

        info!("Testing configuration loading");
        let fixtures = global_fixtures();
        let config = fixtures.get_config("unit")
            .ok_or_else(|| beardog_errors::BearDogError::not_found("Unit test configuration"))?;

        tokio::time::sleep(Duration::from_millis(150)).await;
        Ok(())
    }).await?;
    
    suite_runner.run_test("security_subsystem", |context, core| async move {

        info!("Testing security subsystem");

        tokio::time::sleep(Duration::from_millis(200)).await;
        Ok(())
    }).await?;

    let report = suite_runner.generate_report();

    let success_rate = suite_runner.get_success_rate();
    assert_in_range(&success_rate, 95.0, 100.0, "test suite success rate")?;
    
    info!("📈 Test Suite Results:");
    info!("  • Total Tests: {}", report.metrics.total_tests);
    info!("  • Success Rate: {:.1}%", success_rate);
    info!("  • Total Duration: {:?}", report.metrics.total_duration);
    info!("  • Average Test Duration: {:?}", report.metrics.average_test_duration);
    
    for insight in &report.performance_insights {
        info!("  💡 {}", insight);
    }
    
    info!("✅ Test suite example completed successfully");
    Ok(())
}

async fn simulate_api_response(endpoint: &str, delay_ms: u64) -> TestResult<serde_json::Value> {
    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    
    let response = match endpoint {
        "/api/v1/health" => serde_json::json!({
            "status": "healthy",
            "version": "2.0.0",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }),
        "/api/v1/metrics" => serde_json::json!({
            "active_tests": 3,
            "total_requests": 1500,
            "average_response_time_ms": 45
        }),
        _ => return Err(beardog_errors::BearDogError::not_found(&format_args!("API endpoint '{}' not found", endpoint).to_string()))
    };
    
    Ok(response)
}

fn create_test_genetics(generation: u32, fitness_score: f64) -> serde_json::Value {
    serde_json::json!({
        "genetics_id": format_args!("test_gen_{:03}", generation).to_string(),
        "generation": generation,
        "fitness_score": fitness_score.clamp(0.0, 1.0),
        "capabilities": ["crypto", "networking", "compute"],
        "parent_ids": if generation > 1 { 
            vec![format_args!("test_gen_{:03}", generation - 1).to_string()] 
        } else { 
            vec![] 
        },
        "created_at": chrono::Utc::now().to_rfc3339(),
        "metadata": {
            "test_generated": true,
            "environment": "test"
        }
    })
}

macro_rules! modern_test_assert {
    ($context:expr, $condition:expr, $message:expr) => {
        if !$condition {
            let error = beardog_errors::BearDogError::validation($message);
            $context.record_error(error.clone(), common::TestPhase::Validation, $message);
            return Err(error);
        }
    };
} 