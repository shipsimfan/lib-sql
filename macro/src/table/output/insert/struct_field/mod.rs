use proc_macro_util::{
    ast::{OuterAttribute, Type},
    tokens::Identifier,
};
use std::borrow::Cow;

mod from_input;
mod to_tokens;

/// A field in the insert struct
pub struct InsertStructField<'a> {
    /// The attributes affecting this field
    attributes: Vec<OuterAttribute<'a>>,

    /// The name of the field
    name: Cow<'a, Identifier>,

    /// The type of the field
    r#type: Type<'a>,

    /// Does this value a default?
    has_default: bool,
}
