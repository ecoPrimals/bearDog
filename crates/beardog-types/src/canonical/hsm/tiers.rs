// SPDX-License-Identifier: AGPL-3.0-only

use serde::{Deserialize, Serialize};

/// `HsmSecurityTier`
///
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Default,
)]
pub enum HsmSecurityTier {
    #[default]
    /// Software
    Software,

    /// Hardware variant
    /// Hardware
    Hardware,

    /// `CommonCriteria` variant
    /// `CommonCriteria`
    CommonCriteria,

    /// `Fips140Level2`
    Fips140Level2,

    /// `Fips140Level3`
    Fips140Level3,

    /// `Fips140Level4`
    Fips140Level4,
}

/// `AttestationLevel`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum AttestationLevel {
    /// None variant
    /// None
    #[default]
    None,

    /// Basic variant
    /// Basic
    Basic,

    /// Enhanced variant
    /// Enhanced
    Enhanced,

    /// Strong variant
    /// Strong
    Strong,

    /// Hardware variant
    /// Hardware
    Hardware,

    /// `CertifiedHardware` variant
    /// `CertifiedHardware`
    CertifiedHardware,
}

/// `TamperResistanceLevel`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum TamperResistanceLevel {
    /// None variant
    /// None
    None,

    /// Basic variant
    /// Basic
    #[default]
    Basic,

    /// Evidence variant
    /// Evidence
    Evidence,

    /// Response variant
    /// Response
    Response,

    /// Hardware variant
    /// Hardware
    Hardware,

    /// `MilitaryGrade` variant
    /// `MilitaryGrade`
    MilitaryGrade,
}
