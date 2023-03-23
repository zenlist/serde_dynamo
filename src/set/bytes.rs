//! Serializer codec for serializing a list of byte arrays as a set
//!
//! # Usage
//!
//! To use, annotate the field with `#[serde(with = "serde_dynamo::set::bytes")]`.
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
//! * the sequence contains any value that is not a byte array
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
//!     #[serde(with = "serde_dynamo::set::bytes")]
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
pub(crate) fn should_serialize_as_bytes_set(name: &str) -> bool {
    std::ptr::eq(name, NEWTYPE_SYMBOL)
}

/// Serializes the given value as a byte array set
///
/// See the [module documentation][crate::set::bytes] for
/// additional usage information.
///
/// # Errors
///
/// The serializer in this module will return an error if:
///
/// * the value does not serialize as a sequence
/// * the sequence contains any value that is not a byte array
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
                Err(crate::error::ErrorImpl::BytesSetExpectedType.into())
            }
        })
        .collect::<Result<_, _>>()?;

    Ok(crate::AttributeValue::Bs(set))
}

#[cfg(test)]
mod tests {
    use serde_derive::{Deserialize, Serialize};

    #[test]
    fn newtype_byte_arrays_set_in_struct() {
        use serde_bytes::ByteBuf;
        let set = vec![
            ByteBuf::from(b"test".as_slice()),
            ByteBuf::from(b"test2".as_slice()),
        ];
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        struct Struct {
            #[serde(with = "crate::set::bytes")]
            set: Vec<ByteBuf>,
        }

        let item: crate::Item = dbg!(crate::to_item(Struct { set }).unwrap());
        assert_eq!(
            item["set"],
            crate::AttributeValue::Bs(vec![b"test".to_vec(), b"test2".to_vec(),])
        );
    }
}
