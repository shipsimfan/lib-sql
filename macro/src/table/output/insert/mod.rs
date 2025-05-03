use proc_macro_util::{
    ast::{OuterAttribute, Visibility},
    tokens::Identifier,
};

mod from_input;
mod to_tokens;

/// Generates code to allow inserting rows into a table
pub struct InsertOutput<'a> {
    /// The attributes affecting the struct
    attributes: Vec<OuterAttribute<'a>>,

    /// The visibility of the struct
    visibility: Option<Visibility<'a>>,

    /// The name of this struct
    name: Identifier,
}
