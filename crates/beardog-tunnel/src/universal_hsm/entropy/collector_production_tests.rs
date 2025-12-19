// Strategic Test Coverage - EntropyCollector Real Multi-Source Entropy
//
// Tests for the newly evolved real entropy collection

use crate::universal_hsm::entropy::collector::EntropyCollector;
use crate::universal_hsm::entropy::config::EntropyConfig;
use beardog_errors::BearDogError;

#[tokio::test]
async fn test_collect_is_not_zeros() -> Result<(), BearDogError> {
    let config = EntropyConfig::default();
    let collector = EntropyCollector::new(config);

    let entropy = collector.collect(32).await?;

    // Verify not all zeros (would indicate mock)
    let all_zeros = entropy.iter().all(|&b| b == 0);
    assert!(
        !all_zeros,
        "Entropy collection must not return all zeros (mock indicator)"
    );

    Ok(())
}

#[tokio::test]
async fn test_collect_uniqueness() -> Result<(), BearDogError> {
    let config = EntropyConfig::default();
    let collector = EntropyCollector::new(config);

    // Collect two samples
    let entropy1 = collector.collect(32).await?;
    let entropy2 = collector.collect(32).await?;

    // They should be different (multi-source with timing)
    assert_ne!(
        entropy1, entropy2,
        "Consecutive entropy samples must be unique"
    );

    Ok(())
}

#[tokio::test]
async fn test_collect_size_correct() -> Result<(), BearDogError> {
    let config = EntropyConfig::default();
    let collector = EntropyCollector::new(config);

    // Test different sizes
    for size in [16, 32, 64, 256] {
        let entropy = collector.collect(size).await?;
        assert_eq!(
            entropy.len(),
            size,
            "Entropy size should match requested size"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_get_quality_high_threshold() -> Result<(), BearDogError> {
    let config = EntropyConfig::default();
    let collector = EntropyCollector::new(config);

    // Collect larger sample for quality assessment
    let entropy = collector.collect(1024).await?;
    let quality = collector.get_quality(&entropy).await?;

    // Real entropy should have high quality (>= 0.9)
    assert!(
        quality >= 0.9,
        "Real entropy quality should be >= 0.9, got {quality}"
    );

    Ok(())
}

#[tokio::test]
async fn test_get_quality_perfect_randomness() -> Result<(), BearDogError> {
    let config = EntropyConfig::default();
    let collector = EntropyCollector::new(config);

    // Collect large sample for statistical testing
    let entropy = collector.collect(4096).await?;
    let quality = collector.get_quality(&entropy).await?;

    // Large samples should show excellent quality (>= 0.95)
    assert!(
        quality >= 0.95,
        "Large entropy samples should have quality >= 0.95, got {quality}"
    );

    Ok(())
}

#[tokio::test]
async fn test_get_quality_rejects_zeros() -> Result<(), BearDogError> {
    let config = EntropyConfig::default();
    let collector = EntropyCollector::new(config);

    // Test with all zeros (mock indicator)
    let zeros = vec![0u8; 256];
    let quality = collector.get_quality(&zeros).await?;

    // All zeros should have very low quality
    assert!(
        quality < 0.5,
        "All zeros should have low quality (<0.5), got {quality}"
    );

    Ok(())
}

#[tokio::test]
async fn test_get_quality_rejects_repeated_pattern() -> Result<(), BearDogError> {
    let config = EntropyConfig::default();
    let collector = EntropyCollector::new(config);

    // Test with repeated pattern (bad entropy)
    let pattern = vec![0xAA, 0xBB, 0xCC, 0xDD]
        .repeat(64)
        .into_iter()
        .collect::<Vec<u8>>();
    let quality = collector.get_quality(&pattern).await?;

    // Repeated patterns should have low quality
    assert!(
        quality < 0.7,
        "Repeated pattern should have low quality (<0.7), got {quality}"
    );

    Ok(())
}

#[tokio::test]
async fn test_entropy_distribution_balanced() -> Result<(), BearDogError> {
    let config = EntropyConfig::default();
    let collector = EntropyCollector::new(config);

    // Collect large sample
    let entropy = collector.collect(2048).await?;

    // Calculate byte frequency
    let mut frequency = [0u32; 256];
    for &byte in &entropy {
        frequency[byte as usize] += 1;
    }

    // Count unique bytes
    let unique_bytes = frequency.iter().filter(|&&count| count > 0).count();

    // Good entropy should have diverse distribution
    assert!(
        unique_bytes > 200,
        "Entropy should have diverse byte distribution, got {unique_bytes} unique bytes"
    );

    Ok(())
}

#[tokio::test]
async fn test_entropy_chi_square_passes() -> Result<(), BearDogError> {
    let config = EntropyConfig::default();
    let collector = EntropyCollector::new(config);

    // Collect large sample for chi-square test
    let entropy = collector.collect(4096).await?;

    // Calculate byte frequency
    let mut frequency = [0u32; 256];
    for &byte in &entropy {
        frequency[byte as usize] += 1;
    }

    // Chi-square test
    let expected = entropy.len() as f64 / 256.0;
    let chi_square: f64 = frequency
        .iter()
        .map(|&count| {
            let diff = count as f64 - expected;
            (diff * diff) / expected
        })
        .sum();

    // For 255 degrees of freedom at p=0.05, critical value ≈ 293.25
    // For random data, chi-square should be much lower
    assert!(
        chi_square < 350.0,
        "Chi-square test should pass, got {chi_square}"
    );

    Ok(())
}

#[tokio::test]
async fn test_concurrent_entropy_collection() -> Result<(), BearDogError> {
    let config = EntropyConfig::default();
    let collector = EntropyCollector::new(config);

    // Collect entropy concurrently
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let c = collector.clone();
            tokio::spawn(async move { c.collect(32).await })
        })
        .collect();

    // Collect all results
    let mut results = Vec::new();
    for handle in handles {
        let entropy = handle.await.unwrap()?;
        results.push(entropy);
    }

    // All should be unique
    for i in 0..results.len() {
        for j in i + 1..results.len() {
            assert_ne!(
                results[i], results[j],
                "Concurrent entropy collection must produce unique outputs"
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_entropy_timing_variance() -> Result<(), BearDogError> {
    let config = EntropyConfig::default();
    let collector = EntropyCollector::new(config);

    // Collect multiple samples
    let mut samples = Vec::new();
    for _ in 0..5 {
        let entropy = collector.collect(32).await?;
        samples.push(entropy);
    }

    // Each sample should be unique (timing entropy contributes)
    for i in 0..samples.len() {
        for j in i + 1..samples.len() {
            assert_ne!(
                samples[i], samples[j],
                "Timing entropy should make each sample unique"
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_entropy_minimal_size() -> Result<(), BearDogError> {
    let config = EntropyConfig::default();
    let collector = EntropyCollector::new(config);

    // Test with minimal size
    let entropy = collector.collect(1).await?;
    assert_eq!(entropy.len(), 1);

    // Even single byte should not be zero (high probability)
    // Run multiple times to verify
    let mut has_nonzero = false;
    for _ in 0..10 {
        let single_byte = collector.collect(1).await?;
        if single_byte[0] != 0 {
            has_nonzero = true;
            break;
        }
    }
    assert!(has_nonzero, "Entropy collection should produce non-zero bytes");

    Ok(())
}

