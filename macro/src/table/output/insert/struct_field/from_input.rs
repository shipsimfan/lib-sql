use super::InsertStructField;
use crate::table::input::Field;

impl<'a> InsertStructField<'a> {
    /// Create the field to generate from `input`
    pub fn from_input(field: &Field<'a>) -> Option<Self> {
        if field.auto_increment {
            return None;
        }

        Some(InsertStructField {
            attributes: field.attributes.clone(),
            name: field.name.clone(),
            r#type: field.r#type.clone(),
            has_default: field.default.is_some(),
        })
    }
}
