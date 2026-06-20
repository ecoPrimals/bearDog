// SPDX-License-Identifier: AGPL-3.0-or-later

//! Identity, IPC, ecosystem, capabilities, genetics, and neural API environment variable keys.

// ── Identity ─────────────────────────────────────────────────────────

/// Ecosystem family identifier (unprefixed).
pub const ENV_FAMILY_ID: &str = "FAMILY_ID";
/// BearDog-prefixed family identifier.
pub const ENV_FAMILY_ID_PREFIXED: &str = "BEARDOG_FAMILY_ID";
/// Comma-separated bootstrap trusted peers (`peer_id:family_id` pairs).
pub const ENV_TRUSTED_PEERS: &str = "BEARDOG_TRUSTED_PEERS";
/// Ecosystem family seed (unprefixed).
pub const ENV_FAMILY_SEED: &str = "FAMILY_SEED";
/// BearDog-prefixed family seed.
pub const ENV_FAMILY_SEED_PREFIXED: &str = "BEARDOG_FAMILY_SEED";
/// Ecosystem node identifier (unprefixed).
pub const ENV_NODE_ID: &str = "NODE_ID";
/// BearDog-prefixed node identifier.
pub const ENV_NODE_ID_PREFIXED: &str = "BEARDOG_NODE_ID";
/// Primal name for IPC path resolution (unprefixed).
pub const ENV_PRIMAL_NAME: &str = "PRIMAL_NAME";
/// BearDog-prefixed primal name override.
pub const ENV_PRIMAL_NAME_PREFIXED: &str = "BEARDOG_PRIMAL_NAME";
/// Fallback primal name when no env var is set.
pub const DEFAULT_PRIMAL_NAME: &str = "beardog";

/// Resolve the primal's own name from the environment.
///
/// Returns [`ENV_PRIMAL_NAME`] if set, otherwise falls back to the compiled default.
#[must_use]
pub fn resolve_primal_name() -> String {
    std::env::var(ENV_PRIMAL_NAME).unwrap_or_else(|_| DEFAULT_PRIMAL_NAME.to_owned())
}
/// Primal type / role (unprefixed).
pub const ENV_PRIMAL_TYPE: &str = "PRIMAL_TYPE";
/// BearDog-prefixed primal type override.
pub const ENV_PRIMAL_TYPE_PREFIXED: &str = "BEARDOG_PRIMAL_TYPE";
/// Orchestrator identifier override.
pub const ENV_ORCHESTRATOR_ID: &str = "BEARDOG_ORCHESTRATOR_ID";
/// System hostname.
pub const ENV_HOSTNAME: &str = "HOSTNAME";
/// Windows hostname (`COMPUTERNAME`).
pub const ENV_COMPUTERNAME: &str = "COMPUTERNAME";
/// Service type alias for primal type detection.
pub const ENV_SERVICE_TYPE: &str = "SERVICE_TYPE";
/// Service display name (unprefixed).
pub const ENV_SERVICE_NAME: &str = "SERVICE_NAME";
/// Human-readable display name override.
pub const ENV_DISPLAY_NAME: &str = "BEARDOG_DISPLAY_NAME";
/// Deployment environment (`development`, `staging`, `production`).
pub const ENV_ENVIRONMENT: &str = "BEARDOG_ENVIRONMENT";
/// Runtime environment alias (`BEARDOG_ENV`).
pub const ENV_ENV: &str = "BEARDOG_ENV";
/// Deployment identifier (unprefixed).
pub const ENV_DEPLOYMENT_ID: &str = "DEPLOYMENT_ID";
/// Real user id (Unix).
pub const ENV_UID: &str = "UID";
/// Effective user id (Unix).
pub const ENV_EUID: &str = "EUID";

// ── Socket / IPC ─────────────────────────────────────────────────────

/// Primal-specific Unix socket path (tier 1).
pub const ENV_SOCKET: &str = "BEARDOG_SOCKET";
/// Override root for tier-5 temp socket fallback.
pub const ENV_SOCKET_TMP_DIR: &str = "BEARDOG_SOCKET_TMP_DIR";
/// Comma-separated IPC capability domain stems for symlink creation.
pub const ENV_IPC_CAPABILITY_STEMS: &str = "BEARDOG_IPC_CAPABILITY_STEMS";
/// Windows named pipe path override.
pub const ENV_PIPE: &str = "BEARDOG_PIPE";
/// Neural registration instance override.
pub const ENV_NEURAL_REGISTRATION_INSTANCE: &str = "BEARDOG_NEURAL_REGISTRATION_INSTANCE";
/// Neural API socket path.
pub const ENV_NEURAL_API_SOCKET: &str = "NEURAL_API_SOCKET";
/// Legacy alias for [`ENV_NEURALS_SOCKET`] (typo-tolerant `NEURALS_SOCKET` env var).
pub const ENV_NEURAL_API_SOCKET_LEGACY: &str = "NEURALS_SOCKET";
/// Neurals socket path.
pub const ENV_NEURALS_SOCKET: &str = "NEURALS_SOCKET";
/// Override the Neural API socket filename (default: `neural-api.sock`).
pub const ENV_NEURAL_API_SOCKET_NAME: &str = "BEARDOG_NEURAL_API_SOCKET_NAME";

