use super::{InsertBody, InsertOutput, InsertStructField};
use crate::table::input::Input;
use proc_macro_util::tokens::Identifier;

impl<'a> InsertOutput<'a> {
    /// Create the structures to generate from `input`
    pub fn from_input(input: &Input<'a>) -> Self {
        let table_name = input.name.to_string();
        let name = Identifier::new(&format!("New{table_name}"));

        let mut struct_fields = Vec::with_capacity(input.fields.fields.len());
        for field in &input.fields.fields {
            if let Some(struct_field) = InsertStructField::from_input(field) {
                struct_fields.push(struct_field);
            }
        }

        let body = InsertBody::from_input(&input.fields.fields, &table_name);

        InsertOutput {
            attributes: input.attributes.clone(),
            visibility: input.visibility.clone(),
            name,
            struct_fields,
            body,
        }
    }
}
