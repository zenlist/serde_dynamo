use super::{AttributeValue, Error, Result, Serializer};
use serde::{ser, Serialize};
use std::collections::HashMap;

pub struct SerializerTupleVariant {
    key: &'static str,
    vec: Vec<AttributeValue>,
}

impl SerializerTupleVariant {
    pub fn new(key: &'static str, len: usize) -> Self {
        Self {
            key,
            vec: Vec::with_capacity(len),
        }
    }
}

impl ser::SerializeTupleVariant for SerializerTupleVariant {
    type Ok = AttributeValue;
    type Error = Error;

    fn serialize_field<F: ?Sized>(&mut self, value: &F) -> Result<(), Self::Error>
    where
        F: Serialize,
    {
        let serializer = Serializer;
        let value = value.serialize(serializer)?;
        self.vec.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut hashmap = HashMap::with_capacity(1);
        hashmap.insert(self.key.to_string(), AttributeValue::L(self.vec));

        Ok(AttributeValue::M(hashmap))
    }
}
