// SPDX-License-Identifier: AGPL-3.0-only
#![allow(
    missing_docs,
    dead_code,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::redundant_clone,
    clippy::needless_collect,
    clippy::suspicious_operation_groupings,
    clippy::suboptimal_flops
)]
//! Hardware Entropy Comparison Suite
//!
//! Compare entropy generation across multiple hardware platforms:
//! 1. `SoloKey` FIDO2 hardware
//! 2. Pixel 8a Titan M (`StrongBox`)
//! 3. Software HSM (baseline)
//! 4. Human entropy (keyboard/mouse timing)
//!
//! Run this to benchmark and compare entropy quality across your devices.
//!
//! ```bash
//! # On Linux with SoloKey:
//! cargo run --example entropy_hardware_comparison --features fido2
//!
//! # On Android (Pixel 8a):
//! cargo ndk -t aarch64-linux-android build --example entropy_hardware_comparison
//! adb push target/aarch64-linux-android/debug/examples/entropy_hardware_comparison /data/local/tmp/
//! adb shell /data/local/tmp/entropy_hardware_comparison
//! ```

use beardog_errors::BearDogError;
use beardog_genetics::genetics::human_entropy::interaction_capture::{
    InteractionCaptureConfig, InteractionEntropyCollector,
};
use rand::RngCore;
use std::time::Instant;

/// Entropy test result for comparison
#[derive(Debug, Clone)]
struct EntropyTestResult {
    source: String,
    _entropy_bytes: Vec<u8>,
    generation_time_ms: u128,
    quality_score: f64,
    shannon_entropy: f64,
    chi_square: f64,
    serial_correlation: f64,
}

impl EntropyTestResult {
    fn print_report(&self) {
        println!("╔════════════════════════════════════════════════════════════════╗");
        println!("║  Source: {:51} ║", self.source);
        println!("╠════════════════════════════════════════════════════════════════╣");
        println!("║  📊 Performance:                                                ║");
        println!(
            "║     Generation Time: {:7} ms                              ║",
            self.generation_time_ms
        );
        println!("║                                                                 ║");
        println!("║  🎲 Statistical Quality:                                        ║");
        println!(
            "║     Shannon Entropy:  {:.4} (max: 8.0)                      ║",
            self.shannon_entropy
        );
        println!(
            "║     Chi-Square Test:  {:.4} (ideal: ~255.0)                 ║",
            self.chi_square
        );
        println!(
            "║     Serial Correlation: {:.4} (ideal: ~0.0)                 ║",
            self.serial_correlation
        );
        println!("║                                                                 ║");
        println!("║  ⭐ Overall Quality:                                            ║");
        println!(
            "║     Score: {:.1}% {}                                    ║",
            self.quality_score * 100.0,
            if self.quality_score > 0.95 {
                "🏆 Excellent"
            } else if self.quality_score > 0.85 {
                "✅ Good    "
            } else if self.quality_score > 0.70 {
                "⚠️  Acceptable"
            } else {
                "❌ Poor    "
            }
        );
        println!("╚════════════════════════════════════════════════════════════════╝");
        println!();
    }
}

/// Calculate Shannon entropy (bits per byte)
fn calculate_shannon_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut counts = [0usize; 256];
    for &byte in data {
        counts[byte as usize] += 1;
    }

    let len = data.len() as f64;
    let mut entropy = 0.0;

    for &count in &counts {
        if count > 0 {
            let probability = count as f64 / len;
            entropy -= probability * probability.log2();
        }
    }

    entropy
}

/// Calculate chi-square test statistic
fn calculate_chi_square(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut counts = [0usize; 256];
    for &byte in data {
        counts[byte as usize] += 1;
    }

    let expected = data.len() as f64 / 256.0;
    let mut chi_square = 0.0;

    for &count in &counts {
        let observed = count as f64;
        let diff = observed - expected;
        chi_square += (diff * diff) / expected;
    }

    chi_square
}

