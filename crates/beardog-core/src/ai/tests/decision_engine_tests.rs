// Comprehensive tests for the Decision Engine module

use crate::ai::hybrid_intelligence::decision_engine::*;
use std::collections::HashMap;

#[test]
fn test_decision_strategy_variants() {
    let strategies = [
        DecisionStrategy::RuleBased,
        DecisionStrategy::MachineLearning,
        DecisionStrategy::ExpertSystem,
        DecisionStrategy::FuzzyLogic,
        DecisionStrategy::MultiCriteria,
        DecisionStrategy::GameTheory,
        DecisionStrategy::ReinforcementLearning,
        DecisionStrategy::Ensemble,
    ];

    assert_eq!(strategies.len(), 8);
}

#[test]
fn test_decision_strategy_serialization() {
    let strategy = DecisionStrategy::MachineLearning;
    let serialized = serde_json::to_string(&strategy).unwrap();
    let deserialized: DecisionStrategy = serde_json::from_str(&serialized).unwrap();
    assert_eq!(strategy, deserialized);
}

#[test]
fn test_criterion_type_variants() {
    let types = [
        CriterionType::Quantitative,
        CriterionType::Qualitative,
        CriterionType::Boolean,
        CriterionType::Categorical,
    ];

    assert_eq!(types.len(), 4);
}

#[test]
fn test_evaluation_function_variants() {
    let functions = [
        EvaluationFunction::Linear,
        EvaluationFunction::Exponential,
        EvaluationFunction::Logarithmic,
        EvaluationFunction::Step,
        EvaluationFunction::Sigmoid,
        EvaluationFunction::Custom("custom_fn".to_string()),
    ];

    assert_eq!(functions.len(), 6);
}

#[test]
fn test_evaluation_function_serialization() {
    let func = EvaluationFunction::Sigmoid;
    let serialized = serde_json::to_string(&func).unwrap();
    let deserialized: EvaluationFunction = serde_json::from_str(&serialized).unwrap();
    assert_eq!(func, deserialized);
}

#[test]
fn test_consensus_strategy_variants() {
    let strategies = [
        ConsensusStrategy::Majority,
        ConsensusStrategy::Weighted,
        ConsensusStrategy::Unanimous,
        ConsensusStrategy::Quorum,
        ConsensusStrategy::RankedChoice,
        ConsensusStrategy::BordaCount,
        ConsensusStrategy::Condorcet,
    ];

    assert_eq!(strategies.len(), 7);
}

#[test]
fn test_comparison_operator_variants() {
    let operators = [
        ComparisonOperator::Equal,
        ComparisonOperator::NotEqual,
        ComparisonOperator::GreaterThan,
        ComparisonOperator::LessThan,
        ComparisonOperator::GreaterThanOrEqual,
        ComparisonOperator::LessThanOrEqual,
        ComparisonOperator::Contains,
        ComparisonOperator::InRange,
        ComparisonOperator::MatchesPattern,
    ];

    assert_eq!(operators.len(), 9);
}

#[test]
fn test_condition_value_variants() {
    let values = [
        ConditionValue::Number(42.0),
        ConditionValue::String("test".to_string()),
        ConditionValue::Boolean(true),
        ConditionValue::List(vec!["a".to_string(), "b".to_string()]),
        ConditionValue::Range(0.0, 100.0),
    ];

    assert_eq!(values.len(), 5);
}

#[test]
fn test_condition_value_serialization() {
    let test_number = std::f64::consts::PI;
    let value = ConditionValue::Number(test_number);
    let serialized = serde_json::to_string(&value).unwrap();
    let deserialized: ConditionValue = serde_json::from_str(&serialized).unwrap();

    match deserialized {
        ConditionValue::Number(n) => assert!((n - std::f64::consts::PI).abs() < 0.001),
        _ => unreachable!("Expected Number variant"),
    }
}

#[test]
fn test_action_type_variants() {
    let actions = [
        ActionType::SetVariable,
        ActionType::ExecuteFunction,
        ActionType::SendNotification,
        ActionType::LogEvent,
        ActionType::TriggerWorkflow,
        ActionType::CustomAction,
    ];

    assert_eq!(actions.len(), 6);
}

#[test]
fn test_rule_evaluation_order_variants() {
    let orders = [
        RuleEvaluationOrder::Priority,
        RuleEvaluationOrder::Sequential,
        RuleEvaluationOrder::Parallel,
        RuleEvaluationOrder::Random,
    ];

    assert_eq!(orders.len(), 4);
}

#[test]
fn test_conflict_resolution_variants() {
    let resolutions = [
        ConflictResolution::FirstWins,
        ConflictResolution::LastWins,
        ConflictResolution::HighestPriority,
        ConflictResolution::HighestConfidence,
        ConflictResolution::CombineActions,
    ];

    assert_eq!(resolutions.len(), 5);
}

