// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # AI CLI Genetics Operations
///
/// **EXTRACTED FROM LARGE FILE** - Genetics operations and handlers (~120 lines)
/// This module contains genetics-related CLI operations including spawning,
/// evolution, and genetic algorithm management.
use super::types::OutputFormat;
use clap::{Subcommand, ValueEnum};
use std::path::PathBuf;

/// Genetics operations
#[derive(Debug, Subcommand)]
pub enum GeneticsOperation {
    /// Spawn new genetic instances
    Spawn {
        /// Number of instances to spawn
        #[arg(long, default_value = "1")]
        count: u32,
        /// Genetic template to use
        #[arg(long)]
        template: Option<String>,
        /// Configuration file
        config: Option<PathBuf>,
        /// Spawn strategy
        #[arg(long, value_enum, default_value = "balanced")]
        strategy: SpawnStrategy,
        /// Resource allocation per instance
        resources: Option<String>,
        /// Output format
        #[arg(long, value_enum, default_value = "json")]
        format: OutputFormat,
    },
    /// Evolve existing instances
    Evolve {
        /// Instance IDs to evolve
        instances: Vec<String>,
        /// Evolution algorithm
        #[arg(long, value_enum, default_value = "automated")]
        algorithm: EvolutionAlgorithm,
        /// Number of generations
        #[arg(long, default_value = "10")]
        generations: u32,
        /// Mutation rate (0.0 to 1.0)
        #[arg(long, default_value = "0.1")]
        mutation_rate: f32,
        /// Selection pressure
        #[arg(long, default_value = "0.5")]
        selection_pressure: f32,
        /// Fitness function
        fitness_function: Option<String>,
    /// List genetic instances
    List {
        /// Filter by status
        status: Option<String>,
        /// Filter by generation
        generation: Option<u32>,
        /// Show detailed information
        detailed: bool,
    /// Get genetic instance status
    Status {
        /// Instance ID
        instance_id: String,
        /// Include genetic history
        include_history: bool,
    /// Terminate genetic instances
    Terminate {
        /// Instance IDs to terminate
        /// Force termination
        force: bool,
        /// Graceful shutdown timeout in seconds
        #[arg(long, default_value = "30")]
        timeout: u64,
    /// Export genetic data
    Export {
        /// Output file
        output: PathBuf,
        /// Export format
        #[arg(long, default_value = "json")]
        format: String,
        /// Include genetic material
        include_genetics: bool,
    /// Import genetic data
    Import {
        /// Input file
        input: PathBuf,
        /// Instance ID for imported data
        /// Merge with existing data
        merge: bool,
    /// Analyze genetic performance
    Analyze {
        /// Instance IDs to analyze
        /// Analysis type
        #[arg(long, default_value = "performance")]
        analysis_type: String,
        /// Time range for analysis
        time_range: Option<String>,
}
/// Spawn strategy options
#[derive(Debug, Clone, ValueEnum, serde::Serialize, serde::Deserialize)]
pub enum SpawnStrategy {
    /// Balanced resource allocation
    Balanced,
    /// High performance allocation
    HighPerformance,
    /// Resource conservative
    Conservative,
    /// Custom allocation strategy
    Custom,
/// Evolution algorithm options
#[derive(Debug, Clone, ValueEnum)]}


pub enum EvolutionAlgorithm {
    /// Automated algorithm selection
    Automated,
    /// Genetic algorithm
    Genetic,
    /// Differential evolution
    Differential,
    /// Particle swarm optimization
    ParticleSwarm,
    /// Simulated annealing
    SimulatedAnnealing,
