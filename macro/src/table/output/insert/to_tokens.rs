use super::InsertOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for InsertOutput<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let InsertOutput {
            attributes,
            visibility,
            name,
        } = self;

        let name2 = name.clone();

        #[cfg(feature = "lib-data-format")]
        let derive = Some(super::super::derive_deserialize::DeriveDeserialize);
        #[cfg(not(feature = "lib-data-format"))]
        let derive: Option<InsertOutput> = None;

        to_tokens! { generator
            #attributes
            #derive
            #visibility struct #name {

            }

            impl #name2 {
                /// Inserts a single row into the table
                pub fn insert<'a, DB: ::sql::SqlContext<'a>>(&self, db: DB) {

                }

                /// Inserts multiple rows into the table
                pub fn insert_many<'a, 'b, DB: ::sql::SqlContext<'a>, I: IntoIterator<Item = &'b Self>>(db: DB, iter: I) {

                }
            }
        }
    }
}