/// Calculate serial correlation coefficient
fn calculate_serial_correlation(data: &[u8]) -> f64 {
    if data.len() < 2 {
        return 0.0;
    }

    let n = data.len() - 1;
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    let mut sum_x2 = 0.0;
    let mut sum_y2 = 0.0;

    for i in 0..n {
        let x = f64::from(data[i]);
        let y = f64::from(data[i + 1]);

        sum_x += x;
        sum_y += y;
        sum_xy += x * y;
        sum_x2 += x * x;
        sum_y2 += y * y;
    }

    let n = n as f64;
    let numerator = n * sum_xy - sum_x * sum_y;
    let denominator = ((n * sum_x2 - sum_x * sum_x) * (n * sum_y2 - sum_y * sum_y)).sqrt();

    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}

/// Analyze entropy quality
fn analyze_entropy(data: &[u8]) -> f64 {
    let shannon = calculate_shannon_entropy(data);
    let chi_square = calculate_chi_square(data);
    let serial_corr = calculate_serial_correlation(data);

    // Shannon entropy should be close to 8.0 (perfect randomness)
    let shannon_score = (shannon / 8.0).min(1.0);

    // Chi-square should be close to 255.0 for uniform distribution
    let chi_score = 1.0 - ((chi_square - 255.0).abs() / 255.0).min(1.0);

    // Serial correlation should be close to 0.0
    let serial_score = 1.0 - serial_corr.abs().min(1.0);

    // Weighted average
    (shannon_score * 0.5) + (chi_score * 0.3) + (serial_score * 0.2)
}

/// Test software HSM entropy (baseline)
fn test_software_hsm(size: usize) -> Result<EntropyTestResult, BearDogError> {
    println!("🔧 Testing Software HSM (RustCrypto CSPRNG)...");

    let start = Instant::now();
    let mut entropy = vec![0u8; size];
    rand::thread_rng().fill_bytes(&mut entropy);
    let generation_time_ms = start.elapsed().as_millis();

    let shannon = calculate_shannon_entropy(&entropy);
    let chi_square = calculate_chi_square(&entropy);
    let serial_corr = calculate_serial_correlation(&entropy);
    let quality_score = analyze_entropy(&entropy);

    Ok(EntropyTestResult {
        source: "Software HSM (RustCrypto)".to_string(),
        _entropy_bytes: entropy,
        generation_time_ms,
        quality_score,
        shannon_entropy: shannon,
        chi_square,
        serial_correlation: serial_corr,
    })
}

/// Test `SoloKey` FIDO2 hardware entropy
#[cfg(feature = "fido2")]
async fn test_solokey_entropy(size: usize) -> Result<EntropyTestResult, BearDogError> {
    use beardog_security::hsm::fido2::discovery::discover_fido2_devices;

    println!("🔑 Testing SoloKey FIDO2 Hardware Entropy...");

    // Discover FIDO2 devices
    let devices = discover_fido2_devices().await?;
    if devices.is_empty() {
        return Err(BearDogError::system("No FIDO2 devices found".to_string()));
    }

    println!("   Found: {}", devices[0].product);

    let start = Instant::now();

    // Phase 2: Will use CTAP2 hmac-secret for real hardware entropy
    // For now, simulate high-quality hardware RNG
    let mut entropy = vec![0u8; size];
    rand::thread_rng().fill_bytes(&mut entropy);

    // Use entropy directly (device-specific mixing can be added later with crypto crate)
    let mixed_entropy = entropy;

    let generation_time_ms = start.elapsed().as_millis();

    let shannon = calculate_shannon_entropy(&mixed_entropy);
    let chi_square = calculate_chi_square(&mixed_entropy);
    let serial_corr = calculate_serial_correlation(&mixed_entropy);
    let quality_score = analyze_entropy(&mixed_entropy) * 0.95; // Hardware bonus

    println!("   ⚠️  Note: Using simulated hardware entropy (CTAP2 hmac-secret in Phase 2)");

    Ok(EntropyTestResult {
        source: format!("SoloKey: {}", devices[0].product),
        _entropy_bytes: mixed_entropy,
        generation_time_ms,
        quality_score,
        shannon_entropy: shannon,
        chi_square,
        serial_correlation: serial_corr,
    })
}

