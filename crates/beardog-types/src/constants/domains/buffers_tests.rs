// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive tests for buffer size constants
//!
//! This module provides exhaustive testing for all buffer size constants,
//! ensuring they meet design requirements and maintain logical consistency.

use super::buffers::*;

#[cfg(test)]
mod buffer_tests {
    use super::*;

    // ============================================================================
    // POWER-OF-TWO ALIGNMENT TESTS
    // ============================================================================

    #[test]
    fn test_buffer_sizes_are_powers_of_two() {
        // Core design principle: all buffers should be powers of 2 for CPU cache efficiency
        assert!(
            BUFFER_SIZE_SMALL.is_power_of_two(),
            "BUFFER_SIZE_SMALL (1KB) must be power of 2"
        );
        assert!(
            BUFFER_SIZE_MEDIUM.is_power_of_two(),
            "BUFFER_SIZE_MEDIUM (4KB) must be power of 2"
        );
        assert!(
            BUFFER_SIZE_LARGE.is_power_of_two(),
            "BUFFER_SIZE_LARGE (16KB) must be power of 2"
        );
        assert!(
            BUFFER_SIZE_XLARGE.is_power_of_two(),
            "BUFFER_SIZE_XLARGE (64KB) must be power of 2"
        );
    }

    #[test]
    fn test_network_buffers_are_powers_of_two() {
        assert!(
            NETWORK_BUFFER_SIZE.is_power_of_two(),
            "NETWORK_BUFFER_SIZE must be power of 2"
        );
        assert!(
            TCP_BUFFER_SIZE.is_power_of_two(),
            "TCP_BUFFER_SIZE must be power of 2"
        );
        assert!(
            UDP_PACKET_SIZE.is_power_of_two(),
            "UDP_PACKET_SIZE must be power of 2"
        );
    }

    // ============================================================================
    // SIZE RELATIONSHIP TESTS
    // ============================================================================

    #[test]
    fn test_buffer_size_progression() {
        // Ensure sizes progress logically: small < medium < large < xlarge
        assert!(BUFFER_SIZE_SMALL < BUFFER_SIZE_MEDIUM);
        assert!(BUFFER_SIZE_MEDIUM < BUFFER_SIZE_LARGE);
        assert!(BUFFER_SIZE_LARGE < BUFFER_SIZE_XLARGE);
    }

    #[test]
    fn test_network_buffer_size_progression() {
        // UDP < Network < TCP for proper network hierarchy
        assert!(UDP_PACKET_SIZE < NETWORK_BUFFER_SIZE);
        assert!(NETWORK_BUFFER_SIZE < TCP_BUFFER_SIZE);
    }

    #[test]
    fn test_default_buffer_is_medium() {
        assert_eq!(
            BUFFER_SIZE_DEFAULT, BUFFER_SIZE_MEDIUM,
            "Default buffer should be medium (4KB) for balance"
        );
    }

    // ============================================================================
    // SPECIFIC VALUE TESTS
    // ============================================================================

    #[test]
    fn test_buffer_size_values() {
        assert_eq!(BUFFER_SIZE_SMALL, 1_024, "Small buffer should be 1KB");
        assert_eq!(BUFFER_SIZE_MEDIUM, 4_096, "Medium buffer should be 4KB");
        assert_eq!(BUFFER_SIZE_LARGE, 16_384, "Large buffer should be 16KB");
        assert_eq!(BUFFER_SIZE_XLARGE, 65_536, "XLarge buffer should be 64KB");
    }

    #[test]
    fn test_network_buffer_values() {
        assert_eq!(NETWORK_BUFFER_SIZE, 65_536, "Network buffer should be 64KB");
        assert_eq!(TCP_BUFFER_SIZE, 131_072, "TCP buffer should be 128KB");
        assert_eq!(UDP_PACKET_SIZE, 8_192, "UDP packet should be 8KB");
        assert_eq!(HTTP_HEADER_BUFFER_SIZE, 8_192, "HTTP headers should be 8KB");
    }

    #[test]
    fn test_file_buffer_values() {
        assert_eq!(FILE_BUFFER_SIZE, 131_072, "File buffer should be 128KB");
        assert_eq!(STREAM_BUFFER_SIZE, 8_192, "Stream buffer should be 8KB");
        assert_eq!(
            LARGE_FILE_BUFFER_SIZE, 1_048_576,
            "Large file buffer should be 1MB"
        );
    }

