// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Canonical HSM Tier Types
///
/// **UNIFIED TIER CLASSIFICATION** for the BearDog ecosystem
/// This module provides the canonical definitions for HSM security tiers and classifications.
use serde::{Deserialize, Serialize};

/// HSM Security Tier classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[derive(Default)]
pub enum HsmSecurityTier {
    /// Software-only (lowest security)
    #[default]
    Software,
    /// Hardware-backed but not certified
    Hardware,
    /// Common Criteria certified
    CommonCriteria,
    /// FIPS 140-2 Level 2
    Fips140Level2,
    /// FIPS 140-2 Level 3
    Fips140Level3,
    /// FIPS 140-2 Level 4 (highest security)
    Fips140Level4,
}
/// Attestation levels for HSM operations
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AttestationLevel {
    /// No attestation
    None,
    /// Basic attestation
    Basic,
    /// Enhanced attestation with hardware binding
    Enhanced,
    /// Strong attestation with user presence
    Strong,
    /// Hardware attestation with secure boot
    Hardware,
    /// Certified hardware attestation (highest level)
    CertifiedHardware,
}

/// Tamper resistance levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]


pub enum TamperResistanceLevel {
    /// No tamper resistance
    None,
    /// Basic tamper detection
    Basic,
    /// Tamper evident
    Evidence,
    /// Tamper responsive
    Response,
    /// Hardware tamper resistance
    Hardware,
    /// Military-grade tamper resistance
    MilitaryGrade,
}


impl Default for AttestationLevel {
    fn default() -> Self {
        Self::None
    }
}


impl Default for TamperResistanceLevel {
    fn default() -> Self {
        Self::Basic
    }
}
