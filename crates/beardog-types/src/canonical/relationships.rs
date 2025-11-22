// Canonical Relationship System - Ecosystem Evolution Patterns
//
// This module implements the horizontal gene transfer patterns from the parent
// ecosystem, replacing binary allowlist/blocklist patterns with spectrum-based
// relationship intelligence.
//
// ## Architecture
//
// Based on biological ecosystem patterns from parent reference:
// - EcosystemMembership: Spectrum-based access control
// - SymbioticCoordination: Collaborative relationship models  
// - TrustEvolution: Dynamic trust building and healing
// - BiologicalRelationships: Mutualistic ecosystem interactions

use beardog_errors::BearDogError;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Ecosystem relationship modules
pub mod membership;
pub mod coordination;
pub mod trust_dynamics;
pub mod biological_types;

// Re-export core relationship types
pub use membership::{EcosystemMembership, MembershipTransition, MembershipRegistry};
pub use coordination::{SymbioticCoordination, CoordinationModel, CollaborationTerms};
pub use trust_dynamics::{TrustEvolution, TrustLevel, TrustTransition};
pub use biological_types::{BiologicalRelationship, RelationshipType, EcosystemRole};

/// Canonical relationship context for all ecosystem interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipContext {
    /// Current ecosystem membership level
    pub membership: EcosystemMembership,
    
    /// Active coordination model
    pub coordination: SymbioticCoordination,
    
    /// Trust evolution state
    pub trust_state: TrustEvolution,
    
    /// Biological relationship type
    pub biological_type: BiologicalRelationship,
    
    /// Relationship metadata
    pub metadata: RelationshipMetadata,
}

/// Metadata about relationship interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipMetadata {
    /// When this relationship was established
    pub established_at: SystemTime,
    
    /// Last interaction timestamp
    pub last_interaction: SystemTime,
    
    /// Total interaction count
    pub interaction_count: u64,
    
    /// Relationship health score (0.0-1.0)
    pub health_score: f64,
    
    /// Custom relationship attributes
    pub attributes: HashMap<String, String>,
}

/// Relationship registry for managing ecosystem relationships
#[derive(Debug, Clone)]
pub struct EcosystemRelationshipRegistry {
    /// Active relationships by entity ID
    relationships: HashMap<String, RelationshipContext>,
    
    /// Relationship transition history for auditing and analysis
    /// Tracks all changes to relationships over time
    transition_history: Vec<RelationshipTransition>,
    
    /// Registry configuration
    config: RelationshipRegistryConfig,
}

/// Configuration for the relationship registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipRegistryConfig {
    /// Maximum relationships to track
    pub max_relationships: usize,
    
    /// Trust decay rate over time
    pub trust_decay_rate: f64,
    
    /// Minimum trust level for interactions
    pub min_trust_threshold: f64,
    
    /// Health check interval
    pub health_check_interval: Duration,
}

/// Relationship state transition record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipTransition {
    /// Entity involved in transition
    pub entity_id: String,
    
    /// Previous relationship state
    pub from_state: RelationshipContext,
    
    /// New relationship state
    pub to_state: RelationshipContext,
    
    /// Reason for transition
    pub transition_reason: String,
    
    /// When transition occurred
    pub timestamp: SystemTime,
}

impl EcosystemRelationshipRegistry {
    /// Create a new relationship registry
    pub fn new(config: RelationshipRegistryConfig) -> Self {
        Self {
            relationships: HashMap::new(),
            transition_history: Vec::new(),
            config,
        }
    }
    
    /// Register a new relationship
    pub fn register_relationship(
        &mut self,
        entity_id: String,
        context: RelationshipContext,
    ) -> Result<()> {
        if self.relationships.len() >= self.config.max_relationships {
            return Err(BearDogError::system("Relationship registry at capacity".to_string()));
        }
        
        // Track transition if this is an update
        if let Some(old_context) = self.relationships.get(&entity_id) {
            self.record_transition(&entity_id, old_context.clone(), context.clone(), "relationship_update");
        }
        
        self.relationships.insert(entity_id, context);
        Ok(())
    }
    
    /// Get relationship context for an entity
    pub fn get_relationship(&self, entity_id: &str) -> Option<&RelationshipContext> {
        self.relationships.get(entity_id)
    }
    
