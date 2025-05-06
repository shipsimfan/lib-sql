use bind_sites::BindSites;
use binds::Binds;
use insert::InsertOutput;
use names::Names;

mod bind_sites;
mod binds;
#[cfg(feature = "lib-data-format")]
mod derive_deserialize;
mod insert;
mod names;

mod from_input;
mod to_tokens;

/// The output to generate
pub struct Output<'a> {
    /// Generates the code to insert new rows
    insert: Option<InsertOutput<'a>>,
}
