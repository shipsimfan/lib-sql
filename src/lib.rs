//! Common interface definition for all supported SQL database management systems

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod bind;
mod column;
mod connection;
mod row;
mod statement;
mod transaction;

pub use bind::Bind;
pub use column::{Column, FromColumn};
pub use connection::Connection;
pub use row::{FromRow, FromRowError, Row};
pub use statement::Statement;
pub use transaction::Transaction;
