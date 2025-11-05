// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use crate::zero_cost::types::{
    ArcDynAnalysisReport, ArcDynPattern, MigrationComplexity, ZeroCostMigrationPlan,
};

/// Zero-cost migration analyzer and planner
pub struct ZeroCostMigrator;

impl ZeroCostMigrator {
    pub fn analyze_arc_dyn_patterns(crate_name: &str) -> ArcDynAnalysisReport {
        ArcDynAnalysisReport {
            crate_name: crate_name.to_string(),
            total_arc_dyn_patterns: 0,
            migratable_patterns: 0,
            estimated_performance_gain: 0.0,
            migration_complexity: MigrationComplexity::Low,
        }
    }

    /// Generate a comprehensive migration plan from identified patterns
    pub fn generate_migration_plan(patterns: Vec<ArcDynPattern>) -> ZeroCostMigrationPlan {
        ZeroCostMigrationPlan {
            patterns,
            estimated_compilation_time_increase: 1.05, // 5% increase
            estimated_runtime_improvement: 0.25,       // 25% improvement
            migration_steps: vec![
                "Convert Arc<dyn Trait> to generic parameters".to_string(),
                "Replace async_trait with native fn".to_string(),
                "Add const generic capabilities".to_string(),
                "Update call sites to use generics".to_string(),
            ],
        }
    }

    pub fn analyze_performance_impact(patterns: &[ArcDynPattern]) -> f64 {
        // Estimate performance improvement based on pattern complexity
        let mut total_impact = 0.0;

        for pattern in patterns {
            let impact = match pattern.migration_difficulty {
                MigrationComplexity::Low => 0.15,    // 15% improvement
                MigrationComplexity::Medium => 0.25, // 25% improvement
                MigrationComplexity::High => 0.40,   // 40% improvement
            };
            total_impact += impact;
        }

        // Average the impact across all patterns
        if patterns.is_empty() {
            0.0
        } else {
            total_impact / patterns.len() as f64
        }
    }

    pub fn estimate_migration_effort(patterns: &[ArcDynPattern]) -> f64 {
        let mut total_hours = 0.0;

        for pattern in patterns {
            let hours = match pattern.migration_difficulty {
                MigrationComplexity::Low => 2.0,    // 2 hours
                MigrationComplexity::Medium => 8.0, // 8 hours
                MigrationComplexity::High => 24.0,  // 24 hours
            };
            total_hours += hours;
        }

        total_hours
    }

    /// Generate migration priority recommendations
    pub fn prioritize_migrations(patterns: Vec<ArcDynPattern>) -> Vec<ArcDynPattern> {
        let mut prioritized = patterns;

        // Sort by impact (high complexity first, then by usage context)
        prioritized.sort_by(|a, b| {
            // Primary sort: migration difficulty (high impact first)
            let difficulty_cmp = match (
                a.migration_difficulty.clone(),
                b.migration_difficulty.clone(),
            ) {
                (MigrationComplexity::High, MigrationComplexity::High) => std::cmp::Ordering::Equal,
                (MigrationComplexity::High, _) => std::cmp::Ordering::Less,
                (_, MigrationComplexity::High) => std::cmp::Ordering::Greater,
                (MigrationComplexity::Medium, MigrationComplexity::Medium) => {
                    std::cmp::Ordering::Equal
                }
                (MigrationComplexity::Medium, _) => std::cmp::Ordering::Less,
                (_, MigrationComplexity::Medium) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            };

            if difficulty_cmp != std::cmp::Ordering::Equal {
                return difficulty_cmp;
            }

            // Secondary sort: usage context (hot paths first)
            match (
                a.usage_context.contains("hot"),
                b.usage_context.contains("hot"),
            ) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.trait_name.cmp(&b.trait_name),
            }
        });

        prioritized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_analysis() {
        let report = ZeroCostMigrator::analyze_arc_dyn_patterns("test-crate");
        assert_eq!(report.crate_name, "test-crate");
        assert_eq!(report.total_arc_dyn_patterns, 0);
        assert_eq!(report.migratable_patterns, 0);
    }

    #[test]
    fn test_migration_plan_generation() {
        let patterns = vec![ArcDynPattern {
            trait_name: "TestTrait".to_string(),
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            file_path: "src/test.rs".to_string(),
            line_number: 42,
            usage_context: "struct field".to_string(),
            migration_difficulty: MigrationComplexity::Low,
        }];
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: types
 // TEST_PRIORITY: normal

        let plan = ZeroCostMigrator::generate_migration_plan(patterns);
        assert!(!plan.migration_steps.is_empty());
        assert!(plan.estimated_runtime_improvement > 0.0);
        assert!(plan.estimated_compilation_time_increase > 1.0);
    }

    #[test]
    fn test_performance_impact_analysis() {
        let patterns = vec![
            ArcDynPattern {
                trait_name: "LowImpact".to_string(),
                file_path: "src/low.rs".to_string(),
                // TEST_CATEGORY: unit
                // TEST_DOMAIN: types
                // TEST_PRIORITY: normal
                line_number: 10,
                usage_context: "parameter".to_string(),
                migration_difficulty: MigrationComplexity::Low,
            },
            ArcDynPattern {
                trait_name: "HighImpact".to_string(),
                file_path: "src/high.rs".to_string(),
                line_number: 20,
                usage_context: "hot path".to_string(),
                migration_difficulty: MigrationComplexity::High,
            },
        ];

        let impact = ZeroCostMigrator::analyze_performance_impact(&patterns);
        assert!(impact > 0.0);
        assert!(impact < 1.0); // Should be a reasonable percentage
    }

    #[test]
    fn test_migration_effort_estimation() {
        let patterns = vec![
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            ArcDynPattern {
                trait_name: "Simple".to_string(),
                file_path: "src/simple.rs".to_string(),
                line_number: 5,
                usage_context: "return type".to_string(),
                migration_difficulty: MigrationComplexity::Low,
            },
            ArcDynPattern {
                trait_name: "Complex".to_string(),
                file_path: "src/complex.rs".to_string(),
                line_number: 15,
                usage_context: "async trait".to_string(),
                migration_difficulty: MigrationComplexity::High,
            },
        ];

        let effort = ZeroCostMigrator::estimate_migration_effort(&patterns);
        assert_eq!(effort, 26.0); // 2 + 24 hours
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_migration_prioritization() {
        let patterns = vec![
            ArcDynPattern {
                trait_name: "LowPriority".to_string(),
                file_path: "src/low.rs".to_string(),
                line_number: 10,
                usage_context: "cold path".to_string(),
                migration_difficulty: MigrationComplexity::Low,
            },
            ArcDynPattern {
                trait_name: "HighPriority".to_string(),
                file_path: "src/high.rs".to_string(),
                line_number: 20,
                usage_context: "hot path".to_string(),
                migration_difficulty: MigrationComplexity::High,
            },
            ArcDynPattern {
                trait_name: "MediumPriority".to_string(),
                file_path: "src/medium.rs".to_string(),
                line_number: 15,
                usage_context: "service layer".to_string(),
                migration_difficulty: MigrationComplexity::Medium,
            },
        ];

        let prioritized = ZeroCostMigrator::prioritize_migrations(patterns);

        // High complexity should come first
        assert!(matches!(
            prioritized[0].migration_difficulty,
            MigrationComplexity::High
        ));
        assert!(matches!(
            prioritized[1].migration_difficulty,
            MigrationComplexity::Medium
        ));
        assert!(matches!(
            prioritized[2].migration_difficulty,
            MigrationComplexity::Low
        ));
    }
}
