// SPDX-License-Identifier: AGPL-3.0-only

// Decision making logic and algorithms

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Removed unused Duration import

/// Decision strategies available
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DecisionStrategy {
    /// Rule-based decision making
    RuleBased,
    /// Machine learning-based decisions
    MachineLearning,
    /// Expert system decisions
    ExpertSystem,
    /// Fuzzy logic decisions
    FuzzyLogic,
    /// Multi-criteria decision analysis
    MultiCriteria,
    /// Game theory-based decisions
    GameTheory,
    /// Reinforcement learning-based decisions
    ReinforcementLearning,
    /// Ensemble decision making
    Ensemble,
}

/// Decision criteria configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionCriteria {
    /// Primary criteria
    /// Collection of primary criteria
    pub primary_criteria: Vec<Criterion>,
    /// Secondary criteria
    /// Collection of secondary criteria
    pub secondary_criteria: Vec<Criterion>,
    /// Mapping of weights
    pub weights: HashMap<String, f64>,
    /// Threshold values
    /// Mapping of thresholds
    pub thresholds: HashMap<String, f64>,
}

/// Individual decision criterion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Criterion {
    /// Criterion name
    /// Name of the item
    pub name: String,
    /// Criterion type
    /// The criterion type value
    pub criterion_type: CriterionType,
    /// Importance weight
    /// The weight value
    pub weight: f64,
    /// Evaluation function
    /// The evaluation function value
    pub evaluation_function: EvaluationFunction,
}

/// Types of decision criteria
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
/// Types of criterion
pub enum CriterionType {
    /// Quantitative criterion
    Quantitative,
    /// Qualitative criterion
    Qualitative,
    /// Boolean criterion
    Boolean,
    /// Categorical criterion
    Categorical,
}

/// Evaluation function types for criterion assessment
///
/// Defines mathematical functions used to evaluate decision criteria
/// and transform raw values into normalized scores.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EvaluationFunction {
    /// Linear evaluation
    Linear,
    /// Exponential evaluation
    Exponential,
    /// Logarithmic evaluation
    Logarithmic,
    /// Step function evaluation
    Step,
    /// Sigmoid evaluation
    Sigmoid,
    /// Custom evaluation function
    Custom(String),
}

/// Consensus strategy for multi-agent decisions
///
/// Specifies how multiple decision makers reach consensus
/// in collaborative decision scenarios.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsensusStrategy {
    /// Simple majority voting
    Majority,
    /// Weighted voting
    Weighted,
    /// Unanimous consensus
    Unanimous,
    /// Quorum-based decision
    Quorum,
    /// Ranked choice voting
    RankedChoice,
    /// Borda count method
    BordaCount,
    /// Condorcet method
    Condorcet,
}

/// Decision priority levels
///
/// Indicates the urgency and importance of a decision,
/// affecting scheduling and resource allocation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DecisionPriority {
    /// Low priority tasks
    Low,
    /// Medium priority tasks
    Medium,
    /// High priority tasks
    High,
    /// Critical priority tasks
    Critical,
}

/// Rule-based decision configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleBasedConfig {
    /// Decision rules
    /// Collection of rules
    pub rules: Vec<DecisionRule>,
    /// Rule evaluation order
    /// The evaluation order value
    pub evaluation_order: RuleEvaluationOrder,
    /// Conflict resolution strategy
    /// The conflict resolution value
    pub conflict_resolution: ConflictResolution,
}

/// Individual decision rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionRule {
    /// Rule identifier
    pub id: String,
    /// Rule conditions
    /// Collection of conditions
    pub conditions: Vec<RuleCondition>,
    /// Rule action
    /// The action value
    pub action: RuleAction,
    /// Rule priority
    /// Number of priority
    pub priority: u32,
    /// Rule confidence
    pub confidence: f64,
}

/// Rule condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    /// Variable name
    /// The variable value
    pub variable: String,
    /// Comparison operator
    /// The operator value
    pub operator: ComparisonOperator,
    /// Comparison value
    /// The value value
    pub value: ConditionValue,
}

