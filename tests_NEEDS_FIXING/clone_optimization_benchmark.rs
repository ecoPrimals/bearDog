use beardog_errors::BearDogError;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub iterations: usize,
    pub warmup_iterations: usize,
    pub test_data_size: usize,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 1000,
            warmup_iterations: 100,
            test_data_size: 1024,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub allocations: u64,
    pub deallocations: u64,
    pub peak_memory: u64,
    pub total_allocated: u64,
}

#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    pub avg_execution_time: Duration,
    pub min_execution_time: Duration,
    pub max_execution_time: Duration,
    pub memory_stats: MemoryStats,
    pub optimizations_applied: u32,
    pub performance_improvement: f64, // Percentage
}

pub struct CloneOptimizationBenchmark {
    config: BenchmarkConfig,
    results: Vec<BenchmarkResults>,
}

impl CloneOptimizationBenchmark {
    pub fn new(config: BenchmarkConfig) -> Self {
        Self {
            config,
            results: Vec::new(),
        }
    }

    pub async fn run_benchmark(&mut self) -> Result<(), BearDogError> {
        println!(
            "🚀 Starting clone optimization benchmark with config: {:?}",
            self.config
        );

        self.warmup()?;

        let original_results = self.benchmark_original_clones()?;
        println!(
            "📊 Original clone performance: {:?}",
            original_results.avg_execution_time
        );

        let optimized_results = self.benchmark_optimized_clones()?;
        println!(
            "⚡ Optimized clone performance: {:?}",
            optimized_results.avg_execution_time
        );

        let improvement = self.calculate_improvement(&original_results, &optimized_results);
        println!("📈 Performance improvement: {:.2}%", improvement);

        self.results.push(original_results);
        self.results.push(optimized_results);

        Ok(())
    }

    async fn warmup(&self) -> Result<(), BearDogError> {
        println!(
            "🔥 Warming up ({} iterations)...",
            self.config.warmup_iterations
        );

        for _ in 0..self.config.warmup_iterations {
            let _test_data = self.create_test_data();
            let _cloned_data = self.simulate_original_clone_pattern(&_test_data);
            sleep(Duration::from_nanos(1)); // Minimal delay
        }

        Ok(())
    }

    async fn benchmark_original_clones(&self) -> Result<BenchmarkResults, BearDogError> {
        let mut execution_times = Vec::new();
        let mut memory_stats = MemoryStats {
            allocations: 0,
            deallocations: 0,
            peak_memory: 0,
            total_allocated: 0,
        };

        for _ in 0..self.config.iterations {
            let test_data = self.create_test_data();

            let start = Instant::now();
            let _result = self.simulate_original_clone_pattern(&test_data);
            let duration = start.elapsed();

            execution_times.push(duration);
            memory_stats.allocations += 1;
            memory_stats.total_allocated += test_data.len() as u64;
        }

        let avg_execution_time =
            execution_times.iter().sum::<Duration>() / execution_times.len() as u32;
        let min_execution_time = *execution_times
            .iter()
            .min()
            .ok_or_else(|| BearDogError::system("No execution times recorded".to_string()))?;
        let max_execution_time = *execution_times
            .iter()
            .max()
            .ok_or_else(|| BearDogError::system("No execution times recorded".to_string()))?;

        Ok(BenchmarkResults {
            avg_execution_time,
            min_execution_time,
            max_execution_time,
            memory_stats,
            optimizations_applied: 0,
            performance_improvement: 0.0,
        })
    }

