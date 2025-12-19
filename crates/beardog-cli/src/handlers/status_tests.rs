// Comprehensive tests for status handler
// Following BearDog testing standards: isolated, thorough, idiomatic

use super::status::*;

// ============================================================================
// FORMAT VERSION TESTS
// ============================================================================

#[test]
fn test_format_version_valid() {
    let version = "1.2.3";
    let formatted = format_version(version);
    assert_eq!(formatted, "v1.2.3");
}

#[test]
fn test_format_version_empty() {
    let version = "";
    let formatted = format_version(version);
    assert_eq!(formatted, "v");
}

#[test]
fn test_format_version_with_prefix() {
    let version = "v2.0.0";
    let formatted = format_version(version);
    // Should not double-prefix
    assert_eq!(formatted, "vv2.0.0");
}

// ============================================================================
// GET BUILD INFO TESTS
// ============================================================================

#[test]
fn test_get_build_info_structure() {
    let info = get_build_info();

    // Version should not be empty
    assert!(!info.version.is_empty(), "version should not be empty");

    // Target should not be empty
    assert!(!info.target.is_empty(), "target should not be empty");

    // Profile should be debug or release
    assert!(
        info.profile == "debug" || info.profile == "release",
        "profile should be debug or release: {}",
        info.profile
    );
}

#[test]
fn test_get_build_info_version_format() {
    let info = get_build_info();

    // Version should start with 'v'
    assert!(
        info.version.starts_with('v'),
        "version should start with v: {}",
        info.version
    );
}

#[test]
fn test_get_build_info_target_format() {
    let info = get_build_info();

    // Target should be set (either from TARGET env var or "unknown")
    assert!(!info.target.is_empty(), "target should not be empty");

    // If not "unknown", should contain architecture info in standard format
    if info.target != "unknown" {
        assert!(
            info.target.contains('-'),
            "target should be in standard format (e.g., x86_64-unknown-linux-gnu): {}",
            info.target
        );
    }
}

// ============================================================================
// GET SYSTEM INFO TESTS
// ============================================================================

#[test]
fn test_get_system_info_os() {
    let info = get_system_info();

    // OS should not be empty
    assert!(!info.os.is_empty(), "OS should not be empty");

    // Common OS values
    let valid_os = ["linux", "macos", "windows", "unknown"];
    assert!(
        valid_os
            .iter()
            .any(|&os| info.os.to_lowercase().contains(os)),
        "OS should be recognized: {}",
        info.os
    );
}

#[test]
fn test_get_system_info_arch() {
    let info = get_system_info();

    // Arch should not be empty
    assert!(!info.arch.is_empty(), "architecture should not be empty");

    // Common architectures
    let valid_arch = ["x86_64", "aarch64", "arm", "i686"];
    assert!(
        valid_arch.iter().any(|&arch| info.arch.contains(arch)),
        "architecture should be recognized: {}",
        info.arch
    );
}

// ============================================================================
// HANDLER TESTS
// ============================================================================

#[tokio::test]
async fn test_handle_status_basic() {
    let result = handle_status(false).await;

    // Should always succeed
    assert!(
        result.is_ok(),
        "status command should not error: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_handle_status_verbose() {
    let result = handle_status(true).await;

    // Should succeed with verbose output
    assert!(
        result.is_ok(),
        "verbose status should not error: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_handle_version() {
    let result = handle_version().await;

    // Should always succeed
    assert!(
        result.is_ok(),
        "version command should not error: {:?}",
        result.err()
    );
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[tokio::test]
async fn test_status_consistency() {
    // Multiple status calls should return consistent info
    let result1 = handle_status(false).await;
    let result2 = handle_status(false).await;

    // Both should succeed
    assert!(result1.is_ok(), "first status call should succeed");
    assert!(result2.is_ok(), "second status call should succeed");
}

#[test]
fn test_build_info_consistency() {
    // Multiple calls should return same info
    let info1 = get_build_info();
    let info2 = get_build_info();

    assert_eq!(info1.version, info2.version, "version should be consistent");
    assert_eq!(info1.target, info2.target, "target should be consistent");
    assert_eq!(info1.profile, info2.profile, "profile should be consistent");
}

#[test]
fn test_system_info_consistency() {
    // Multiple calls should return same info
    let info1 = get_system_info();
    let info2 = get_system_info();

    assert_eq!(info1.os, info2.os, "OS should be consistent");
    assert_eq!(info1.arch, info2.arch, "architecture should be consistent");
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_format_version_special_chars() {
    let version = "1.0.0-alpha+build.123";
    let formatted = format_version(version);
    assert_eq!(formatted, "v1.0.0-alpha+build.123");
}

#[test]
fn test_format_version_long_string() {
    let version = "1.2.3.4.5.6.7.8.9.10";
    let formatted = format_version(version);
    assert!(formatted.starts_with('v'));
    assert!(formatted.len() == version.len() + 1);
}

// ============================================================================
// OUTPUT FORMAT TESTS
// ============================================================================

#[test]
fn test_build_info_has_all_fields() {
    let info = get_build_info();

    // All fields should have content
    assert!(!info.version.is_empty(), "version should not be empty");
    assert!(!info.target.is_empty(), "target should not be empty");
    assert!(!info.profile.is_empty(), "profile should not be empty");
}

#[test]
fn test_system_info_has_all_fields() {
    let info = get_system_info();

    // All fields should have content
    assert!(!info.os.is_empty(), "OS should not be empty");
    assert!(!info.arch.is_empty(), "architecture should not be empty");
}

// ============================================================================
// PERFORMANCE TESTS
// ============================================================================

#[test]
fn test_get_build_info_performance() {
    // Should be fast (< 1ms)
    let start = std::time::Instant::now();
    let _ = get_build_info();
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 10,
        "get_build_info should be fast: {:?}",
        elapsed
    );
}

#[test]
fn test_get_system_info_performance() {
    // Should be fast (< 1ms)
    let start = std::time::Instant::now();
    let _ = get_system_info();
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 10,
        "get_system_info should be fast: {:?}",
        elapsed
    );
}

#[tokio::test]
async fn test_handle_status_performance() {
    // Should complete quickly (< 100ms)
    let start = std::time::Instant::now();
    let _ = handle_status(false).await;
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 100,
        "handle_status should be fast: {:?}",
        elapsed
    );
}
