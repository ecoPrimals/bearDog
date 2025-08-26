

pub mod adaptive;
pub mod circuit_breaker;
pub mod multi_criteria;
pub mod performance;
pub mod router;
pub mod traits;

pub use router::{RouterConfig, UniversalRequestRouter};

pub use traits::{RoutingContext, RoutingDecision, RoutingStrategy};

pub use adaptive::{AdaptiveRouting, LearningRouting, RoutingHistory};
pub use circuit_breaker::{CircuitBreakerRouting, CircuitConfig, CircuitState};
pub use multi_criteria::{ExecutionMetric, MultiCriteriaRouting, RoutingWeights};
pub use performance::{PerformanceFirstRouting, PerformanceMetric};

pub use adaptive::create_adaptive_routing;
pub use circuit_breaker::create_circuit_breaker_routing;
pub use multi_criteria::create_balanced_routing;
pub use performance::create_performance_routing;
