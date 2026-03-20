// SPDX-License-Identifier: AGPL-3.0-only
//! Fuzzing Test Framework for `BearDog`
//!
//! Implements property-based and fuzz testing infrastructure
//! to discover edge cases and security vulnerabilities through
//! automated random input generation.

#![cfg(test)]

/// Fuzzing configuration
#[derive(Debug, Clone)]
pub struct FuzzConfig {
    /// Number of iterations
    pub iterations: usize,
    /// Maximum input size
    pub max_input_size: usize,
    /// Random seed for reproducibility
    pub seed: Option<u64>,
    /// Timeout per iteration
    pub timeout_ms: u64,
}

impl Default for FuzzConfig {
    fn default() -> Self {
        Self {
            iterations: 1000,
            max_input_size: 4096,
            seed: None,
            timeout_ms: 100,
        }
    }
}

/// Fuzzing result
#[derive(Debug)]
pub struct FuzzResult {
    /// Total iterations run
    pub iterations_run: usize,
    /// Number of failures found
    pub failures: usize,
    /// Unique failure inputs (first 10)
    pub failure_inputs: Vec<Vec<u8>>,
    /// Coverage achieved (if measurable)
    pub coverage_percent: Option<f64>,
}

/// Fuzzing target trait
pub trait FuzzTarget {
    /// Target name
    fn name(&self) -> &str;

    /// Process input and return whether it was valid
    fn process(&mut self, input: &[u8]) -> Result<bool, String>;

    /// Reset state between iterations
    fn reset(&mut self);
}

/// String parsing fuzzer
pub struct StringParserFuzzer {
    max_length: usize,
}

impl StringParserFuzzer {
    #[must_use]
    pub fn new(max_length: usize) -> Self {
        Self { max_length }
    }
}

impl FuzzTarget for StringParserFuzzer {
    fn name(&self) -> &'static str {
        "String Parser"
    }

    fn process(&mut self, input: &[u8]) -> Result<bool, String> {
        // Validate input length
        if input.len() > self.max_length {
            return Ok(false);
        }

        // Try to parse as UTF-8
        match std::str::from_utf8(input) {
            Ok(s) => {
                // Validate string doesn't contain null bytes
                if s.contains('\0') {
                    return Err("Contains null byte".to_string());
                }
                Ok(true)
            }
            Err(_) => Ok(false), // Invalid UTF-8 is expected, not an error
        }
    }

    fn reset(&mut self) {
        // Stateless, nothing to reset
    }
}

/// JSON parsing fuzzer
pub struct JsonParserFuzzer;

impl JsonParserFuzzer {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonParserFuzzer {
    fn default() -> Self {
        Self::new()
    }
}

impl FuzzTarget for JsonParserFuzzer {
    fn name(&self) -> &'static str {
        "JSON Parser"
    }

    fn process(&mut self, input: &[u8]) -> Result<bool, String> {
        match serde_json::from_slice::<serde_json::Value>(input) {
            Ok(_) => Ok(true),
            Err(e) => {
                // Expected errors are OK
                if e.is_eof() || e.is_syntax() {
                    Ok(false)
                } else {
                    // Unexpected errors might indicate bugs
                    Err(format!("Unexpected error: {e}"))
                }
            }
        }
    }

    fn reset(&mut self) {
        // Stateless
    }
}

/// Configuration parser fuzzer
pub struct ConfigParserFuzzer;

impl ConfigParserFuzzer {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for ConfigParserFuzzer {
    fn default() -> Self {
        Self::new()
    }
}

impl FuzzTarget for ConfigParserFuzzer {
    fn name(&self) -> &'static str {
        "Config Parser"
    }

    fn process(&mut self, input: &[u8]) -> Result<bool, String> {
        // Try parsing as TOML (simplified - just check if valid UTF-8)
        match std::str::from_utf8(input) {
            Ok(_s) => {
                // In a real implementation, would parse TOML here
                // For now, just validate UTF-8
                Ok(true)
            }
            Err(_) => Ok(false), // Invalid UTF-8 expected
        }
    }

