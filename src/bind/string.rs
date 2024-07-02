use crate::{Bind, Statement};

impl Bind for str {
    fn bind<'a, S: Statement<'a>>(
        &'a self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::BindError> {
        statement.bind_str(idx, self)
    }
}

impl Bind for String {
    fn bind<'a, S: Statement<'a>>(
        &'a self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::BindError> {
        statement.bind_str(idx, self)
    }
}
