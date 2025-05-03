use insert::InsertOutput;

#[cfg(feature = "lib-data-format")]
mod derive_deserialize;
mod insert;

mod from_input;
mod to_tokens;

/// The output to generate
pub struct Output<'a> {
    /// Generates the code to insert new rows
    insert: Option<InsertOutput<'a>>,
}
