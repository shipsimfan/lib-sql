use crate::Nullable;
use data_format::Serialize;

impl<T: Serialize> Serialize for Nullable<T> {
    fn serialize<S: data_format::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Nullable::NonNull(value) => value.serialize(serializer),
            Nullable::Null => serializer.serialize_unit(),
        }
    }
}
