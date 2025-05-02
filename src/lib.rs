//! Common interface definition for all supported SQL database management systems

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod bind;
mod column;
mod connection;
mod connection_pool;
mod context;
mod row;
mod statement;
mod transaction;

pub use bind::Bind;
pub use column::{Column, FromColumn, FromColumnError};
pub use connection::Connection;
pub use connection_pool::ConnectionPool;
pub use context::SqlContext;
pub use row::{FromRow, FromRowError, Row};
pub use statement::Statement;
pub use transaction::Transaction;

#[cfg(feature = "macro")]
pub use sql_macro::sql_table;
