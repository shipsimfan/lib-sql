use crate::Bind;
use std::borrow::Cow;

impl<'a, T: Bind + ToOwned> Bind for Cow<'a, T> {
    fn bind<'statement, S: crate::Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        self.as_ref().bind(idx, statement)
    }

    fn validate<E: crate::FromColumnError>(
        &self,
        min: Option<f64>,
        max: Option<f64>,
    ) -> Result<(), E> {
        self.as_ref().validate(min, max)
    }
}
