use super::{
    deserializer_bytes::DeserializerBytes,
    deserializer_enum::DeserializerEnum,
    deserializer_map::DeserializerMap,
    deserializer_number::DeserializerNumber,
    deserializer_seq::{
        DeserializerSeq, DeserializerSeqBytes, DeserializerSeqNumbers, DeserializerSeqStrings,
    },
    AttributeValue, Error, ErrorImpl, ErrorPath, Result,
};
use serde_core::de::{self, IntoDeserializer, Visitor};

/// A structure that deserializes [`AttributeValue`]s into Rust values.
#[derive(Debug)]
pub struct Deserializer<'a> {
    input: AttributeValue,
    path: ErrorPath<'a>,
}

impl<'a> Deserializer<'a> {
    /// Create a Deserializer from an AttributeValue
    pub fn from_attribute_value(input: AttributeValue) -> Self {
        Self::from_attribute_value_path(input, ErrorPath::Root)
    }

    pub(crate) fn from_attribute_value_path(input: AttributeValue, path: ErrorPath<'a>) -> Self {
        Deserializer { input, path }
    }

    /// Helper that creates an error with context
    fn error(self, kind: ErrorImpl) -> Error {
        Error::from_path(kind, &self.path, self.input)
    }
}

macro_rules! deserialize_number {
    ($self:expr, $visitor:expr, $ty:ty, $fn:ident) => {
        if let AttributeValue::N(n) = $self.input {
            let de = DeserializerNumber::from_string(n, $self.path);
            de.$fn($visitor)
        } else {
            return Err($self.error(ErrorImpl::ExpectedNum));
        }
    };
}

impl<'de, 'a> de::Deserializer<'de> for Deserializer<'a> {
    type Error = Error;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let AttributeValue::N(s) = self.input {
            DeserializerNumber::from_string(s, self.path).deserialize_any(visitor)
        } else {
            match self.input {
                AttributeValue::S(_) => self.deserialize_string(visitor),
                AttributeValue::Bool(_) => self.deserialize_bool(visitor),
                AttributeValue::B(_) => self.deserialize_bytes(visitor),
                AttributeValue::Null(_) => self.deserialize_unit(visitor),
                AttributeValue::M(_) => self.deserialize_map(visitor),
                AttributeValue::L(_)
                | AttributeValue::Ss(_)
                | AttributeValue::Ns(_)
                | AttributeValue::Bs(_) => self.deserialize_seq(visitor),
                _ => unreachable!(),
            }
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_number!(self, visitor, i8, deserialize_i8)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_number!(self, visitor, u8, deserialize_u8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_number!(self, visitor, i16, deserialize_i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_number!(self, visitor, i32, deserialize_i32)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_number!(self, visitor, i64, deserialize_i64)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_number!(self, visitor, u16, deserialize_u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_number!(self, visitor, u32, deserialize_u32)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_number!(self, visitor, u64, deserialize_u64)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_number!(self, visitor, f32, deserialize_f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_number!(self, visitor, f64, deserialize_f64)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let AttributeValue::S(s) = self.input {
            visitor.visit_string(s)
        } else {
            Err(self.error(ErrorImpl::ExpectedString))
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let AttributeValue::S(s) = self.input {
            visitor.visit_string(s)
        } else {
            Err(self.error(ErrorImpl::ExpectedString))
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            AttributeValue::L(l) => {
                let deserializer_seq = DeserializerSeq::from_vec(l, self.path);
                visitor.visit_seq(deserializer_seq)
            }
            AttributeValue::Ss(ss) => {
                let deserializer_seq = DeserializerSeqStrings::from_vec(ss);
                visitor.visit_seq(deserializer_seq)
            }
            AttributeValue::Ns(ns) => {
                let deserializer_seq = DeserializerSeqNumbers::from_vec(ns, self.path);
                visitor.visit_seq(deserializer_seq)
            }
            AttributeValue::Bs(bs) => {
                let deserializer_seq = DeserializerSeqBytes::from_vec(bs);
                visitor.visit_seq(deserializer_seq)
            }
            _ => Err(self.error(ErrorImpl::ExpectedSeq)),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let AttributeValue::M(mut m) = self.input {
            let deserializer_map = DeserializerMap::from_item(&mut m, self.path);
            visitor.visit_map(deserializer_map)
        } else {
            Err(self.error(ErrorImpl::ExpectedMap))
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let AttributeValue::Bool(b) = self.input {
            visitor.visit_bool(b)
        } else {
            Err(self.error(ErrorImpl::ExpectedBool))
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let AttributeValue::S(s) = &self.input {
            let mut chars = s.chars();
            if let Some(ch) = chars.next() {
                let result = visitor.visit_char(ch)?;
                if chars.next().is_none() {
                    return Ok(result);
                }
            }
        }
        Err(self.error(ErrorImpl::ExpectedChar))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let AttributeValue::Null(true) = self.input {
            visitor.visit_unit()
        } else {
            Err(self.error(ErrorImpl::ExpectedUnit))
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            AttributeValue::S(s) => visitor.visit_enum(s.into_deserializer()),
            AttributeValue::M(m) => visitor.visit_enum(DeserializerEnum::from_item(m, &self.path)),
            _ => Err(self.error(ErrorImpl::ExpectedEnum)),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let AttributeValue::B(b) = self.input {
            let de = DeserializerBytes::from_bytes(b);
            de.deserialize_bytes(visitor)
        } else {
            Err(self.error(ErrorImpl::ExpectedBytes))
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let AttributeValue::Null(true) = self.input {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let AttributeValue::L(_) = self.input {
            self.deserialize_seq(visitor)
        } else {
            self.deserialize_map(visitor)
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let AttributeValue::S(s) = self.input {
            visitor.visit_string(s)
        } else {
            Err(self.error(ErrorImpl::ExpectedString))
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let AttributeValue::Null(true) = self.input {
            visitor.visit_unit()
        } else {
            Err(self.error(ErrorImpl::ExpectedUnitStruct))
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }
}
