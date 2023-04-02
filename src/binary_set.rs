//! Serializer codec for serializing a list of binaries as a set
//!
//! # Usage
//!
//! To use, annotate the field with `#[serde(with = "serde_dynamo::binary_set")]`.
//!
//! DynamoDB will return an error if given an empty set. Thus, it may
//! be beneficial to additionally annotate the field with `#[serde(default)]`
//! and `#[serde(skip_serializing_if = "<empty check>")]`. This will make sure
//! that the field is omitted when empty.
//!
//! This serializer does not check for duplicate values or an empty set.
//! If the set contains duplicate values or is empty, DynamoDB will return a
//! validation error when the attribute value is used.
//!
//! # Errors
//!
//! The serializer in this module will return an error if:
//!
//! * the value does not serialize as a sequence
//! * the sequence contains any value that is not a binary
//!
//! # Examples
//!
//! ```
//! use serde_bytes::ByteBuf;
//! use serde_derive::{Serialize, Deserialize};
//! use serde_dynamo::{Item, AttributeValue};
//!
//! #[derive(Serialize, Deserialize)]
//! struct MyStruct {
//!     #[serde(with = "serde_dynamo::binary_set")]
//!     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//!     data: Vec<ByteBuf>,
//! }
//!
//! let my_struct = MyStruct {
//!     data: vec![
//!         ByteBuf::from(b"hello".to_vec()),
//!         ByteBuf::from(b"world".to_vec())
//!     ].into(),
//! };
//!
//! let serialized: Item = serde_dynamo::to_item(&my_struct).unwrap();
//! assert_eq!(
//!     serialized["data"],
//!     AttributeValue::Bs(vec![b"hello".to_vec(), b"world".to_vec()])
//! );
//! ```

pub(super) static NEWTYPE_SYMBOL: &str = "\u{037E}BYTESSET\u{037E}";

#[inline]
pub(crate) fn should_serialize_as_binary_set(name: &str) -> bool {
    std::ptr::eq(name, NEWTYPE_SYMBOL)
}

/// Serializes the given value as a binary set
///
/// See the [module documentation][crate::binary_set] for
/// additional usage information.
///
/// # Errors
///
/// The serializer in this module will return an error if:
///
/// * the value does not serialize as a sequence
/// * the sequence contains any value that is not a binary
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

/// Serializes the wrapped value as a binary set
///
/// This is useful for [`to_attribute_value`][crate::to_attribute_value]
/// when you want to serialize a sequence as a set of binaries.
///
/// # Examples
///
/// ```
/// use serde_bytes::ByteBuf;
/// use serde_dynamo::{binary_set::BinarySet, AttributeValue};
///
/// let set = vec![
///     ByteBuf::from(b"hello".to_vec()),
///     ByteBuf::from(b"world".to_vec()),
/// ];
///
/// let val: AttributeValue = serde_dynamo::to_attribute_value(BinarySet(set)).unwrap();
/// assert_eq!(val, AttributeValue::Bs(vec![
///     b"hello".to_vec(),
///     b"world".to_vec(),
/// ]));
/// ```
pub struct BinarySet<T>(pub T);

impl<T> serde::Serialize for BinarySet<T>
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

#[inline(never)]
pub(crate) fn convert_to_set(value: crate::AttributeValue) -> crate::Result<crate::AttributeValue> {
    let vals = match value {
        crate::AttributeValue::L(vals) => vals,
        _ => return Err(crate::error::ErrorImpl::NotSetlike.into()),
    };

    let set = vals
        .into_iter()
        .map(|v| {
            if let crate::AttributeValue::B(s) = v {
                Ok(s)
            } else {
                Err(crate::error::ErrorImpl::BinarySetExpectedType.into())
            }
        })
        .collect::<Result<_, _>>()?;

    Ok(crate::AttributeValue::Bs(set))
}

#[cfg(test)]
mod tests {
    use serde_derive::{Deserialize, Serialize};

    use crate::binary_set::BinarySet;

    #[test]
    fn newtype_binaries_set_in_struct() {
        use serde_bytes::ByteBuf;
        let set = vec![
            ByteBuf::from(b"test".as_slice()),
            ByteBuf::from(b"test2".as_slice()),
        ];
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        struct Struct {
            #[serde(with = "crate::binary_set")]
            set: Vec<ByteBuf>,
        }

        let item: crate::Item = dbg!(crate::to_item(Struct { set }).unwrap());
        assert_eq!(
            item["set"],
            crate::AttributeValue::Bs(vec![b"test".to_vec(), b"test2".to_vec(),])
        );
    }

    #[test]
    fn newtype_set_for_binaries() {
        use serde_bytes::Bytes;
        let set = vec![
            Bytes::new(b"test".as_slice()),
            Bytes::new(b"test2".as_slice()),
        ];

        let val: crate::AttributeValue = dbg!(crate::to_attribute_value(BinarySet(set)).unwrap());
        assert_eq!(
            val,
            crate::AttributeValue::Bs(vec![b"test".to_vec(), b"test2".to_vec(),])
        );
    }
}
