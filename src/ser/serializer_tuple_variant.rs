use super::{AttributeValue, Error, Result, Serializer};
use serde::{ser, Serialize};
use std::collections::HashMap;

pub struct SerializerTupleVariant<T> {
    key: &'static str,
    vec: Vec<T>,
}

impl<T> SerializerTupleVariant<T> {
    pub fn new(key: &'static str, len: usize) -> Self {
        Self {
            key,
            vec: Vec::with_capacity(len),
        }
    }
}

impl<'a, T> ser::SerializeTupleVariant for SerializerTupleVariant<T>
where
    T: AttributeValue,
{
    type Ok = T;
    type Error = Error;

    fn serialize_field<F: ?Sized>(&mut self, value: &F) -> Result<(), Self::Error>
    where
        F: Serialize,
    {
        let serializer = Serializer::<T>::default();
        let value = value.serialize(serializer)?;
        self.vec.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut hashmap = HashMap::with_capacity(1);
        hashmap.insert(self.key.to_string(), T::construct_l(self.vec));

        Ok(T::construct_m(hashmap))
    }
}
