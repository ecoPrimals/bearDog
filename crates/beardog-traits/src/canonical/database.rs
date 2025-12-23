// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::base::BaseProvider;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Temporary type definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    /// Collection of rows
    pub rows: Vec<std::collections::HashMap<String, serde_json::Value>>,
    /// Number of `affected_rows`
    pub affected_rows: u64,
}

#[allow(clippy::type_complexity)]
pub trait DatabaseProvider: BaseProvider {
    /// Executes query
    fn execute_query(
        query: &str,
        params: Option<HashMap<&str, &str>>,
    ) -> impl std::future::Future<Output = Result<QueryResult, BearDogError>> + Send;

    fn insert_record(
        table: &str,
        data: HashMap<&str, &str>,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    /// Updates record
    fn update_record(
        table: &str,
        id: &str,
        data: HashMap<&str, &str>,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Removes record
    fn delete_record(
        table: &str,
        id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Gets record
    fn get_record(
        table: &str,
        id: &str,
    ) -> impl std::future::Future<Output = Result<Option<HashMap<String, String>>, BearDogError>> + Send;

    fn list_records(
        table: &str,
        filter: Option<HashMap<&str, &str>>,
        limit: Option<usize>,
    ) -> impl std::future::Future<Output = Result<Vec<HashMap<String, String>>, BearDogError>> + Send;

    fn begin_transaction(
        &self,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    fn commit_transaction(
        transaction_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    fn rollback_transaction(
        transaction_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Creates backup
    fn create_backup(
        location: &str,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    fn restore_backup(
        backup_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;
}
