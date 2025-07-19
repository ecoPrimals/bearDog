//! Hardware Detection and Capabilities for SIMD Cryptography
//!
//! This module handles hardware detection, CPU architecture support, and capability management.

use serde::{Deserialize, Serialize};

/// Hardware detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareDetectionConfig {
    /// Enable automatic hardware detection
    pub auto_detection: bool,
    /// Supported CPU architectures
    pub supported_architectures: Vec<CPUArchitecture>,
    /// Hardware capabilities
    pub capabilities: HardwareCapabilities,
    /// Fallback configuration
    pub fallback: FallbackConfig,
}

/// CPU architecture
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CPUArchitecture {
    /// x86-64 architecture
    X86_64,
    /// ARM64 architecture
    ARM64,
    /// RISC-V architecture
    RISCV,
    /// PowerPC architecture
    PowerPC,
}

/// Hardware capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareCapabilities {
    /// Available instruction sets
    pub instruction_sets: Vec<String>,
    /// CPU core count
    pub core_count: u32,
    /// Cache sizes
    pub cache_sizes: CacheSizes,
    /// Memory bandwidth
    pub memory_bandwidth: MemoryBandwidth,
    /// Thermal constraints
    pub thermal_constraints: ThermalConstraints,
}

/// Cache sizes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSizes {
    /// L1 cache size in KB
    pub l1_cache_kb: u32,
    /// L2 cache size in KB
    pub l2_cache_kb: u32,
    /// L3 cache size in KB
    pub l3_cache_kb: u32,
}

/// Memory bandwidth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBandwidth {
    /// Read bandwidth in MB/s
    pub read_mb_per_sec: u32,
    /// Write bandwidth in MB/s
    pub write_mb_per_sec: u32,
}

/// Thermal constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalConstraints {
    /// Maximum temperature in Celsius
    pub max_temperature_celsius: u32,
    /// Thermal throttling enabled
    pub thermal_throttling: bool,
}

/// Fallback configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallbackConfig {
    /// Enable fallback to software implementation
    pub enable_software_fallback: bool,
    /// Fallback architecture
    pub fallback_architecture: CPUArchitecture,
    /// Fallback performance expectations
    pub fallback_performance_factor: f64,
}

impl Default for HardwareDetectionConfig {
    fn default() -> Self {
        Self {
            auto_detection: true,
            supported_architectures: vec![CPUArchitecture::X86_64, CPUArchitecture::ARM64],
            capabilities: HardwareCapabilities::default(),
            fallback: FallbackConfig::default(),
        }
    }
}

impl Default for HardwareCapabilities {
    fn default() -> Self {
        Self {
            instruction_sets: vec!["SSE2".to_string(), "AVX2".to_string()],
            core_count: 4,
            cache_sizes: CacheSizes::default(),
            memory_bandwidth: MemoryBandwidth::default(),
            thermal_constraints: ThermalConstraints::default(),
        }
    }
}

impl Default for CacheSizes {
    fn default() -> Self {
        Self {
            l1_cache_kb: 32,
            l2_cache_kb: 256,
            l3_cache_kb: 8192,
        }
    }
}

impl Default for MemoryBandwidth {
    fn default() -> Self {
        Self {
            read_mb_per_sec: 25600,
            write_mb_per_sec: 25600,
        }
    }
}

impl Default for ThermalConstraints {
    fn default() -> Self {
        Self {
            max_temperature_celsius: 85,
            thermal_throttling: true,
        }
    }
}

impl Default for FallbackConfig {
    fn default() -> Self {
        Self {
            enable_software_fallback: true,
            fallback_architecture: CPUArchitecture::X86_64,
            fallback_performance_factor: 0.1,
        }
    }
}

impl HardwareDetectionConfig {
    /// Create production hardware detection configuration
    pub fn production() -> Self {
        Self {
            auto_detection: true,
            supported_architectures: vec![
                CPUArchitecture::X86_64,
                CPUArchitecture::ARM64,
                CPUArchitecture::RISCV,
                CPUArchitecture::PowerPC,
            ],
            capabilities: HardwareCapabilities::production(),
            fallback: FallbackConfig::production(),
        }
    }

    /// Create development hardware detection configuration
    pub fn development() -> Self {
        Self {
            auto_detection: false,
            supported_architectures: vec![CPUArchitecture::X86_64],
            capabilities: HardwareCapabilities::development(),
            fallback: FallbackConfig::development(),
        }
    }

