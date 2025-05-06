use proc_macro_util::{
    ast::{Expression, OuterAttribute, Type},
    tokens::Identifier,
};
use std::borrow::Cow;

mod extract;

/// A single field in the input struct
pub struct Field<'a> {
    /// The attributes affecting this field
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The name of the field
    pub name: Cow<'a, Identifier>,

    /// The type of the field
    pub r#type: Type<'a>,

    /// Is this field a primary key?
    pub primary_key: bool,

    /// Does this field auto increment?
    pub auto_increment: bool,

    /// Do the values of this field have to be unique?
    pub unique: bool,

    /// Does this field reference a field in another table?
    pub references: Option<(Identifier, Identifier)>,

    /// Does this field have a minimum?
    pub min: Option<Expression<'a>>,

    /// Does this field have a maximum?
    pub max: Option<Expression<'a>>,

    /// A default value to insert if none is provided
    pub default: Option<Option<Expression<'a>>>,
}
