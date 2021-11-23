use super::{AttributeValue, Error, Result, Serializer};
use serde::{ser, Serialize};
use std::collections::HashMap;

pub struct SerializerStructVariant<T> {
    key: &'static str,
    item: HashMap<String, T>,
}

impl<T> SerializerStructVariant<T> {
    pub fn new(key: &'static str, len: usize) -> Self {
        Self {
            key,
            item: HashMap::with_capacity(len),
        }
    }
}

impl<'a, T> ser::SerializeStructVariant for SerializerStructVariant<T>
where
    T: AttributeValue,
{
    type Ok = T;
    type Error = Error;

    fn serialize_field<F: ?Sized>(
        &mut self,
        key: &'static str,
        value: &F,
    ) -> Result<(), Self::Error>
    where
        F: Serialize,
    {
        let serializer = Serializer::<T>::default();
        let value = value.serialize(serializer)?;
        self.item.insert(key.to_string(), value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut hashmap = HashMap::with_capacity(1);
        hashmap.insert(self.key.to_string(), T::construct_m(self.item));

        Ok(T::construct_m(hashmap))
    }
}
