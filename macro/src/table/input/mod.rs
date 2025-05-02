use fields::Fields;
use proc_macro_util::{
    ast::{GenericParams, OuterAttribute, Visibility, WhereClause},
    tokens::Identifier,
};
use std::borrow::Cow;

mod fields;

mod extract;

/// The extracted input values for potentially producing the required tables
pub struct Input<'a> {
    /// The attributes defined for the struct
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The visibility of the struct
    pub visibility: Option<Visibility<'a>>,

    /// The name of the struct
    pub name: Cow<'a, Identifier>,

    /// The fields which make up the struct
    pub fields: Fields,
}
