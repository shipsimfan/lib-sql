use super::Fields;
use proc_macro_util::{ast::items::StructFields, Result};

impl Fields {
    pub fn extract(fields: Option<StructFields>) -> Result<Self> {
        let raw_fields = match fields {
            Some(raw_fields) => raw_fields,
            None => return Ok(Fields { fields: Vec::new() }),
        };

        let mut fields = Vec::with_capacity(raw_fields.remaining.len() + 1);

        Ok(Fields { fields })
    }
}
