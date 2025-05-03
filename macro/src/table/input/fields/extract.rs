use super::{Field, Fields};
use proc_macro_util::{ast::items::StructFields, Result};

impl<'a> Fields<'a> {
    /// Extracts the required information from `fields`
    pub fn extract(fields: Option<StructFields<'a>>) -> Result<Self> {
        let raw_fields = match fields {
            Some(raw_fields) => raw_fields,
            None => return Ok(Fields { fields: Vec::new() }),
        };

        let mut fields = Vec::with_capacity(raw_fields.remaining.len() + 1);
        fields.push(Field::extract(raw_fields.first)?);

        for (_, field) in raw_fields.remaining {
            fields.push(Field::extract(field)?);
        }

        Ok(Fields { fields })
    }
}
