//! Serialization modules for serializing lists of values as sets
//!
//! When using the serializers in these submodules, it may be beneficial to
//! annotate the field with `#[serde(default)]` and
//! `#[serde(skip_serializing_if = "<empty check>")]`. This will make sure
//! that the field is omitted when empty. DynamoDB will return an error if
//! an empty set is used.
//!
//! These serializers do not check for duplicate values. If the set contains
//! duplicate values, DynamoDB will return a validation error when the
//! set is used.
//!
//! # Examples
//!
//! ```
//! use std::collections::{HashSet, VecDeque};
//!
//! use serde_bytes::ByteBuf;
//! use serde_derive::{Serialize, Deserialize};
//! use serde_dynamo::{Item, AttributeValue};
//!
//! #[derive(Serialize, Deserialize)]
//! struct MyStruct {
//!     #[serde(with = "serde_dynamo::set::strings")]
//!     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//!     names: Vec<String>,
//!     #[serde(with = "serde_dynamo::set::numbers")]
//!     #[serde(default, skip_serializing_if = "HashSet::is_empty")]
//!     numbers: HashSet<u64>,
//!     #[serde(with = "serde_dynamo::set::bytes")]
//!     #[serde(default, skip_serializing_if = "VecDeque::is_empty")]
//!     data: VecDeque<ByteBuf>,
//! }
//!
//! let my_struct = MyStruct {
//!     names: vec!["John".to_string(), "Jane".to_string()],
//!     numbers: [14, 25, 32].into_iter().collect(),
//!     data: vec![ByteBuf::from(b"hello".to_vec()), ByteBuf::from(b"world".to_vec())].into(),
//! };
//!
//! let serialized: Item = serde_dynamo::to_item(&my_struct).unwrap();
//! assert_eq!(
//!     serialized["names"],
//!     AttributeValue::Ss(vec!["John".to_string(), "Jane".to_string()])
//! );
//! if let AttributeValue::Ns(mut sorted_numbers) = serialized["numbers"].clone() {
//!     sorted_numbers.sort();
//!     assert_eq!(
//!         sorted_numbers,
//!         vec!["14".to_string(), "25".to_string(), "32".to_string()]
//!     );
//! } else {
//!     panic!("Expected numbers to be a set of numbers");
//! }
//! assert_eq!(
//!     serialized["data"],
//!     AttributeValue::Bs(vec![b"hello".to_vec(), b"world".to_vec()])
//! );
//! ```

pub mod bytes;
pub mod numbers;
pub mod strings;

/// Serializes the wrapped value as a string set
///
/// This is useful for [`to_attribute_value`][crate::to_attribute_value]
/// when you want to serialize a sequence as a set of strings.
///
/// # Examples
///
/// ```
/// use serde_dynamo::{set::StringSet, AttributeValue};
///
/// let set = vec![
///     "orange",
///     "apple",
/// ];
///
/// let val: AttributeValue = serde_dynamo::to_attribute_value(StringSet(set)).unwrap();
/// assert_eq!(val, AttributeValue::Ss(vec![
///     "orange".to_string(),
///     "apple".to_string(),
/// ]));
/// ```
pub struct StringSet<T>(pub T);

impl<T> serde::Serialize for StringSet<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_newtype_struct(strings::NEWTYPE_SYMBOL, &self.0)
    }
}

/// Serializes the wrapped value as a number set
///
/// This is useful for [`to_attribute_value`][crate::to_attribute_value]
/// when you want to serialize a sequence as a set of numbers.
///
/// # Examples
///
/// ```
/// use serde_dynamo::{set::NumberSet, AttributeValue};
///
/// let set = vec![
///     1432,
///     5342,
/// ];
///
/// let val: AttributeValue = serde_dynamo::to_attribute_value(NumberSet(set)).unwrap();
/// assert_eq!(val, AttributeValue::Ns(vec![
///     "1432".to_string(),
///     "5342".to_string(),
/// ]));
/// ```
pub struct NumberSet<T>(pub T);

impl<T> serde::Serialize for NumberSet<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_newtype_struct(numbers::NEWTYPE_SYMBOL, &self.0)
    }
}

/// Serializes the wrapped value as a byte array set
///
/// This is useful for [`to_attribute_value`][crate::to_attribute_value]
/// when you want to serialize a sequence as a set of byte arrays.
///
/// # Examples
///
/// ```
/// use serde_bytes::ByteBuf;
/// use serde_dynamo::{set::BytesSet, AttributeValue};
///
/// let set = vec![
///     ByteBuf::from(b"hello".to_vec()),
///     ByteBuf::from(b"world".to_vec()),
/// ];
///
/// let val: AttributeValue = serde_dynamo::to_attribute_value(BytesSet(set)).unwrap();
/// assert_eq!(val, AttributeValue::Bs(vec![
///     b"hello".to_vec(),
///     b"world".to_vec(),
/// ]));
/// ```
pub struct BytesSet<T>(pub T);

impl<T> serde::Serialize for BytesSet<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_newtype_struct(bytes::NEWTYPE_SYMBOL, &self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_newtype_addresses_different() {
        assert!(!std::ptr::eq(
            strings::NEWTYPE_SYMBOL,
            numbers::NEWTYPE_SYMBOL
        ));
        assert!(!std::ptr::eq(
            strings::NEWTYPE_SYMBOL,
            bytes::NEWTYPE_SYMBOL
        ));
        assert!(!std::ptr::eq(
            numbers::NEWTYPE_SYMBOL,
            bytes::NEWTYPE_SYMBOL
        ));
    }

    #[test]
    fn newtype_set_for_strings() {
        let set = vec!["test", "test2"];

        let val: crate::AttributeValue = dbg!(crate::to_attribute_value(StringSet(set)).unwrap());
        assert_eq!(
            val,
            crate::AttributeValue::Ss(vec!["test".to_string(), "test2".to_string(),])
        );
    }

    #[test]
    fn newtype_set_for_numbers() {
        let set = vec![85, 99];

        let val: crate::AttributeValue = dbg!(crate::to_attribute_value(NumberSet(set)).unwrap());
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

        let val: crate::AttributeValue = dbg!(crate::to_attribute_value(BytesSet(set)).unwrap());
        assert_eq!(
            val,
            crate::AttributeValue::Bs(vec![b"test".to_vec(), b"test2".to_vec(),])
        );
    }
}
