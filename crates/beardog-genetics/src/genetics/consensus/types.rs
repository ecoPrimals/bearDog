

use crate::genetics::biome_genetics::TrustLevel;
use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
    /// Number of minimum_participants
    pub minimum_participants: u32,
    /// Number of maximum_participants
    pub maximum_participants: u32,
    pub voting_timeout_seconds: u32,
    /// The genetic weight factor value
    pub genetic_weight_factor: f64,
    /// The trust weight factor value
    pub trust_weight_factor: f64,
    /// The stake weight factor value
    pub stake_weight_factor: f64,
    /// The quorum threshold value
    pub quorum_threshold: f64,
    /// The consensus threshold value
    pub consensus_threshold: f64,
    pub enable_genetic_validation: bool,
    /// Whether enable_dynamic_weighting is enabled
    pub enable_dynamic_weighting: bool,
    /// Whether allow_delegation is enabled
    pub allow_delegation: bool,
}

#[derive(Debug, Clone)]
    /// The proposal type value
    pub proposal_type: ProposalType,
    /// The title value
    pub title: String,
    /// The description value
    pub description: String,
    /// The proposer biome value
    pub proposer_biome: String,
    /// Collection of target biomes
    pub target_biomes: Vec<String>,
    /// The proposal data value
    pub proposal_data: ProposalData,
    /// Collection of voting options
    pub voting_options: Vec<VotingOption>,
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// The voting deadline value
    pub voting_deadline: DateTime<Utc>,
    /// The minimum genetic quality value
    pub minimum_genetic_quality: f64,
    /// The required trust level value
    pub required_trust_level: TrustLevel,
    /// Current status of the component
    pub status: ProposalStatus,
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

#[derive(HashMap<String, String>,
    /// Collection of affected resources
    pub affected_resources: Vec<String>,
    /// The impact assessment value
    pub impact_assessment: ImpactAssessment,
    /// Optional implementation plan
    pub implementation_plan: Option<ImplementationPlan>,
    /// Optional rollback plan
    pub rollback_plan: Option<RollbackPlan>,
}

#[derive(Debug, Clone)]
    pub performance_impact: ImpactLevel,
    /// The resource impact value
    pub resource_impact: ImpactLevel,
    /// The network impact value
    pub network_impact: ImpactLevel,
    /// The genetic impact value
    pub genetic_impact: ImpactLevel,
    /// The risk score value
    pub risk_score: f64,
    /// The benefit score value
    pub benefit_score: f64,
    /// Number of affected_biome
    pub affected_biome_count: u32,
}

#[derive(Debug, Clone)]
    pub timeline_days: u32,
    /// Collection of resource requirements
    pub resource_requirements: Vec<String>,
    /// Collection of dependencies
    pub dependencies: Vec<String>,
    /// Collection of success criteria
    pub success_criteria: Vec<String>,
}

#[derive(Debug, Clone)]
    /// The description value
    pub description: String,
    /// Number of duration_days
    pub duration_days: u32,
    /// Collection of prerequisites
    pub prerequisites: Vec<String>,
    /// Collection of deliverables
    pub deliverables: Vec<String>,
}

#[derive(Debug, Clone)]
    /// Collection of rollback steps
    pub rollback_steps: Vec<String>,
    pub rollback_timeline_hours: u32,
    /// Collection of data preservation
    pub data_preservation: Vec<String>,
}

#[derive(Debug, Clone)]
    /// The option text value
    pub option_text: String,
    /// The option type value
    pub option_type: VotingOptionType,
    /// Optional implementation details
    pub implementation_details: Option<String>,
    /// Collection of additional requirements
    pub additional_requirements: Vec<String>,
}

#[derive(Debug, Clone)]
    pub proposal_id: String,
    /// The voter biome value
    pub voter_biome: String,
    /// The selected option value
    pub selected_option: String,
    /// The vote weight value
    pub vote_weight: f64,
    /// The genetic quality value
    pub genetic_quality: f64,
    /// The trust score value
    pub trust_score: f64,
    /// Optional justification
    pub justification: Option<String>,
    /// The cast at value
    pub cast_at: DateTime<Utc>,
    /// The signature value
    pub signature: String,
}

#[derive(Debug, Clone)]
    /// The result value
    pub result: ConsensusOutcome,
    /// The final tally value
    pub final_tally: VoteTally,
    /// The participation rate value
    pub participation_rate: f64,
    /// The consensus reached at value
    pub consensus_reached_at: DateTime<Utc>,
    /// Current status of the implementation
    pub implementation_status: ImplementationStatus,
    /// Collection of post consensus actions
    pub post_consensus_actions: Vec<String>,
}

#[derive(Debug, Clone)]
    /// Number of total_votes_cast
    pub total_votes_cast: u32,
    /// Mapping of weighted votes
    pub weighted_votes: HashMap<String, f64>,
    /// The genetic quality average value
    pub genetic_quality_average: f64,
    /// The trust score average value
    pub trust_score_average: f64,
    /// Optional winning option
    pub winning_option: Option<String>,
    /// The margin of victory value
    pub margin_of_victory: f64,
}

#[derive(Debug, Clone)]
            minimum_participants: 3,
            maximum_participants: 100,
            voting_timeout_seconds: 3600,
            genetic_weight_factor: 0.4,
            trust_weight_factor: 0.3,
            stake_weight_factor: 0.3,
            quorum_threshold: 0.51,
            consensus_threshold: 0.66,
            enable_genetic_validation: true,
            enable_dynamic_weighting: true,
            allow_delegation: false,
        }
    }
} 
