use super::{
    deserializer_bytes::DeserializerBytes,
    deserializer_enum::DeserializerEnum,
    deserializer_map::DeserializerMap,
    deserializer_number::DeserializerNumber,
    deserializer_seq::{
        DeserializerSeq, DeserializerSeqBytes, DeserializerSeqNumbers, DeserializerSeqStrings,
    },
    AttributeValue, Error, ErrorImpl, Result,
};
use serde::de::{self, IntoDeserializer, Visitor};

/// A structure that deserializes AttributeValues into Rust values.
#[derive(Debug)]
pub struct Deserializer<T> {
    input: T,
}

impl<T> Deserializer<T> {
    /// Create a Deserializer from an AttributeValue
    pub fn from_attribute_value(input: T) -> Self {
        Deserializer { input }
    }
}

macro_rules! deserialize_number {
    ($self:expr, $visitor:expr, $ty:ty, $fn:ident) => {
        if let Some(n) = $self.input.into_n() {
            let de = DeserializerNumber::from_string(n);
            de.$fn($visitor)
        } else {
            return Err(ErrorImpl::ExpectedNum.into());
        }
    };
}

impl<'de, 'a, T> de::Deserializer<'de> for Deserializer<T>
where
    T: AttributeValue,
{
    type Error = Error;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.input.is_n() {
            DeserializerNumber::from_string(self.input.into_n().unwrap()).deserialize_any(visitor)
        } else if self.input.is_s() {
            self.deserialize_string(visitor)
        } else if self.input.is_bool() {
            self.deserialize_bool(visitor)
        } else if self.input.is_b() {
            self.deserialize_bytes(visitor)
        } else if self.input.is_null() {
            self.deserialize_unit(visitor)
        } else if self.input.is_m() {
            self.deserialize_map(visitor)
        } else if self.input.is_l()
            || self.input.is_ss()
            || self.input.is_ns()
            || self.input.is_bs()
        {
            self.deserialize_seq(visitor)
        } else {
            unreachable!()
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
        if let Some(s) = self.input.into_s() {
            visitor.visit_string(s)
        } else {
            Err(ErrorImpl::ExpectedString.into())
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(s) = self.input.into_s() {
            visitor.visit_string(s)
        } else {
            Err(ErrorImpl::ExpectedString.into())
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.input.is_l() {
            let l = self.input.into_l().unwrap();
            let deserializer_seq = DeserializerSeq::from_vec(l);
            visitor.visit_seq(deserializer_seq)
        } else if self.input.is_ss() {
            let ss = self.input.into_ss().unwrap();
            let deserializer_seq = DeserializerSeqStrings::from_vec(ss);
            visitor.visit_seq(deserializer_seq)
        } else if self.input.is_ns() {
            let ns = self.input.into_ns().unwrap();
            let deserializer_seq = DeserializerSeqNumbers::from_vec(ns);
            visitor.visit_seq(deserializer_seq)
        } else if self.input.is_bs() {
            let bs = self.input.into_bs().unwrap();
            let deserializer_seq = DeserializerSeqBytes::from_vec(bs);
            visitor.visit_seq(deserializer_seq)
        } else {
            Err(ErrorImpl::ExpectedSeq.into())
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(mut m) = self.input.into_m() {
            let deserializer_map = DeserializerMap::from_item(&mut m);
            visitor.visit_map(deserializer_map)
        } else {
            Err(ErrorImpl::ExpectedMap.into())
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(b) = self.input.into_bool() {
            visitor.visit_bool(b)
        } else {
            Err(ErrorImpl::ExpectedBool.into())
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(s) = self.input.into_s() {
            let mut chars = s.chars();
            if let Some(ch) = chars.next() {
                let result = visitor.visit_char(ch)?;
                if chars.next().is_some() {
                    Err(ErrorImpl::ExpectedChar.into())
                } else {
                    Ok(result)
                }
            } else {
                Err(ErrorImpl::ExpectedChar.into())
            }
        } else {
            Err(ErrorImpl::ExpectedChar.into())
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(true) = self.input.into_null() {
            visitor.visit_unit()
        } else {
            Err(ErrorImpl::ExpectedUnit.into())
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
        if self.input.is_s() {
            let s = self.input.into_s().unwrap();
            visitor.visit_enum(s.into_deserializer())
        } else if self.input.is_m() {
            let m = self.input.into_m().unwrap();
            visitor.visit_enum(DeserializerEnum::from_item(m))
        } else {
            Err(ErrorImpl::ExpectedEnum.into())
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(b) = self.input.into_b() {
            let de = DeserializerBytes::from_bytes(b);
            de.deserialize_bytes(visitor)
        } else {
            Err(ErrorImpl::ExpectedBytes.into())
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
        if let Some(true) = self.input.as_null() {
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
        self.deserialize_map(visitor)
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
        if let Some(s) = self.input.into_s() {
            visitor.visit_string(s)
        } else {
            Err(ErrorImpl::ExpectedString.into())
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
        if let Some(true) = self.input.into_null() {
            visitor.visit_unit()
        } else {
            Err(ErrorImpl::ExpectedUnitStruct.into())
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
