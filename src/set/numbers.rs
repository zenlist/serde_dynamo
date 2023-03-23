//! Serializer codec for serializing a list of numbers as a set
//!
//! # Usage
//!
//! To use, annotate the field with `#[serde(with = "serde_dynamo::set::numbers")]`.
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
//! * the sequence contains any value that is not a number
//!
//! # Examples
//!
//! ```
//! use serde_derive::{Serialize, Deserialize};
//! use serde_dynamo::{Item, AttributeValue};
//!
//! #[derive(Serialize, Deserialize)]
//! struct MyStruct {
//!     #[serde(with = "serde_dynamo::set::numbers")]
//!     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//!     numbers: Vec<u64>,
//! }
//!
//! let my_struct = MyStruct {
//!     numbers: vec![14, 25, 32],
//! };
//!
//! let serialized: Item = serde_dynamo::to_item(&my_struct).unwrap();
//! assert_eq!(
//!     serialized["numbers"],
//!     AttributeValue::Ns(vec!["14".to_string(), "25".to_string(), "32".to_string()])
//! );
//! ```

pub(super) static NEWTYPE_SYMBOL: &str = "\u{037E}NUMBERSET\u{037E}";

#[inline]
pub(crate) fn should_serialize_as_numbers_set(name: &str) -> bool {
    std::ptr::eq(name, NEWTYPE_SYMBOL)
}

/// Serializes the given value as a number set
///
/// See the [module documentation]p[crate::set::numbets] for
/// additional usage information.
///
/// # Errors
///
/// The serializer in this module will return an error if:
///
/// * the value does not serialize as a sequence
/// * the sequence contains any value that is not a number
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
            if let crate::AttributeValue::N(s) = v {
                Ok(s)
            } else {
                Err(crate::error::ErrorImpl::NumberSetExpectedType.into())
            }
        })
        .collect::<Result<_, _>>()?;

    Ok(crate::AttributeValue::Ns(set))
}

#[cfg(test)]
mod tests {
    use serde_derive::{Deserialize, Serialize};

    #[test]
    fn newtype_numbers_set_in_struct() {
        let set = vec![123234, 535622];
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        struct Struct {
            #[serde(with = "crate::set::numbers")]
            set: Vec<u64>,
        }

        let item: crate::Item = dbg!(crate::to_item(Struct { set }).unwrap());
        assert_eq!(
            item["set"],
            crate::AttributeValue::Ns(vec!["123234".to_string(), "535622".to_string(),])
        );
    }
}
