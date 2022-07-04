use super::{Error, ErrorImpl, Result};
use serde::de::{self, Visitor};
use serde::forward_to_deserialize_any;

pub struct DeserializerNumber {
    input: String,
}

impl DeserializerNumber {
    pub fn from_string(input: String) -> Self {
        DeserializerNumber { input }
    }

    fn deserialize_number<'de, V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let i = self.input.parse::<i64>();
        let u = self.input.parse::<u64>();
        let f = self.input.parse::<f64>();
        match (i, u, f) {
            (Ok(i), _, _) => visitor.visit_i64(i),
            (_, Ok(u), _) => visitor.visit_u64(u),
            (_, _, Ok(f)) => visitor.visit_f64(f),
            (Err(_), Err(_), Err(e)) => Err(ErrorImpl::FailedToParseFloat(self.input, e).into()),
        }
    }
}

macro_rules! deserialize_int {
    ($self:expr, $visitor:expr, $ty:ty, $fn:ident) => {{
        let n = $self
            .input
            .parse::<$ty>()
            .map_err(|e| ErrorImpl::FailedToParseInt($self.input, e).into())?;
        $visitor.$fn(n)
    }};
}

macro_rules! deserialize_float {
    ($self:expr, $visitor:expr, $ty:ty, $fn:ident) => {{
        let n = $self
            .input
            .parse::<$ty>()
            .map_err(|e| ErrorImpl::FailedToParseFloat($self.input, e).into())?;
        $visitor.$fn(n)
    }};
}

impl<'de> de::Deserializer<'de> for DeserializerNumber {
    type Error = Error;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_number(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_int!(self, visitor, i8, visit_i8)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_int!(self, visitor, u8, visit_u8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_int!(self, visitor, i16, visit_i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_int!(self, visitor, i32, visit_i32)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_int!(self, visitor, i64, visit_i64)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_int!(self, visitor, u16, visit_u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_int!(self, visitor, u32, visit_u32)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_int!(self, visitor, u64, visit_u64)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_float!(self, visitor, f32, visit_f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_float!(self, visitor, f64, visit_f64)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    forward_to_deserialize_any! {
        str string seq map bool char unit enum bytes tuple option struct byte_buf identifier
        unit_struct tuple_struct newtype_struct
    }
}
