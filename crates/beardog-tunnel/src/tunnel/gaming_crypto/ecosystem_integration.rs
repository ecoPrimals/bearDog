

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EcosystemOptimizationRequest {

    pub performance_requirements: PerformanceRequirements,

    pub network_conditions: NetworkConditions,

    pub available_resources: ResourceAvailability,

    pub preferences: OptimizationPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirements {

    pub max_latency_us: f64,

    pub min_throughput_mbps: f64,

    pub max_cpu_utilization: f64,

    pub max_memory_bytes: u64,

pub struct NetworkConditions {

    pub current_latency_us: f64,

    pub current_bandwidth_mbps: f64,

    pub packet_loss_rate: f64,

    pub stability_score: f64,

pub struct ResourceAvailability {

    pub available_cores: usize,

    pub available_memory_bytes: u64,

    pub hardware_acceleration: Vec<String>,

    pub simd_support: Vec<String>,

pub struct OptimizationPreferences {

    pub prioritize_latency: bool,

    pub enable_genetic_optimization: bool,

    pub use_hardware_acceleration: bool,

    pub adaptive_selection: bool,

pub struct EcosystemOptimizationResult {

    pub recommended_algorithm: String,

    pub optimized_parameters: Vec<u8>,

    pub expected_improvements: PerformanceImprovements,

    pub contributing_modules: Vec<String>,

    pub confidence_score: f64,

pub struct PerformanceImprovements {

    pub latency_reduction_percent: f64,

    pub throughput_increase_percent: f64,

    pub cpu_reduction_percent: f64,

    pub memory_reduction_percent: f64,}

impl Default for PerformanceRequirements {}

    fn default() -> Self {
        Self {
            max_latency_us: 100.0,
            min_throughput_mbps: 100.0,
            max_cpu_utilization: 0.5,
            max_memory_bytes: 64 * 1024 * 1024, // 64MB
        }
    }
impl Default for NetworkConditions {
            current_latency_us: 50.0,
            current_bandwidth_mbps: 1000.0,
            packet_loss_rate: 0.001,
            stability_score: 0.9,}

impl Default for ResourceAvailability {
            available_cores: 8,
            available_memory_bytes: 16 * 1024 * 1024 * 1024, // 16GB
            hardware_acceleration: vec!["AES-NI".to_string()],
            simd_support: vec!["SSE4.2".to_string(), "AVX2".to_string()],
impl Default for OptimizationPreferences {
            prioritize_latency: true,
            enable_genetic_optimization: true,
            use_hardware_acceleration: true,
            adaptive_selection: true,}

impl Default for EcosystemOptimizationResult {
            recommended_algorithm: "ChaCha20Poly1305".to_string(),
            optimized_parameters: vec![],
            expected_improvements: PerformanceImprovements::default(),
            contributing_modules: vec!["GeneticOptimizer".to_string()],
            confidence_score: 0.85,
impl Default for PerformanceImprovements {
            latency_reduction_percent: 15.0,
            throughput_increase_percent: 25.0,
            cpu_reduction_percent: 10.0,
            memory_reduction_percent: 5.0,
