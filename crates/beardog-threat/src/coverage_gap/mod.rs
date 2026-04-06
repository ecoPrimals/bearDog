// SPDX-License-Identifier: AGPL-3.0-or-later

//! Coverage gap tests for beardog-threat (split by domain).

#![allow(
    unused_imports,
    clippy::float_cmp,
    clippy::absurd_extreme_comparisons,
    unused_comparisons,
    clippy::nonminimal_bool
)]

mod detection_management;
mod incidents_and_conditions;
mod ml_models_and_ml_engine;
mod rules_and_response;
mod threat_api_and_core_engine;
mod threat_engine_core;
mod threat_types_mod;