/// Comparison operators for decision criteria
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComparisonOperator {
    /// Equal to
    Equal,
    /// Not equal to
    NotEqual,
    /// Greater than
    GreaterThan,
    /// Less than
    LessThan,
    /// Greater than or equal to
    GreaterThanOrEqual,
    /// Less than or equal to
    LessThanOrEqual,
    /// Contains
    Contains,
    /// In range
    InRange,
    /// Matches pattern
    MatchesPattern,
}

/// Condition values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionValue {
    /// Numeric value
    Number(f64),
    /// String value
    String(String),
    /// Boolean value
    Boolean(bool),
    /// List of values
    List(Vec<String>),
    /// Range of values
    Range(f64, f64),
}

/// Rule actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleAction {
    /// Action type
    /// The action type value
    pub action_type: ActionType,
    /// Action parameters
    /// Mapping of parameters
    pub parameters: HashMap<String, String>,
    /// Expected outcome
    /// The expected outcome value
    pub expected_outcome: String,
}

/// Types of rule actions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
/// Types of action
pub enum ActionType {
    /// Set variable value
    SetVariable,
    /// Execute function
    ExecuteFunction,
    /// Send notification
    SendNotification,
    /// Log event
    LogEvent,
    /// Trigger workflow
    TriggerWorkflow,
    /// Custom action
    CustomAction,
}

/// Rule evaluation orders
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RuleEvaluationOrder {
    /// Priority-based order
    Priority,
    /// Sequential order
    Sequential,
    /// Parallel evaluation
    Parallel,
    /// Random order
    Random,
}

/// Conflict resolution strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConflictResolution {
    /// First rule wins
    FirstWins,
    /// Last rule wins
    LastWins,
    /// Highest priority wins
    HighestPriority,
    /// Highest confidence wins
    HighestConfidence,
    /// Combine all actions
    CombineActions,
}

/// Fuzzy logic configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzyLogicConfig {
    /// Fuzzy sets
    /// Collection of fuzzy sets
    pub fuzzy_sets: Vec<FuzzySet>,
    /// Fuzzy rules
    /// Collection of fuzzy rules
    pub fuzzy_rules: Vec<FuzzyRule>,
    /// Defuzzification method
    /// The defuzzification value
    pub defuzzification: DefuzzificationMethod,
}

/// Fuzzy set definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzySet {
    /// Set name
    /// Name of the item
    pub name: String,
    /// Membership function
    /// The membership function value
    pub membership_function: MembershipFunction,
    /// Universe of discourse
    /// The universe value
    pub universe: (f64, f64),
}

/// Membership functions for fuzzy logic
///
/// Defines the shape of fuzzy set membership functions. Different function types
/// are suited for different fuzzy logic applications - triangular for simplicity,
/// Gaussian for smoothness, trapezoidal for stability regions, and sigmoid for
/// smooth transitions.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MembershipFunction {
    /// Triangular membership function with three control points
    Triangular {
        /// Left base point of the triangle
        a: f64,
        /// Peak point of the triangle
        b: f64,
        /// Right base point of the triangle
        c: f64,
    },
    /// Trapezoidal membership function with four control points
    Trapezoidal {
        /// Left base point of the trapezoid
        a: f64,
        /// Left top point of the trapezoid
        b: f64,
        /// Right top point of the trapezoid
        c: f64,
        /// Right base point of the trapezoid
        d: f64,
    },
    /// Gaussian membership function with mean and standard deviation
    Gaussian {
        /// Mean value of the Gaussian distribution
        mean: f64,
        /// Standard deviation of the Gaussian distribution
        sigma: f64,
    },
    /// Sigmoid membership function with slope and center parameters
    Sigmoid {
        /// Slope parameter controlling steepness
        a: f64,
        /// Center point of the sigmoid function
        c: f64,
    },
}

/// Fuzzy rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzyRule {
    /// Rule identifier
    pub id: String,
    /// Antecedent (IF part)
    /// The antecedent value
    pub antecedent: FuzzyExpression,
    /// Consequent (THEN part)
    /// The consequent value
    pub consequent: FuzzyExpression,
    /// Rule weight
    /// The weight value
    pub weight: f64,
}

