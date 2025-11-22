// Primal Hardcoding Elimination Template
//
// This template shows how to migrate from hardcoded primal names
// to capability-based universal adapter discovery.
//
// PRINCIPLE: Each primal only knows itself and discovers others via capabilities.

use beardog_adapters::UniversalPrimalAdapter;
use beardog_errors::BearDogError;
use beardog_types::canonical::discovery::{
    ComputeAbility, NetworkFunction, SecurityService, StorageCharacteristic,
    UniversalCapabilityType,
};
use serde_json::json;

/// ❌ ANTI-PATTERN: Hardcoded primal name references
/// 
/// This violates primal sovereignty - each primal should only know itself
mod anti_pattern {
    use beardog_errors::BearDogError;

    pub struct BadSongbirdClient {
        endpoint: String, // Hardcoded: "songbird.local:8080"
    }

    impl BadSongbirdClient {
        // ❌ WRONG: Hardcoded primal name in type
        pub fn new() -> Self {
            Self {
                endpoint: "http://songbird.local:8080".to_string(), // ❌ Hardcoded endpoint
            }
        }

        // ❌ WRONG: Direct primal-specific method
        pub async fn connect_to_songbird(&self) -> Result<(), BearDogError> {
            // Hardcoded primal interaction
            todo!("This creates 2^n connection complexity")
        }
    }

    // ❌ WRONG: Hardcoded primal references in business logic
    pub async fn process_data(data: Vec<u8>) -> Result<String, BearDogError> {
        let toadstool = "http://toadstool.local:8081"; // ❌ Hardcoded
        let squirrel = "http://squirrel.local:8082"; // ❌ Hardcoded
        let nestgate = "http://nestgate.local:8083"; // ❌ Hardcoded

        // ❌ N^2 hardcoded integrations - doesn't scale
        todo!("Send to toadstool, then squirrel, then nestgate")
    }
}

/// ✅ CORRECT PATTERN: Capability-based universal adapter
///
/// Primal sovereignty: discover capabilities, not names
mod correct_pattern {
    use super::*;

    pub struct SovereignPrimalClient {
        adapter: UniversalPrimalAdapter,
    }

    impl SovereignPrimalClient {
        /// ✅ Correct: No hardcoded primal names, uses universal adapter
        pub async fn new(adapter: UniversalPrimalAdapter) -> Result<Self, BearDogError> {
            Ok(Self { adapter })
        }

        /// ✅ Correct: Request network capability (was: connect_to_songbird)
        /// Discovery happens at runtime based on capability needs
        pub async fn request_network_routing(
            &self,
            routing_config: serde_json::Value,
        ) -> Result<serde_json::Value, BearDogError> {
            // Discover primals with network routing capability
            let network_capability = UniversalCapabilityType::Network {
                functions: vec![NetworkFunction::TrafficRouting],
            };

            // Universal adapter finds ANY primal with this capability
            // Could be songbird, could be a different network primal
            self.adapter
                .send_capability_request(network_capability, routing_config)
                .map(|response| response.data)
        }

        /// ✅ Correct: Request compute capability (was: call toadstool directly)
        pub async fn request_compute_analysis(
            &self,
            data: serde_json::Value,
        ) -> Result<serde_json::Value, BearDogError> {
            let compute_capability = UniversalCapabilityType::Compute {
                abilities: vec![ComputeAbility::DataAnalysis],
            };

            self.adapter
                .send_capability_request(compute_capability, data)
                .map(|response| response.data)
        }

        /// ✅ Correct: Request AI capability (was: call squirrel directly)
        pub async fn request_ai_inference(
            &self,
            model_input: serde_json::Value,
        ) -> Result<serde_json::Value, BearDogError> {
            let ai_capability = UniversalCapabilityType::Compute {
                abilities: vec![ComputeAbility::MachineLearning],
            };

            self.adapter
                .send_capability_request(ai_capability, model_input)
                .map(|response| response.data)
        }

        /// ✅ Correct: Request storage capability (was: call nestgate directly)
        pub async fn request_data_storage(
            &self,
            data: serde_json::Value,
        ) -> Result<serde_json::Value, BearDogError> {
            let storage_capability = UniversalCapabilityType::Storage {
                characteristics: vec![StorageCharacteristic::HighDurability],
            };

            self.adapter
                .send_capability_request(storage_capability, data)
                .map(|response| response.data)
        }

        /// ✅ Correct: Capability-based workflow (O(1) discovery complexity)
        pub async fn process_data_flow(
            &self,
            input_data: serde_json::Value,
        ) -> Result<serde_json::Value, BearDogError> {
            // Phase 1: Request compute analysis (capability-based)
            let compute_capability = UniversalCapabilityType::Compute {
                abilities: vec![ComputeAbility::DataAnalysis],
            };
            let analysis_result = self
                .adapter
                .send_capability_request(compute_capability, input_data)?;

            // Phase 2: Request AI inference (capability-based)
            let ai_capability = UniversalCapabilityType::Compute {
                abilities: vec![ComputeAbility::MachineLearning],
            };
            let ai_result = self
                .adapter
                .send_capability_request(ai_capability, analysis_result.data)?;

            // Phase 3: Store results (capability-based)
            let storage_capability = UniversalCapabilityType::Storage {
                characteristics: vec![
                    StorageCharacteristic::HighDurability,
                    StorageCharacteristic::LowLatency,
                ],
            };
            let storage_result = self
                .adapter
                .send_capability_request(storage_capability, ai_result.data)?;

            Ok(storage_result.data)
        }
    }
}

/// Migration Examples - Before and After
mod migration_examples {
    use super::*;