    fn reset(&mut self) {
        // Stateless
    }
}

/// Simple PRNG for fuzzing (not cryptographically secure)
struct SimplePrng {
    state: u64,
}

impl SimplePrng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        // LCG parameters (same as glibc)
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        self.state
    }

    fn next_bytes(&mut self, buf: &mut [u8]) {
        for byte in buf.iter_mut() {
            *byte = (self.next() & 0xFF) as u8;
        }
    }
}

/// Fuzzing engine
pub struct FuzzEngine {
    config: FuzzConfig,
    prng: SimplePrng,
}

impl FuzzEngine {
    #[must_use]
    pub fn new(config: FuzzConfig) -> Self {
        let seed = config.seed.unwrap_or_else(|| {
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });

        Self {
            config,
            prng: SimplePrng::new(seed),
        }
    }

    pub fn fuzz<T: FuzzTarget>(&mut self, target: &mut T) -> FuzzResult {
        let mut failures = 0;
        let mut failure_inputs = Vec::new();

        for i in 0..self.config.iterations {
            // Generate random input
            let input_size = (self.prng.next() as usize) % self.config.max_input_size;
            let mut input = vec![0u8; input_size];
            self.prng.next_bytes(&mut input);

            // Reset target state
            target.reset();

            // Process input
            if let Ok(_) = target.process(&input) {
                // Valid or expected failure
            } else {
                // Unexpected failure
                failures += 1;
                if failure_inputs.len() < 10 {
                    failure_inputs.push(input);
                }
            }

            // Progress indicator
            if (i + 1) % 100 == 0 {
                eprintln!("Fuzzing {}: {} iterations", target.name(), i + 1);
            }
        }

        FuzzResult {
            iterations_run: self.config.iterations,
            failures,
            failure_inputs,
            coverage_percent: None,
        }
    }

    pub fn fuzz_all(&mut self, targets: &mut [&mut dyn FuzzTarget]) -> Vec<FuzzResult> {
        targets
            .iter_mut()
            .map(|t| {
                let mut failures = 0;
                let mut failure_inputs = Vec::new();

                for i in 0..self.config.iterations {
                    let input_size = (self.prng.next() as usize) % self.config.max_input_size;
                    let mut input = vec![0u8; input_size];
                    self.prng.next_bytes(&mut input);

                    (*t).reset();

                    if let Ok(_) = (*t).process(&input) {
                    } else {
                        failures += 1;
                        if failure_inputs.len() < 10 {
                            failure_inputs.push(input);
                        }
                    }

                    if (i + 1) % 100 == 0 {
                        eprintln!("Fuzzing {}: {} iterations", (*t).name(), i + 1);
                    }
                }

                FuzzResult {
                    iterations_run: self.config.iterations,
                    failures,
                    failure_inputs,
                    coverage_percent: None,
                }
            })
            .collect()
    }