#[test]
fn test_membership_function_triangular() {
    let func = MembershipFunction::Triangular {
        a: 0.0,
        b: 50.0,
        c: 100.0,
    };

    match func {
        MembershipFunction::Triangular { a, b, c } => {
            assert!((a - 0.0).abs() < 1e-6);
            assert!((b - 50.0).abs() < 1e-6);
            assert!((c - 100.0).abs() < 1e-6);
        }
        _ => unreachable!("Expected Triangular variant"),
    }
}

#[test]
fn test_membership_function_gaussian() {
    let func = MembershipFunction::Gaussian {
        mean: 50.0,
        sigma: 10.0,
    };

    match func {
        MembershipFunction::Gaussian { mean, sigma } => {
            assert!((mean - 50.0).abs() < 1e-6);
            assert!((sigma - 10.0).abs() < 1e-6);
        }
        _ => unreachable!("Expected Gaussian variant"),
    }
}

#[test]
fn test_defuzzification_method_variants() {
    let methods = [
        DefuzzificationMethod::Centroid,
        DefuzzificationMethod::Bisector,
        DefuzzificationMethod::MeanOfMaximum,
        DefuzzificationMethod::SmallestOfMaximum,
        DefuzzificationMethod::LargestOfMaximum,
    ];

    assert_eq!(methods.len(), 5);
}

#[test]
fn test_mcda_method_variants() {
    let methods = [
        McdaMethod::WeightedSum,
        McdaMethod::Topsis,
        McdaMethod::Ahp,
        McdaMethod::Electre,
        McdaMethod::Promethee,
        McdaMethod::Vikor,
    ];

    assert_eq!(methods.len(), 6);
}

#[test]
fn test_optimization_direction_variants() {
    let directions = [
        OptimizationDirection::Maximize,
        OptimizationDirection::Minimize,
    ];

    assert_eq!(directions.len(), 2);
}

#[test]
fn test_game_type_variants() {
    let types = [
        GameType::Cooperative,
        GameType::NonCooperative,
        GameType::ZeroSum,
        GameType::NonZeroSum,
        GameType::Sequential,
        GameType::Simultaneous,
    ];

    assert_eq!(types.len(), 6);
}

#[test]
fn test_player_type_variants() {
    let types = [
        PlayerType::Rational,
        PlayerType::BoundedRational,
        PlayerType::Altruistic,
        PlayerType::Random,
    ];

    assert_eq!(types.len(), 4);
}

#[test]
fn test_solution_concept_variants() {
    let concepts = [
        SolutionConcept::NashEquilibrium,
        SolutionConcept::DominantStrategy,
        SolutionConcept::ParetoOptimal,
        SolutionConcept::Minimax,
        SolutionConcept::Maximin,
        SolutionConcept::ShapleyValue,
    ];

    assert_eq!(concepts.len(), 6);
}

#[test]
fn test_decision_method_type_variants() {
    let types = [
        DecisionMethodType::RuleBased,
        DecisionMethodType::MachineLearning,
        DecisionMethodType::FuzzyLogic,
        DecisionMethodType::MultiCriteria,
        DecisionMethodType::ExpertSystem,
    ];

    assert_eq!(types.len(), 5);
}

#[test]
fn test_combination_method_variants() {
    let methods = [
        CombinationMethod::Voting,
        CombinationMethod::WeightedVoting,
        CombinationMethod::Stacking,
        CombinationMethod::Bagging,
        CombinationMethod::Boosting,
        CombinationMethod::Averaging,
    ];

    assert_eq!(methods.len(), 6);
}

#[test]
fn test_decision_criteria_creation() {
    let mut weights = HashMap::new();
    weights.insert("cost".to_string(), 0.4);
    weights.insert("quality".to_string(), 0.6);

    let mut thresholds = HashMap::new();
    thresholds.insert("min_quality".to_string(), 0.7);

    let criteria = DecisionCriteria {
        primary_criteria: vec![],
        secondary_criteria: vec![],
        weights,
        thresholds,
    };

    assert_eq!(criteria.weights.len(), 2);
    assert_eq!(criteria.thresholds.len(), 1);
}

#[test]
fn test_criterion_creation() {
    let criterion = Criterion {
        name: "performance".to_string(),
        criterion_type: CriterionType::Quantitative,
        weight: 0.8,
        evaluation_function: EvaluationFunction::Linear,
    };

    assert_eq!(criterion.name, "performance");
    assert!((criterion.weight - 0.8).abs() < 1e-6);
}

#[test]
fn test_rule_condition_creation() {
    let condition = RuleCondition {
        variable: "temperature".to_string(),
        operator: ComparisonOperator::GreaterThan,
        value: ConditionValue::Number(25.0),
    };

    assert_eq!(condition.variable, "temperature");
    assert_eq!(condition.operator, ComparisonOperator::GreaterThan);
}

#[test]
fn test_rule_action_creation() {
    let mut parameters = HashMap::new();
    parameters.insert("action".to_string(), "increase_cooling".to_string());

    let action = RuleAction {
        action_type: ActionType::ExecuteFunction,
        parameters,
        expected_outcome: "temperature_reduced".to_string(),
    };

    assert_eq!(action.action_type, ActionType::ExecuteFunction);
    assert_eq!(action.expected_outcome, "temperature_reduced");
}