    /// Example 1: Network Service Discovery
    pub mod network_example {
        use super::*;

        // ❌ BEFORE: Hardcoded songbird reference
        pub async fn old_discover_services_wrong() -> Result<Vec<String>, BearDogError> {
            // Hardcoded primal name
            let songbird_endpoint = "http://songbird.local:8080/discover";
            todo!("Call songbird directly")
        }

        // ✅ AFTER: Capability-based discovery
        pub async fn new_discover_services_correct(
            adapter: &UniversalPrimalAdapter,
        ) -> Result<Vec<String>, BearDogError> {
            // Discover ANY primal with service discovery capability
            let discovery_capability = UniversalCapabilityType::Network {
                functions: vec![NetworkFunction::ServiceDiscovery],
            };

            let response = adapter.send_capability_request(discovery_capability, json!({}))?;

            Ok(response
                .data
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default())
        }
    }

    /// Example 2: Compute Request
    pub mod compute_example {
        use super::*;

        // ❌ BEFORE: Hardcoded toadstool reference
        pub async fn old_run_computation_wrong(
            data: Vec<f64>,
        ) -> Result<Vec<f64>, BearDogError> {
            let toadstool = "http://toadstool.local:8081"; // ❌ Hardcoded
            todo!("Call toadstool API directly")
        }

        // ✅ AFTER: Capability-based compute
        pub async fn new_run_computation_correct(
            adapter: &UniversalPrimalAdapter,
            data: Vec<f64>,
        ) -> Result<Vec<f64>, BearDogError> {
            // Request compute capability - could be ANY compute provider
            let compute_capability = UniversalCapabilityType::Compute {
                abilities: vec![
                    ComputeAbility::DataAnalysis,
                    ComputeAbility::ParallelProcessing,
                ],
            };

            let payload = json!({ "data": data });
            let response = adapter.send_capability_request(compute_capability, payload)?;

            Ok(response
                .data
                .get("result")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_f64())
                        .collect()
                })
                .unwrap_or_default())
        }
    }

    /// Example 3: Multi-Primal Workflow
    pub mod workflow_example {
        use super::*;

        // ❌ BEFORE: Hardcoded primal sequence
        pub async fn old_data_pipeline_wrong(
            input: Vec<u8>,
        ) -> Result<String, BearDogError> {
            // Hardcoded primal dependency chain
            let toadstool = "http://toadstool.local:8081";
            let squirrel = "http://squirrel.local:8082";
            let nestgate = "http://nestgate.local:8083";

            // N^2 integration complexity
            todo!("toadstool -> squirrel -> nestgate (hardcoded)")
        }

        // ✅ AFTER: Capability-based pipeline
        pub async fn new_data_pipeline_correct(
            adapter: &UniversalPrimalAdapter,
            input: Vec<u8>,
        ) -> Result<String, BearDogError> {
            // Step 1: Data analysis capability (whoever provides it)
            let analysis_cap = UniversalCapabilityType::Compute {
                abilities: vec![ComputeAbility::DataAnalysis],
            };
            let analyzed = adapter.send_capability_request(analysis_cap, json!({ "data": input }))?;

            // Step 2: AI inference capability (whoever provides it)
            let ai_cap = UniversalCapabilityType::Compute {
                abilities: vec![ComputeAbility::MachineLearning],
            };
            let inferred = adapter.send_capability_request(ai_cap, analyzed.data)?;

            // Step 3: Storage capability (whoever provides it)
            let storage_cap = UniversalCapabilityType::Storage {
                characteristics: vec![StorageCharacteristic::HighDurability],
            };
            let stored = adapter.send_capability_request(storage_cap, inferred.data)?;

            Ok(stored
                .data
                .get("storage_id")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string())
        }
    }
}

/// Testing Patterns
#[cfg(test)]
mod tests {
    use super::*;

    /// ✅ Test with mock capability adapter (no hardcoded primals)
    #[tokio::test]
    async fn test_capability_based_discovery() {
        // Create mock discovery client
        let discovery_client = create_mock_discovery_client();
        let adapter = UniversalPrimalAdapter::new(Arc::new(discovery_client))
            .await
            .unwrap();

        // Request capability - no hardcoded primal names
        let compute_capability = UniversalCapabilityType::Compute {
            abilities: vec![ComputeAbility::DataAnalysis],
        };

        let result = adapter
            .send_capability_request(compute_capability, json!({"test": "data"}))
            .expect("Should discover and connect to compute capability");

        assert!(!result.service_id.is_empty());
        // Don't assert on specific primal names - could be any compute provider
    }

    fn create_mock_discovery_client() -> impl PrimalDiscoveryClient {
        // Mock implementation that returns capability-based responses
        MockDiscoveryClient::new()
    }
}

/// Key Principles Summary
///
/// 1. ✅ NO hardcoded primal names in production code
/// 2. ✅ Discover capabilities, not primals
/// 3. ✅ Each primal only knows itself
/// 4. ✅ Universal adapter handles all external communication
/// 5. ✅ O(1) discovery complexity vs 2^n hardcoded connections
/// 6. ✅ Truly pluggable ecosystem - any primal can provide any capability
/// 7. ✅ Zero-knowledge infant deployment
///
/// Migration Checklist:
/// - [ ] Replace all primal name references with capability types
/// - [ ] Use UniversalPrimalAdapter for all external communication
/// - [ ] Remove hardcoded endpoints (songbird.local, toadstool.local, etc.)
/// - [ ] Test with mock adapter (capability-based, not name-based)
/// - [ ] Validate zero-knowledge deployment
/// - [ ] Update documentation to show capability patterns
