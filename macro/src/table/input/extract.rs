use super::{Fields, Input};
use proc_macro_util::{
    ast::{items::StructBody, Item, ItemKind, VisItemKind},
    Error, Result,
};

impl<'a> Input<'a> {
    /// Extract the required information from `item`
    pub fn extract(item: Item<'a>) -> Result<Self> {
        let vis_item = match item.kind {
            ItemKind::Vis(vis) => vis,
            _ => {
                return Err(Error::new(
                    "only structs can be converted into an sql table",
                ))
            }
        };

        let r#struct = match vis_item.kind {
            VisItemKind::Struct(r#struct) => r#struct,
            _ => {
                return Err(Error::new(
                    "only structs can be converted into an sql table",
                ))
            }
        };

        let fields = match r#struct.body {
            StructBody::Normal {
                where_clause: _,
                fields,
            } => Fields::extract(fields)?,
            _ => return Err(Error::new("struct must have named fields for sql_table")),
        };

        Ok(Input {
            attributes: item.attributes,
            visibility: vis_item.visibility,
            name: r#struct.name,
            fields,
        })
    }
}
