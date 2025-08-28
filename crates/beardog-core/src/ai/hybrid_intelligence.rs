

use beardog_errors::BearDogError;
use beardog_errors::idiomatic::SecurityResult;
use beardog_types::{AuthorizationLevel, SecurityContext};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridAIWorkflow {
    pub workflow_id: String,
    pub steps: Vec<HybridAIStep>,
    pub routing_strategy: AIRoutingStrategy,
    pub security_context: SecurityContext,
}

pub struct HybridAIStep {
    pub step_id: String,
    pub step_type: AIStepType,
    pub capability_required: String, // Using string for universal compatibility
    pub input_data: WorkflowInputData,
    pub expected_output: String,

pub enum AIStepType {
    InternalML(SecurityMLStep),
    ExternalAI(ExternalAIStep),
    Hybrid {
        internal_weight: f64,
        external_weight: f64,
    },

pub struct SecurityMLStep {
    pub model_type: String,
    pub analysis_type: String, // Simplified from NLPAnalysisType
    pub security_level: AuthorizationLevel,

pub struct ExternalAIStep {
    pub provider: String,
    pub service_type: String,
    pub api_endpoint: String,

pub struct WorkflowInputData {
    pub data: HashMap<String, serde_json::Value>,
    pub metadata: HashMap<String, String>,

pub enum AIRoutingStrategy {
    InternalFirst,
    ExternalFirst,
    Parallel,
    CapabilityBased,

pub struct HybridWorkflowResult {
    pub results: Vec<StepResult>,
    pub overall_confidence: f64,
    pub processing_time_ms: u64,

pub struct StepResult {
    pub success: bool,
    pub result_data: serde_json::Value,
    pub confidence: f64,

pub struct BearDogAIArchitecture {
    pub internal_capabilities: Vec<String>, // Using strings for universal capability representation
    pub external_integrations: Vec<ExternalIntegration>,
    pub routing_config: AIRoutingConfig,}

impl Default for BearDogAIArchitecture {}

    fn default() -> Self {
        Self {
            internal_capabilities: vec![
                "security_ml_analysis".to_string(),
                "threat_pattern_recognition".to_string(),
                "behavioral_anomaly_detection".to_string(),
            ],
            external_integrations: vec![],
            routing_config: AIRoutingConfig {
                default_strategy: AIRoutingStrategy::InternalFirst,
                capability_routing: ahash::HashMap::default(),
                fallback_enabled: true,
            },
        }
    }

pub struct ExternalIntegration {
    pub provider_name: String,
    pub capabilities: Vec<String>, // Universal capability strings
    pub api_config: HashMap<String, String>,

pub struct AIRoutingConfig {
    pub default_strategy: AIRoutingStrategy,
    pub capability_routing: HashMap<String, AIRoutingStrategy>, // Universal capability routing
    pub fallback_enabled: bool,

pub struct HybridIntelligenceManager {

    internal_ml_engine: Arc<SecurityMLEngine>,

    universal_adapter: Arc<Mutex<dyn UniversalAdapter>>,

    ai_architecture: Arc<RwLock<BearDogAIArchitecture>>,

    active_workflows: Arc<RwLock<HashMap<String, ActiveHybridWorkflow>>>,

pub struct SecurityMLEngine {

    threat_models: HashMap<String, ThreatModel>,

    anomaly_detector: BehavioralAnomalyDetector,

    crypto_optimizer: CryptographicMLOptimizer,

