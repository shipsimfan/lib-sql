use crate::Nullable;

impl<T> Into<Option<T>> for Nullable<T> {
    fn into(self) -> Option<T> {
        match self {
            Nullable::NonNull(value) => Some(value),
            Nullable::Null => None,
        }
    }
}
