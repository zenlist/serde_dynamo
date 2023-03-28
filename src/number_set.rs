//! Serializer codec for serializing a list of numbers as a set
//!
//! # Usage
//!
//! To use, annotate the field with `#[serde(with = "serde_dynamo::number_set")]`.
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
//!     #[serde(with = "serde_dynamo::number_set")]
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
/// See the [module documentation][crate::number_set] for
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

/// Serializes the wrapped value as a number set
///
/// This is useful for [`to_attribute_value`][crate::to_attribute_value]
/// when you want to serialize a sequence as a set of numbers.
///
/// # Examples
///
/// ```
/// use serde_dynamo::{number_set::NumberSet, AttributeValue};
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

    use crate::number_set::NumberSet;

    #[test]
    fn newtype_numbers_set_in_struct() {
        let set = vec![123234, 535622];
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        struct Struct {
            #[serde(with = "crate::number_set")]
            set: Vec<u64>,
        }

        let item: crate::Item = dbg!(crate::to_item(Struct { set }).unwrap());
        assert_eq!(
            item["set"],
            crate::AttributeValue::Ns(vec!["123234".to_string(), "535622".to_string(),])
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
}
