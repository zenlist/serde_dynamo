use super::{AttributeValue, Error, Result, Serializer};
use serde::{ser, Serialize};
use std::collections::HashMap;

pub struct SerializerStruct<T> {
    item: HashMap<String, T>,
}

impl<T> SerializerStruct<T> {
    pub fn new(len: usize) -> Self {
        SerializerStruct {
            item: HashMap::with_capacity(len),
        }
    }
}

impl<'a, T> ser::SerializeStruct for SerializerStruct<T>
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
        Ok(T::construct_m(self.item))
    }
}
