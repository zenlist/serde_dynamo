use super::{AttributeValue, Deserializer, Error, ErrorImpl, ErrorPath, Result};
use serde_core::de::{
    DeserializeSeed, Deserializer as _, EnumAccess, IntoDeserializer, VariantAccess, Visitor,
};
use std::collections::HashMap;

pub struct DeserializerEnum<'a> {
    input: HashMap<String, AttributeValue>,
    path: ErrorPath<'a>,
}

impl<'a> DeserializerEnum<'a> {
    pub fn from_item(input: HashMap<String, AttributeValue>, path: ErrorPath<'a>) -> Self {
        Self { input, path }
    }
}

impl<'de, 'a> EnumAccess<'de> for DeserializerEnum<'a> {
    type Variant = DeserializerVariant<'a>;
    type Error = Error;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let mut drain = self.input.drain();
        let (key, value) = drain
            .next()
            .ok_or_else(|| Error::from_path(ErrorImpl::ExpectedSingleKey, &self.path))?;
        if drain.next().is_some() {
            return Err(Error::from_path(ErrorImpl::ExpectedSingleKey, &self.path));
        }
        let deserializer = DeserializerVariant::from_attribute_value(
            value,
            ErrorPath::Enum(key.clone(), Box::new(self.path)),
        );
        let value = seed.deserialize(key.into_deserializer())?;

        Ok((value, deserializer))
    }
}

pub struct DeserializerVariant<'a> {
    input: AttributeValue,
    path: ErrorPath<'a>,
}

impl<'a> DeserializerVariant<'a> {
    pub fn from_attribute_value(input: AttributeValue, path: ErrorPath<'a>) -> Self {
        Self { input, path }
    }
}

impl<'de, 'a> VariantAccess<'de> for DeserializerVariant<'a> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<S>(self, seed: S) -> Result<S::Value>
    where
        S: DeserializeSeed<'de>,
    {
        let deserializer = Deserializer::from_attribute_value_path(self.input, self.path);
        seed.deserialize(deserializer)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let deserializer = Deserializer::from_attribute_value_path(self.input, self.path);
        deserializer.deserialize_seq(visitor)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let deserializer = Deserializer::from_attribute_value_path(self.input, self.path);
        deserializer.deserialize_map(visitor)
    }
}
