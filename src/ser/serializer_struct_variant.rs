use super::{AttributeValue, Error, Item, Result, Serializer};
use serde::{ser, Serialize};

pub struct SerializerStructVariant {
    key: &'static str,
    item: Item,
}

impl SerializerStructVariant {
    pub fn new(key: &'static str, len: usize) -> Self {
        Self {
            key,
            item: Item::with_capacity(len),
        }
    }
}

impl<'a> ser::SerializeStructVariant for SerializerStructVariant {
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
        let mut hashmap = Item::with_capacity(1);
        hashmap.insert(
            self.key.to_string(),
            AttributeValue {
                m: Some(self.item),
                ..AttributeValue::default()
            },
        );

        Ok(AttributeValue {
            m: Some(hashmap),
            ..AttributeValue::default()
        })
    }
}
