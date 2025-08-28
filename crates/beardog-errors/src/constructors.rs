use beardog_errors::BearDogError;

use crate::BearDogError;
impl BearDogError {

    pub fn internal<T: std::fmt::Display>(message: T) -> Self {
        Self::Internal {
            message: message.to_string(),
        }
    }

    pub fn from_error<E: std::error::Error>(error: E, context: &str) -> Self {
            message: format_args!("{}: {}", context, error).to_string(),

    pub fn validation<T: std::fmt::Display>(message: T) -> Self {
        Self::Validation {

    pub fn config<T: std::fmt::Display>(message: T) -> Self {
        Self::Config {

    pub fn auth<T: std::fmt::Display>(message: T) -> Self {
        Self::Auth {

    pub fn network<T: std::fmt::Display>(message: T) -> Self {
        Self::Network {

    pub fn timeout<T: std::fmt::Display>(message: T) -> Self {
        Self::Timeout {

    pub fn serialization<T: std::fmt::Display>(message: T) -> Self {
        Self::Serialization {

    pub fn invalid_input<T: std::fmt::Display>(message: T) -> Self {
        Self::InvalidInput {

    pub fn cryptographic<T: std::fmt::Display>(operation: T) -> Self {
        Self::Cryptographic {
            operation: operation.to_string(),

    pub fn protocol<T: std::fmt::Display>(message: T) -> Self {
        Self::Protocol {

    pub fn database<T: std::fmt::Display>(message: T) -> Self {
        Self::Database {

    pub fn monitoring<T: std::fmt::Display>(message: T) -> Self {
        Self::Monitoring {

    pub fn health_check<T: std::fmt::Display>(message: T) -> Self {
        Self::HealthCheck {

    pub fn caching<T: std::fmt::Display>(message: T) -> Self {
        Self::Caching {
}
