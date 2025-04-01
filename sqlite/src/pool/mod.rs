use crate::SQLite3Connection;
use std::sync::Mutex;

mod connection_pool;
mod open;

/// A connection pool allowing many users access to the same database
pub struct SQLite3ConnectionPool {
    /// The underlying connection to the database
    connection: Mutex<SQLite3Connection>,
}
