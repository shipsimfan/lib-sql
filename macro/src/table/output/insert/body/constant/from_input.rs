use super::InsertConstantBody;
use crate::table::{
    input::Field,
    output::{BindSites, Binds, Names},
};
use proc_macro_util::tokens::Literal;

impl InsertConstantBody {
    /// Create the function body to generate from `fields` and `table_name`
    pub fn from_input(fields: &[Field], table_name: &str) -> Self {
        let mut bind_sites = 0;
        let mut binds = Vec::with_capacity(fields.len());
        let mut field_names = Vec::with_capacity(fields.len());
        for field in fields {
            if field.auto_increment {
                continue;
            }

            bind_sites += 1;
            field_names.push(field.name.to_string());
            binds.push(field.name.as_ref().clone());
        }

        InsertConstantBody {
            table_name: Literal::new(table_name),
            fields: Names::new(field_names),
            bind_sites: BindSites::new(bind_sites),
            binds: Binds::new(0, binds),
        }
    }
}
