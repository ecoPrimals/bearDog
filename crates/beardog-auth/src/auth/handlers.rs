

use beardog_errors::BearDogResult;

pub struct CrossNodeAuthEngine {
    config: UnifiedAuthConfig,
}

#[derive(Default)]
#[deprecated(since = "3.1.0", note = "Use UnifiedAuthConfig instead")]
#[deprecated(since = "3.1.0", note = "Use UnifiedAuthConfig instead")]
pub struct AuthConfig {
    pub require_consensus: bool,
}

impl CrossNodeAuthEngine {
    pub fn new() -> Self {
        Self {
            config: UnifiedAuthConfig::default(),
        }
    }
    
    pub fn with_config(config: UnifiedAuthConfig) -> Self {
        Self { config }
    }
    
    pub fn set_workflow_engine(&mut self, _workflow_engine: Box<dyn WorkflowEngine + Send + Sync>) {

    }
    
    pub async fn authorize_operation(
        &self,
        _operation: &str,
        _context: &str,
    ) -> BearDogResult<AuthorizationResult> {
        Ok(AuthorizationResult::Allow)
    }
}

pub use beardog_traits::WorkflowProvider;

#[derive(Debug)]
pub enum AuthorizationResult {
    Allow,
    Deny,
}

pub struct ConsensusResult {
    pub approved: bool,
}
