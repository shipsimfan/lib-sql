use super::{constant::InsertConstantBody, InsertBody, InsertVariableBody};
use crate::table::input::Field;

impl InsertBody {
    /// Create the function body to generate from `fields` and `table_name`
    pub fn from_input(fields: &[Field], table_name: &str) -> Self {
        for field in fields {
            if field.default.is_some() {
                return InsertBody::Variable(InsertVariableBody::from_input(fields, table_name));
            }
        }

        InsertBody::Constant(InsertConstantBody::from_input(fields, table_name))
    }
}
