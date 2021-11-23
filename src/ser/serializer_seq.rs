use super::{AttributeValue, Error, Result, Serializer};
use serde::{ser, Serialize};

pub struct SerializerSeq<T> {
    vec: Vec<T>,
}

impl<T> SerializerSeq<T> {
    pub fn new(len: Option<usize>) -> Self {
        let vec = if let Some(len) = len {
            Vec::with_capacity(len)
        } else {
            Vec::new()
        };

        SerializerSeq { vec }
    }
}

impl<'a, T> ser::SerializeSeq for SerializerSeq<T>
where
    T: AttributeValue,
{
    type Ok = T;
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<E>(&mut self, value: &E) -> Result<()>
    where
        E: ?Sized + Serialize,
    {
        let serializer = Serializer::<T>::default();
        let value = value.serialize(serializer)?;
        self.vec.push(value);
        Ok(())
    }

    // Close the sequence.
    fn end(self) -> Result<T> {
        Ok(T::construct_l(self.vec))
    }
}

impl<'a, T> ser::SerializeTupleStruct for SerializerSeq<T>
where
    T: AttributeValue,
{
    type Ok = T;
    type Error = Error;

    fn serialize_field<F>(&mut self, value: &F) -> Result<()>
    where
        F: ?Sized + Serialize,
    {
        let serializer = Serializer::<T>::default();
        let value = value.serialize(serializer)?;
        self.vec.push(value);
        Ok(())
    }

    // Close the sequence.
    fn end(self) -> Result<T> {
        Ok(T::construct_l(self.vec))
    }
}

impl<'a, T> ser::SerializeTuple for SerializerSeq<T>
where
    T: AttributeValue,
{
    type Ok = T;
    type Error = Error;

    fn serialize_element<E>(&mut self, value: &E) -> Result<()>
    where
        E: ?Sized + Serialize,
    {
        let serializer = Serializer::<T>::default();
        let value = value.serialize(serializer)?;
        self.vec.push(value);
        Ok(())
    }

    // Close the sequence.
    fn end(self) -> Result<T> {
        Ok(T::construct_l(self.vec))
    }
}
