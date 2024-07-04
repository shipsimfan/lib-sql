//! Common interface definition for all supported SQL database management systems

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod bind;
mod column;
mod connection;
mod from_column;
mod from_row;
mod row;
mod statement;

pub use bind::Bind;
pub use column::Column;
pub use connection::Connection;
pub use from_column::FromColumn;
pub use from_row::FromRow;
pub use row::Row;
pub use statement::Statement;
