use super::deserializer_bytes::DeserializerBytes;
use super::deserializer_number::DeserializerNumber;
use super::{AttributeValue, Deserializer, Error, Result};
use serde::de::{DeserializeSeed, IntoDeserializer, SeqAccess};

pub struct DeserializerSeq {
    iter: std::vec::IntoIter<AttributeValue>,
}

impl DeserializerSeq {
    pub fn from_vec(vec: Vec<AttributeValue>) -> Self {
        Self {
            iter: vec.into_iter(),
        }
    }
}

impl<'de> SeqAccess<'de> for DeserializerSeq {
    type Error = Error;

    fn next_element_seed<S>(&mut self, seed: S) -> Result<Option<S::Value>, Self::Error>
    where
        S: DeserializeSeed<'de>,
    {
        if let Some(value) = self.iter.next() {
            let de = Deserializer::from_attribute_value(value);
            seed.deserialize(de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.iter.len())
    }
}

pub struct DeserializerSeqStrings {
    iter: std::vec::IntoIter<String>,
}

impl DeserializerSeqStrings {
    pub fn from_vec(vec: Vec<String>) -> Self {
        Self {
            iter: vec.into_iter(),
        }
    }
}

impl<'de> SeqAccess<'de> for DeserializerSeqStrings {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(value) = self.iter.next() {
            let de = value.into_deserializer();
            seed.deserialize(de).map(Some)
        } else {
            Ok(None)
        }
    }
}

pub struct DeserializerSeqNumbers {
    iter: std::vec::IntoIter<String>,
}

impl DeserializerSeqNumbers {
    pub fn from_vec(vec: Vec<String>) -> Self {
        Self {
            iter: vec.into_iter(),
        }
    }
}

impl<'de> SeqAccess<'de> for DeserializerSeqNumbers {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(value) = self.iter.next() {
            let de = DeserializerNumber::from_string(value);
            seed.deserialize(de).map(Some)
        } else {
            Ok(None)
        }
    }
}

pub struct DeserializerSeqBytes<T> {
    iter: std::vec::IntoIter<T>,
}

impl<T> DeserializerSeqBytes<T> {
    pub fn from_vec(vec: Vec<T>) -> Self {
        Self {
            iter: vec.into_iter(),
        }
    }
}

impl<'de, B> SeqAccess<'de> for DeserializerSeqBytes<B>
where
    B: AsRef<[u8]>,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(value) = self.iter.next() {
            let de = DeserializerBytes::from_bytes(value);
            seed.deserialize(de).map(Some)
        } else {
            Ok(None)
        }
    }
}
