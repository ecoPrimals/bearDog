//! Performance Benchmarking Infrastructure for `BearDog`
//!
//! Provides infrastructure for tracking performance metrics,
//! detecting regressions, and generating performance reports.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Warmup iterations
    pub warmup_iterations: usize,
    /// Measurement iterations
    pub measurement_iterations: usize,
    /// Sample size
    pub sample_size: usize,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            warmup_iterations: 100,
            measurement_iterations: 1000,
            sample_size: 100,
        }
    }
}

/// Benchmark statistics
#[derive(Debug, Clone)]
pub struct BenchmarkStats {
    /// Benchmark name
    pub name: String,
    /// Mean duration
    pub mean: Duration,
    /// Median duration
    pub median: Duration,
    /// Standard deviation
    pub std_dev: Duration,
    /// Minimum duration
    pub min: Duration,
    /// Maximum duration
    pub max: Duration,
    /// Throughput (operations per second)
    pub throughput: f64,
}

impl BenchmarkStats {
    /// Create from samples
    #[must_use]
    pub fn from_samples(name: String, samples: &[Duration]) -> Self {
        let mut sorted = samples.to_vec();
        sorted.sort();

        let mean = Self::calculate_mean(samples);
        let median = sorted[sorted.len() / 2];
        let std_dev = Self::calculate_std_dev(samples, mean);
        let min = *sorted.first().unwrap();
        let max = *sorted.last().unwrap();
        let throughput = 1_000_000_000.0 / mean.as_nanos() as f64;

        Self {
            name,
            mean,
            median,
            std_dev,
            min,
            max,
            throughput,
        }
    }

    fn calculate_mean(samples: &[Duration]) -> Duration {
        let total: u128 = samples.iter().map(std::time::Duration::as_nanos).sum();
        Duration::from_nanos((total / samples.len() as u128) as u64)
    }

    fn calculate_std_dev(samples: &[Duration], mean: Duration) -> Duration {
        let mean_nanos = mean.as_nanos() as f64;
        let variance: f64 = samples
            .iter()
            .map(|d| {
                let diff = d.as_nanos() as f64 - mean_nanos;
                diff * diff
            })
            .sum::<f64>()
            / samples.len() as f64;

        Duration::from_nanos(variance.sqrt() as u64)
    }

    /// Format for display
    #[must_use]
    pub fn format_duration(d: Duration) -> String {
        let nanos = d.as_nanos();
        if nanos < 1_000 {
            format!("{nanos}ns")
        } else if nanos < 1_000_000 {
            format!("{:.2}µs", nanos as f64 / 1_000.0)
        } else if nanos < 1_000_000_000 {
            format!("{:.2}ms", nanos as f64 / 1_000_000.0)
        } else {
            format!("{:.2}s", nanos as f64 / 1_000_000_000.0)
        }
    }
}

/// Benchmark result
#[derive(Debug)]
pub struct BenchmarkResult {
    /// Statistics
    pub stats: BenchmarkStats,
    /// Comparison to baseline (if available)
    pub comparison: Option<f64>,
}

/// Benchmark suite
pub struct BenchmarkSuite {
    benchmarks: HashMap<String, Box<dyn Fn() + Send + Sync>>,
    config: BenchmarkConfig,
    baselines: HashMap<String, BenchmarkStats>,
}

impl BenchmarkSuite {
    #[must_use]
    pub fn new(config: BenchmarkConfig) -> Self {
        Self {
            benchmarks: HashMap::new(),
            config,
            baselines: HashMap::new(),
        }
    }

    pub fn add_benchmark<F>(&mut self, name: &str, f: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.benchmarks.insert(name.to_string(), Box::new(f));
    }

    pub fn set_baseline(&mut self, name: &str, stats: BenchmarkStats) {
        self.baselines.insert(name.to_string(), stats);
    }

    #[must_use]
    pub fn run_benchmark(&self, name: &str) -> Option<BenchmarkResult> {
        let benchmark = self.benchmarks.get(name)?;

        // Warmup
        for _ in 0..self.config.warmup_iterations {
            benchmark();
        }

        // Measurement
        let mut samples = Vec::with_capacity(self.config.sample_size);
        for _ in 0..self.config.sample_size {
            let start = Instant::now();
            for _ in 0..self.config.measurement_iterations {
                benchmark();
            }
            let elapsed = start.elapsed() / self.config.measurement_iterations as u32;
            samples.push(elapsed);
        }

        let stats = BenchmarkStats::from_samples(name.to_string(), &samples);

        // Compare to baseline if available
        let comparison = self.baselines.get(name).map(|baseline| {
            let current = stats.mean.as_nanos() as f64;
            let base = baseline.mean.as_nanos() as f64;
            ((current - base) / base) * 100.0
        });

        Some(BenchmarkResult { stats, comparison })
    }

    #[must_use]
    pub fn run_all(&self) -> Vec<BenchmarkResult> {
        let mut results = Vec::new();
        for name in self.benchmarks.keys() {
            if let Some(result) = self.run_benchmark(name) {
                results.push(result);
            }
        }
        results
    }

