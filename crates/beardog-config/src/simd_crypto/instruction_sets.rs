//! SIMD Instruction Set Configuration
//!
//! This module handles SIMD instruction set configuration, compatibility, and priorities.

use serde::{Deserialize, Serialize};

/// Instruction set configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionSetConfig {
    /// Enabled instruction sets
    pub enabled_sets: Vec<InstructionSet>,
    /// Instruction set priorities
    pub priorities: InstructionSetPriorities,
    /// Compatibility matrix
    pub compatibility: CompatibilityMatrix,
}

/// Instruction set
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InstructionSet {
    /// SSE2 instruction set
    SSE2,
    /// SSE3 instruction set
    SSE3,
    /// SSSE3 instruction set
    SSSE3,
    /// SSE4.1 instruction set
    SSE4_1,
    /// SSE4.2 instruction set
    SSE4_2,
    /// AVX instruction set
    AVX,
    /// AVX2 instruction set
    AVX2,
    /// AVX512F instruction set
    AVX512F,
    /// AVX512BW instruction set
    AVX512BW,
    /// AVX512VL instruction set
    AVX512VL,
    /// ARM NEON instruction set
    NEON,
    /// ARM SVE instruction set
    SVE,
    /// ARM SVE2 instruction set
    SVE2,
    /// RISC-V Vector Extension
    RVV,
    /// PowerPC AltiVec
    AltiVec,
    /// PowerPC VSX
    VSX,
}

/// Instruction set priorities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionSetPriorities {
    /// Hash operations priority order
    pub hash_priorities: Vec<InstructionSet>,
    /// Encryption operations priority order
    pub encryption_priorities: Vec<InstructionSet>,
    /// Signature operations priority order
    pub signature_priorities: Vec<InstructionSet>,
}

/// Compatibility matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityMatrix {
    /// Compatibility entries
    pub entries: Vec<CompatibilityEntry>,
}

/// Compatibility entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityEntry {
    /// Instruction set
    pub instruction_set: InstructionSet,
    /// Operation compatibility
    pub operations: OperationCompatibility,
}

/// Operation compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationCompatibility {
    /// Hash operations supported
    pub hash_operations: bool,
    /// Encryption operations supported
    pub encryption_operations: bool,
    /// Signature operations supported
    pub signature_operations: bool,
}

impl Default for InstructionSetConfig {
    fn default() -> Self {
        Self {
            enabled_sets: vec![InstructionSet::SSE2, InstructionSet::AVX2],
            priorities: InstructionSetPriorities::default(),
            compatibility: CompatibilityMatrix::default(),
        }
    }
}

impl Default for InstructionSetPriorities {
    fn default() -> Self {
        Self {
            hash_priorities: vec![InstructionSet::AVX2, InstructionSet::SSE2],
            encryption_priorities: vec![InstructionSet::AVX2, InstructionSet::SSE2],
            signature_priorities: vec![InstructionSet::AVX2, InstructionSet::SSE2],
        }
    }
}

impl Default for CompatibilityMatrix {
    fn default() -> Self {
        Self {
            entries: vec![
                CompatibilityEntry {
                    instruction_set: InstructionSet::SSE2,
                    operations: OperationCompatibility {
                        hash_operations: true,
                        encryption_operations: true,
                        signature_operations: false,
                    },
                },
                CompatibilityEntry {
                    instruction_set: InstructionSet::AVX2,
                    operations: OperationCompatibility {
                        hash_operations: true,
                        encryption_operations: true,
                        signature_operations: true,
                    },
                },
            ],
        }
    }
}

impl InstructionSetConfig {
    /// Create production instruction set configuration
    pub fn production() -> Self {
        Self {
            enabled_sets: vec![
                InstructionSet::SSE2,
                InstructionSet::SSE3,
                InstructionSet::SSSE3,
                InstructionSet::SSE4_1,
                InstructionSet::SSE4_2,
                InstructionSet::AVX,
                InstructionSet::AVX2,
                InstructionSet::AVX512F,
                InstructionSet::NEON,
                InstructionSet::SVE,
            ],
            priorities: InstructionSetPriorities::production(),
            compatibility: CompatibilityMatrix::production(),
        }
    }

    /// Create development instruction set configuration
    pub fn development() -> Self {
        Self {
            enabled_sets: vec![InstructionSet::SSE2],
            priorities: InstructionSetPriorities::development(),
            compatibility: CompatibilityMatrix::development(),
        }
    }

    /// Check if an instruction set is enabled
    pub fn is_enabled(&self, instruction_set: &InstructionSet) -> bool {
        self.enabled_sets.contains(instruction_set)
    }

