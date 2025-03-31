use crate::{Bind, Statement};

impl Bind for str {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::BindError> {
        statement.bind_str(idx, self)
    }
}

impl Bind for String {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::BindError> {
        statement.bind_str(idx, self)
    }
}
