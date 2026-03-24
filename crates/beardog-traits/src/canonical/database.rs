// SPDX-License-Identifier: AGPL-3.0-only

//! Generic relational-style [`DatabaseProvider`] with stringly-typed rows and transactions.

use super::base::BaseProvider;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result set and write-affected row count from [`DatabaseProvider::execute_query`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    /// Collection of rows
    pub rows: Vec<std::collections::HashMap<String, serde_json::Value>>,
    /// Number of `affected_rows`
    pub affected_rows: u64,
}

/// CRUD, listing, transactions, and backup hooks for embedded or remote databases.
pub trait DatabaseProvider: BaseProvider {
    /// Executes query
    fn execute_query(
        query: &str,
        params: Option<HashMap<&str, &str>>,
    ) -> impl std::future::Future<Output = Result<QueryResult, BearDogError>> + Send;

    /// Inserts a row and returns a generated id when applicable.
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

    /// Paginated scan with optional equality filters.
    fn list_records(
        table: &str,
        filter: Option<HashMap<&str, &str>>,
        limit: Option<usize>,
    ) -> impl std::future::Future<Output = Result<Vec<HashMap<String, String>>, BearDogError>> + Send;

    /// Opens a transaction; returns an opaque handle for commit/rollback.
    fn begin_transaction(
        &self,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    /// Persists all operations in the transaction identified by `transaction_id`.
    fn commit_transaction(
        transaction_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Discards all operations in the transaction identified by `transaction_id`.
    fn rollback_transaction(
        transaction_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Creates backup
    fn create_backup(
        location: &str,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    /// Restores from a backup created by [`Self::create_backup`].
    fn restore_backup(
        backup_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;
}
