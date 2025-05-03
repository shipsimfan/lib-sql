use proc_macro_util::{
    ast::{Expression, OuterAttribute, Type},
    tokens::Identifier,
};
use std::borrow::Cow;

mod extract;

/// A single field in the input struct
pub struct Field<'a> {
    /// The attributes affecting this field
    attributes: Vec<OuterAttribute<'a>>,

    /// The name of the field
    name: Cow<'a, Identifier>,

    /// The type of the field
    r#type: Type<'a>,

    /// Is this field a primary key?
    primary_key: bool,

    /// Does this field auto increment?
    auto_increment: bool,

    /// Do the values of this field have to be unique?
    unique: bool,

    /// Does this field reference a field in another table?
    references: Option<(Identifier, Identifier)>,

    /// Does this field have a minimum?
    min: Option<Expression<'a>>,

    /// Does this field have a maximum?
    max: Option<Expression<'a>>,
}
