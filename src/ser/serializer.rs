use super::{
    AttributeValue, Error, SerializerMap, SerializerSeq, SerializerStruct, SerializerStructVariant,
    SerializerTupleVariant,
};
use serde::{ser, Serialize};
use std::collections::HashMap;

/// A structure for serializing Rust values into [`AttributeValue`]s.
#[derive(Copy, Clone, Debug, Default)]
pub struct Serializer;

impl ser::Serializer for Serializer {
    type Ok = AttributeValue;
    type Error = Error;

    type SerializeSeq = SerializerSeq;
    type SerializeTuple = SerializerSeq;
    type SerializeTupleStruct = SerializerSeq;
    type SerializeTupleVariant = SerializerTupleVariant;
    type SerializeMap = SerializerMap;
    type SerializeStruct = SerializerStruct;
    type SerializeStructVariant = SerializerStructVariant;

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::N(v.to_string()))
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::N(v.to_string()))
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::N(v.to_string()))
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::N(v.to_string()))
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::N(v.to_string()))
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::N(v.to_string()))
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::N(v.to_string()))
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::N(v.to_string()))
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::N(v.to_string()))
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::N(v.to_string()))
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::S(v.to_string()))
    }
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let serializer = SerializerSeq::new(len);
        Ok(serializer)
    }
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let serializer = SerializerMap::new(len);
        Ok(serializer)
    }
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::Bool(v))
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::S(v.to_string()))
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::Null(true))
    }
    fn serialize_some<V: ?Sized>(self, value: &V) -> Result<Self::Ok, Self::Error>
    where
        V: Serialize,
    {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::Null(true))
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::B(v.to_vec()))
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
        Ok(AttributeValue::Null(true))
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue::S(variant.to_string()))
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
        name: &'static str,
        value: &V,
    ) -> Result<Self::Ok, Self::Error>
    where
        V: Serialize,
    {
        let av = value.serialize(self)?;

        if crate::string_set::should_serialize_as_string_set(name) {
            crate::string_set::convert_to_set(av)
        } else if crate::number_set::should_serialize_as_numbers_set(name) {
            crate::number_set::convert_to_set(av)
        } else if crate::binary_set::should_serialize_as_binary_set(name) {
            crate::binary_set::convert_to_set(av)
        } else {
            Ok(av)
        }
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
        let serializer = Serializer;
        let av = value.serialize(serializer)?;
        let mut item = HashMap::new();
        item.insert(variant.to_string(), av);
        Ok(AttributeValue::M(item))
    }
}
