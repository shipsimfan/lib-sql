//! SQL common interface implementation for SQLite3

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod connection;
mod pool;

pub use connection::*;
pub use pool::SQLite3ConnectionPool;

pub use sql;
pub use sqlite3::SQLiteError;
