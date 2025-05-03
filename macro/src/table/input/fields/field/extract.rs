use super::Field;
use proc_macro_util::{
    ast::{items::StructField, AttrInput, SimplePathSegment},
    Error, Result, Token,
};

impl<'a> Field<'a> {
    /// Extracts the needed information from `field`
    pub fn extract(field: StructField<'a>) -> Result<Self> {
        let mut attributes = Vec::new();
        let mut primary_key = false;
        let mut auto_increment = false;
        let mut unique = false;
        let mut references = None;
        let mut min = None;
        let mut max = None;

        for attribute in field.attributes {
            if attribute.attr.path.leading.is_some() || attribute.attr.path.remaining.len() > 0 {
                attributes.push(attribute);
                continue;
            }

            let (identifier, span) = match &attribute.attr.path.first {
                SimplePathSegment::Identifier(identifier) => {
                    (identifier.to_string(), identifier.span())
                }
                _ => {
                    attributes.push(attribute);
                    continue;
                }
            };

            match identifier.as_str() {
                "primary_key" => primary_key = true,
                "auto_increment" => auto_increment = true,
                "unique" => unique = true,
                "references" => {
                    if references.is_some() {
                        return Err(Error::new_at(
                            "cannot have multiple references on a single field",
                            span,
                        ));
                    }

                    let group = match attribute.attr.input {
                        Some(AttrInput::Group(group)) => group,
                        _ => {
                            return Err(Error::new_at(
                                "`references` must have table and field in parentheses",
                                span,
                            ))
                        }
                    };

                    let mut parser = group.parser();

                    let table_name = parser.parse()?;
                    parser.parse::<Token![.]>()?;
                    let field_name = parser.parse()?;

                    if !parser.empty() {
                        return Err(Error::new_at("unexpected token", parser.span()));
                    }

                    references = Some((table_name, field_name));
                }
                "min" => {
                    if min.is_some() {
                        return Err(Error::new_at(
                            "cannot have multiple minimums on a single field",
                            span,
                        ));
                    }

                    match attribute.attr.input {
                        Some(AttrInput::Expression(_, expression)) => min = Some(expression),
                        _ => return Err(Error::new_at("`min` must have a value specified", span)),
                    }
                }
                "max" => {
                    if max.is_some() {
                        return Err(Error::new_at(
                            "cannot have multiple maximums on a single field",
                            span,
                        ));
                    }

                    match attribute.attr.input {
                        Some(AttrInput::Expression(_, expression)) => max = Some(expression),
                        _ => return Err(Error::new_at("`max` must have a value specified", span)),
                    }
                }
                _ => attributes.push(attribute),
            }
        }

        Ok(Field {
            attributes,
            name: field.name,
            r#type: field.r#type,
            primary_key,
            auto_increment,
            unique,
            references,
            min,
            max,
        })
    }
}
