use crate::{Bind, FromColumnError, Statement};
use std::num::NonZero;

macro_rules! bind_integer {
    [$($type: ident -> $fn: ident),*] => {$(
        impl Bind for $type {
            fn bind<'statement, S: Statement<'statement>>(
                &'statement self,
                idx: usize,
                statement: &mut S,
            ) -> Result<(), S::Error> {
                statement.$fn(idx, *self)
            }

            fn validate<E: FromColumnError>(&self, min: Option<f64>, max: Option<f64>) -> Result<(), E> {
                if let Some(min) = min {
                    if *self < min as _ {
                        return Err(E::custom("value is too small"));
                    }
                }

                if let Some(max) = max {
                    if *self > max as _ {
                        return Err(E::custom("value is too large"))
                    }
                }

                Ok(())
            }
        }

        impl Bind for NonZero<$type> {
            fn bind<'statement, S: Statement<'statement>>(
                &'statement self,
                idx: usize,
                statement: &mut S,
            ) -> Result<(), S::Error> {
                statement.$fn(idx, self.get())
            }

            fn validate<E: FromColumnError>(&self, min: Option<f64>, max: Option<f64>) -> Result<(), E> {
                self.get().validate(min, max)
            }
        }
    )*};
}

bind_integer!(
    u8 -> bind_u8,
    u16 -> bind_u16,
    u32 -> bind_u32,
    u64 -> bind_u64,
    usize -> bind_usize,
    i8 -> bind_i8,
    i16 -> bind_i16,
    i32 -> bind_i32,
    i64 -> bind_i64,
    isize -> bind_isize
);

impl Bind for f32 {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        statement.bind_f32(idx, *self)
    }

    fn validate<E: FromColumnError>(&self, min: Option<f64>, max: Option<f64>) -> Result<(), E> {
        if let Some(min) = min {
            if *self < min as _ {
                return Err(E::custom("value is too small"));
            }
        }

        if let Some(max) = max {
            if *self > max as _ {
                return Err(E::custom("value is too large"));
            }
        }

        Ok(())
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

    fn validate<E: FromColumnError>(&self, min: Option<f64>, max: Option<f64>) -> Result<(), E> {
        if let Some(min) = min {
            if *self < min as _ {
                return Err(E::custom("value is too small"));
            }
        }

        if let Some(max) = max {
            if *self > max as _ {
                return Err(E::custom("value is too large"));
            }
        }

        Ok(())
    }
}
