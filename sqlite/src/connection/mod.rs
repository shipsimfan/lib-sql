use sqlite3::SQLite3;

mod column;
mod rows;
mod statement;
mod transaction;

mod connection;
mod drop;
mod execute;
mod open;
mod prepare;
mod sql_context;

pub use column::{SQLite3Column, SQLite3FromColumnError};
pub use rows::{SQLite3Row, SQLite3Rows};
pub use statement::SQLite3Statement;
pub use transaction::SQLite3Transaction;

/// A connection an SQLite3 database
pub struct SQLite3Connection {
    handle: *mut SQLite3,
}

unsafe impl Send for SQLite3Connection {}
