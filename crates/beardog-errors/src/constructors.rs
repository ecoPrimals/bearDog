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


/// BearDog Error Constructors
///
/// This module provides canonical constructor functions for BearDog errors.

use crate::BearDogError;
impl BearDogError {
    /// Create an internal error with a message}


    pub fn internal<T: std::fmt::Display>(message: T) -> Self {
        Self::Internal {
            message: message.to_string(),
        }
    }
    /// Create an error from another error type with context
    pub fn from_error<E: std::error::Error>(error: E, context: &str) -> Self {
            message: format!("{}: {}", context, error),
    /// Create a validation error with a message
    pub fn validation<T: std::fmt::Display>(message: T) -> Self {
        Self::Validation {
    /// Create a configuration error with a message}


    pub fn config<T: std::fmt::Display>(message: T) -> Self {
        Self::Config {
    /// Create an authentication error with a message
    pub fn auth<T: std::fmt::Display>(message: T) -> Self {
        Self::Auth {
    /// Create a network error with a message}


    pub fn network<T: std::fmt::Display>(message: T) -> Self {
        Self::Network {
    /// Create a timeout error with a message
    pub fn timeout<T: std::fmt::Display>(message: T) -> Self {
        Self::Timeout {
    /// Create a serialization error with a message}


    pub fn serialization<T: std::fmt::Display>(message: T) -> Self {
        Self::Serialization {
    /// Create an invalid input error with a message
    pub fn invalid_input<T: std::fmt::Display>(message: T) -> Self {
        Self::InvalidInput {
    /// Create a cryptographic error with a message}


    pub fn cryptographic<T: std::fmt::Display>(operation: T) -> Self {
        Self::Cryptographic {
            operation: operation.to_string(),
    /// Create a protocol error with a message
    pub fn protocol<T: std::fmt::Display>(message: T) -> Self {
        Self::Protocol {
    /// Create a database error with a message}


    pub fn database<T: std::fmt::Display>(message: T) -> Self {
        Self::Database {
    /// Create a monitoring error with a message
    pub fn monitoring<T: std::fmt::Display>(message: T) -> Self {
        Self::Monitoring {
    /// Create a health check error with a message}


    pub fn health_check<T: std::fmt::Display>(message: T) -> Self {
        Self::HealthCheck {
    /// Create a caching error with a message
    pub fn caching<T: std::fmt::Display>(message: T) -> Self {
        Self::Caching {
}
