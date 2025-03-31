use sqlite3::SQLite3;

mod column;
mod execute_error;
mod rows;
mod statement;
mod transaction;

mod connection;
mod drop;
mod execute;
mod open;
mod prepare;

pub use column::SQLite3Column;
pub use execute_error::SQLite3ExecuteError;
pub use rows::{SQLite3FromRowError, SQLite3Row, SQLite3Rows};
pub use statement::SQLite3Statement;
pub use transaction::SQLite3Transaction;

/// A connection an SQLite3 database
pub struct SQLite3Connection {
    handle: *mut SQLite3,
}

unsafe impl Send for SQLite3Connection {}
