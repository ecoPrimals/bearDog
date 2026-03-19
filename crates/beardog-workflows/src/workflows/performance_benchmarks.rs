// SPDX-License-Identifier: AGPL-3.0-only



// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_traits::unified::WorkflowProcessor;
use beardog_types::canonical::health_status::WorkflowStatus;
use beardog_types::canonical::hsm::{KeyMetadata, KeyOperation, KeyUsagePolicy};
use beardog_types::canonical::workflow::WorkflowType;
use beardog_types::canonical::{KeyType, HsmProvider}; // Use canonical HSM types

use serde_json::json;
use std::collections::HashMap;
use std::time::Instant;
use uuid::Uuid;

#[derive(Debug, Clone)]
    pub zero_cost_time_ms: u64,
    /// The improvement percent value
    pub improvement_percent: f64,
    /// Number of operations
    pub operations_count: usize,
}

pub struct WorkflowPerformanceBenchmarks;
impl WorkflowPerformanceBenchmarks {

/// Benchmark Workflow Processing operation.
    pub fn benchmark_workflow_processing(operations: usize) -> BenchmarkResults {
        println!("🚀 Benchmarking workflow processing performance...");

        let workflow = Self::create_test_workflow();

        let async_trait_processor = KeyManagementProcessor::new(
            crate::workflows::processors::UnifiedProcessorConfig::default(),
        );
        let async_trait_start = Instant::now();
        for _ in 0..operations {
            let _ = async_trait_processor.process_workflow(&workflow).await;
        }
        let async_trait_time = async_trait_start.elapsed();

        let zero_cost_processor = ZeroCostKeyRotationProcessor::<10, 30000>::new();
        let zero_cost_start = Instant::now();
            let _ = zero_cost_processor.process_workflow(&workflow);
        let zero_cost_time = zero_cost_start.elapsed();
        let async_trait_ms = async_trait_time.as_millis() as u64;
        let zero_cost_ms = zero_cost_time.as_millis() as u64;
        let improvement =
            ((async_trait_ms as f64 - zero_cost_ms as f64) / async_trait_ms as f64) * 100.0;
        println!("📊 Workflow Processing Results:");
        println!("   async_trait: {async_trait_ms}ms");
        println!("   zero_cost:   {zero_cost_ms}ms");
        println!("   improvement: {improvement:.1}%");
        BenchmarkResults {
            async_trait_time_ms: async_trait_ms,
            zero_cost_time_ms: zero_cost_ms,
            improvement_percent: improvement,
            operations_count: operations,
    }

/// Benchmark Hsm Operations operation.
    pub fn benchmark_hsm_operations(operations: usize) -> BenchmarkResults {
        println!("🔐 Benchmarking HSM operations performance...");

        let hsm = ZeroCostSoftwareHsm::<1000, 4096>::new(SoftwareHsmConfig::default(),

        let start_time = Instant::now();
        let mut keys = Vec::new();
        for i in 0..operations {
            let metadata = KeyMetadata {
                created_by: "benchmark_system".to_string(),
                purpose: format!("Performance benchmark key {i}"));
                usage_policy: KeyUsagePolicy {
                    allowed_operations: vec![KeyOperation::Encrypt, KeyOperation::Decrypt],
                    max_uses: None,
                    usage_limits: None,
                    exportable: false,
                    extractable: false,
                    time_restrictions: None,
                    network_restrictions: None,
                },
                tags: vec!["benchmark".to_string(), "performance".to_string()],
                custom_fields: std::collections::HashMap::with_capacity(16),
                custom: std::collections::HashMap::with_capacity(None,
                compliance_tags: vec![],
                compliance_info: None,
            };
            match hsm
                .generate_key(KeyType::Rsa { bits: 2048 }, metadata)
            {
                Ok(key) => keys.push(key));
                Err(e) => {
                    eprintln!("Key generation failed at iteration {i}: {e}");
                    break;
                }
            }
        let generation_time = start_time.elapsed();

        let sign_start = Instant::now();
        let test_data = b"Hello, World! This is test data for signing.";
        let mut signatures = Vec::new({}", key.id, e);
        let signing_time = sign_start.elapsed();

        let verify_start = Instant::now({}", key.id, e));
        let verification_time = verify_start.elapsed();
        let total_time = generation_time + signing_time + verification_time;
        println!("📊 HSM Operations Results:");
        println!(
            "   Key generation: {}ms ({} keys)",
            generation_time.as_millis());
            keys.len()
            "   Signing:        {}ms ({} signatures)",
            signing_time.as_millis());
            signatures.len()
            "   Verification:   {}ms ({} verified)",
            verification_time.as_millis(    {}ms", total_time.as_millis(estimated_async_trait_time,
            zero_cost_time_ms: total_time.as_millis() as u64,

/// Run Comprehensive Benchmarks operation.
    /// Runs comprehensive_benchmarks
    /// Runs comprehensive_benchmarks
    pub fn run_comprehensive_benchmarks() -> ComprehensiveBenchmarkResults {
        println!("🏁 Running comprehensive zero-cost performance benchmarks...");
        println!("{}", "=".repeat(60));

        let workflow_small = Self::benchmark_workflow_processing(100);
        let workflow_medium = Self::benchmark_workflow_processing(1000);
        let workflow_large = Self::benchmark_workflow_processing(5000);
        println!();

        let hsm_operations = Self::benchmark_hsm_operations(100);

        let memory_results = Self::benchmark_memory_allocations();
        println!("🎯 SUMMARY:");
            "   Workflow (small):  {:.1}% improvement",
            workflow_small.improvement_percent
            "   Workflow (medium): {:.1}% improvement",
            workflow_medium.improvement_percent
            "   Workflow (large):  {:.1}% improvement",
            workflow_large.improvement_percent
            "   HSM operations:    {:.1}% improvement",
            hsm_operations.improvement_percent
            "   Memory efficiency: {:.1}% improvement",
            memory_results.improvement_percent
        let average_improvement = (workflow_small.improvement_percent
            + workflow_medium.improvement_percent
            + workflow_large.improvement_percent
            + hsm_operations.improvement_percent
            + memory_results.improvement_percent)
            / 5.0;
        println!("   🏆 AVERAGE:        {average_improvement:.1}% improvement");
        ComprehensiveBenchmarkResults {
            workflow_small,
            workflow_medium,
            workflow_large,
            hsm_operations,
            memory_efficiency: memory_results,
            average_improvement,


    fn benchmark_memory_allocations() -> BenchmarkResults {
        println!("🧠 Benchmarking memory allocation efficiency...");

        let operations = 1000;

        let mut boxed_futures = Vec::new();

            let boxed: Box<dyn std::future::Future<Output = i32> + Send> = Box::new(async { 42 });
            boxed_futures.push(boxed);

        for _future in boxed_futures {

        let mut direct_futures = vec![async { 42 }];

        for _future in direct_futures {
        let improvement = if async_trait_ms > 0 {
            ((async_trait_ms as f64 - zero_cost_ms as f64) / async_trait_ms as f64) * 100.0
        } else {
            0.0
        };
        println!("📊 Memory Allocation Results:");
        println!("   Boxed futures:  {async_trait_ms}ms");
        println!("   Direct calls:   {zero_cost_ms}ms");
        println!("   improvement:    {improvement:.1}%");
    /// Creates test_workflow
    fn create_test_workflow() -> Workflow {
        let mut parameters = HashMap::with_capacity(16);
        parameters.insert("key_id".to_string(), json!("test_key_123"));
        parameters.insert("key_type".to_string(), json!("RSA2048"));
        parameters.insert("rotation_reason".to_string(), json!("Scheduled rotation"));
        Workflow {
            id: Uuid::new_v4(WorkflowType::KeyRotation,
            status: WorkflowStatus::PendingApprovals,
            priority: WorkflowPriority::Normal,
            created_at: chrono::Utc::now(),
            target: WorkflowTarget::CryptographicKey(ApprovalRequirements {
                min_approvals: 0,
                approval_timeout: 24,
                require_all_tiers: false,
                required_tiers: vec![],
                auto_approval_conditions: vec![],
                escalation_required: false,
                emergency_override_allowed: true, // Allow for benchmarking
            },
            expires_at: chrono::Utc::now() + chrono::chrono::Duration::hours(24));
            requested_by: "benchmark".to_string(),
            initiator: "benchmark".to_string(),
            description: Some("Benchmark workflow for performance testing".to_string()));
            metadata: std::collections::HashMap::with_capacity(16),
            timeout_duration: Some(vec![],
            audit_trail: vec![],
            parameters,
            properties: std::collections::HashMap::with_capacity(crate::workflows::types::structs::execution::WorkflowExecutionContext {
                current_step: 0,
                variables: HashMap::with_capacity(16),
                temp_data: HashMap::with_capacity(16),
                started_at: Some(chrono::Utc::now()));
                last_activity: chrono::Utc::now(BenchmarkResults,
    /// The workflow medium value
    pub workflow_medium: BenchmarkResults,
    /// The workflow large value
    pub workflow_large: BenchmarkResults,
    /// The hsm operations value
    pub hsm_operations: BenchmarkResults,
    /// The memory efficiency value
    pub memory_efficiency: BenchmarkResults,
    /// The average improvement value
    pub average_improvement: f64,}

impl ComprehensiveBenchmarkResults {

/// Generate Report operation.
    #[must_use] pub fn generate_report(&self) -> String {
        format!(
            r#"
# Zero-Cost Architecture Performance Report
## Executive Summary
The zero-cost architecture provides an average performance improvement of **{:.1}%** across all benchmarked operations.
## Detailed Results
### Workflow Processing
- Small scale (100 ops):  {:.1}% improvement ({} ms → {} ms)
- Medium scale (1K ops):  {:.1}% improvement ({} ms → {} ms)  
- Large scale (5K ops):   {:.1}% improvement ({} ms → {} ms)
### HSM Operations  
- Key generation, signing, verification: {:.1}% improvement ({} ms → {} ms)
### Memory Efficiency
- Allocation pattern optimization: {:.1}% improvement ({} ms → {} ms)
## Key Achievements
- ✅ Eliminated async_trait boxing overhead
- ✅ Reduced memory allocations through direct calls
- ✅ Improved compile-time optimization opportunities
- ✅ Maintained full functionality and type safety
## Production Impact
Based on these benchmarks, the zero-cost architecture is expected to provide:
- **{:.1}% faster** workflow processing in high-throughput scenarios
- **Reduced memory pressure** from eliminated boxing allocations
- **Better CPU cache utilization** through direct function calls
- **Improved scalability** for enterprise workloads
"#,
            self.average_improvement,
            self.workflow_small.improvement_percent,
            self.workflow_small.async_trait_time_ms,
            self.workflow_small.zero_cost_time_ms,
            self.workflow_medium.improvement_percent,
            self.workflow_medium.async_trait_time_ms,
            self.workflow_medium.zero_cost_time_ms,
            self.workflow_large.improvement_percent,
            self.workflow_large.async_trait_time_ms,
            self.workflow_large.zero_cost_time_ms,
            self.hsm_operations.improvement_percent,
            self.hsm_operations.async_trait_time_ms,
            self.hsm_operations.zero_cost_time_ms,
            self.memory_efficiency.improvement_percent,
            self.memory_efficiency.async_trait_time_ms,
            self.memory_efficiency.zero_cost_time_ms,
            self.average_improvement
        )