    /// Check if an architecture is supported
    pub fn supports_architecture(&self, arch: &CPUArchitecture) -> bool {
        self.supported_architectures.contains(arch)
    }

    /// Get the current architecture
    pub fn detect_current_architecture(&self) -> CPUArchitecture {
        if !self.auto_detection {
            return self.fallback.fallback_architecture.clone();
        }

        // In a real implementation, this would detect the actual architecture
        #[cfg(target_arch = "x86_64")]
        {
            CPUArchitecture::X86_64
        }
        #[cfg(target_arch = "aarch64")]
        {
            CPUArchitecture::ARM64
        }
        #[cfg(target_arch = "riscv64")]
        {
            CPUArchitecture::RISCV
        }
        #[cfg(target_arch = "powerpc64")]
        {
            CPUArchitecture::PowerPC
        }
        #[cfg(not(any(
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "riscv64",
            target_arch = "powerpc64"
        )))]
        {
            // Default fallback for unknown architectures
            CPUArchitecture::X86_64
        }
    }
}

impl HardwareCapabilities {
    /// Create production hardware capabilities
    pub fn production() -> Self {
        Self {
            instruction_sets: vec![
                "SSE2".to_string(),
                "SSE3".to_string(),
                "SSSE3".to_string(),
                "SSE4.1".to_string(),
                "SSE4.2".to_string(),
                "AVX".to_string(),
                "AVX2".to_string(),
                "AVX512F".to_string(),
            ],
            core_count: 16,
            cache_sizes: CacheSizes::production(),
            memory_bandwidth: MemoryBandwidth::production(),
            thermal_constraints: ThermalConstraints::production(),
        }
    }

    /// Create development hardware capabilities
    pub fn development() -> Self {
        Self {
            instruction_sets: vec!["SSE2".to_string()],
            core_count: 2,
            cache_sizes: CacheSizes::development(),
            memory_bandwidth: MemoryBandwidth::development(),
            thermal_constraints: ThermalConstraints::development(),
        }
    }

    /// Check if an instruction set is available
    pub fn has_instruction_set(&self, instruction_set: &str) -> bool {
        self.instruction_sets.contains(&instruction_set.to_string())
    }

    /// Calculate total cache size
    pub fn total_cache_size_kb(&self) -> u32 {
        self.cache_sizes.l1_cache_kb + self.cache_sizes.l2_cache_kb + self.cache_sizes.l3_cache_kb
    }
}

impl CacheSizes {
    /// Create production cache sizes
    pub fn production() -> Self {
        Self {
            l1_cache_kb: 64,
            l2_cache_kb: 512,
            l3_cache_kb: 32768,
        }
    }

    /// Create development cache sizes
    pub fn development() -> Self {
        Self {
            l1_cache_kb: 32,
            l2_cache_kb: 128,
            l3_cache_kb: 4096,
        }
    }
}

impl MemoryBandwidth {
    /// Create production memory bandwidth
    pub fn production() -> Self {
        Self {
            read_mb_per_sec: 51200,
            write_mb_per_sec: 51200,
        }
    }

    /// Create development memory bandwidth
    pub fn development() -> Self {
        Self {
            read_mb_per_sec: 12800,
            write_mb_per_sec: 12800,
        }
    }

    /// Get total bandwidth
    pub fn total_bandwidth_mb_per_sec(&self) -> u32 {
        self.read_mb_per_sec + self.write_mb_per_sec
    }
}

impl ThermalConstraints {
    /// Create production thermal constraints
    pub fn production() -> Self {
        Self {
            max_temperature_celsius: 95,
            thermal_throttling: true,
        }
    }

    /// Create development thermal constraints
    pub fn development() -> Self {
        Self {
            max_temperature_celsius: 70,
            thermal_throttling: false,
        }
    }

    /// Check if thermal throttling is needed
    pub fn needs_throttling(&self, current_temp: u32) -> bool {
        self.thermal_throttling && current_temp > self.max_temperature_celsius
    }
}

impl FallbackConfig {
    /// Create production fallback configuration
    pub fn production() -> Self {
        Self {
            enable_software_fallback: true,
            fallback_architecture: CPUArchitecture::X86_64,
            fallback_performance_factor: 0.2,
        }
    }

    /// Create development fallback configuration
    pub fn development() -> Self {
        Self {
            enable_software_fallback: false,
            fallback_architecture: CPUArchitecture::X86_64,
            fallback_performance_factor: 0.05,
        }
    }
}
