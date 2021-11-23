use super::{AttributeValue, Deserializer, Error, ErrorImpl, Result};
use serde::de::{
    DeserializeSeed, Deserializer as _, EnumAccess, IntoDeserializer, VariantAccess, Visitor,
};
use std::collections::HashMap;

pub struct DeserializerEnum<T> {
    input: HashMap<String, T>,
}

impl<T> DeserializerEnum<T> {
    pub fn from_item(input: HashMap<String, T>) -> Self {
        Self { input }
    }
}

impl<'de, 'a, T> EnumAccess<'de> for DeserializerEnum<T>
where
    T: AttributeValue,
{
    type Variant = DeserializerVariant<T>;
    type Error = Error;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let mut drain = self.input.drain();
        let (key, value) = drain
            .next()
            .ok_or_else(|| ErrorImpl::ExpectedSingleKey.into())?;
        if drain.next().is_some() {
            return Err(ErrorImpl::ExpectedSingleKey.into());
        }
        let deserializer = DeserializerVariant::from_attribute_value(value);
        let value = seed.deserialize(key.into_deserializer())?;
        Ok((value, deserializer))
    }
}

pub struct DeserializerVariant<T> {
    input: T,
}

impl<T> DeserializerVariant<T> {
    pub fn from_attribute_value(input: T) -> Self {
        Self { input }
    }
}

impl<'de, 'a, T> VariantAccess<'de> for DeserializerVariant<T>
where
    T: AttributeValue,
{
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<S>(self, seed: S) -> Result<S::Value>
    where
        S: DeserializeSeed<'de>,
    {
        let deserializer = Deserializer::from_attribute_value(self.input);
        seed.deserialize(deserializer)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let deserializer = Deserializer::from_attribute_value(self.input);
        deserializer.deserialize_seq(visitor)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let deserializer = Deserializer::from_attribute_value(self.input);
        deserializer.deserialize_map(visitor)
    }
}
