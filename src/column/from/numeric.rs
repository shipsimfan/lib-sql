use crate::{Column, FromColumn, FromColumnError};
use std::num::NonZero;

macro_rules! from_number {
    [$($type: ident -> $fn: ident),*] => {$(
        impl FromColumn for $type {
            fn from_column<'column, C: Column<'column>>(column: C) -> Result<$type, C::Error> {
                column.$fn()
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

        impl FromColumn for NonZero<$type> {
            fn from_column<'column, C: Column<'column>>(column: C) -> Result<NonZero<$type>, C::Error> {
                NonZero::new(column.$fn()?).ok_or(C::Error::custom("value cannot be zero"))
            }

            fn validate<E: FromColumnError>(&self, min: Option<f64>, max: Option<f64>) -> Result<(), E> {
                self.get().validate(min, max)
            }
        }
    )*};
}

from_number!(
    u8 -> into_u8,
    u16 -> into_u16,
    u32 -> into_u32,
    u64 -> into_u64,
    usize -> into_usize,
    i8 -> into_i8,
    i16 -> into_i16,
    i32 -> into_i32,
    i64 -> into_i64,
    isize -> into_isize
);

impl FromColumn for f32 {
    fn from_column<'column, C: Column<'column>>(column: C) -> Result<f32, C::Error> {
        column.into_f32()
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

impl FromColumn for f64 {
    fn from_column<'column, C: Column<'column>>(column: C) -> Result<f64, C::Error> {
        column.into_f64()
    }

    fn validate<E: FromColumnError>(&self, min: Option<f64>, max: Option<f64>) -> Result<(), E> {
        if let Some(min) = min {
            if *self < min {
                return Err(E::custom("value is too small"));
            }
        }

        if let Some(max) = max {
            if *self > max {
                return Err(E::custom("value is too large"));
            }
        }

        Ok(())
    }
}
