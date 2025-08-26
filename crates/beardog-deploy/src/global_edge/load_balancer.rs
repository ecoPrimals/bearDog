

use super::types::*;
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, debug};

pub struct IntelligentLoadBalancer {
    routing_algorithm: RoutingAlgorithm,
    traffic_patterns: Arc<RwLock<TrafficPatterns>>,
    latency_matrix: Arc<RwLock<LatencyMatrix>>,
    prediction_model: Arc<RwLock<LoadPredictionModel>>,
}

impl IntelligentLoadBalancer {

    pub fn new(config: LoadBalancingConfig) -> BearDogResult<Self> {
        Ok(Self {
            routing_algorithm: config.algorithm,
            traffic_patterns: Arc::new(RwLock::new(TrafficPatterns::new())),
            latency_matrix: Arc::new(RwLock::new(LatencyMatrix::new())),
            prediction_model: Arc::new(RwLock::new(LoadPredictionModel::new())),
        })
    }

    pub async fn initialize(&self) -> BearDogResult<()> {
        info!("Initializing intelligent load balancer with algorithm: {:?}", self.routing_algorithm);

        let mut patterns = self.traffic_patterns.write().await;
        patterns.initialize().await?;

        let mut matrix = self.latency_matrix.write().await;
        matrix.initialize().await?;

        let mut model = self.prediction_model.write().await;
        model.initialize().await?;
        
        info!("Load balancer initialized successfully");
        Ok(())
    }

    pub async fn find_optimal_node(
        &self,
        client_location: &GeographicalLocation,
        regions: &Arc<RwLock<HashMap<&str, EdgeRegion>>>,
    ) -> BearDogResult<EdgeNode> {
        debug!("Finding optimal node for client at: {}, {}", 
               client_location.city, client_location.country);
        
        let regions_guard = regions.read().await;
        if regions_guard.is_empty() {
            return Err(BearDogError::not_found("No regions available"));
        }
        
        match self.routing_algorithm {
            RoutingAlgorithm::Geographic => {
                self.find_nearest_node(client_location, &regions_guard).await
            }
            RoutingAlgorithm::LoadBased => {
                self.find_least_loaded_node(&regions_guard).await
            }
            RoutingAlgorithm::LatencyBased => {
                self.find_lowest_latency_node(client_location, &regions_guard).await
            }
            RoutingAlgorithm::Hybrid => {
                self.find_hybrid_optimal_node(client_location, &regions_guard).await
            }
        }
    }

    async fn find_nearest_node(
        &self,
        client_location: &GeographicalLocation,
        regions: &HashMap<&str, EdgeRegion>,
    ) -> BearDogResult<EdgeNode> {
        let mut best_node: Option<EdgeNode> = None;
        let mut min_distance = f64::MAX;
        
        for region in regions.values() {
            if region.status != RegionStatus::Active {
                continue;
            }
            
            let distance = self.calculate_distance(client_location, &region.geographical_location);
            
            for node in &region.edge_nodes {
                if node.health_status == HealthStatus::Healthy && distance < min_distance {
                    min_distance = distance;
                    best_node = Some(node.clone());
                }
            }
        }
        
        best_node.ok_or_else(|| BearDogError::not_found("No healthy nodes available"))
    }

    async fn find_least_loaded_node(
        &self,
        regions: &HashMap<&str, EdgeRegion>,
    ) -> BearDogResult<EdgeNode> {
        let mut best_node: Option<EdgeNode> = None;
        let mut min_load = f64::MAX;
        
        for region in regions.values() {
            if region.status != RegionStatus::Active {
                continue;
            }
            
            for node in &region.edge_nodes {
                if node.health_status == HealthStatus::Healthy && node.current_load < min_load {
                    min_load = node.current_load;
                    best_node = Some(node.clone());
                }
            }
        }
        
        best_node.ok_or_else(|| BearDogError::not_found("No healthy nodes available"))
    }

    async fn find_lowest_latency_node(
        &self,
        client_location: &GeographicalLocation,
        regions: &HashMap<&str, EdgeRegion>,
    ) -> BearDogResult<EdgeNode> {
        let latency_matrix = self.latency_matrix.read().await;
        let mut best_node: Option<EdgeNode> = None;
        let mut min_latency = f64::MAX;
        
        for region in regions.values() {
            if region.status != RegionStatus::Active {
                continue;
            }
            
            let latency = latency_matrix.get_latency(client_location, &region.geographical_location);
            
            for node in &region.edge_nodes {
                if node.health_status == HealthStatus::Healthy && latency < min_latency {
                    min_latency = latency;
                    best_node = Some(node.clone());
                }
            }
        }
        
        best_node.ok_or_else(|| BearDogError::not_found("No healthy nodes available"))
    }

    async fn find_hybrid_optimal_node(
        &self,
        client_location: &GeographicalLocation,
        regions: &HashMap<&str, EdgeRegion>,
    ) -> BearDogResult<EdgeNode> {
        let latency_matrix = self.latency_matrix.read().await;
        let mut best_node: Option<EdgeNode> = None;
        let mut best_score = f64::MAX;
        
        for region in regions.values() {
            if region.status != RegionStatus::Active {
                continue;
            }
            
            let distance = self.calculate_distance(client_location, &region.geographical_location);
            let latency = latency_matrix.get_latency(client_location, &region.geographical_location);
            
            for node in &region.edge_nodes {
                if node.health_status == HealthStatus::Healthy {

                    let score = (distance * 0.3) + (latency * 0.4) + (node.current_load * 0.3);
                    
                    if score < best_score {
                        best_score = score;
                        best_node = Some(node.clone());
                    }
                }
            }
        }
        
        best_node.ok_or_else(|| BearDogError::not_found("No healthy nodes available"))
    }

    pub fn calculate_distance(&self, loc1: &GeographicalLocation, loc2: &GeographicalLocation) -> f64 {
        let lat1 = loc1.latitude.to_radians();
        let lat2 = loc2.latitude.to_radians();
        let delta_lat = (loc2.latitude - loc1.latitude).to_radians();
        let delta_lon = (loc2.longitude - loc1.longitude).to_radians();
        
        let a = (delta_lat / 2.0).sin().powi(2) + 
                lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        
        6371.0 * c // Earth's radius in kilometers
    }

    pub async fn update_traffic_patterns(&self, patterns: TrafficPatterns) -> BearDogResult<()> {
        let mut traffic_patterns = self.traffic_patterns.write().await;
        *traffic_patterns = patterns;
        Ok(())
    }

    pub async fn get_load_prediction(&self) -> BearDogResult<LoadPredictionModel> {
        let model = self.prediction_model.read().await;
        Ok(model.clone())
    }

    pub async fn shutdown(&self) -> BearDogResult<()> {
        info!("Shutting down intelligent load balancer");
        Ok(())
    }
} 