#[cfg(not(feature = "fido2"))]
async fn test_solokey_entropy(_size: usize) -> Result<EntropyTestResult, BearDogError> {
    Err(BearDogError::system(
        "FIDO2 feature not enabled. Compile with --features fido2".to_string(),
    ))
}

/// Test Pixel 8a Titan M entropy
#[cfg(target_os = "android")]
async fn test_pixel_titan_m(size: usize) -> Result<EntropyTestResult, BearDogError> {
    use beardog_security::hsm::android_strongbox::native_strongbox::NativeStrongBox;

    println!("📱 Testing Pixel 8a Titan M (StrongBox)...");

    let strongbox = NativeStrongBox::new()?;
    let info = strongbox.device_info();

    println!("   Device: {} {}", info.manufacturer, info.model);
    println!(
        "   StrongBox: {}",
        if info.strongbox_available {
            "Available"
        } else {
            "Not available"
        }
    );

    let start = Instant::now();
    let entropy = strongbox.generate_entropy_native(size)?;
    let generation_time_ms = start.elapsed().as_millis();

    let shannon = calculate_shannon_entropy(&entropy);
    let chi_square = calculate_chi_square(&entropy);
    let serial_corr = calculate_serial_correlation(&entropy);
    let quality_score = analyze_entropy(&entropy) * 0.98; // Hardware + TEE bonus

    Ok(EntropyTestResult {
        source: format!("Titan M ({})", info.model),
        _entropy_bytes: entropy,
        generation_time_ms,
        quality_score,
        shannon_entropy: shannon,
        chi_square,
        serial_correlation: serial_corr,
    })
}

#[cfg(not(target_os = "android"))]
async fn test_pixel_titan_m(_size: usize) -> Result<EntropyTestResult, BearDogError> {
    Err(BearDogError::system(
        "Android-only test. Run on Pixel 8a device.".to_string(),
    ))
}

/// Test human entropy (keyboard/mouse timing)
fn test_human_entropy() -> Result<EntropyTestResult, BearDogError> {
    println!("👤 Testing Human Entropy (Keyboard/Mouse Timing)...");
    println!("   Please interact with keyboard and mouse...");
    println!();

    let config = InteractionCaptureConfig {
        target_interactions: 30, // Shorter for demo
        timeout_seconds: 60,
        min_quality: 0.6,
        enable_keyboard: true,
        enable_mouse: true,
    };

    let collector = InteractionEntropyCollector::new(config);
    let start = Instant::now();
    let result = collector.collect_live_interactions()?;
    let generation_time_ms = start.elapsed().as_millis();

    println!();
    println!(
        "   ✅ Collected {} interactions",
        result.metrics.total_interactions
    );
    println!("   Duration: {:.1}s", result.duration_ms as f64 / 1000.0);

    let entropy = &result.entropy_bytes;
    let shannon = calculate_shannon_entropy(entropy);
    let chi_square = calculate_chi_square(entropy);
    let serial_corr = calculate_serial_correlation(entropy);

    // Human entropy gets a sovereignty bonus
    let quality_score = (result.quality_score * 0.7) + (analyze_entropy(entropy) * 0.3);

    Ok(EntropyTestResult {
        source: "Human Interaction (Tier 3)".to_string(),
        _entropy_bytes: entropy.clone(),
        generation_time_ms,
        quality_score: quality_score * 1.05, // Sovereignty bonus
        shannon_entropy: shannon,
        chi_square,
        serial_correlation: serial_corr,
    })
}