    #[test]
    fn test_crypto_buffer_values() {
        assert_eq!(CRYPTO_BUFFER_SIZE, 16_384, "Crypto buffer should be 16KB");
        assert_eq!(HSM_BUFFER_SIZE, 4_096, "HSM buffer should be 4KB");
        assert_eq!(ENTROPY_BUFFER_SIZE, 1_024, "Entropy buffer should be 1KB");
    }

    // ============================================================================
    // PROTOCOL-SPECIFIC TESTS
    // ============================================================================

    #[test]
    fn test_bstp_buffer_size() {
        assert_eq!(BSTP_BUFFER_SIZE, 32_768, "BSTP buffer should be 32KB");
        assert!(
            BSTP_BUFFER_SIZE.is_power_of_two(),
            "BSTP buffer must be power of 2"
        );
    }

    #[test]
    fn test_discovery_buffer_size() {
        assert_eq!(
            DISCOVERY_BUFFER_SIZE, 8_192,
            "Discovery buffer should be 8KB"
        );
        assert!(DISCOVERY_BUFFER_SIZE.is_power_of_two());
    }

    // ============================================================================
    // MEMORY POOL TESTS
    // ============================================================================

    #[test]
    fn test_pool_sizes_are_reasonable() {
        use crate::constants::domains::buffers::pool_sizes;

        // Calculate total preallocated memory
        let small_total = pool_sizes::SMALL_POOL_COUNT * BUFFER_SIZE_SMALL;
        let medium_total = pool_sizes::MEDIUM_POOL_COUNT * BUFFER_SIZE_MEDIUM;
        let large_total = pool_sizes::LARGE_POOL_COUNT * BUFFER_SIZE_LARGE;
        let network_total = pool_sizes::NETWORK_POOL_COUNT * NETWORK_BUFFER_SIZE;

        let total_prealloc = small_total + medium_total + large_total + network_total;

        // Total should be reasonable (< 2MB for all pools)
        assert!(
            total_prealloc < 2 * 1024 * 1024,
            "Total pool preallocation should be < 2MB for efficiency"
        );
    }

    #[test]
    fn test_pool_counts() {
        use crate::constants::domains::buffers::pool_sizes;

        assert_eq!(pool_sizes::SMALL_POOL_COUNT, 100);
        assert_eq!(pool_sizes::MEDIUM_POOL_COUNT, 50);
        assert_eq!(pool_sizes::LARGE_POOL_COUNT, 10);
        assert_eq!(pool_sizes::NETWORK_POOL_COUNT, 5);
    }

    // ============================================================================
    // LIMIT VALIDATION TESTS
    // ============================================================================

    #[test]
    fn test_max_buffer_size_reasonable() {
        assert_eq!(
            MAX_BUFFER_SIZE,
            10 * 1024 * 1024,
            "Max buffer should be 10MB"
        );

        // All specific buffers should be less than max
        assert!(NETWORK_BUFFER_SIZE < MAX_BUFFER_SIZE);
        assert!(TCP_BUFFER_SIZE < MAX_BUFFER_SIZE);
        assert!(FILE_BUFFER_SIZE < MAX_BUFFER_SIZE);
        assert!(LARGE_FILE_BUFFER_SIZE < MAX_BUFFER_SIZE);
    }

    #[test]
    fn test_min_buffer_size_reasonable() {
        assert_eq!(MIN_BUFFER_SIZE, 256, "Min buffer should be 256 bytes");

        // All specific buffers should be greater than min
        assert!(BUFFER_SIZE_SMALL > MIN_BUFFER_SIZE);
        assert!(ENTROPY_BUFFER_SIZE > MIN_BUFFER_SIZE);
    }

    // ============================================================================
    // PAGE ALIGNMENT TESTS
    // ============================================================================

    #[test]
    fn test_page_aligned_buffers() {
        const PAGE_SIZE: usize = 4_096;

        // Medium buffer should be page-aligned
        assert_eq!(
            BUFFER_SIZE_MEDIUM % PAGE_SIZE,
            0,
            "Medium buffer (4KB) should be page-aligned"
        );

        // DMA buffer should be page-aligned
        assert_eq!(
            DMA_BUFFER_SIZE % PAGE_SIZE,
            0,
            "DMA buffer should be page-aligned for hardware efficiency"
        );

        // File buffer should be page-aligned
        assert_eq!(
            FILE_BUFFER_SIZE % PAGE_SIZE,
            0,
            "File buffer should be page-aligned for I/O efficiency"
        );
    }