    risk_assessor: SecurityRiskAssessor,

#[allow(async_fn_in_trait)]
pub trait UniversalAdapter: Send + Sync {
    async fn route_capability_request(
        &mut self,
        capability: &str, // Universal capability identifier
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse, BearDogError>;

pub enum CapabilityRequest {

    AIIntelligence(ExternalAIRequest),

    ComputeOrchestration(ComputeRequest),

    ServiceMesh(ServiceMeshRequest),

    Storage(StorageRequest),

    Identity(IdentityRequest),

    SecretsManagement(SecretsRequest),

    Monitoring(MonitoringRequest),

    ContainerOrchestration(ContainerRequest),

pub struct CapabilityResponse {
    pub data: serde_json::Value,

#[derive(Debug, Clone)]
pub struct ActiveHybridWorkflow {
    pub workflow_config: HybridAIWorkflow,
    pub current_step: usize,
    pub internal_results: Vec<SecurityMLResult>,
    pub external_results: Vec<ExternalAIResult>,
    pub status: WorkflowStatus,
pub use beardog_types::canonical::WorkflowStatus;}

impl HybridIntelligenceManager {

    pub async fn new(universal_adapter: Arc<Mutex<dyn UniversalAdapter>>) -> Result<Self, BearDogError> {
        info!("🧠 Initializing `BearDog` Hybrid AI Intelligence Manager");
        let internal_ml_engine = Arc::new(SecurityMLEngine::new().await?);
        let ai_architecture = Arc::new(RwLock::new(BearDogAIArchitecture::default()));
        let active_workflows = Arc::new(RwLock::new(ahash::HashMap::default()));
        Ok(Self {
            internal_ml_engine,
            universal_adapter,
            ai_architecture,
            active_workflows,
        })

    pub async fn analyze_threat_internal(
        &self,
        threat_data: &ThreatData,
    ) -> Result<SecurityMLResult, BearDogError> {
        info!("🔍 Analyzing threat with `BearDog` internal ML");

        let pattern_result = self
            .internal_ml_engine
            .threat_models
            .get("primary_threat_detector")
            .ok_or_else(|| BearDogError::configuration("Primary threat model not found".to_string(),
            ))?
            .analyze(threat_data)
            .await?;
        let anomaly_result = self
            .anomaly_detector
            .detect_anomaly(&threat_data.behavioral_patterns)
        let risk_assessment = self
            .risk_assessor
            .assess_risk(threat_data, &pattern_result, &anomaly_result)
        Ok(SecurityMLResult {
            threat_confidence: pattern_result.confidence,
            anomaly_score: anomaly_result.anomaly_score,
            risk_level: risk_assessment.risk_level,
            recommendations: risk_assessment.recommendations,
            internal_processing: true,

    pub async fn analyze_with_squirrel_ai(
        ai_request: ExternalAIRequest,
    ) -> Result<ExternalAIResult, BearDogError> {
        info!("🐿️ Routing AI analysis to Squirrel via universal adapter");

        let capability_request = CapabilityRequest::AIIntelligence(ai_request.clone());
        let response = self
            .universal_adapter
            .lock()
            .await
            .route_capability_request("intelligence".to_string(), capability_request)
        if !response.success {
            return Err(BearDogError::network("AI capability request failed".to_string(),
            ));

        let ai_result: ExternalAIResult = serde_json::from_value(response.data).map_err(|e| {
            BearDogError::internal(e.to_string(),
            )
        })?;
        debug!("✅ Received AI analysis from capability provider");
        Ok(ai_result)

    pub async fn execute_hybrid_workflow(
        workflow_name: &str,
        input_data: &WorkflowInputData,
    ) -> Result<HybridWorkflowResult, BearDogError> {
        info!("🔄 Executing hybrid workflow: {}", workflow_name);

        let workflow = HybridAIWorkflow {
            workflow_id: workflow_name.to_string(),
            steps: vec![
                HybridAIStep {
                    step_id: "internal_step".to_string(),
                    step_type: AIStepType::InternalML(SecurityMLStep {
                        model_type: "threat_detection_v1".to_string(),
                        analysis_type: "threat_detection".to_string(),
                        security_level: AuthorizationLevel::Root,
                    }),
                    capability_required: "intelligence".to_string(),
                    input_data: WorkflowInputData {
                        data: ahash::HashMap::default(),
                        metadata: ahash::HashMap::default(),
                    },
                    expected_output: "threat_analysis_result".to_string(),
                },
                    step_id: "external_step".to_string(),
                    step_type: AIStepType::ExternalAI(ExternalAIStep {
                        provider: "squirrel".to_string(),
                        service_type: "nlp_analysis".to_string(),
                        api_endpoint: "/api/v1/analysis".to_string(),
                    expected_output: "nlp_analysis_result".to_string(),
            routing_strategy: AIRoutingStrategy::InternalFirst,
            security_context: SecurityContext {
                user_id: Some("beardog_ai_system".to_string()),
                client_ip: None,
                timestamp: chrono::Utc::now(),
                authenticated: true,
                authorization_level: AuthorizationLevel::Root,
                method: None,
                path: None,
                user_agent: None,
                session_id: Some("ai_workflow_session".to_string()),
                security_flags: beardog_types::canonical::SecurityFlags::default(),
                metadata: ahash::HashMap::default(),
        };
        match workflow.routing_strategy {
            AIRoutingStrategy::InternalFirst => {
                self.execute_sequential_workflow(&workflow, input_data)
                    .await
            }
            AIRoutingStrategy::Parallel => {
                self.execute_parallel_workflow(&workflow, input_data).await
            AIRoutingStrategy::CapabilityBased => {
                self.execute_validation_workflow(&workflow, input_data)
            AIRoutingStrategy::ExternalFirst => {
                self.execute_enhancement_workflow(&workflow, input_data)

    async fn execute_sequential_workflow(
        workflow: &HybridAIWorkflow,

        let mut internal_results = Vec::new();
        for step in &workflow.steps {
            if matches!(step.step_type, AIStepType::InternalML(_)) {
                let result = self.execute_internal_step(step, input_data).await?;
                internal_results.push(result);

        let mut external_results = Vec::new();
            if matches!(step.step_type, AIStepType::ExternalAI(_)) {
                let internal_values: Vec<serde_json::Value> = internal_results
                    .iter()
                    .map(|result| result.result_data.clone())
                    .collect();
                let enhanced_input =
                    self.enhance_input_with_beardog_context(input_data, &internal_values);
                let result = self.execute_external_step(step, &enhanced_input).await?;
                external_results.push(result);

        let mut all_results = Vec::new();
        all_results.extend(internal_results);
        all_results.extend(external_results);
        Ok(HybridWorkflowResult {
            workflow_id: workflow.workflow_id.clone(),
            results: all_results,
            overall_confidence: 0.85,
            processing_time_ms: 100,
        })
    }

    async fn execute_parallel_workflow(
        &self,
        workflow: &HybridAIWorkflow,
        input_data: &WorkflowInputData,
    ) -> Result<HybridWorkflowResult, BearDogError> {
        let start_time = std::time::Instant::now();

        let mut tasks = Vec::new();
        for step in &workflow.steps {
            let step_clone = step.clone();
            let input_clone = input_data.clone();
            tasks.push(tokio::task::spawn_local(async move {
                Self::execute_internal_step(&step_clone, &input_clone).await
            }));
        }

        let mut results = Vec::new();
        for task in tasks {
            let result = task.await
                .map_err(|e| BearDogError::system(format_args!("Task execution failed: {}", e).to_string()))?;
            results.push(result?);
        }
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        let overall_confidence = results.iter()
            .map(|r| r.confidence)
            .sum::<f32>() / results.len() as f32;
            
        Ok(HybridWorkflowResult {
            workflow_id: workflow.workflow_id.clone(),
            results,
            overall_confidence,
            processing_time_ms: processing_time,
        })
    }

    async fn execute_validation_workflow(
        &self,
        workflow: &HybridAIWorkflow,
        input_data: &WorkflowInputData,
    ) -> Result<HybridWorkflowResult, BearDogError> {
        let start_time = std::time::Instant::now();

        if input_data.parameters.is_empty() {
            return Err(BearDogError::validation("Input data parameters cannot be empty"));
        }

        let mut results = Vec::new();
        for step in &workflow.steps {
            let result = Self::execute_internal_step(step, input_data).await?;

            if !result.success {
                return Err(BearDogError::validation(
                    format_args!("Validation failed at step: {}", step.step_id).to_string()
                ));
            }
            results.push(result);
        }
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        let overall_confidence = 0.95; // High confidence for validated workflows
        
        Ok(HybridWorkflowResult {
            workflow_id: workflow.workflow_id.clone(),
            results,
            overall_confidence,
            processing_time_ms: processing_time,
        })
    }

    async fn execute_enhancement_workflow(
        &self,
        workflow: &HybridAIWorkflow,
        input_data: &WorkflowInputData,
    ) -> Result<HybridWorkflowResult, BearDogError> {
        let start_time = std::time::Instant::now();

        let enhanced_input = self.enhance_input_with_beardog_context(input_data, &[]).await?;

        let mut results = Vec::new();
        for step in &workflow.steps {
            let mut result = Self::execute_internal_step(step, &enhanced_input).await?;

            result.confidence = (result.confidence * 1.1).min(1.0);
            results.push(result);
        }
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        let overall_confidence = results.iter()
            .map(|r| r.confidence)
            .sum::<f32>() / results.len() as f32;
        
        Ok(HybridWorkflowResult {
            workflow_id: workflow.workflow_id.clone(),
            results,
            overall_confidence,
            processing_time_ms: processing_time,
        })
    }

    async fn execute_internal_step(
        step: &HybridAIStep,
        input_data: &WorkflowInputData,
    ) -> Result<StepResult, BearDogError> {
        let start_time = std::time::Instant::now();

        let (success, result_data, confidence) = match step.step_type.as_str() {
            "security_analysis" => {

                let security_score = Self::calculate_security_score(&input_data.parameters)?;
                (
                    security_score > 0.7,
                    serde_json::json!({
                        "security_score": security_score,
                        "status": "security_analyzed",
                        "threats_detected": security_score < 0.8
                    }),
                    security_score
                )
            },
            "performance_optimization" => {

                let perf_score = Self::calculate_performance_score(&input_data.parameters)?;
                (
                    perf_score > 0.6,
                    serde_json::json!({
                        "performance_score": perf_score,
                        "status": "performance_optimized",
                        "optimizations_applied": true
                    }),
                    perf_score
                )
            },
            _ => {

                (
                    true,
                    serde_json::json!({"status": "processed", "step_type": step.step_type}),
                    0.85
                )
            }
        };
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        Ok(StepResult {
            step_id: step.step_id.clone(),
            success,
            result_data,
            confidence,
            processing_time_ms: processing_time,
        })
    }

    async fn enhance_input_with_beardog_context(
        &self,
        input_data: &WorkflowInputData,
        internal_results: &[serde_json::Value],
    ) -> Result<WorkflowInputData, BearDogError> {
        let mut enhanced_params = input_data.parameters.clone();

        enhanced_params.insert("beardog_context".to_string(), serde_json::json!({
            "ecosystem_integration": true,
            "security_level": "enterprise",
            "performance_profile": "optimized"
        }));

        if !internal_results.is_empty() {
            enhanced_params.insert("internal_analysis".to_string(), 
                serde_json::json!(internal_results));
        }

        enhanced_params.insert("enhancement_metadata".to_string(), serde_json::json!({
            "enhanced_at": chrono::Utc::now(),
            "enhancement_version": "1.0.0",
            "confidence_boost": 0.1
        }));
        
        Ok(WorkflowInputData {
            parameters: enhanced_params,
            context: input_data.context.clone(),
        })
    }

    async fn execute_external_step(
        &self,
        step: &HybridAIStep,
        input_data: &WorkflowInputData,
    ) -> Result<StepResult, BearDogError> {
        let start_time = std::time::Instant::now();

        let external_result = match step.step_type.as_str() {
            "songbird_integration" => {
                serde_json::json!({
                    "service": "songbird",
                    "status": "integrated",
                    "mesh_connectivity": true
                })
            },
            "nestgate_storage" => {
                serde_json::json!({
                    "service": "nestgate", 
                    "status": "storage_ready",
                    "capacity_available": true
                })
            },
            _ => {
                serde_json::json!({
                    "service": "external",
                    "status": "completed",
                    "integration": true
                })
            }
        };
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        Ok(StepResult {
            step_id: step.step_id.clone(),
            success: true,
            result_data: external_result,
            confidence: 0.85,
            processing_time_ms: processing_time,
        })
    }

    fn calculate_security_score(parameters: &std::collections::HashMap<&str, serde_json::Value>) -> Result<f32, BearDogError> {
        let mut score = 0.8; // Base security score

        if parameters.contains_key("hsm_enabled") {
            score += 0.1;
        }
        if parameters.contains_key("encryption_level") {
            score += 0.05;
        }
        if parameters.contains_key("audit_enabled") {
            score += 0.05;
        }
        
        Ok(score.min(1.0))
    }

    fn calculate_performance_score(parameters: &std::collections::HashMap<&str, serde_json::Value>) -> Result<f32, BearDogError> {
        let mut score = 0.7; // Base performance score

        if parameters.contains_key("async_enabled") {
            score += 0.1;
        }
        if parameters.contains_key("caching_enabled") {
            score += 0.1;
        }
        if parameters.contains_key("optimization_level") {
            score += 0.1;
        }
        
        Ok(score.min(1.0))
    }

    pub async fn analyze_threat_with_semantic_enhancement(
    ) -> Result<EnhancedThreatAnalysis, BearDogError> {

        let internal_analysis = self.analyze_threat_internal(threat_data).await?;

        let semantic_request = ExternalAIRequest {
            request_type: "nlp_threat_extraction".to_string(),
            payload: serde_json::json!({
                "log_content": threat_data.raw_logs,
                "analysis_type": "ThreatExtraction"
            }),
            context: SecurityContext {
                user_id: Some("beardog_security".to_string()),
                authorization_level: AuthorizationLevel::Standard,
                session_id: Some("beardog_ai_session".to_string()),
                metadata: std::collections::ahash::HashMap::default(),
        let semantic_analysis = self.analyze_with_squirrel_ai(semantic_request).await?;

        let combined_threat_score =
            self.calculate_combined_threat_score(&internal_analysis, &semantic_analysis);
        let confidence_level =
            self.calculate_hybrid_confidence(&internal_analysis, &semantic_analysis);
        let recommended_actions =
            self.generate_hybrid_recommendations(&internal_analysis, &semantic_analysis);

        Ok(EnhancedThreatAnalysis {
            beardog_ml_analysis: internal_analysis,
            squirrel_semantic_analysis: semantic_analysis,
            combined_threat_score,
            confidence_level,
            recommended_actions,

    const fn calculate_combined_threat_score(
        _internal_analysis: &SecurityMLResult,
        _semantic_analysis: &ExternalAIResult,
    ) -> f64 {
        0.75
    }

    const fn calculate_hybrid_confidence() -> f64 {
        0.85
    }

    fn generate_hybrid_recommendations() -> Vec<String> {
        vec![
            "Monitor system behavior".to_string(),
            "Review security logs".to_string(),
        ]
    }

pub struct ThreatData {
    pub threat_indicators: Vec<String>,
    pub behavioral_patterns: HashMap<String, f64>,
    pub raw_logs: String,
    pub timestamp: String,
}

pub struct SecurityMLResult {
    pub threat_confidence: f64,
    pub anomaly_score: f64,
    pub risk_level: String,
    pub recommendations: Vec<String>,
    pub internal_processing: bool,
pub struct ExternalAIResult {
    pub analysis_type: String,
    pub results: serde_json::Value,
    pub external_processing: bool,
}

pub struct EnhancedThreatAnalysis {
    pub beardog_ml_analysis: SecurityMLResult,
    pub squirrel_semantic_analysis: ExternalAIResult,
    pub combined_threat_score: f64,
    pub confidence_level: f64,
    pub recommended_actions: Vec<String>,

impl SecurityMLEngine {
    async fn new() -> Result<Self, BearDogError> {
            threat_models: ahash::HashMap::default(),
            anomaly_detector: BehavioralAnomalyDetector::new(),
            crypto_optimizer: CryptographicMLOptimizer::new(),
            risk_assessor: SecurityRiskAssessor::new(),}

struct ThreatModel;
struct BehavioralAnomalyDetector;
struct CryptographicMLOptimizer;
struct SecurityRiskAssessor;

struct AnomalyResult {
    anomaly_score: f64,
    patterns_detected: Vec<String>,
    confidence: f64,
struct ThreatAnalysisResult {
    threat_type: String,
    severity: String,
struct RiskAssessment {
    risk_level: String,
    recommendations: Vec<String>,

impl ThreatModel {
    async fn analyze(&self, _threat_data: &ThreatData) -> Result<ThreatAnalysisResult, BearDogError> {

        Ok(ThreatAnalysisResult {
            threat_type: "behavioral_anomaly".to_string(),
            severity: "low".to_string(),}

impl BehavioralAnomalyDetector {
    const fn new() -> Self {
        Self}

    async fn detect_anomaly(
        _behavioral_patterns: &HashMap<&str, f64>,
    ) -> Result<AnomalyResult, BearDogError> {

        Ok(AnomalyResult {
            anomaly_score: 0.1,
            patterns_detected: vec!["normal_behavior".to_string()],
            confidence: 0.95,}

impl CryptographicMLOptimizer {
impl SecurityRiskAssessor {
    async fn assess_risk(
        _threat_data: &ThreatData,
        _pattern_result: &ThreatAnalysisResult,
        _anomaly_result: &AnomalyResult,
    ) -> Result<RiskAssessment, SecurityError> {

        Ok(RiskAssessment {
            risk_level: "LOW".to_string(),
            recommendations: vec!["Continue monitoring".to_string()],
            confidence: 0.90,

pub struct ExternalAIRequest {
    pub request_type: String,
    pub payload: serde_json::Value,
    pub context: SecurityContext,

pub struct ComputeRequest {
    pub operation: String,
    pub resources: HashMap<String, serde_json::Value>,

pub struct ServiceMeshRequest {
    pub action: String,
    pub service_id: String,
    pub configuration: HashMap<String, serde_json::Value>,

pub struct StorageRequest {
    pub path: String,
    pub data: Option<Vec<u8>>,

pub struct IdentityRequest {
    pub identity_id: String,
    pub attributes: HashMap<String, String>,

pub struct SecretsRequest {
    pub secret_id: String,
    pub value: Option<String>,

pub struct MonitoringRequest {
    pub metric_type: String,
    pub target: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

pub struct ContainerRequest {
    pub container_id: String,
}