/// Fuzzy logic expressions for rules
///
/// Represents logical expressions in fuzzy logic rules, supporting variables,
/// AND/OR operations, and NOT operations for building complex rule conditions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FuzzyExpression {
    /// A fuzzy variable with its associated fuzzy set
    Variable {
        /// Name of the fuzzy variable
        variable: String,
        /// Name of the fuzzy set associated with the variable
        fuzzy_set: String,
    },
    /// AND operation
    And(Box<FuzzyExpression>, Box<FuzzyExpression>),
    /// OR operation
    Or(Box<FuzzyExpression>, Box<FuzzyExpression>),
    /// NOT operation
    Not(Box<FuzzyExpression>),
}

/// Defuzzification methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DefuzzificationMethod {
    /// Centroid method
    Centroid,
    /// Bisector method
    Bisector,
    /// Mean of maximum
    MeanOfMaximum,
    /// Smallest of maximum
    SmallestOfMaximum,
    /// Largest of maximum
    LargestOfMaximum,
}

/// Multi-criteria decision analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiCriteriaConfig {
    /// MCDA method
    /// The method value
    pub method: McdaMethod,
    /// Criteria definitions
    /// Collection of criteria
    pub criteria: Vec<McdaCriterion>,
    /// Alternatives to evaluate
    /// Collection of alternatives
    pub alternatives: Vec<Alternative>,
    /// Optional preferences
    pub preferences: Option<PreferenceInformation>,
}

/// MCDA methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum McdaMethod {
    /// Weighted Sum Model
    WeightedSum,
    /// Technique for Order of Preference by Similarity to Ideal Solution
    Topsis,
    /// Analytic Hierarchy Process
    Ahp,
    /// `ELimination` Et Choix Traduisant la `REalité`
    Electre,
    /// Preference Ranking Organization `METHod`
    Promethee,
    /// `VlseKriterijumska` Optimizacija I Kompromisno Resenje
    Vikor,
}

/// MCDA criterion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McdaCriterion {
    /// Criterion name
    /// Name of the item
    pub name: String,
    /// Criterion weight
    /// The weight value
    pub weight: f64,
    /// Optimization direction
    /// The direction value
    pub direction: OptimizationDirection,
    /// Criterion type
    /// The criterion type value
    pub criterion_type: CriterionDataType,
}

/// Optimization directions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizationDirection {
    /// Maximize the criterion
    Maximize,
    /// Minimize the criterion
    Minimize,
}

/// Criterion data types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
/// Types of criterion data
pub enum CriterionDataType {
    /// Numeric criterion
    Numeric,
    /// Ordinal criterion
    Ordinal,
    /// Categorical criterion
    Categorical,
}

/// Decision alternative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alternative {
    /// Alternative identifier
    pub id: String,
    /// Alternative name
    /// Name of the item
    pub name: String,
    /// Criterion values
    /// Mapping of values
    pub values: HashMap<String, f64>,
    /// Alternative metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

/// Preference information for multi-criteria decision analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferenceInformation {
    /// Preference thresholds
    /// Mapping of thresholds
    pub thresholds: HashMap<String, f64>,
    /// Indifference thresholds
    /// Mapping of indifference
    pub indifference: HashMap<String, f64>,
    /// Veto thresholds
    /// Mapping of veto
    pub veto: HashMap<String, f64>,
}

/// Game theory configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTheoryConfig {
    /// Game type
    /// The game type value
    pub game_type: GameType,
    /// Players in the game
    /// Collection of players
    pub players: Vec<Player>,
    /// Payoff matrix
    /// The payoff matrix value
    pub payoff_matrix: PayoffMatrix,
    /// Solution concept
    /// The solution concept value
    pub solution_concept: SolutionConcept,
}

/// Game types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
/// Types of game
pub enum GameType {
    /// Cooperative game
    Cooperative,
    /// Non-cooperative game
    NonCooperative,
    /// Zero-sum game
    ZeroSum,
    /// Non-zero-sum game
    NonZeroSum,
    /// Sequential game
    Sequential,
    /// Simultaneous game
    Simultaneous,
}

