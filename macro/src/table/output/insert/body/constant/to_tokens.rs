use super::InsertConstantBody;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for InsertConstantBody {
    fn to_tokens(self, generator: &mut Generator) {
        let InsertConstantBody {
            table_name,
            fields,
            bind_sites,
            binds,
        } = self;

        to_tokens! { generator
            #[allow(unused_imports)]
            use ::sql::Statement;

            const SQL: &str = concat!("INSERT INTO \"", #table_name, "\" (", #fields, ") VALUES (", #bind_sites, ");");

            #[allow(unused_mut)]
            let mut statement = db.prepare(SQL)?;

            #binds

            statement.execute()
        };
    }
}
