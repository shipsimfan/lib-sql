use crate::{Bind, Nullable};

impl<T: Bind> Bind for Nullable<T> {
    fn bind<'statement, S: crate::Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        match self {
            Nullable::NonNull(value) => value.bind(idx, statement),
            Nullable::Null => statement.bind_null(idx),
        }
    }

    fn validate<E: crate::FromColumnError>(
        &self,
        min: Option<f64>,
        max: Option<f64>,
    ) -> Result<(), E> {
        match self {
            Nullable::NonNull(value) => value.validate(min, max),
            Nullable::Null => Ok(()),
        }
    }
}
