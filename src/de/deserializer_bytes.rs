use super::{Error, Result};
use serde::de::{self, Visitor};
use serde::forward_to_deserialize_any;

pub struct DeserializerBytes<T> {
    input: T,
}

impl<T> DeserializerBytes<T> {
    pub fn from_bytes(input: T) -> Self {
        DeserializerBytes { input }
    }
}

impl<'de, T> de::Deserializer<'de> for DeserializerBytes<T>
where
    T: AsRef<[u8]>,
{
    type Error = Error;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bytes(self.input.as_ref())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 str string seq map bool char unit enum tuple option struct identifier
        unit_struct tuple_struct newtype_struct
    }
}
