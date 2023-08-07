use super::{AttributeValue, Deserializer, Error, ErrorImpl, Result};
use serde::{
    de::{self, DeserializeSeed, MapAccess, Visitor},
    forward_to_deserialize_any, serde_if_integer128,
};
use std::collections::HashMap;

pub struct DeserializerMap<'a> {
    drain: std::collections::hash_map::Drain<'a, String, AttributeValue>,
    remaining_value: Option<AttributeValue>,
}

impl<'a> DeserializerMap<'a> {
    pub fn from_item(item: &'a mut HashMap<String, AttributeValue>) -> Self {
        Self {
            drain: item.drain(),
            remaining_value: None,
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
            self.remaining_value = Some(value);
            let de = DeserializerMapKey::from_string(key);
            seed.deserialize(de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some(value) = self.remaining_value.take() {
            let de = Deserializer::from_attribute_value(value);
            seed.deserialize(de)
        } else {
            unreachable!("Value without a corresponding key")
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.drain.len())
    }
}

struct DeserializerMapKey {
    input: String,
}

impl DeserializerMapKey {
    fn from_string(input: String) -> Self {
        Self { input }
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
                .map_err(|_| ErrorImpl::ExpectedNum.into())?;

            visitor.$visit(number)
        }
    };
}

impl<'de> de::Deserializer<'de> for DeserializerMapKey {
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
        visitor.visit_string(self.input)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.input)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.input)
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
        let de = Deserializer::from_attribute_value(AttributeValue::S(self.input));
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
    serde_if_integer128! {
        deserialize_integer_key!(deserialize_i128 => visit_i128);
    }
    deserialize_integer_key!(deserialize_u8   => visit_u8);
    deserialize_integer_key!(deserialize_u16  => visit_u16);
    deserialize_integer_key!(deserialize_u32  => visit_u32);
    deserialize_integer_key!(deserialize_u64  => visit_u64);
    serde_if_integer128! {
        deserialize_integer_key!(deserialize_u128 => visit_u128);
    }

    forward_to_deserialize_any! {
        bool f32 f64 char bytes byte_buf option unit
        unit_struct seq tuple tuple_struct map struct ignored_any
    }
}