    pub fn print_results(&self, results: &[BenchmarkResult]) {
        println!("\n========== Benchmark Results ==========\n");

        for result in results {
            println!("📊 {}", result.stats.name);
            println!(
                "   Mean:       {}",
                BenchmarkStats::format_duration(result.stats.mean)
            );
            println!(
                "   Median:     {}",
                BenchmarkStats::format_duration(result.stats.median)
            );
            println!(
                "   Std Dev:    {}",
                BenchmarkStats::format_duration(result.stats.std_dev)
            );
            println!(
                "   Min:        {}",
                BenchmarkStats::format_duration(result.stats.min)
            );
            println!(
                "   Max:        {}",
                BenchmarkStats::format_duration(result.stats.max)
            );
            println!("   Throughput: {:.2} ops/sec", result.stats.throughput);

            if let Some(comparison) = result.comparison {
                let symbol = if comparison > 0.0 { "📈" } else { "📉" };
                let status = if comparison.abs() < 5.0 {
                    "✅ No significant change"
                } else if comparison < 0.0 {
                    "✅ Improvement"
                } else {
                    "⚠️ Regression"
                };
                println!("   {symbol} Comparison: {comparison:+.2}% ({status})");
            }
            println!();
        }
    }
}

/// Simple benchmarks for common operations
pub mod benchmarks {
    use super::*;

    pub fn string_allocation() {
        let _s = "benchmark_test_string".to_string();
    }

    pub fn arc_clone() {
        use std::sync::Arc;
        let arc = Arc::new(vec![1, 2, 3, 4, 5]);
        let _clone = Arc::clone(&arc);
    }

    pub fn hash_map_insert() {
        let mut map = HashMap::new();
        map.insert("key", "value");
    }

    pub fn vec_push() {
        let _vec = [42];
    }

    pub fn json_parse() {
        let _: Result<serde_json::Value, _> = serde_json::from_str(r#"{"key": "value"}"#);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_config_default() {
        let config = BenchmarkConfig::default();
        assert_eq!(config.warmup_iterations, 100);
        assert_eq!(config.measurement_iterations, 1000);
        assert_eq!(config.sample_size, 100);
    }

    #[test]
    fn test_benchmark_stats_from_samples() {
        let samples = vec![
            Duration::from_nanos(100),
            Duration::from_nanos(200),
            Duration::from_nanos(300),
        ];

        let stats = BenchmarkStats::from_samples("test".to_string(), &samples);

        assert_eq!(stats.name, "test");
        assert_eq!(stats.median, Duration::from_nanos(200));
        assert_eq!(stats.min, Duration::from_nanos(100));
        assert_eq!(stats.max, Duration::from_nanos(300));
    }

    #[test]
    fn test_benchmark_stats_format() {
        assert!(BenchmarkStats::format_duration(Duration::from_nanos(500)).contains("ns"));
        assert!(BenchmarkStats::format_duration(Duration::from_micros(500)).contains("µs"));
        assert!(BenchmarkStats::format_duration(Duration::from_millis(500)).contains("ms"));
        assert!(BenchmarkStats::format_duration(Duration::from_secs(2)).contains('s'));
    }

    #[test]
    fn test_benchmark_suite() {
        let config = BenchmarkConfig {
            warmup_iterations: 10,
            measurement_iterations: 50,
            sample_size: 10,
        };

        let mut suite = BenchmarkSuite::new(config);
        suite.add_benchmark("test_bench", || {
            let _x = 1 + 1;
        });

        let result = suite.run_benchmark("test_bench");
        assert!(result.is_some());

        let result = result.unwrap();
        assert_eq!(result.stats.name, "test_bench");
    }

    #[test]
    fn test_benchmark_baseline_comparison() {
        let config = BenchmarkConfig {
            warmup_iterations: 5,
            measurement_iterations: 20,
            sample_size: 5,
        };

        let mut suite = BenchmarkSuite::new(config);
        suite.add_benchmark("baseline_test", || {
            let _x = 1 + 1;
        });

        // Set a baseline
        let baseline = BenchmarkStats {
            name: "baseline_test".to_string(),
            mean: Duration::from_nanos(100),
            median: Duration::from_nanos(100),
            std_dev: Duration::from_nanos(10),
            min: Duration::from_nanos(90),
            max: Duration::from_nanos(110),
            throughput: 10_000_000.0,
        };
        suite.set_baseline("baseline_test", baseline);

        let result = suite.run_benchmark("baseline_test");
        assert!(result.is_some());

        let result = result.unwrap();
        assert!(result.comparison.is_some());
    }

    #[test]
    fn test_run_all_benchmarks() {
        let config = BenchmarkConfig {
            warmup_iterations: 5,
            measurement_iterations: 20,
            sample_size: 5,
        };

        let mut suite = BenchmarkSuite::new(config);
        suite.add_benchmark("bench1", || {});
        suite.add_benchmark("bench2", || {});

        let results = suite.run_all();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_builtin_benchmarks() {
        // Just verify they don't panic
        benchmarks::string_allocation();
        benchmarks::arc_clone();
        benchmarks::hash_map_insert();
        benchmarks::vec_push();
        benchmarks::json_parse();
    }
}