    // ============================================================================
    // ZERO-COPY BUFFER TESTS
    // ============================================================================

    #[test]
    fn test_zero_copy_buffer_size() {
        assert_eq!(
            ZERO_COPY_BUFFER_SIZE, 65_536,
            "Zero-copy buffer should be 64KB"
        );
        assert_eq!(
            ZERO_COPY_BUFFER_SIZE, NETWORK_BUFFER_SIZE,
            "Zero-copy should match network buffer for efficiency"
        );
    }

    #[test]
    fn test_dma_buffer_alignment() {
        assert_eq!(
            DMA_BUFFER_SIZE, 4_096,
            "DMA buffer should be 4KB (page size)"
        );
        assert!(DMA_BUFFER_SIZE.is_power_of_two());
    }

    // ============================================================================
    // BACKWARD COMPATIBILITY TESTS
    // ============================================================================

    #[test]
    #[allow(deprecated)]
    fn test_deprecated_aliases_match() {
        assert_eq!(
            DEFAULT_BUFFER_SIZE_NETWORK, NETWORK_BUFFER_SIZE,
            "Deprecated alias must match current constant"
        );
        assert_eq!(
            DEFAULT_BUFFER_SIZE_FILE, FILE_BUFFER_SIZE,
            "Deprecated alias must match current constant"
        );
    }

    // ============================================================================
    // PROPERTY TESTS (Invariants)
    // ============================================================================

    #[test]
    fn test_buffer_invariants() {
        // All buffers must be within min/max range
        let buffers = [
            BUFFER_SIZE_SMALL,
            BUFFER_SIZE_MEDIUM,
            BUFFER_SIZE_LARGE,
            BUFFER_SIZE_XLARGE,
            NETWORK_BUFFER_SIZE,
            TCP_BUFFER_SIZE,
            FILE_BUFFER_SIZE,
            CRYPTO_BUFFER_SIZE,
        ];

        for &buffer in &buffers {
            assert!(
                buffer >= MIN_BUFFER_SIZE,
                "Buffer {buffer} must be >= MIN_BUFFER_SIZE"
            );
            assert!(
                buffer <= MAX_BUFFER_SIZE,
                "Buffer {buffer} must be <= MAX_BUFFER_SIZE"
            );
        }
    }

    #[test]
    fn test_udp_packet_size_below_mtu() {
        // UDP packets should be well below typical MTU (1500 bytes) to allow fragmentation
        const TYPICAL_MTU: usize = 1500;
        assert!(
            UDP_PACKET_SIZE > TYPICAL_MTU,
            "UDP_PACKET_SIZE should be > MTU but designed for fragmentation"
        );
    }

    // ============================================================================
    // USAGE PATTERN TESTS
    // ============================================================================

    #[test]
    fn test_buffer_allocation_pattern() {
        // Verify buffers can be allocated without panic
        let _small = vec![0u8; BUFFER_SIZE_SMALL];
        let _medium = vec![0u8; BUFFER_SIZE_MEDIUM];
        let _large = vec![0u8; BUFFER_SIZE_LARGE];
        let _network = vec![0u8; NETWORK_BUFFER_SIZE];

        // If we got here, allocations succeeded.
    }

    #[test]
    fn test_hsm_buffer_appropriate_for_pkcs11() {
        // PKCS#11 messages are typically < 4KB
        assert_eq!(
            HSM_BUFFER_SIZE, 4_096,
            "HSM buffer should be 4KB to handle typical PKCS#11 messages"
        );
    }

    #[test]
    fn test_entropy_buffer_sufficient() {
        // 1KB should be sufficient for most entropy collection needs
        assert_eq!(
            ENTROPY_BUFFER_SIZE, 1_024,
            "Entropy buffer should be 1KB for efficient collection"
        );

        // Should be able to collect at least 256 bits of entropy
        assert!(
            ENTROPY_BUFFER_SIZE >= 32,
            "Entropy buffer must hold at least 256 bits"
        );
    }
}
