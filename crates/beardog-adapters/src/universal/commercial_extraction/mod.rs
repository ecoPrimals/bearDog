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


/// Commercial Extraction Detection Module
///
/// Revolutionary system for distinguishing human users from commercial extraction attempts.
/// Implements the core principle: "Open gates for humans, locked tight for commercial extraction"
pub mod detector;
pub mod implementation;

// Re-export specific types to avoid UniversalRequest ambiguity
pub use detector::{
    AccessLevel, CommercialClassification, CommercialExtractionDetector, EntropyHistory,
    ExtractionRisk, GeneticKeyEvolutionEngine, HumanEntropyUsage, UsagePattern,
};
// Note: UniversalRequest is intentionally not re-exported to avoid ambiguity
