use crate::{Bind, Statement};

impl Bind for u8 {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        statement.bind_u8(idx, *self)
    }
}

impl Bind for u16 {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        statement.bind_u16(idx, *self)
    }
}

impl Bind for u32 {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        statement.bind_u32(idx, *self)
    }
}

impl Bind for u64 {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        statement.bind_u64(idx, *self)
    }
}

impl Bind for usize {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        statement.bind_usize(idx, *self)
    }
}

impl Bind for i8 {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        statement.bind_i8(idx, *self)
    }
}

impl Bind for i16 {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        statement.bind_i16(idx, *self)
    }
}

impl Bind for i32 {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        statement.bind_i32(idx, *self)
    }
}

impl Bind for i64 {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        statement.bind_i64(idx, *self)
    }
}

impl Bind for isize {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        statement.bind_isize(idx, *self)
    }
}

impl Bind for f32 {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        statement.bind_f32(idx, *self)
    }
}

impl Bind for f64 {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        statement.bind_f64(idx, *self)
    }
}
