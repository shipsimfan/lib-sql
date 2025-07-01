use crate::Nullable;
use data_format::{deserialize::OptionConverter, Deserialize, DeserializeError};

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Nullable<T> {
    fn deserialize<D: data_format::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer
            .deserialize_option(OptionConverter::default())
            .map(Into::into)
    }

    fn unwrap<F: FnOnce() -> Self, E: DeserializeError<'de>>(
        val: Option<Self>,
        _: &'static str,
        _: Option<F>,
    ) -> Result<Self, E> {
        Ok(match val {
            Some(val) => val,
            None => Nullable::Null,
        })
    }
}
