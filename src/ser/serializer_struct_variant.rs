use super::{AttributeValue, Error, Result, Serializer};
use serde::{ser, Serialize};
use std::collections::HashMap;

pub struct SerializerStructVariant {
    key: &'static str,
    item: HashMap<String, AttributeValue>,
}

impl SerializerStructVariant {
    pub fn new(key: &'static str, len: usize) -> Self {
        Self {
            key,
            item: HashMap::with_capacity(len),
        }
    }
}

impl ser::SerializeStructVariant for SerializerStructVariant {
    type Ok = AttributeValue;
    type Error = Error;

    fn serialize_field<F: ?Sized>(
        &mut self,
        key: &'static str,
        value: &F,
    ) -> Result<(), Self::Error>
    where
        F: Serialize,
    {
        let serializer = Serializer;
        let value = value.serialize(serializer)?;
        self.item.insert(key.to_string(), value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut hashmap = HashMap::with_capacity(1);
        hashmap.insert(self.key.to_string(), AttributeValue::M(self.item));

        Ok(AttributeValue::M(hashmap))
    }
}
