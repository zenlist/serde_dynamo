use super::{AttributeValue, Deserializer, Error, Item, Result};
use serde::de::{self, DeserializeSeed, MapAccess, Visitor};
use serde::forward_to_deserialize_any;

pub struct DeserializerMap<'a> {
    drain: std::collections::hash_map::Drain<'a, String, AttributeValue>,
    remaining_value: Option<AttributeValue>,
}

impl<'a> DeserializerMap<'a> {
    pub fn from_item(item: &'a mut Item) -> Self {
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
            return Ok(None);
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
}

struct DeserializerMapKey {
    input: String,
}

impl DeserializerMapKey {
    fn from_string(input: String) -> Self {
        Self { input }
    }
}

impl<'de, 'a> de::Deserializer<'de> for DeserializerMapKey {
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

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char enum bytes byte_buf option unit
        unit_struct newtype_struct seq tuple tuple_struct map struct ignored_any
    }
}
