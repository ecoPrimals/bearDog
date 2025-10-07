use beardog_errors::BearDogError;
use std::collections::HashMap;
use tracing::{info, warn};

/// Boundary testing for mathematical certainty in BearDog operations
pub struct BoundaryTester {
    tolerance: f64,
    max_iterations: usize,
}

impl Default for BoundaryTester {
    fn default() -> Self {
        Self {
            tolerance: 1e-10, // High precision for mathematical certainty
            max_iterations: 10000,
        }
    }
}

impl BoundaryTester {
    pub fn new(tolerance: f64, max_iterations: usize) -> Self {
        Self {
            tolerance,
            max_iterations,
        }
    }

    /// Test cryptographic key size boundaries
    pub fn test_key_size_boundaries(&self, key_sizes: &[usize]) -> Result<HashMap<usize, bool>, BearDogError> {
        info!("Testing key size boundaries for cryptographic operations");
        
        let mut results = HashMap::new();
        
        for &key_size in key_sizes {
            let is_valid = match key_size {
                // RSA key sizes
                1024 | 2048 | 3072 | 4096 => true,
                // AES key sizes (in bits)
                128 | 192 | 256 => true,
                // ECC key sizes
                224 | 256 | 384 | 521 => true,
                _ => {
                    warn!("Invalid key size: {}", key_size);
                    false
                }
            };
            
            results.insert(key_size, is_valid);
        }
        
        info!("Key size boundary test completed: {}/{} valid", 
              results.values().filter(|&&v| v).count(), 
              results.len());
        
        Ok(results)
    }

    /// Test numerical precision boundaries
    pub fn test_precision_boundaries(&self, values: &[f64]) -> Result<Vec<bool>, BearDogError> {
        info!("Testing numerical precision boundaries");
        
        let mut results = Vec::new();
        
        for &value in values {
            let is_within_bounds = self.check_precision_bounds(value)?;
            results.push(is_within_bounds);
        }
        
        info!("Precision boundary test completed: {}/{} within bounds",
              results.iter().filter(|&&v| v).count(),
              results.len());
        
        Ok(results)
    }

    /// Test memory allocation boundaries
    pub fn test_memory_boundaries(&self, allocation_sizes: &[usize]) -> Result<HashMap<usize, bool>, BearDogError> {
        info!("Testing memory allocation boundaries");
        
        let mut results = HashMap::new();
        let max_safe_allocation = 1024 * 1024 * 1024; // 1GB limit for safety
        
        for &size in allocation_sizes {
            let is_safe = size <= max_safe_allocation && size > 0;
            
            if !is_safe {
                warn!("Unsafe memory allocation size: {} bytes", size);
            }
            
            results.insert(size, is_safe);
        }
        
        info!("Memory boundary test completed: {}/{} safe allocations",
              results.values().filter(|&&v| v).count(),
              results.len());
        
        Ok(results)
    }

    /// Test timeout boundaries for operations
    pub fn test_timeout_boundaries(&self, timeouts_ms: &[u64]) -> Result<Vec<bool>, BearDogError> {
        info!("Testing timeout boundaries");
        
        let min_timeout = 100;  // 100ms minimum
        let max_timeout = 300_000; // 5 minutes maximum
        
        let results = timeouts_ms.iter().map(|&timeout| {
            let is_valid = timeout >= min_timeout && timeout <= max_timeout;
            if !is_valid {
                warn!("Invalid timeout: {}ms (must be between {}ms and {}ms)", 
                      timeout, min_timeout, max_timeout);
            }
            is_valid
        }).collect();
        
        Ok(results)
    }

    /// Test integer overflow boundaries
    pub fn test_overflow_boundaries(&self, values: &[u64]) -> Result<Vec<bool>, BearDogError> {
        info!("Testing integer overflow boundaries");
        
        let mut results = Vec::new();
        
        for &value in values {
            // Test for potential overflow in common operations
            let safe_for_addition = value <= u64::MAX / 2;
            let safe_for_multiplication = value <= (u64::MAX as f64).sqrt() as u64;
            
            let is_safe = safe_for_addition && safe_for_multiplication;
            
            if !is_safe {
                warn!("Value {} may cause overflow in arithmetic operations", value);
            }
            
            results.push(is_safe);
        }
        
        info!("Overflow boundary test completed: {}/{} safe values",
              results.iter().filter(|&&v| v).count(),
              results.len());
        
        Ok(results)
    }

    fn check_precision_bounds(&self, value: f64) -> Result<bool, BearDogError> {
        if value.is_nan() {
            warn!("NaN value detected");
            return Ok(false);
        }
        
        if value.is_infinite() {
            warn!("Infinite value detected");
            return Ok(false);
        }
        
        // Check if value is within representable range
        if value.abs() < f64::MIN_POSITIVE && value != 0.0 {
            warn!("Value {} is below minimum representable positive value", value);
            return Ok(false);
        }
        
        if value.abs() > f64::MAX {
            warn!("Value {} exceeds maximum representable value", value);
            return Ok(false);
        }
        
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_size_boundaries() {
        let tester = BoundaryTester::default();
        let key_sizes = vec![1024, 2048, 3000, 4096]; // 3000 should be invalid
        
        let results = tester.test_key_size_boundaries(&key_sizes).unwrap();
        
        assert_eq!(results[&1024], true);
        assert_eq!(results[&2048], true);
        assert_eq!(results[&3000], false); // Invalid size
        assert_eq!(results[&4096], true);
    }

    #[test]
    fn test_precision_boundaries() {
        let tester = BoundaryTester::default();
        let values = vec![1.0, f64::NAN, f64::INFINITY, 0.0];
        
        let results = tester.test_precision_boundaries(&values).unwrap();
        
        assert_eq!(results[0], true);  // 1.0 is valid
        assert_eq!(results[1], false); // NaN is invalid
        assert_eq!(results[2], false); // Infinity is invalid
        assert_eq!(results[3], true);  // 0.0 is valid
    }

    #[test]
    fn test_memory_boundaries() {
        let tester = BoundaryTester::default();
        let sizes = vec![1024, 1024 * 1024, 2 * 1024 * 1024 * 1024]; // 1KB, 1MB, 2GB
        
        let results = tester.test_memory_boundaries(&sizes).unwrap();
        
        assert_eq!(results[&1024], true);
        assert_eq!(results[&(1024 * 1024)], true);
        assert_eq!(results[&(2 * 1024 * 1024 * 1024)], false); // Too large
    }

    #[test]
    fn test_timeout_boundaries() {
        let tester = BoundaryTester::default();
        let timeouts = vec![50, 1000, 60000, 400000]; // 50ms, 1s, 1min, 6.67min
        
        let results = tester.test_timeout_boundaries(&timeouts).unwrap();
        
        assert_eq!(results[0], false); // Too short
        assert_eq!(results[1], true);
        assert_eq!(results[2], true);
        assert_eq!(results[3], false); // Too long
    }

    #[test]
    fn test_overflow_boundaries() {
        let tester = BoundaryTester::default();
        let values = vec![1000, u64::MAX / 4, u64::MAX - 1];
        
        let results = tester.test_overflow_boundaries(&values).unwrap();
        
        assert_eq!(results[0], true);  // Small value is safe
        assert_eq!(results[1], true);  // Quarter max is safe
        assert_eq!(results[2], false); // Near max is unsafe
    }
}