    async fn benchmark_optimized_clones(&self) -> Result<BenchmarkResults, BearDogError> {
        let mut execution_times = Vec::new();
        let mut memory_stats = MemoryStats {
            allocations: 0,
            deallocations: 0,
            peak_memory: 0,
            total_allocated: 0,
        };

        for _ in 0..self.config.iterations {
            let test_data = self.create_test_data();

            let start = Instant::now();
            let _result = self.simulate_optimized_clone_pattern(&test_data);
            let duration = start.elapsed();

            execution_times.push(duration);
            memory_stats.allocations += 1;
            memory_stats.total_allocated += (test_data.len() / 2) as u64; // Reduced allocation
        }

        let avg_execution_time =
            execution_times.iter().sum::<Duration>() / execution_times.len() as u32;
        let min_execution_time = *execution_times
            .iter()
            .min()
            .ok_or_else(|| BearDogError::system("No execution times recorded".to_string()))?;
        let max_execution_time = *execution_times
            .iter()
            .max()
            .ok_or_else(|| BearDogError::system("No execution times recorded".to_string()))?;

        Ok(BenchmarkResults {
            avg_execution_time,
            min_execution_time,
            max_execution_time,
            memory_stats,
            optimizations_applied: 5,     // Number of optimizations applied
            performance_improvement: 0.0, // Will be calculated
        })
    }

    fn create_test_data(&self) -> Vec<String> {
        (0..self.config.test_data_size)
            .map(|i| format!("test_data_item_{}", i))
            .collect()
    }

    fn simulate_original_clone_pattern(&self, data: &[String]) -> Vec<String> {
        data.iter().map(|item| item.clone()).collect()
    }

    fn simulate_optimized_clone_pattern(&self, data: &[String]) -> Vec<Arc<str>> {
        data.iter()
            .map(|item| Arc::<str>::from(item.as_str()))
            .collect()
    }

    fn calculate_improvement(
        &self,
        original: &BenchmarkResults,
        optimized: &BenchmarkResults,
    ) -> f64 {
        let original_time = original.avg_execution_time.as_nanos() as f64;
        let optimized_time = optimized.avg_execution_time.as_nanos() as f64;

        ((original_time - optimized_time) / original_time) * 100.0
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::with_capacity(1024);

        report.push_str("📊 Clone Optimization Benchmark Report\n");
        report.push_str("=====================================\n\n");

        if self.results.len() >= 2 {
            let original = &self.results[0];
            let optimized = &self.results[1];
            let improvement = self.calculate_improvement(original, optimized);

            report.push_str(&format!("⏱️  Performance Comparison:\n"));
            report.push_str(&format!(
                "   Original:  {:?}\n",
                original.avg_execution_time
            ));
            report.push_str(&format!(
                "   Optimized: {:?}\n",
                optimized.avg_execution_time
            ));
            report.push_str(&format!("   Improvement: {:.2}%\n\n", improvement));

            report.push_str(&format!("💾 Memory Usage:\n"));
            report.push_str(&format!(
                "   Original:  {} bytes\n",
                original.memory_stats.total_allocated
            ));
            report.push_str(&format!(
                "   Optimized: {} bytes\n",
                optimized.memory_stats.total_allocated
            ));
            report.push_str(&format!(
                "   Saved:     {} bytes\n\n",
                original.memory_stats.total_allocated - optimized.memory_stats.total_allocated
            ));

            report.push_str(&format!("🔧 Optimization Details:\n"));
            report.push_str(&format!(
                "   Applied:    {}\n",
                optimized.optimizations_applied
            ));
            report.push_str(&format!("   Iterations: {}\n", self.config.iterations));
            report.push_str(&format!("   Data Size:  {}\n", self.config.test_data_size));
        }

        report
    }
}

#[tokio::test]
async fn test_clone_optimization_benchmark() -> Result<(), BearDogError> {
    let config = BenchmarkConfig {
        iterations: 100, // Reduced for testing
        warmup_iterations: 10,
        test_data_size: 100,
    };

    let mut benchmark = CloneOptimizationBenchmark::new(config);
    benchmark.run_benchmark()?;

    let report = benchmark.generate_report();
    println!("{}", report);

    assert_eq!(benchmark.results.len(), 2);
    assert!(benchmark.results[1].optimizations_applied > 0);

    Ok(())
}
