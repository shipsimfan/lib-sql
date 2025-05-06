use body::InsertBody;
use proc_macro_util::{
    ast::{OuterAttribute, Visibility},
    tokens::Identifier,
};
use struct_field::InsertStructField;

mod body;
mod struct_field;

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

    /// The fields to insert into the structure
    struct_fields: Vec<InsertStructField<'a>>,

    /// The body of the insert function
    body: InsertBody,
}