    /// Get the best instruction set for a specific operation
    pub fn get_best_for_operation(&self, operation: &str) -> Option<&InstructionSet> {
        let priorities = match operation {
            "hash" => &self.priorities.hash_priorities,
            "encryption" => &self.priorities.encryption_priorities,
            "signature" => &self.priorities.signature_priorities,
            _ => &self.priorities.hash_priorities,
        };

        priorities
            .iter()
            .find(|&instruction_set| self.enabled_sets.contains(instruction_set))
    }
}

impl InstructionSetPriorities {
    /// Create production instruction set priorities
    pub fn production() -> Self {
        Self {
            hash_priorities: vec![
                InstructionSet::AVX512F,
                InstructionSet::AVX2,
                InstructionSet::AVX,
                InstructionSet::SSE4_2,
                InstructionSet::SSE2,
                InstructionSet::SVE2,
                InstructionSet::SVE,
                InstructionSet::NEON,
            ],
            encryption_priorities: vec![
                InstructionSet::AVX512F,
                InstructionSet::AVX2,
                InstructionSet::AVX,
                InstructionSet::SSE4_2,
                InstructionSet::SSE2,
                InstructionSet::SVE2,
                InstructionSet::SVE,
                InstructionSet::NEON,
            ],
            signature_priorities: vec![
                InstructionSet::AVX512F,
                InstructionSet::AVX2,
                InstructionSet::AVX,
                InstructionSet::SSE4_2,
                InstructionSet::SVE2,
                InstructionSet::SVE,
                InstructionSet::NEON,
            ],
        }
    }

    /// Create development instruction set priorities
    pub fn development() -> Self {
        Self {
            hash_priorities: vec![InstructionSet::SSE2],
            encryption_priorities: vec![InstructionSet::SSE2],
            signature_priorities: vec![InstructionSet::SSE2],
        }
    }
}

impl CompatibilityMatrix {
    /// Create production compatibility matrix
    pub fn production() -> Self {
        Self {
            entries: vec![
                CompatibilityEntry {
                    instruction_set: InstructionSet::SSE2,
                    operations: OperationCompatibility {
                        hash_operations: true,
                        encryption_operations: true,
                        signature_operations: false,
                    },
                },
                CompatibilityEntry {
                    instruction_set: InstructionSet::AVX2,
                    operations: OperationCompatibility {
                        hash_operations: true,
                        encryption_operations: true,
                        signature_operations: true,
                    },
                },
                CompatibilityEntry {
                    instruction_set: InstructionSet::AVX512F,
                    operations: OperationCompatibility {
                        hash_operations: true,
                        encryption_operations: true,
                        signature_operations: true,
                    },
                },
                CompatibilityEntry {
                    instruction_set: InstructionSet::NEON,
                    operations: OperationCompatibility {
                        hash_operations: true,
                        encryption_operations: true,
                        signature_operations: true,
                    },
                },
                CompatibilityEntry {
                    instruction_set: InstructionSet::SVE,
                    operations: OperationCompatibility {
                        hash_operations: true,
                        encryption_operations: true,
                        signature_operations: true,
                    },
                },
            ],
        }
    }

    /// Create development compatibility matrix
    pub fn development() -> Self {
        Self {
            entries: vec![CompatibilityEntry {
                instruction_set: InstructionSet::SSE2,
                operations: OperationCompatibility {
                    hash_operations: true,
                    encryption_operations: true,
                    signature_operations: false,
                },
            }],
        }
    }

    /// Check if an instruction set supports a specific operation
    pub fn supports_operation(&self, instruction_set: &InstructionSet, operation: &str) -> bool {
        for entry in &self.entries {
            if entry.instruction_set == *instruction_set {
                return match operation {
                    "hash" => entry.operations.hash_operations,
                    "encryption" => entry.operations.encryption_operations,
                    "signature" => entry.operations.signature_operations,
                    _ => false,
                };
            }
        }
        false
    }

    /// Get all supported operations for an instruction set
    pub fn get_supported_operations(&self, instruction_set: &InstructionSet) -> Vec<String> {
        let mut operations = Vec::new();

        for entry in &self.entries {
            if entry.instruction_set == *instruction_set {
                if entry.operations.hash_operations {
                    operations.push("hash".to_string());
                }
                if entry.operations.encryption_operations {
                    operations.push("encryption".to_string());
                }
                if entry.operations.signature_operations {
                    operations.push("signature".to_string());
                }
                break;
            }
        }

        operations
    }
}
