//! Utilities for serializing lists of values as sets
//!
//! This module can be used to serialize a list of values as a set.
//! To do so, annotate the field with `#[serde(with = "serde_dynamo::set")]`.
//! It is often beneficial to also annotate the field with `#[serde(default)]` and
//! `#[serde(skip_serializing_if = "<empty check>")]` to ensure that the field is
//! omitted when empty. Failure to skip the field when empty will result in an error
//! when serializing the item.
//!
//! # Errors
//!
//! The serializer in this module will return an error if:
//!
//! * the value does not serialize as a sequence
//! * the sequence is empty
//! * the sequence contains any value that is not a string, number, or byte array
//! * the sequence is composed of values that serialize to different attribute value types
//!
//! This serializer does not check for duplicate values. If the set contains
//! duplicate values, DynamoDB will return a validation error when the attribute
//! value is used.
//!
//! # Examples
//!
//! ```
//! use serde_derive::{Serialize, Deserialize};
//! use serde_dynamo::{Item, AttributeValue};
//!
//! #[derive(Serialize, Deserialize)]
//! struct MyStruct {
//!     #[serde(with = "serde_dynamo::set")]
//!     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//!     names: Vec<String>,
//! }
//!
//! let my_struct = MyStruct {
//!     names: vec!["John".to_string(), "Jane".to_string()],
//! };
//!
//! let serialized: Item = serde_dynamo::to_item(&my_struct).unwrap();
//! assert_eq!(
//!    serialized["names"],
//!    AttributeValue::Ss(vec!["John".to_string(), "Jane".to_string()])
//! );
//! ```

/// An internal symbol used to identify newtype structs that should be serialized as sets
///
/// The value must be a `static` reference so that it can be compared by pointer equality
pub(crate) static NEWTYPE_SYMBOL: &str = "\u{037E}INTERNAL_NEWTYPE_FOR_SET\u{037E}";

/// Serializes the given list value as a set
///
/// # Errors
///
/// This function will return an error if:
///
/// * the value does not serialize as a sequence
/// * the sequence is empty
/// * the sequence contains any value that is not a string, number, or byte array
/// * the sequence is composed of values that serialize to different attribute value types
///
/// This serializer does not check for duplicate values. If the set contains
/// duplicate values, DynamoDB will return a validation error when the attribute
/// value is used.
pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: serde::Serialize,
    S: serde::Serializer,
{
    serializer.serialize_newtype_struct(NEWTYPE_SYMBOL, &value)
}

/// Deserializes the given value as a set
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    T::deserialize(deserializer)
}

/// Serializes the wrapped value as a set
///
/// This is useful for [`to_attribute_value`][crate::to_attribute_value]
/// when you want to serialize a sequence as a set.
///
/// # Examples
///
/// ```
/// use serde_dynamo::{set::Set, AttributeValue};
///
/// let set = vec![
///    "orange",
///    "apple",
/// ];
///
/// let val: AttributeValue = serde_dynamo::to_attribute_value(Set(set)).unwrap();
/// assert_eq!(val, AttributeValue::Ss(vec![
///     "orange".to_string(),
///     "apple".to_string(),
/// ]));
/// ```
pub struct Set<T>(pub T);

impl<T> serde::Serialize for Set<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_newtype_struct(NEWTYPE_SYMBOL, &self.0)
    }
}

#[cfg(test)]
mod tests {
    use serde_derive::{Deserialize, Serialize};

    use super::*;

    #[test]
    fn newtype_set_for_strings() {
        let set = vec!["test", "test2"];

        let val: crate::AttributeValue = dbg!(crate::to_attribute_value(Set(set)).unwrap());
        assert_eq!(
            val,
            crate::AttributeValue::Ss(vec!["test".to_string(), "test2".to_string(),])
        );
    }

    #[test]
    fn newtype_set_for_numbers() {
        let set = vec![85, 99];

        let val: crate::AttributeValue = dbg!(crate::to_attribute_value(Set(set)).unwrap());
        assert_eq!(
            val,
            crate::AttributeValue::Ns(vec!["85".to_string(), "99".to_string(),])
        );
    }

    #[test]
    fn newtype_set_for_byte_arrays() {
        use serde_bytes::Bytes;
        let set = vec![
            Bytes::new(b"test".as_slice()),
            Bytes::new(b"test2".as_slice()),
        ];

        let val: crate::AttributeValue = dbg!(crate::to_attribute_value(Set(set)).unwrap());
        assert_eq!(
            val,
            crate::AttributeValue::Bs(vec![b"test".to_vec(), b"test2".to_vec(),])
        );
    }

    #[test]
    fn newtype_strings_set_in_struct() {
        let set = vec!["test".to_string(), "test2".to_string()];
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        struct Struct {
            #[serde(with = "crate::set")]
            set: Vec<String>,
        }

        let item: crate::Item = dbg!(crate::to_item(Struct { set }).unwrap());
        assert_eq!(
            item["set"],
            crate::AttributeValue::Ss(vec!["test".to_string(), "test2".to_string(),])
        );
    }

    #[test]
    fn newtype_numbers_set_in_struct() {
        let set = vec![123234, 535622];
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        struct Struct {
            #[serde(with = "crate::set")]
            set: Vec<u64>,
        }

        let item: crate::Item = dbg!(crate::to_item(Struct { set }).unwrap());
        assert_eq!(
            item["set"],
            crate::AttributeValue::Ns(vec!["123234".to_string(), "535622".to_string(),])
        );
    }

    #[test]
    fn newtype_byte_arrays_set_in_struct() {
        use serde_bytes::ByteBuf;
        let set = vec![
            ByteBuf::from(b"test".as_slice()),
            ByteBuf::from(b"test2".as_slice()),
        ];
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        struct Struct {
            #[serde(with = "crate::set")]
            set: Vec<ByteBuf>,
        }

        let item: crate::Item = dbg!(crate::to_item(Struct { set }).unwrap());
        assert_eq!(
            item["set"],
            crate::AttributeValue::Bs(vec![b"test".to_vec(), b"test2".to_vec(),])
        );
    }
}
