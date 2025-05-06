use crate::table::output::{BindSites, Binds, Names};
use proc_macro_util::tokens::Literal;

mod from_input;
mod to_tokens;

/// Generates the body of the insert function when the SQL query can be a constant
pub struct InsertConstantBody {
    /// The name of the table to insert the row into
    table_name: Literal,

    /// The fields being inserted
    fields: Names,

    /// The binding sites for the fields
    bind_sites: BindSites,

    /// The binds to the statement
    binds: Binds,
}
