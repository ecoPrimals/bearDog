// SPDX-License-Identifier: AGPL-3.0-only

//! Production entropy tests — validates that multi-source collection
//! produces genuinely random output.

use crate::universal_hsm::entropy::collector::EntropyCollector;
use beardog_errors::BearDogError;

#[tokio::test]
async fn test_collect_is_not_zeros() -> Result<(), BearDogError> {
    let collector = EntropyCollector::new();
    let entropy = collector.collect(32).await?;

    assert!(
        !entropy.iter().all(|&b| b == 0),
        "Entropy collection must not return all zeros"
    );

    Ok(())
}

#[tokio::test]
async fn test_collect_uniqueness() -> Result<(), BearDogError> {
    let collector = EntropyCollector::new();

    let entropy1 = collector.collect(32).await?;
    let entropy2 = collector.collect(32).await?;

    assert_ne!(
        entropy1, entropy2,
        "Consecutive entropy samples must be unique"
    );

    Ok(())
}

#[tokio::test]
async fn test_collect_size_correct() -> Result<(), BearDogError> {
    let collector = EntropyCollector::new();

    for size in [16, 32, 64, 256] {
        let entropy = collector.collect(size).await?;
        assert_eq!(entropy.len(), size, "Entropy size should match requested");
    }

    Ok(())
}

#[tokio::test]
async fn test_quality_report_high_threshold() -> Result<(), BearDogError> {
    let collector = EntropyCollector::new();

    let entropy = collector.collect(1024).await?;
    let report = collector.assess_quality(&entropy);

    assert!(
        report.overall_quality >= 0.9,
        "Real entropy quality should be >= 0.9, got {}",
        report.overall_quality
    );

    Ok(())
}

#[tokio::test]
async fn test_quality_report_large_sample() -> Result<(), BearDogError> {
    let collector = EntropyCollector::new();

    let entropy = collector.collect(4096).await?;
    let report = collector.assess_quality(&entropy);

    assert!(
        report.overall_quality >= 0.95,
        "Large sample quality should be >= 0.95, got {}",
        report.overall_quality
    );
    assert!(
        report.passes_chi_square,
        "Chi-square test failed: {}",
        report.chi_square_statistic
    );

    Ok(())
}

#[tokio::test]
async fn test_quality_report_rejects_zeros() {
    let collector = EntropyCollector::new();
    let report = collector.assess_quality(&vec![0u8; 256]);

    assert!(
        report.overall_quality < 0.5,
        "All zeros should have low quality, got {}",
        report.overall_quality
    );
}

#[tokio::test]
async fn test_entropy_distribution_balanced() -> Result<(), BearDogError> {
    let collector = EntropyCollector::new();
    let entropy = collector.collect(2048).await?;

    let mut frequency = [0u32; 256];
    for &byte in &entropy {
        frequency[byte as usize] += 1;
    }

    let unique_bytes = frequency.iter().filter(|&&count| count > 0).count();
    assert!(
        unique_bytes > 200,
        "Entropy should have diverse bytes, got {unique_bytes} unique"
    );

    Ok(())
}

#[tokio::test]
async fn test_concurrent_entropy_collection() -> Result<(), BearDogError> {
    let collector = EntropyCollector::new();

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let c = collector.clone();
            tokio::spawn(async move { c.collect(32).await })
        })
        .collect();

    let mut results = Vec::new();
    for handle in handles {
        let entropy = handle.await.expect("join")?;
        results.push(entropy);
    }

    for i in 0..results.len() {
        for j in i + 1..results.len() {
            assert_ne!(
                results[i], results[j],
                "Concurrent entropy outputs must be unique"
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_entropy_minimal_size() -> Result<(), BearDogError> {
    let collector = EntropyCollector::new();

    let entropy = collector.collect(1).await?;
    assert_eq!(entropy.len(), 1);

    let mut has_nonzero = false;
    for _ in 0..10 {
        let single = collector.collect(1).await?;
        if single[0] != 0 {
            has_nonzero = true;
            break;
        }
    }
    assert!(
        has_nonzero,
        "Entropy collection should produce non-zero bytes"
    );

    Ok(())
}