#[test]
fn test_decision_rule_creation() {
    let rule = DecisionRule {
        id: "rule_001".to_string(),
        conditions: vec![],
        action: RuleAction {
            action_type: ActionType::LogEvent,
            parameters: HashMap::new(),
            expected_outcome: "logged".to_string(),
        },
        priority: 10,
        confidence: 0.95,
    };

    assert_eq!(rule.id, "rule_001");
    assert_eq!(rule.priority, 10);
    assert!((rule.confidence - 0.95).abs() < 1e-6);
}

#[test]
fn test_fuzzy_set_creation() {
    let fuzzy_set = FuzzySet {
        name: "low_temperature".to_string(),
        membership_function: MembershipFunction::Triangular {
            a: 0.0,
            b: 10.0,
            c: 20.0,
        },
        universe: (0.0, 50.0),
    };

    assert_eq!(fuzzy_set.name, "low_temperature");
    assert_eq!(fuzzy_set.universe, (0.0, 50.0));
}

#[test]
fn test_fuzzy_expression_variable() {
    let expr = FuzzyExpression::Variable {
        variable: "temp".to_string(),
        fuzzy_set: "low".to_string(),
    };

    match expr {
        FuzzyExpression::Variable {
            variable,
            fuzzy_set,
        } => {
            assert_eq!(variable, "temp");
            assert_eq!(fuzzy_set, "low");
        }
        _ => unreachable!("Expected Variable variant"),
    }
}

#[test]
fn test_alternative_creation() {
    let mut values = HashMap::new();
    values.insert("cost".to_string(), 100.0);
    values.insert("quality".to_string(), 0.9);

    let alternative = Alternative {
        id: "alt_1".to_string(),
        name: "Option A".to_string(),
        values,
        metadata: HashMap::new(),
    };

    assert_eq!(alternative.id, "alt_1");
    assert_eq!(alternative.values.len(), 2);
}

#[test]
fn test_player_creation() {
    let player = Player {
        id: "player_1".to_string(),
        name: "AI Agent".to_string(),
        strategies: vec![],
        player_type: PlayerType::Rational,
    };

    assert_eq!(player.id, "player_1");
    assert_eq!(player.player_type, PlayerType::Rational);
}

#[test]
fn test_strategy_creation() {
    let mut parameters = HashMap::new();
    parameters.insert("aggressiveness".to_string(), 0.7);

    let strategy = Strategy {
        id: "strategy_1".to_string(),
        name: "Cooperative".to_string(),
        parameters,
    };

    assert_eq!(strategy.id, "strategy_1");
    assert_eq!(strategy.parameters.len(), 1);
}

#[test]
fn test_payoff_matrix_creation() {
    let mut player_mapping = HashMap::new();
    player_mapping.insert("player_1".to_string(), 0);
    player_mapping.insert("player_2".to_string(), 1);

    let payoff_matrix = PayoffMatrix {
        dimensions: vec![2, 2],
        payoffs: vec![vec![3.0, 0.0], vec![5.0, 1.0]],
        player_mapping,
    };

    assert_eq!(payoff_matrix.dimensions, vec![2, 2]);
    assert_eq!(payoff_matrix.payoffs.len(), 2);
}

#[test]
fn test_decision_engine_stats_creation() {
    let stats = DecisionEngineStats {
        decisions_made: 1000,
        avg_decision_time: 25.5,
        confidence_score: 0.85,
        human_interventions: 50,
        autonomy_rate: 0.95,
        error_count: 5,
        success_rate: 0.99,
        current_load: 45.0,
        peak_load: 78.0,
        feedback_wait_time: 120.0,
        pending_decisions: 3,
        uptime_seconds: 86400,
        memory_usage_bytes: 1024 * 1024 * 512,
    };

    assert_eq!(stats.decisions_made, 1000);
    assert!((stats.confidence_score - 0.85).abs() < 1e-6);
    assert!((stats.success_rate - 0.99).abs() < 1e-6);
}

#[test]
fn test_decision_engine_stats_serialization() {
    let stats = DecisionEngineStats {
        decisions_made: 500,
        avg_decision_time: 30.0,
        confidence_score: 0.9,
        human_interventions: 20,
        autonomy_rate: 0.96,
        error_count: 2,
        success_rate: 0.99,
        current_load: 50.0,
        peak_load: 75.0,
        feedback_wait_time: 100.0,
        pending_decisions: 1,
        uptime_seconds: 3600,
        memory_usage_bytes: 1024 * 1024 * 256,
    };

    let serialized = serde_json::to_string(&stats).unwrap();
    let deserialized: DecisionEngineStats = serde_json::from_str(&serialized).unwrap();

    assert_eq!(stats.decisions_made, deserialized.decisions_made);
    assert!((stats.confidence_score - deserialized.confidence_score).abs() < 1e-6);
}
