// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    CoordinationConfig, CoordinationModel, assess_coordination_health, migrate_from_primary_replica,
};

#[test]
fn default_config_validates() {
    let config = CoordinationConfig::default();
    assert!(config.validate().is_ok());
}

#[test]
fn distributed_constructor_sets_model() {
    let config = CoordinationConfig::distributed();
    assert!(matches!(
        config.coordination_model,
        CoordinationModel::Distributed { .. }
    ));
}

#[test]
fn get_coordination_type_matches_variant() {
    let config = CoordinationConfig::rotational();
    assert_eq!(config.get_coordination_type(), "rotational");
}

#[test]
fn migrate_from_primary_replica_produces_collaborative() {
    let config = migrate_from_primary_replica("primary", &["r1".to_string()]).expect("migrate");
    assert!(matches!(
        config.coordination_model,
        CoordinationModel::Collaborative { .. }
    ));
}

#[test]
fn assess_coordination_health_returns_score() {
    let config = CoordinationConfig::default();
    let score = assess_coordination_health(&config);
    assert!((score - 0.8).abs() < f64::EPSILON);
}
