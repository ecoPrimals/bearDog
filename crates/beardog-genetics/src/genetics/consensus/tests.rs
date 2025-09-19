

use super::*;
use crate::genetics::biome_genetics::{BiomeGenetics, BiomeIdentity, GeneticSignature, TrustLevel, Authorization};
use crate::genetics::entropy_hierarchy::EntropyClass;
use std::collections::HashMap;

#[derive(Debug, Clone)]
    genetic_quality: f64,
    trust_level: TrustLevel,
}

impl MockBiome {
    fn new(&str, genetic_quality: f64) -> Self {
        Self {
            biome_id: biome_id.to_string() -> Result<Authorization, beardog_errors::BearDogError> {
        Ok(true,
            authorization_level: self.trust_level.clone(format!("auth_proof_{}", self.biome_id),
            expiry_timestamp: None,
            authorization_metadata: HashMap::with_capacity(16),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consensus_config_default() {
        let config = ConsensusConfig::default();
        
        assert_eq!(config.minimum_participants, 3);
        assert_eq!(config.maximum_participants, 100);
        assert_eq!(config.voting_timeout_seconds, 3600);
        assert_eq!(config.genetic_weight_factor, 0.4);
        assert_eq!(config.trust_weight_factor, 0.3);
        assert_eq!(config.stake_weight_factor, 0.3);
        assert_eq!(config.quorum_threshold, 0.51);
        assert_eq!(config.consensus_threshold, 0.66);
        assert!(config.enable_genetic_validation);
        assert!(config.enable_dynamic_weighting);
        assert!(!config.allow_delegation);
    }

    #[tokio::test]
    fn test_consensus_engine_creation() {
        let config = ConsensusConfig::default();
        let engine = GeneticConsensusEngine::new(config);
        
        let metrics = engine.get_metrics();
        assert_eq!(metrics.get_total_proposals(), 0);
        assert_eq!(metrics.get_successful_consensus(), 0);
        assert_eq!(metrics.get_failed_consensus(), 0);
    }

    #[tokio::test]
    fn test_create_proposal() {
        let config = ConsensusConfig::default();
        let engine = GeneticConsensusEngine::new(config);
        let mock_biome = MockBiome::new("test_biome", 0.8);

        let proposal_data = ProposalData {
            parameters: HashMap::with_capacity(vec![],
            impact_assessment: ImpactAssessment {
                security_impact: ImpactLevel::Low,
                performance_impact: ImpactLevel::Low,
                resource_impact: ImpactLevel::Low,
                network_impact: ImpactLevel::Low,
                genetic_impact: ImpactLevel::Low,
                risk_score: 0.1,
                benefit_score: 0.8,
                affected_biome_count: 1,
            },
            implementation_plan: None,
            rollback_plan: None,
        };

        let voting_options = vec![VotingOption {
            option_id: "approve".to_string(),
            option_text: "Approve".to_string(),
                "Test Proposal".to_string(),
                "Test Description".to_string(),
                proposal_data,
                voting_options,
                vec!["test_biome".to_string()],
            )
            ;

        assert!(result.is_ok());
        let proposal_id = result.unwrap();
        assert!(!proposal_id.is_empty());

        let proposal = engine.get_proposal(&proposal_id);
        assert!(proposal.is_ok());
        let proposal = proposal.unwrap();
        assert_eq!(proposal.title, "Test Proposal");
        assert_eq!(proposal.description, "Test Description");
        assert_eq!(proposal.proposer_biome, "test_biome");
    }

    #[tokio::test]
    fn test_voting_manager() {
        let voting_manager = VotingManager::new();
        
        let vote_id = voting_manager
            .submit_vote(
                "proposal_1",
                "biome_1",
                "approve",
                1.0,
                0.8,
                0.7,
                Some("Good proposal".to_string()),
            )
            ;

        assert!(vote_id.is_ok());

        let has_voted = voting_manager.has_voted("proposal_1", "biome_1");
        assert!(has_voted.is_ok());
        assert!(has_voted.unwrap());

        let count = voting_manager.get_vote_count("proposal_1");
        assert!(count.is_ok());
        assert_eq!(count.unwrap(), 1);

        let votes = voting_manager.get_votes_for_proposal("proposal_1");
        assert!(votes.is_ok());
        let votes = votes.unwrap();
        assert_eq!(votes.len(), 1);
        assert_eq!(votes[0].voter_biome, "biome_1");
        assert_eq!(votes[0].selected_option, "approve");
    }

    #[tokio::test]
    fn test_trust_network() {
        let trust_network = TrustNetwork::new();

        let score = trust_network.get_trust_score("biome_1");
        assert_eq!(score, 0.5); // Default neutral trust

        let result = trust_network.set_trust_score("biome_1", 0.8);
        assert!(result.is_ok());

        let score = trust_network.get_trust_score("biome_1");
        assert_eq!(score, 0.8);

        let result = trust_network.set_trust_score("biome_1", 1.5);
        assert!(result.is_err());

        let result = trust_network
            .update_trust_relationship("biome_1", "biome_2", 0.9)
            ;
        assert!(result.is_ok());

        let relationship = trust_network
            .get_trust_relationship("biome_1", "biome_2")
            ;
        assert_eq!(relationship, 0.9);
    }

    #[tokio::test]
    fn test_consensus_metrics() {
        let metrics = ConsensusMetrics::new();

        assert_eq!(metrics.get_total_proposals(), 0);
        assert_eq!(metrics.get_successful_consensus(), 0);
        assert_eq!(metrics.get_failed_consensus(), 0);
        assert_eq!(metrics.get_success_rate(), 0.0);

        metrics.increment_total_proposals();
        metrics.increment_successful_consensus();
        
        assert_eq!(metrics.get_total_proposals(), 1);
        assert_eq!(metrics.get_successful_consensus(), 1);
        assert_eq!(metrics.get_success_rate(), 1.0);

        metrics.record_consensus_time(1000);
        metrics.record_participation_rate(0.75);
        metrics.record_genetic_quality(0.85);

        assert_eq!(metrics.get_average_consensus_time_ms(), 1000);
        assert_eq!(metrics.get_average_participation_rate(), 0.75);
    }

    #[tokio::test]
    fn test_genetic_validator() {
        let validator = GeneticValidator::new();
        
        let high_quality_signature = GeneticSignature {
            genetic_hash: "test_hash".to_string(0.8,
            entropy_class: EntropyClass::Standard,
            generation: 1,
            parent_signatures: vec![],
            signature_timestamp: chrono::Utc::now(),
            signature_metadata: HashMap::with_capacity(16),
        };

        let low_quality_signature = GeneticSignature {
            genetic_hash: "test_hash".to_string(0.3,
            entropy_class: EntropyClass::Standard,
            generation: 1,
            parent_signatures: vec![],
            signature_timestamp: chrono::Utc::now(),
            signature_metadata: HashMap::with_capacity(16),
        };

        let result = validator
            .validate_genetic_signature(&high_quality_signature, 0.7)
            ;
        assert!(result.is_ok());

        let result = validator
            .validate_genetic_signature(&low_quality_signature, 0.7)
            ;
        assert!(result.is_err());
    }

    #[tokio::test]
    fn test_proposal_status_transitions() {
        let config = ConsensusConfig::default();
        let engine = GeneticConsensusEngine::new(config);
        let mock_biome = MockBiome::new("test_biome", 0.8);

        let proposal_data = ProposalData {
            parameters: HashMap::with_capacity(vec![],
            impact_assessment: ImpactAssessment {
                security_impact: ImpactLevel::Low,
                performance_impact: ImpactLevel::Low,
                resource_impact: ImpactLevel::Low,
                network_impact: ImpactLevel::Low,
                genetic_impact: ImpactLevel::Low,
                risk_score: 0.1,
                benefit_score: 0.8,
                affected_biome_count: 1,
            },
            implementation_plan: None,
            rollback_plan: None,
        };

        let voting_options = vec![VotingOption {
            option_id: "approve".to_string(),
            option_text: "Approve".to_string(),
                "Test Proposal".to_string(),
                "Test Description".to_string(),
                proposal_data,
                voting_options,
                vec!["test_biome".to_string()],
            )
            .unwrap();

        let proposal = engine.get_proposal(&proposal_id).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Open);

        let history = engine.get_consensus_history();
        assert!(history.is_empty()); // No consensus processed yet
    }

    #[test]
    fn test_proposal_types() {

        let types = vec![
            ProposalType::BiomeRegistration,
            ProposalType::NetworkConfiguration,
            ProposalType::SecurityPolicy,
            ProposalType::ResourceAllocation,
            ProposalType::ProtocolUpgrade,
            ProposalType::ConflictResolution,
            ProposalType::CollaborativeOperation,
            ProposalType::TrustNetworkUpdate,
            ProposalType::GeneticStandardUpdate,
            ProposalType::EmergencyAction,
        ];

        for proposal_type in types {

            let debug_str = format!("{:?}", proposal_type);
            assert!(!debug_str.is_empty());
        }
    }

    #[test]
    fn test_voting_option_types() {
        let option_types = vec![
            VotingOptionType::Approve,
            VotingOptionType::Reject,
            VotingOptionType::Abstain,
            VotingOptionType::ConditionalApprove,
            VotingOptionType::RequestModification,
            VotingOptionType::Delegate,
        ];

        for option_type in option_types {
            let debug_str = format!("{:?}", option_type);
            assert!(!debug_str.is_empty());
        }
    }

    #[test]
    fn test_consensus_algorithms() {
        let algorithms = vec![
            ConsensusAlgorithm::GeneticProofOfStake,
            ConsensusAlgorithm::WeightedGeneticVoting,
            ConsensusAlgorithm::TrustBasedConsensus,
            ConsensusAlgorithm::HybridGeneticBFT,
            ConsensusAlgorithm::AdaptiveConsensus,
        ];

        for algorithm in algorithms {
            let debug_str = format!("{:?}", algorithm);
            assert!(!debug_str.is_empty());
        }
    }
} 
