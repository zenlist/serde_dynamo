use super::{AttributeValue, Error, Item, Result, Serializer};
use serde::{ser, Serialize};

pub struct SerializerStruct {
    item: Item,
}

impl SerializerStruct {
    pub fn new(len: usize) -> Self {
        SerializerStruct {
            item: Item::with_capacity(len),
        }
    }
}

impl<'a> ser::SerializeStruct for SerializerStruct {
    type Ok = AttributeValue;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let serializer = Serializer;
        let value = value.serialize(serializer)?;
        self.item.insert(key.to_string(), value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue {
            m: Some(self.item),
            ..AttributeValue::default()
        })
    }
}
