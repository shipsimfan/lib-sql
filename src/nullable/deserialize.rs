use crate::Nullable;
use data_format::{Deserialize, DeserializeError};

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Nullable<T> {
    fn deserialize<D: data_format::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize(deserializer).map(|value| Nullable::NonNull(value))
    }

    fn unwrap<E: DeserializeError<'de>>(val: Option<Self>, _: &'static str) -> Result<Self, E> {
        Ok(match val {
            Some(val) => val,
            None => Nullable::Null,
        })
    }
}
