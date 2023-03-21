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
        if std::ptr::eq(name, crate::set::NEWTYPE_SYMBOL) {
            let av = value.serialize(self)?;

            let vals = match av {
                AttributeValue::L(vals) => vals,
                _ => return Err(crate::error::ErrorImpl::NotSetlike.into()),
            };

            if vals.is_empty() {
                return Err(crate::error::ErrorImpl::SetEmpty.into());
            }

            fn convert_string_item(v: AttributeValue) -> Result<String, Error> {
                if let AttributeValue::S(s) = v {
                    Ok(s)
                } else {
                    Err(crate::error::ErrorImpl::SetNotHomogenous.into())
                }
            }

            fn convert_number_item(v: AttributeValue) -> Result<String, Error> {
                if let AttributeValue::N(s) = v {
                    Ok(s)
                } else {
                    Err(crate::error::ErrorImpl::SetNotHomogenous.into())
                }
            }

            fn convert_bytes_item(v: AttributeValue) -> Result<Vec<u8>, Error> {
                if let AttributeValue::B(s) = v {
                    Ok(s)
                } else {
                    Err(crate::error::ErrorImpl::SetNotHomogenous.into())
                }
            }

            match vals[0] {
                AttributeValue::S(_) => {
                    let set = vals
                        .into_iter()
                        .map(convert_string_item)
                        .collect::<Result<_, _>>()?;
                    Ok(AttributeValue::Ss(set))
                }
                AttributeValue::N(_) => {
                    let set = vals
                        .into_iter()
                        .map(convert_number_item)
                        .collect::<Result<_, _>>()?;
                    Ok(AttributeValue::Ns(set))
                }
                AttributeValue::B(_) => {
                    let set = vals
                        .into_iter()
                        .map(convert_bytes_item)
                        .collect::<Result<_, _>>()?;
                    Ok(AttributeValue::Bs(set))
                }
                _ => Err(crate::error::ErrorImpl::SetInvalidItem.into()),
            }
        } else {
            value.serialize(self)
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
        let serializer = Serializer::default();
        let av = value.serialize(serializer)?;
        let mut item = HashMap::new();
        item.insert(variant.to_string(), av);
        Ok(AttributeValue::M(item))
    }
}
