use crate::{Bind, Statement};
use std::marker::PhantomData;

impl Bind for () {
    fn bind<'a, S: Statement<'a>>(
        &'a self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::BindError> {
        statement.bind_null(idx)
    }
}

impl<T> Bind for PhantomData<T> {
    fn bind<'a, S: Statement<'a>>(
        &'a self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::BindError> {
        statement.bind_null(idx)
    }
}
