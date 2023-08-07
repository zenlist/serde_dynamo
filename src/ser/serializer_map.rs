use super::{AttributeValue, Error, ErrorImpl, Result, Serializer};
use serde::{ser, serde_if_integer128, Serialize};
use std::collections::HashMap;

pub struct SerializerMap {
    item: HashMap<String, AttributeValue>,
    next_key: Option<String>,
}

impl SerializerMap {
    pub fn new(len: Option<usize>) -> Self {
        let item = if let Some(len) = len {
            HashMap::with_capacity(len)
        } else {
            HashMap::new()
        };
        SerializerMap {
            item,
            next_key: None,
        }
    }
}

impl ser::SerializeMap for SerializerMap {
    type Ok = AttributeValue;
    type Error = Error;

    fn serialize_key<K: ?Sized>(&mut self, key: &K) -> Result<(), Self::Error>
    where
        K: Serialize,
    {
        if self.next_key.is_some() {
            return Err(ErrorImpl::SerializeMapKeyCalledTwice.into());
        }

        let key = key.serialize(MapKeySerializer)?;
        self.next_key = Some(key);
        Ok(())
    }

    fn serialize_value<V: ?Sized>(&mut self, value: &V) -> Result<(), Self::Error>
    where
        V: Serialize,
    {
        let key = self
            .next_key
            .take()
            .ok_or_else(|| ErrorImpl::SerializeMapValueBeforeKey.into())?;

        let value = value.serialize(Serializer)?;
        self.item.insert(key, value);
        Ok(())
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        let key = key.serialize(MapKeySerializer)?;
        let value = value.serialize(Serializer)?;
        self.item.insert(key, value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::M(self.item))
    }
}

struct MapKeySerializer;

impl ser::Serializer for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    serde_if_integer128! {
        fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
            Ok(v.to_string())
        }
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    serde_if_integer128! {
        fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
            Ok(v.to_string())
        }
    }
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(ErrorImpl::KeyMustBeAString.into())
    }
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(ErrorImpl::KeyMustBeAString.into())
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unreachable!()
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unreachable!()
    }
    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(ErrorImpl::KeyMustBeAString.into())
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(ErrorImpl::KeyMustBeAString.into())
    }
    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(ErrorImpl::KeyMustBeAString.into())
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ErrorImpl::KeyMustBeAString.into())
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ErrorImpl::KeyMustBeAString.into())
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(ErrorImpl::KeyMustBeAString.into())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ErrorImpl::KeyMustBeAString.into())
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ErrorImpl::KeyMustBeAString.into())
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ErrorImpl::KeyMustBeAString.into())
    }
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ErrorImpl::KeyMustBeAString.into())
    }
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(ErrorImpl::KeyMustBeAString.into())
    }
}

impl ser::SerializeSeq for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}
impl ser::SerializeTuple for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}
impl ser::SerializeTupleStruct for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}
impl ser::SerializeTupleVariant for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl ser::SerializeStructVariant for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl ser::SerializeMap for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        _key: &K,
        _value: &V,
    ) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl ser::SerializeStruct for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}
