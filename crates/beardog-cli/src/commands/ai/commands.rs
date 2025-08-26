

use super::config::ConfigOperation;
use super::genetics::GeneticsOperation;
use super::hsm::HsmOperation;
use super::security::SecurityOperation;
use super::types::{OutputFormat, StreamType};
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum AiCommand {

    Assistant {

        #[arg(long)]
        prompt: Option<String>,

        context: Option<PathBuf>,

        #[arg(long, default_value = "default")]}

        model: String,

        #[arg(long, value_enum, default_value = "json")]
        format: OutputFormat,

        #[arg(long, default_value = "0.7")]
        temperature: f32,

        #[arg(long, default_value = "2000")]
        max_tokens: u32,

        system: Option<String>,

        history: Option<PathBuf>,

        stream: bool,
    },

    Execute {
        #[command(subcommand)]
        command: AiSubcommand,
}

pub enum AiSubcommand {

    Status {

        detailed: bool,

        watch: bool,

        #[arg(long, default_value = "5")]
        interval: u64,

    Security {
        operation: SecurityOperation,

    Genetics {
        operation: GeneticsOperation,

    Hsm {
        operation: HsmOperation,

    Batch {

        file: PathBuf,

        #[arg(long, default_value = "10")]
        max_parallel: u32,

        continue_on_error: bool,

        output: Option<PathBuf>,

    Stream {

        #[arg(long, value_enum)]
        stream_type: StreamType,

        #[arg(long, default_value = "0")]
        duration: u64,

    Config {
        operation: ConfigOperation,
