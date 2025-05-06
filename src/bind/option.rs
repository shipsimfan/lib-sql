use crate::Bind;

impl<T: Bind> Bind for Option<T> {
    fn bind<'statement, S: crate::Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        match self {
            Some(value) => value.bind(idx, statement),
            None => statement.bind_null(idx),
        }
    }

    fn validate<E: crate::FromColumnError>(
        &self,
        min: Option<f64>,
        max: Option<f64>,
    ) -> Result<(), E> {
        match self {
            Some(value) => value.validate(min, max),
            None => Ok(()),
        }
    }
}
