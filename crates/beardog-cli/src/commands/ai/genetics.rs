

use super::types::OutputFormat;
use clap::{Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone)]
        #[arg(Option<String>,

        config: Option<PathBuf>,

        #[arg(SpawnStrategy,

        resources: Option<String>,

        #[arg(OutputFormat,
    },

    Evolve {

        instances: Vec<String>,

        #[arg(EvolutionAlgorithm,

        #[arg(u32,

        #[arg(f32,

        #[arg(f32,

        fitness_function: Option<String>,

    List {

        status: Option<String>,

        generation: Option<u32>,

        detailed: bool,

    Status {

        instance_id: String,

        include_history: bool,

    Terminate {

        force: bool,

        #[arg(u64,

    Export {

        output: PathBuf,

        #[arg(String,

        include_genetics: bool,

    Import {

        input: PathBuf,

        merge: bool,

    Analyze {

        #[arg(String,

        time_range: Option<String>,
}

#[derive(Debug, Clone, ValueEnum, serde::Serialize, serde::Deserialize)]
pub enum SpawnStrategy {


    /// State indicating balanced
    Balanced,


    HighPerformance,


    /// Represents conservative variant
    Conservative,


    /// Represents custom variant
    Custom,

#[derive(Debug, Clone, ValueEnum)]}
#[derive(Debug, Clone, ValueEnum)]}
#[derive(Debug, Clone, ValueEnum)]}

pub enum EvolutionAlgorithm {


    /// State indicating automated
    Automated,


    /// Represents genetic variant
    Genetic,


    /// Represents differential variant
    Differential,


    /// Represents particle swarm variant
    ParticleSwarm,


    /// Currently simulatedannealing
    SimulatedAnnealing,
