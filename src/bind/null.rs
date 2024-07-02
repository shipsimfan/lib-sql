use crate::{Bind, Statement};

impl Bind for () {
    fn bind<'a, S: Statement<'a>>(
        &'a self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::BindError> {
        statement.bind_null(idx)
    }
}
