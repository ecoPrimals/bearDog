

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[derive(Default)]
pub enum HsmSecurityTier {

    #[default]
    Software,

    Hardware,

    CommonCriteria,

    Fips140Level2,

    Fips140Level3,

    Fips140Level4,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AttestationLevel {

    None,

    Basic,

    Enhanced,

    Strong,

    Hardware,

    CertifiedHardware,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]

pub enum TamperResistanceLevel {

    None,

    Basic,

    Evidence,

    Response,

    Hardware,

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
