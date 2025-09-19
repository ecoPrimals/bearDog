

use super::config::ConfigOperation;
use super::genetics::GeneticsOperation;
use super::hsm::HsmOperation;
use super::security::SecurityOperation;
use super::types::{OutputFormat, StreamType};
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Debug, Clone)]
        context: Option<PathBuf>,

        #[arg(String,

        #[arg(OutputFormat,

        #[arg(f32,

        #[arg(u32,

        system: Option<String>,

        history: Option<PathBuf>,

        stream: bool,
    },

    Execute {
        #[command(AiSubcommand,
}

pub enum AiSubcommand {

    /// Represents status variant
    Status {

        detailed: bool,

        watch: bool,

        #[arg(u64,

    /// Represents security variant
    Security {
        operation: SecurityOperation,

    /// Represents genetics variant
    Genetics {
        operation: GeneticsOperation,

    /// Represents hsm variant
    Hsm {
        operation: HsmOperation,

    /// Represents batch variant
    Batch {

        file: PathBuf,

        #[arg(u32,

        continue_on_error: bool,

        output: Option<PathBuf>,

    /// Represents stream variant
    Stream {

        #[arg(StreamType,

        #[arg(u64,

    /// Represents config variant
    Config {
        operation: ConfigOperation,
