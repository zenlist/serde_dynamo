use super::{AttributeValue, Error, Item, Result, Serializer};
use serde::{ser, Serialize};

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

impl<'a> ser::SerializeTupleVariant for SerializerTupleVariant {
    type Ok = AttributeValue;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let serializer = Serializer;
        let value = value.serialize(serializer)?;
        self.vec.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut hashmap = Item::with_capacity(1);
        hashmap.insert(
            self.key.to_string(),
            AttributeValue {
                l: Some(self.vec),
                ..AttributeValue::default()
            },
        );

        Ok(AttributeValue {
            m: Some(hashmap),
            ..AttributeValue::default()
        })
    }
}