    /// Update relationship based on interaction
    pub fn update_relationship(
        &mut self,
        entity_id: &str,
        interaction_result: InteractionResult,
    ) -> Result<()> {
        // Clone the relationship for health calculation to avoid borrowing issues
        let cloned_relationship = {
            let relationship = self.relationships.get_mut(entity_id)
                .ok_or_else(|| BearDogError::business("Relationship not found".to_string()))?;
                
            // Update interaction metadata
            relationship.metadata.last_interaction = SystemTime::now();
            relationship.metadata.interaction_count += 1;
            
            // Evolve trust based on interaction
            relationship.trust_state.evolve_from_interaction(&interaction_result)?;
            
            relationship.clone()
        };
        
        // Calculate health score with cloned relationship
        let health_score = self.calculate_health_score(entity_id, &cloned_relationship);
        
        // Update health score
        if let Some(relationship) = self.relationships.get_mut(entity_id) {
            relationship.metadata.health_score = health_score;
        }
        
        Ok(())
    }
    
    /// Calculate relationship health score
    fn calculate_health_score(&self, entity_id: &str, relationship: &RelationshipContext) -> f64 {
        let trust_score = relationship.trust_state.current_level();
        let interaction_recency = self.calculate_interaction_recency(relationship);
        let stability_score = self.calculate_stability_score(entity_id, relationship);
        
        (trust_score + interaction_recency + stability_score) / 3.0
    }
    
    /// Calculate how recent interactions are (0.0-1.0)
    fn calculate_interaction_recency(&self, relationship: &RelationshipContext) -> f64 {
        let now = SystemTime::now();
        let last_interaction = relationship.metadata.last_interaction;
        
        match now.duration_since(last_interaction) {
            Ok(duration) => {
                let hours_since = duration.as_secs() as f64 / 3600.0;
                (1.0 / (1.0 + hours_since * 0.1)).max(0.0).min(1.0)
            }
            Err(_) => 0.0,
        }
    }
    
    /// Calculate relationship stability (0.0-1.0)
    fn calculate_stability_score(&self, entity_id: &str, _relationship: &RelationshipContext) -> f64 {
        // Analyze transition history for stability
        let entity_transitions = self.get_transition_history(entity_id);
        if entity_transitions.is_empty() {
            return 0.8; // Default stability for new relationships
        }
        
        // More transitions in recent time = less stability
        let recent_transitions = entity_transitions.len() as f64;
        (1.0 / (1.0 + recent_transitions * 0.1)).max(0.1).min(1.0)
    }
    
    /// Record a relationship transition for audit trail
    pub fn record_transition(
        &mut self,
        entity_id: &str,
        from_state: RelationshipContext,
        to_state: RelationshipContext,
        reason: &str,
    ) {
        let transition = RelationshipTransition {
            entity_id: entity_id.to_string(),
            from_state,
            to_state,
            transition_reason: reason.to_string(),
            timestamp: SystemTime::now(),
        };
        
        self.transition_history.push(transition);
        
        // Keep history size manageable
        if self.transition_history.len() > self.config.max_relationships * beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE {
            self.transition_history.remove(0);
        }
    }
    
    /// Get transition history for an entity
    pub fn get_transition_history(&self, entity_id: &str) -> Vec<&RelationshipTransition> {
        self.transition_history
            .iter()
            .filter(|t| t.entity_id == entity_id)
            .collect()
    }
    
    /// Get all transition history
    pub fn get_all_transitions(&self) -> &Vec<RelationshipTransition> {
        &self.transition_history
    }
}

/// Result of an ecosystem interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionResult {
    /// Positive, trust-building interaction
    Positive { impact: f64, context: String },
    
    /// Neutral interaction with no trust impact
    Neutral { context: String },
    
    /// Concerning interaction that may reduce trust
    Concerning { severity: f64, context: String },
    
    /// Harmful interaction requiring protective measures
    Harmful { damage: f64, context: String },
}

impl Default for RelationshipRegistryConfig {
    fn default() -> Self {
        Self {
            max_relationships: 10000,
            trust_decay_rate: 0.01,
            min_trust_threshold: 0.3,
            health_check_interval: Duration::from_secs(300), // 5 minutes
        }
    }
}

impl Default for RelationshipMetadata {
    fn default() -> Self {
        let now = SystemTime::now();
        Self {
            established_at: now,
            last_interaction: now,
            interaction_count: 0,
            health_score: 0.5,
            attributes: HashMap::new(),
        }
    }
} 