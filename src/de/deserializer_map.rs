use super::{AttributeValue, Deserializer, Error, ErrorImpl, ErrorPath, Result};
use serde_core::{
    de::{self, DeserializeSeed, MapAccess, Visitor},
    forward_to_deserialize_any,
};
use std::collections::HashMap;

pub struct DeserializerMap<'a> {
    drain: std::collections::hash_map::Drain<'a, String, AttributeValue>,
    remaining_value: Option<(String, AttributeValue)>,
    path: ErrorPath<'a>,
}

impl<'a> DeserializerMap<'a> {
    pub fn from_item(item: &'a mut HashMap<String, AttributeValue>, path: ErrorPath<'a>) -> Self {
        Self {
            drain: item.drain(),
            remaining_value: None,
            path,
        }
    }
}

impl<'de, 'a> MapAccess<'de> for DeserializerMap<'a> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some((key, value)) = self.drain.next() {
            let de = DeserializerMapKey::from_string(&key, ErrorPath::Field(&key, &self.path));
            let a = seed.deserialize(de).map(Some);
            self.remaining_value = Some((key, value));
            a
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some((key, value)) = self.remaining_value.take() {
            let de =
                Deserializer::from_attribute_value_path(value, ErrorPath::Field(&key, &self.path));
            seed.deserialize(de)
        } else {
            unreachable!("Value without a corresponding key")
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.drain.len())
    }
}

struct DeserializerMapKey<'a> {
    input: &'a str,
    path: ErrorPath<'a>,
}

impl<'a> DeserializerMapKey<'a> {
    fn from_string(input: &'a str, path: ErrorPath<'a>) -> Self {
        Self { input, path }
    }
}

macro_rules! deserialize_integer_key {
    ($method:ident => $visit:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            let number = self
                .input
                .parse()
                .map_err(|_| Error::from_path(ErrorImpl::ExpectedNum, &self.path, AttributeValue::N(self.input.to_owned())))?;

            visitor.$visit(number)
        }
    };
}

impl<'de, 'a> de::Deserializer<'de> for DeserializerMapKey<'a> {
    type Error = Error;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.input)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.input)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.input)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let de = Deserializer::from_attribute_value_path(
            AttributeValue::S(self.input.to_owned()),
            self.path,
        );
        de.deserialize_enum(name, variants, visitor)
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

    deserialize_integer_key!(deserialize_i8   => visit_i8);
    deserialize_integer_key!(deserialize_i16  => visit_i16);
    deserialize_integer_key!(deserialize_i32  => visit_i32);
    deserialize_integer_key!(deserialize_i64  => visit_i64);
    deserialize_integer_key!(deserialize_i128 => visit_i128);
    deserialize_integer_key!(deserialize_u8   => visit_u8);
    deserialize_integer_key!(deserialize_u16  => visit_u16);
    deserialize_integer_key!(deserialize_u32  => visit_u32);
    deserialize_integer_key!(deserialize_u64  => visit_u64);
    deserialize_integer_key!(deserialize_u128 => visit_u128);

    fn deserialize_bool<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            "true" => visitor.visit_bool(true),
            "false" => visitor.visit_bool(false),
            _ => Err(Error::from_path(
                ErrorImpl::ExpectedString,
                &self.path,
                AttributeValue::S(self.input.to_owned()),
            )),
        }
    }

    forward_to_deserialize_any! {
        f32 f64 char bytes byte_buf option unit
        unit_struct seq tuple tuple_struct map struct ignored_any
    }
}