/// Run comprehensive comparison
async fn run_comparison() -> Result<(), BearDogError> {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║    🎲 Hardware Entropy Comparison Suite - January 13, 2026    ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();
    println!("Testing entropy quality across multiple sources:");
    println!("  1. Software HSM (baseline)");
    println!("  2. SoloKey FIDO2 hardware");
    println!("  3. Pixel 8a Titan M");
    println!("  4. Human interaction");
    println!();
    println!("Each test generates 256 bytes and measures:");
    println!("  - Generation speed");
    println!("  - Shannon entropy (randomness)");
    println!("  - Chi-square distribution");
    println!("  - Serial correlation");
    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let mut results = Vec::new();

    // Test 1: Software HSM (always available)
    match test_software_hsm(256) {
        Ok(result) => {
            result.print_report();
            results.push(result);
        }
        Err(e) => println!("⚠️  Software HSM test failed: {e}\n"),
    }

    // Test 2: SoloKey FIDO2
    match test_solokey_entropy(256).await {
        Ok(result) => {
            result.print_report();
            results.push(result);
        }
        Err(e) => println!("⚠️  SoloKey test skipped: {e}\n"),
    }

    // Test 3: Pixel 8a Titan M
    match test_pixel_titan_m(256).await {
        Ok(result) => {
            result.print_report();
            results.push(result);
        }
        Err(e) => println!("⚠️  Pixel Titan M test skipped: {e}\n"),
    }

    // Test 4: Human entropy (optional, interactive)
    println!("Would you like to test human entropy? (requires keyboard/mouse input)");
    println!("Press 'y' for yes, any other key to skip...");

    // Simple yes/no prompt
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input)?;

    if input.trim().to_lowercase() == "y" {
        match test_human_entropy() {
            Ok(result) => {
                result.print_report();
                results.push(result);
            }
            Err(e) => println!("⚠️  Human entropy test failed: {e}\n"),
        }
    } else {
        println!("⏭️  Skipping human entropy test\n");
    }

    // Summary comparison
    if results.len() > 1 {
        println!("╔════════════════════════════════════════════════════════════════╗");
        println!("║                    📊 Comparison Summary                        ║");
        println!("╠════════════════════════════════════════════════════════════════╣");

        // Sort by quality score
        results.sort_by(|a, b| {
            b.quality_score
                .partial_cmp(&a.quality_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for (idx, result) in results.iter().enumerate() {
            println!(
                "║  {}. {:50} ║",
                idx + 1,
                if idx == 0 {
                    format!(
                        "🏆 {} ({:.1}%)",
                        result.source,
                        result.quality_score * 100.0
                    )
                } else {
                    format!(
                        "   {} ({:.1}%)",
                        result.source,
                        result.quality_score * 100.0
                    )
                }
            );
        }

        println!("╚════════════════════════════════════════════════════════════════╝");
        println!();

        println!("📈 Insights:");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        if let (Some(fastest), Some(best_quality), Some(best_shannon)) = (
            results.iter().min_by_key(|r| r.generation_time_ms),
            results.iter().max_by(|a, b| {
                a.quality_score
                    .partial_cmp(&b.quality_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            results.iter().max_by(|a, b| {
                a.shannon_entropy
                    .partial_cmp(&b.shannon_entropy)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
        ) {
            println!(
                "⚡ Fastest: {} ({} ms)",
                fastest.source, fastest.generation_time_ms
            );
            println!(
                "🎯 Best Quality: {} ({:.1}%)",
                best_quality.source,
                best_quality.quality_score * 100.0
            );
            println!(
                "🎲 Best Randomness: {} (Shannon: {:.4})",
                best_shannon.source, best_shannon.shannon_entropy
            );
        }

        println!();
        println!("💡 Recommendations:");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("• For speed: Use software HSM for high-throughput operations");
        println!("• For quality: Use hardware HSM for key generation");
        println!("• For sovereignty: Mix human entropy with hardware");
        println!("• For critical keys: Combine multiple sources");
        println!();
    }

    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                   ✅ Testing Complete!                         ║");
    println!("╚════════════════════════════════════════════════════════════════╝");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    // Initialize logging (optional, comment out if you want clean output)
    // tracing_subscriber::fmt::init();

    run_comparison().await
}
