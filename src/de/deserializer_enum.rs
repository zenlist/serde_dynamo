use super::{AttributeValue, Deserializer, Error, ErrorImpl, Item, Result};
use serde::de::{
    DeserializeSeed, Deserializer as _, EnumAccess, IntoDeserializer, VariantAccess, Visitor,
};

pub struct DeserializerEnum {
    input: Item,
}

impl DeserializerEnum {
    pub fn from_item(input: Item) -> Self {
        Self { input }
    }
}

impl<'de, 'a> EnumAccess<'de> for DeserializerEnum {
    type Variant = DeserializerVariant;
    type Error = Error;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let mut drain = self.input.drain();
        let (key, value) = drain.next().ok_or(ErrorImpl::ExpectedSingleKey.into())?;
        if !drain.next().is_none() {
            return Err(ErrorImpl::ExpectedSingleKey.into());
        }
        let deserializer = DeserializerVariant::from_attribute_value(value);
        let value = seed.deserialize(key.into_deserializer())?;
        Ok((value, deserializer))
    }
}

pub struct DeserializerVariant {
    input: AttributeValue,
}

impl DeserializerVariant {
    pub fn from_attribute_value(input: AttributeValue) -> Self {
        Self { input }
    }
}

impl<'de, 'a> VariantAccess<'de> for DeserializerVariant {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        // If the `Visitor` expected this variant to be a unit variant, the input should have been
        // the plain string case handled in `deserialize_enum`.
        unreachable!()
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
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