// ── Ecosystem (biomeOS) ──────────────────────────────────────────────

/// Disable BTSP production security (`1` or `true`).
pub const ENV_BIOMEOS_INSECURE: &str = "BIOMEOS_INSECURE";
/// biomeOS family label.
pub const ENV_BIOMEOS_FAMILY: &str = "BIOMEOS_FAMILY";
/// Generic orchestrator socket path (tier 2).
pub const ENV_BIOMEOS_SOCKET_PATH: &str = "BIOMEOS_SOCKET_PATH";
/// Generic orchestrator socket directory (tier 2).
pub const ENV_BIOMEOS_SOCKET_DIR: &str = "BIOMEOS_SOCKET_DIR";
/// Windows biomeOS named pipe directory.
pub const ENV_BIOMEOS_PIPE_DIR: &str = "BIOMEOS_PIPE_DIR";

// ── Capabilities / self-knowledge ────────────────────────────────────

/// Enable HSM capability flag.
pub const ENV_CAPABILITY_HSM: &str = "BEARDOG_CAPABILITY_HSM";
/// Enable encryption capability flag.
pub const ENV_CAPABILITY_ENCRYPTION: &str = "BEARDOG_CAPABILITY_ENCRYPTION";
/// Enable auth capability flag.
pub const ENV_CAPABILITY_AUTH: &str = "BEARDOG_CAPABILITY_AUTH";
/// gRPC host address override.
pub const ENV_GRPC_HOST: &str = "BEARDOG_GRPC_HOST";

// ── Genetics constraints ─────────────────────────────────────────────────

/// Entropy quality threshold (0.0–1.0).
pub const ENV_ENTROPY_QUALITY_THRESHOLD: &str = "BEARDOG_ENTROPY_QUALITY_THRESHOLD";
/// Multisig enforcement mode.
pub const ENV_MULTISIG_MODE: &str = "BEARDOG_MULTISIG_MODE";
/// Multisig signature threshold.
pub const ENV_MULTISIG_THRESHOLD: &str = "BEARDOG_MULTISIG_THRESHOLD";
/// Behavioral verification mode.
pub const ENV_BEHAVIORAL_MODE: &str = "BEARDOG_BEHAVIORAL_MODE";
/// Physical attestation mode.
pub const ENV_ATTESTATION_MODE: &str = "BEARDOG_ATTESTATION_MODE";
/// Genetic algorithm population size.
pub const ENV_GENETICS_POPULATION_SIZE: &str = "BEARDOG_GENETICS_POPULATION_SIZE";
/// Genetic algorithm mutation rate.
pub const ENV_GENETICS_MUTATION_RATE: &str = "BEARDOG_GENETICS_MUTATION_RATE";
/// Genetic algorithm crossover rate.
pub const ENV_GENETICS_CROSSOVER_RATE: &str = "BEARDOG_GENETICS_CROSSOVER_RATE";
/// Genetic algorithm elitism percentage.
pub const ENV_GENETICS_ELITISM_PERCENTAGE: &str = "BEARDOG_GENETICS_ELITISM_PERCENTAGE";
/// Genetic algorithm max generations.
pub const ENV_GENETICS_MAX_GENERATIONS: &str = "BEARDOG_GENETICS_MAX_GENERATIONS";
/// Genetic algorithm fitness threshold.
pub const ENV_GENETICS_FITNESS_THRESHOLD: &str = "BEARDOG_GENETICS_FITNESS_THRESHOLD";
/// Enable adaptive genetic algorithm parameters.
pub const ENV_GENETICS_ADAPTIVE_PARAMETERS: &str = "BEARDOG_GENETICS_ADAPTIVE_PARAMETERS";

// ── Neural API (extended) ──────────────────────────────────────────────

/// Legacy Neural API socket path override.
pub const ENV_NEURAL_API_LEGACY_SOCKET: &str = "BEARDOG_NEURAL_API_LEGACY_SOCKET";
