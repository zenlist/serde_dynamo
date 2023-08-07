use super::{AttributeValue, Error, Result, Serializer};
use serde::{ser, Serialize};
use std::collections::HashMap;

pub struct SerializerStruct {
    item: HashMap<String, AttributeValue>,
}

impl SerializerStruct {
    pub fn new(len: usize) -> Self {
        SerializerStruct {
            item: HashMap::with_capacity(len),
        }
    }
}

impl ser::SerializeStruct for SerializerStruct {
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
        Ok(AttributeValue::M(self.item))
    }
}
