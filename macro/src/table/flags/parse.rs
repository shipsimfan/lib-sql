use super::TableFlags;
use proc_macro_util::{tokens::Identifier, Error, Parse, Parser, Result, Token};

impl<'a> Parse<'a> for TableFlags {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let mut not_deletable = false;
        let mut not_insertable = false;
        let mut not_updatable = false;
        let mut not_selectable = false;

        while !parser.empty() {
            let name_ident = parser.parse::<&'a Identifier>()?;
            let name = name_ident.to_string();
            match name.as_str() {
                "not_deletable" => not_deletable = true,
                "not_insertable" => not_insertable = true,
                "not_updatable" => not_updatable = true,
                "not_selectable" => not_selectable = true,
                _ => return Err(Error::new_at("unknown flag", name_ident.span())),
            }

            if parser.step_parse::<Token![,]>().is_err() {
                break;
            }
        }

        if !parser.empty() {
            return Err(Error::new_at("unexpected token", parser.span()));
        }

        Ok(TableFlags {
            not_deletable,
            not_insertable,
            not_updatable,
            not_selectable,
        })
    }
}
