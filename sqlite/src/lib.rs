//! SQL common interface implementation for SQLite3

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod column;
mod connection;
mod execute_error;
mod from_row_error;
mod row;
mod rows;
mod statement;

pub use column::SQLite3Column;
pub use connection::SQLite3Connection;
pub use execute_error::SQLite3ExecuteError;
pub use from_row_error::SQLite3FromRowError;
pub use row::SQLite3Row;
pub use rows::SQLite3Rows;
pub use statement::SQLite3Statement;

pub use sql;
pub use sqlite3::SQLiteError;
