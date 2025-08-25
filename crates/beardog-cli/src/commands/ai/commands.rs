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


/// # AI CLI Commands
///
/// **EXTRACTED FROM LARGE FILE** - Main command definitions and enums (~100 lines)
/// This module contains the main CLI command structure and subcommand definitions
/// for the AI-first CLI interface.
use super::config::ConfigOperation;
use super::genetics::GeneticsOperation;
use super::hsm::HsmOperation;
use super::security::SecurityOperation;
use super::types::{OutputFormat, StreamType};
use clap::Subcommand;
use std::path::PathBuf;

/// AI CLI command group
#[derive(Debug, Subcommand)]
pub enum AiCommand {
    /// Interactive AI assistant
    Assistant {
        /// Input prompt
        #[arg(long)]
        prompt: Option<String>,
        /// Context file
        context: Option<PathBuf>,
        /// AI model to use
        #[arg(long, default_value = "default")]}


        model: String,
        /// Output format
        #[arg(long, value_enum, default_value = "json")]
        format: OutputFormat,
        /// Temperature for AI responses
        #[arg(long, default_value = "0.7")]
        temperature: f32,
        /// Maximum tokens in response
        #[arg(long, default_value = "2000")]
        max_tokens: u32,
        /// System message/context
        system: Option<String>,
        /// Conversation history file
        history: Option<PathBuf>,
        /// Enable streaming responses
        stream: bool,
    },
    /// Execute AI subcommand
    Execute {
        #[command(subcommand)]
        command: AiSubcommand,
}
/// AI subcommands
pub enum AiSubcommand {
    /// System status and health
    Status {
        /// Include detailed metrics
        detailed: bool,
        /// Monitor continuously
        watch: bool,
        /// Watch interval in seconds
        #[arg(long, default_value = "5")]
        interval: u64,
    /// Security operations
    Security {
        operation: SecurityOperation,
    /// Genetic spawning operations
    Genetics {
        operation: GeneticsOperation,
    /// HSM operations
    Hsm {
        operation: HsmOperation,
    /// Batch operations
    Batch {
        /// Batch operation file (JSON)
        file: PathBuf,
        /// Maximum parallel operations
        #[arg(long, default_value = "10")]
        max_parallel: u32,
        /// Continue on error
        continue_on_error: bool,
        /// Output file for results
        output: Option<PathBuf>,
    /// Stream events
    Stream {
        /// Stream type
        #[arg(long, value_enum)]
        stream_type: StreamType,
        /// Output file (optional)
        /// Duration in seconds (0 for infinite)
        #[arg(long, default_value = "0")]
        duration: u64,
    /// Configuration management
    Config {
        operation: ConfigOperation,
