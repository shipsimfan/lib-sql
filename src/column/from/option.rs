use crate::FromColumn;

impl<T: FromColumn> FromColumn for Option<T> {
    fn from_column<'column, C: crate::Column<'column>>(column: C) -> Result<Self, C::Error> {
        T::from_column(column).map(Some)
    }

    fn validate<E: crate::FromColumnError>(
        &self,
        min: Option<f64>,
        max: Option<f64>,
    ) -> Result<(), E> {
        self.as_ref()
            .map(|value| value.validate(min, max))
            .unwrap_or(Ok(()))
    }

    fn unwrap<'column, E: crate::FromRowError>(
        this: Option<Self>,
        _: &'static str,
    ) -> Result<Self, E> {
        Ok(this.unwrap_or(None))
    }
}