    pub fn print_results(&self, results: &[FuzzResult], target_names: &[&str]) {
        println!("\n========== Fuzzing Results ==========\n");

        for (result, name) in results.iter().zip(target_names.iter()) {
            let status = if result.failures == 0 {
                "✅ PASS"
            } else {
                "❌ FAIL"
            };

            println!("{status} {name}");
            println!("   Iterations: {}", result.iterations_run);
            println!("   Failures: {}", result.failures);

            if !result.failure_inputs.is_empty() {
                println!("   Failure inputs (first {}):", result.failure_inputs.len());
                for (i, input) in result.failure_inputs.iter().enumerate() {
                    println!("      {}: {} bytes", i + 1, input.len());
                }
            }

            if let Some(coverage) = result.coverage_percent {
                println!("   Coverage: {coverage:.2}%");
            }
            println!();
        }

        let total_iterations: usize = results.iter().map(|r| r.iterations_run).sum();
        let total_failures: usize = results.iter().map(|r| r.failures).sum();
        println!("Total iterations: {total_iterations} | Total failures: {total_failures}\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzz_config_default() {
        let config = FuzzConfig::default();
        assert_eq!(config.iterations, 1000);
        assert_eq!(config.max_input_size, 4096);
    }

    #[test]
    fn test_string_parser_fuzzer() {
        let config = FuzzConfig {
            iterations: 100,
            max_input_size: 256,
            seed: Some(42),
            timeout_ms: 100,
        };

        let mut engine = FuzzEngine::new(config);
        let mut target = StringParserFuzzer::new(256);

        let result = engine.fuzz(&mut target);
        assert_eq!(result.iterations_run, 100);
        // String parser should handle most inputs gracefully
    }

    #[test]
    fn test_json_parser_fuzzer() {
        let config = FuzzConfig {
            iterations: 50,
            max_input_size: 128,
            seed: Some(123),
            timeout_ms: 100,
        };

        let mut engine = FuzzEngine::new(config);
        let mut target = JsonParserFuzzer::new();

        let result = engine.fuzz(&mut target);
        assert_eq!(result.iterations_run, 50);
    }

    #[test]
    fn test_config_parser_fuzzer() {
        let config = FuzzConfig {
            iterations: 50,
            max_input_size: 128,
            seed: Some(456),
            timeout_ms: 100,
        };

        let mut engine = FuzzEngine::new(config);
        let mut target = ConfigParserFuzzer::new();

        let result = engine.fuzz(&mut target);
        assert_eq!(result.iterations_run, 50);
    }

    #[test]
    fn test_fuzz_engine_reproducibility() {
        let config1 = FuzzConfig {
            iterations: 10,
            max_input_size: 32,
            seed: Some(789),
            timeout_ms: 100,
        };

        let config2 = config1.clone();

        let mut engine1 = FuzzEngine::new(config1);
        let mut target1 = StringParserFuzzer::new(32);
        let result1 = engine1.fuzz(&mut target1);

        let mut engine2 = FuzzEngine::new(config2);
        let mut target2 = StringParserFuzzer::new(32);
        let result2 = engine2.fuzz(&mut target2);

        // Same seed should produce same results
        assert_eq!(result1.failures, result2.failures);
    }

    #[test]
    fn test_prng() {
        let mut prng = SimplePrng::new(12345);
        let val1 = prng.next();
        let val2 = prng.next();

        // Values should be different
        assert_ne!(val1, val2);
    }

    #[test]
    fn test_prng_bytes() {
        let mut prng = SimplePrng::new(67890);
        let mut buf = vec![0u8; 100];
        prng.next_bytes(&mut buf);

        // Should have generated non-zero bytes
        assert!(buf.iter().any(|&b| b != 0));
    }

    #[test]
    fn test_fuzz_all() {
        let config = FuzzConfig {
            iterations: 20,
            max_input_size: 64,
            seed: Some(999),
            timeout_ms: 100,
        };

        let mut engine = FuzzEngine::new(config);

        let mut target1 = StringParserFuzzer::new(64);
        let mut target2 = JsonParserFuzzer::new();

        let mut targets: Vec<&mut dyn FuzzTarget> = vec![&mut target1, &mut target2];
        let results = engine.fuzz_all(&mut targets);

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.iterations_run == 20));
    }

    #[test]
    fn test_fuzz_result_structure() {
        let result = FuzzResult {
            iterations_run: 1000,
            failures: 5,
            failure_inputs: vec![vec![0xFF; 10]],
            coverage_percent: Some(85.5),
        };

        assert_eq!(result.iterations_run, 1000);
        assert_eq!(result.failures, 5);
        assert_eq!(result.failure_inputs.len(), 1);
        assert_eq!(result.coverage_percent, Some(85.5));
    }
}
