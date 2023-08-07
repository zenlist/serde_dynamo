use super::{AttributeValue, Error, Result, Serializer};
use serde::{ser, Serialize};

pub struct SerializerSeq {
    vec: Vec<AttributeValue>,
}

impl SerializerSeq {
    pub fn new(len: Option<usize>) -> Self {
        let vec = if let Some(len) = len {
            Vec::with_capacity(len)
        } else {
            Vec::new()
        };

        SerializerSeq { vec }
    }
}

impl ser::SerializeSeq for SerializerSeq {
    type Ok = AttributeValue;
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<E>(&mut self, value: &E) -> Result<()>
    where
        E: ?Sized + Serialize,
    {
        let serializer = Serializer;
        let value = value.serialize(serializer)?;
        self.vec.push(value);
        Ok(())
    }

    // Close the sequence.
    fn end(self) -> Result<Self::Ok> {
        Ok(AttributeValue::L(self.vec))
    }
}

impl ser::SerializeTupleStruct for SerializerSeq {
    type Ok = AttributeValue;
    type Error = Error;

    fn serialize_field<F>(&mut self, value: &F) -> Result<()>
    where
        F: ?Sized + Serialize,
    {
        let serializer = Serializer;
        let value = value.serialize(serializer)?;
        self.vec.push(value);
        Ok(())
    }

    // Close the sequence.
    fn end(self) -> Result<Self::Ok> {
        Ok(AttributeValue::L(self.vec))
    }
}

impl ser::SerializeTuple for SerializerSeq {
    type Ok = AttributeValue;
    type Error = Error;

    fn serialize_element<E>(&mut self, value: &E) -> Result<()>
    where
        E: ?Sized + Serialize,
    {
        let serializer = Serializer;
        let value = value.serialize(serializer)?;
        self.vec.push(value);
        Ok(())
    }

    // Close the sequence.
    fn end(self) -> Result<Self::Ok> {
        Ok(AttributeValue::L(self.vec))
    }
}
