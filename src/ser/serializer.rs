use std::marker::PhantomData;

use super::{
    AttributeValue, Error, SerializerMap, SerializerSeq, SerializerStruct, SerializerStructVariant,
    SerializerTupleVariant,
};
use serde::{ser, Serialize};
use std::collections::HashMap;

/// A structure for serializing Rust values into [`AttributeValue`]s.
#[derive(Copy, Clone, Debug)]
pub struct Serializer<T> {
    _phantom: PhantomData<T>,
}

impl<T> Default for Serializer<T> {
    fn default() -> Self {
        Serializer {
            _phantom: PhantomData,
        }
    }
}

impl<'a, T> ser::Serializer for Serializer<T>
where
    T: AttributeValue,
{
    type Ok = T;
    type Error = Error;

    type SerializeSeq = SerializerSeq<T>;
    type SerializeTuple = SerializerSeq<T>;
    type SerializeTupleStruct = SerializerSeq<T>;
    type SerializeTupleVariant = SerializerTupleVariant<T>;
    type SerializeMap = SerializerMap<T>;
    type SerializeStruct = SerializerStruct<T>;
    type SerializeStructVariant = SerializerStructVariant<T>;

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_n(v.to_string()))
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_n(v.to_string()))
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_n(v.to_string()))
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_n(v.to_string()))
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_n(v.to_string()))
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_n(v.to_string()))
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_n(v.to_string()))
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_n(v.to_string()))
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_n(v.to_string()))
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_n(v.to_string()))
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_s(v.to_string()))
    }
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let serializer = SerializerSeq::<T>::new(len);
        Ok(serializer)
    }
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let serializer = SerializerMap::<T>::new(len);
        Ok(serializer)
    }
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_bool(v))
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_s(v.to_string()))
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_null(true))
    }
    fn serialize_some<V: ?Sized>(self, value: &V) -> Result<Self::Ok, Self::Error>
    where
        V: Serialize,
    {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_null(true))
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_b(v))
    }
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        let serializer = SerializerSeq::new(Some(len));
        Ok(serializer)
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let serializer = SerializerStruct::new(len);
        Ok(serializer)
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_null(true))
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(T::construct_s(variant.to_string()))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        let serializer = SerializerSeq::new(Some(len));
        Ok(serializer)
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let serializer = SerializerTupleVariant::new(variant, len);
        Ok(serializer)
    }
    fn serialize_newtype_struct<V: ?Sized>(
        self,
        _name: &'static str,
        value: &V,
    ) -> Result<Self::Ok, Self::Error>
    where
        V: Serialize,
    {
        value.serialize(self)
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let serializer = SerializerStructVariant::new(variant, len);
        Ok(serializer)
    }
    fn serialize_newtype_variant<V: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &V,
    ) -> Result<Self::Ok, Self::Error>
    where
        V: Serialize,
    {
        let serializer = Serializer::default();
        let av = value.serialize(serializer)?;
        let mut item = HashMap::new();
        item.insert(variant.to_string(), av);
        Ok(T::construct_m(item))
    }
}
