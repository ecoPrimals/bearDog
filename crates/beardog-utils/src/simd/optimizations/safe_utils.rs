//! Safe utility functions for parallel processing

use beardog_errors::BearDogError;

/// Safe parallel sum with overflow protection
#[must_use]
pub fn safe_parallel_sum(input_slice: &[u64]) -> u64 {
    input_slice
        .iter()
        .fold(0u64, |acc, &value| acc.saturating_add(value))
}

/// Safe parallel maximum
#[must_use]
pub fn safe_parallel_max(input_slice: &[u64]) -> Option<u64> {
    input_slice.iter().max().copied()
}

/// Safe parallel minimum
#[must_use]
pub fn safe_parallel_min(input_slice: &[u64]) -> Option<u64> {
    input_slice.iter().min().copied()
}

/// Safe element-wise addition with overflow protection
pub fn safe_elementwise_add(a: &[u64], b: &[u64]) -> Result<Vec<u64>, BearDogError> {
    if a.len() != b.len() {
        return Err(BearDogError::validation("Array lengths must match"));
    }

    let result: Result<Vec<_>, _> = a
        .iter()
        .zip(b.iter())
        .map(|(&left, &right)| {
            left.checked_add(right)
                .ok_or_else(|| BearDogError::validation("Addition overflow"))
        })
        .collect();

    result
}

/// Safe pattern matching in byte arrays
#[must_use]
pub fn safe_pattern_match(input_buffer: &[u8], pattern: &[u8]) -> Vec<usize> {
    if pattern.is_empty() {
        return Vec::new();
    }

    input_buffer
        .windows(pattern.len())
        .enumerate()
        .filter_map(|(i, window)| if window == pattern { Some(i) } else { None })
        .collect()
}

