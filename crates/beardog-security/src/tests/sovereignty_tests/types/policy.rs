// SPDX-License-Identifier: AGPL-3.0-or-later

//! Policy Resolution and Data Flow Types
//!
//! Test helper types for policy resolution and data flow analysis.

use super::jurisdiction::{DataResidency, Jurisdiction, SovereigntyPolicy};

/// Policy resolver
pub struct PolicyResolver;

impl PolicyResolver {
    pub fn resolve(policies: &[SovereigntyPolicy]) -> SovereigntyPolicy {
        if policies.is_empty() {
            return SovereigntyPolicy::new("default");
        }

        // Use strictest policy
        let mut result = policies[0].clone();
        for policy in policies.iter().skip(1) {
            if matches!(policy.data_residency(), DataResidency::StrictLocal) {
                result.update_residency(DataResidency::StrictLocal);
            }
        }
        result
    }
}

/// Data flow
#[derive(Debug, Clone)]
pub struct DataFlow {
    source: Jurisdiction,
    destination: Jurisdiction,
}

impl DataFlow {
    pub fn new(source: Jurisdiction, destination: Jurisdiction) -> Self {
        Self {
            source,
            destination,
        }
    }

    pub fn requires_special_authorization(&self) -> bool {
        self.source != self.destination
            && (self.source == Jurisdiction::EuropeanUnion
                || self.destination == Jurisdiction::EuropeanUnion)
    }

    pub fn needs_consent(&self) -> bool {
        self.requires_special_authorization()
    }
}
