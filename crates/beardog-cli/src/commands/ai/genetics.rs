

use super::types::OutputFormat;
use clap::{Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum GeneticsOperation {

    Spawn {

        #[arg(long, default_value = "1")]
        count: u32,

        #[arg(long)]
        template: Option<String>,

        config: Option<PathBuf>,

        #[arg(long, value_enum, default_value = "balanced")]
        strategy: SpawnStrategy,

        resources: Option<String>,

        #[arg(long, value_enum, default_value = "json")]
        format: OutputFormat,
    },

    Evolve {

        instances: Vec<String>,

        #[arg(long, value_enum, default_value = "automated")]
        algorithm: EvolutionAlgorithm,

        #[arg(long, default_value = "10")]
        generations: u32,

        #[arg(long, default_value = "0.1")]
        mutation_rate: f32,

        #[arg(long, default_value = "0.5")]
        selection_pressure: f32,

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

        #[arg(long, default_value = "30")]
        timeout: u64,

    Export {

        output: PathBuf,

        #[arg(long, default_value = "json")]
        format: String,

        include_genetics: bool,

    Import {

        input: PathBuf,

        merge: bool,

    Analyze {

        #[arg(long, default_value = "performance")]
        analysis_type: String,

        time_range: Option<String>,
}

#[derive(Debug, Clone, ValueEnum, serde::Serialize, serde::Deserialize)]
pub enum SpawnStrategy {

    Balanced,

    HighPerformance,

    Conservative,

    Custom,

#[derive(Debug, Clone, ValueEnum)]}

pub enum EvolutionAlgorithm {

    Automated,

    Genetic,

    Differential,

    ParticleSwarm,

    SimulatedAnnealing,