/// Game player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// Player identifier
    pub id: String,
    /// Player name
    /// Name of the item
    pub name: String,
    /// Available strategies
    /// Collection of strategies
    pub strategies: Vec<Strategy>,
    /// Player type
    /// The player type value
    pub player_type: PlayerType,
}

/// Player types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
/// Types of player
pub enum PlayerType {
    /// Rational player
    Rational,
    /// Bounded rational player
    BoundedRational,
    /// Altruistic player
    Altruistic,
    /// Random player
    Random,
}

/// Player strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Strategy {
    /// Strategy identifier
    pub id: String,
    /// Strategy name
    /// Name of the item
    pub name: String,
    /// Strategy parameters
    /// Mapping of parameters
    pub parameters: HashMap<String, f64>,
}

/// Payoff matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoffMatrix {
    /// Matrix dimensions
    /// Collection of dimensions
    pub dimensions: Vec<u32>,
    /// Payoff values
    /// Collection of payoffs
    pub payoffs: Vec<Vec<f64>>,
    /// Player mapping
    /// Mapping of player mapping
    pub player_mapping: HashMap<String, u32>,
}

/// Solution concepts for game-theoretic analysis
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SolutionConcept {
    /// Nash equilibrium
    NashEquilibrium,
    /// Dominant strategy
    DominantStrategy,
    /// Pareto optimal
    ParetoOptimal,
    /// Minimax solution
    Minimax,
    /// Maximin solution
    Maximin,
    /// Shapley value
    ShapleyValue,
}

/// Ensemble decision configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsembleConfig {
    /// Base decision methods
    /// Collection of base methods
    pub base_methods: Vec<DecisionMethod>,
    /// Combination method
    /// The combination method value
    pub combination_method: CombinationMethod,
    /// Voting weights
    /// Optional weights
    pub weights: Option<HashMap<String, f64>>,
}

/// Decision-making method configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionMethod {
    /// Method identifier
    pub id: String,
    /// Method type
    /// The method type value
    pub method_type: DecisionMethodType,
    /// Method configuration
    pub configuration: HashMap<String, serde_json::Value>,
}

/// Decision method types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
/// Types of decision method
pub enum DecisionMethodType {
    /// Rule-based method
    RuleBased,
    /// Machine learning method
    MachineLearning,
    /// Fuzzy logic method
    FuzzyLogic,
    /// Multi-criteria method
    MultiCriteria,
    /// Expert system method
    ExpertSystem,
}

/// Methods for combining multiple decision results
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CombinationMethod {
    /// Simple voting
    Voting,
    /// Weighted voting
    WeightedVoting,
    /// Stacking
    Stacking,
    /// Bagging
    Bagging,
    /// Boosting
    Boosting,
    /// Averaging
    Averaging,
}

/// Statistics for decision engine operations
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DecisionEngineStats {
    /// Total number of decisions processed
    /// Number of `decisions_made`
    pub decisions_made: u64,
    /// Average decision processing time in milliseconds
    pub avg_decision_time: f64,
    /// Average confidence score of decisions
    pub confidence_score: f64,
    /// Number of decisions that required human intervention
    /// Number of `human_interventions`
    pub human_interventions: u64,
    /// Percentage of autonomous decisions made successfully
    /// The autonomy rate value
    pub autonomy_rate: f64,
    /// Number of errors encountered during decision making
    /// Number of error
    pub error_count: u64,
    /// Success rate of implemented decisions
    /// The success rate value
    pub success_rate: f64,
    /// Current processing load as a percentage (0.0-100.0)
    pub current_load: f64,
    /// Maximum processing capacity reached
    /// The peak load value
    pub peak_load: f64,
    /// Average time waiting for human feedback in milliseconds
    pub feedback_wait_time: f64,
    /// Number of decisions currently pending
    /// Number of `pending_decisions`
    pub pending_decisions: u32,
    /// System uptime in seconds since last restart
    pub uptime_seconds: u64,
    /// Total memory usage in bytes
    /// Number of `memory_usage_bytes`
    pub memory_usage_bytes: u64,
}
