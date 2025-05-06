use crate::Nullable;

impl<T> From<T> for Nullable<T> {
    fn from(value: T) -> Self {
        Nullable::NonNull(value)
    }
}

impl<T> From<Option<T>> for Nullable<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Nullable::NonNull(value),
            None => Nullable::Null,
        }
    }
}
