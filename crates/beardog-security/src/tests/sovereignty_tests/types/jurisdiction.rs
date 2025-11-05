//! Jurisdiction and Policy Types
//!
//! Test helper types for jurisdiction-based sovereignty policies.

use beardog_errors::BearDogError;
use std::collections::HashSet;

/// Jurisdiction types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Jurisdiction {
    EuropeanUnion,
    UnitedStates,
    UnitedKingdom,
    Canada,
    Australia,
    Japan,
    Switzerland,
    International,
}

/// Regulation frameworks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Regulation {
    GDPR,     // EU General Data Protection Regulation
    CCPA,     // California Consumer Privacy Act
    HIPAA,    // Health Insurance Portability and Accountability Act
    PIPEDA,   // Personal Information Protection and Electronic Documents Act
    ISO27001, // Information Security Management
    SOC2,     // Service Organization Control 2
}

/// Data residency requirements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataResidency {
    StrictLocal,      // Must stay in jurisdiction
    FlexibleRegional, // Can move within region
    Global,           // Can move globally
}

/// Sovereignty policy
#[derive(Debug, Clone)]
pub struct SovereigntyPolicy {
    region: String,
    jurisdiction: Jurisdiction,
    regulations: HashSet<Regulation>,
    data_residency: DataResidency,
}

impl SovereigntyPolicy {
    pub fn new(region: &str) -> Self {
        Self {
            region: region.to_string(),
            jurisdiction: Jurisdiction::International,
            regulations: HashSet::new(),
            data_residency: DataResidency::Global,
        }
    }

    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = jurisdiction;
        self
    }

    pub fn with_regulation(mut self, regulation: Regulation) -> Self {
        self.regulations.insert(regulation);
        self
    }

    pub fn with_data_residency(mut self, residency: DataResidency) -> Self {
        self.data_residency = residency;
        self
    }

    pub fn inherit_from(mut self, parent: &SovereigntyPolicy) -> Self {
        self.data_residency = parent.data_residency;
        for reg in &parent.regulations {
            self.regulations.insert(*reg);
        }
        self
    }

    pub fn update_residency(&mut self, residency: DataResidency) {
        self.data_residency = residency;
    }

    pub fn region(&self) -> &str {
        &self.region
    }

    pub fn jurisdiction(&self) -> Jurisdiction {
        self.jurisdiction
    }

    pub fn data_residency(&self) -> DataResidency {
        self.data_residency
    }

    pub fn requires_local_storage(&self) -> bool {
        matches!(self.data_residency, DataResidency::StrictLocal)
    }

    pub fn requires_audit_trail(&self) -> bool {
        self.regulations.contains(&Regulation::GDPR)
            || self.regulations.contains(&Regulation::HIPAA)
    }

    pub fn audit_retention_days(&self) -> u32 {
        if self.regulations.contains(&Regulation::GDPR) {
            2555 // 7 years
        } else {
            365 // 1 year default
        }
    }

    pub fn required_frameworks(&self) -> Vec<&str> {
        let mut frameworks = Vec::new();
        for reg in &self.regulations {
            frameworks.push(match reg {
                Regulation::GDPR => "GDPR",
                Regulation::CCPA => "CCPA",
                Regulation::HIPAA => "HIPAA",
                Regulation::PIPEDA => "PIPEDA",
                Regulation::ISO27001 => "ISO27001",
                Regulation::SOC2 => "SOC2",
            });
        }
        frameworks
    }

    pub fn validate_data_location(&self, location: &DataLocation) -> Result<(), BearDogError> {
        match self.data_residency {
            DataResidency::StrictLocal => {
                if location.region() != self.region {
                    return Err(BearDogError::security(format!(
                        "Data must remain in {} jurisdiction",
                        self.region
                    )));
                }
            }
            DataResidency::FlexibleRegional => {
                // Allow within similar jurisdictions
            }
            DataResidency::Global => {
                // Allow anywhere
            }
        }
        Ok(())
    }

    pub fn regulation_count(&self) -> usize {
        self.regulations.len()
    }

    pub fn has_regulation(&self, regulation: Regulation) -> bool {
        self.regulations.contains(&regulation)
    }

    pub fn is_comprehensive(&self) -> bool {
        !self.regulations.is_empty() && !self.region.is_empty()
    }

    pub fn is_default(&self) -> bool {
        self.region == "default" || self.region.is_empty()
    }

    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.region.is_empty() {
            return Err(BearDogError::security("Region cannot be empty".to_string()));
        }
        Ok(())
    }
}

/// Data location
#[derive(Debug, Clone)]
pub struct DataLocation {
    resource_id: String,
    region: String,
    city: String,
}

impl DataLocation {
    pub fn new(resource_id: &str, region: &str, city: &str) -> Self {
        Self {
            resource_id: resource_id.to_string(),
            region: region.to_string(),
            city: city.to_string(),
        }
    }

    pub fn region(&self) -> &str {
        &self.region
    }
}
