use serde::{de, ser};
use std::fmt::{self, Display};

/// This type represents all possible errors that can occur when serializing or deserializing
/// DynamoDB data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(ErrorImpl);

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        <ErrorImpl as ser::Error>::custom(msg).into()
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        <ErrorImpl as de::Error>::custom(msg).into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorImpl {
    /// Serde error
    Message(String),

    /// Not a map-like object
    NotMaplike,
    /// Not a set-like sequence
    NotSetlike,

    /// Expected string
    ExpectedString,
    /// Expected map
    ExpectedMap,
    /// Expected seq
    ExpectedSeq,
    /// Expected num
    ExpectedNum,
    /// Expected bool
    ExpectedBool,
    /// Expected char
    ExpectedChar,
    /// Expected unit
    ExpectedUnit,
    /// Expected unit struct
    ExpectedUnitStruct,
    /// Expected enum
    ExpectedEnum,
    /// Exprected binary data
    ExpectedBytes,
    /// Expected an item with a single key
    ExpectedSingleKey,
    /// Failed to parse as an integer
    FailedToParseInt(String, std::num::ParseIntError),
    /// Failed to parse as a float
    FailedToParseFloat(String, std::num::ParseFloatError),
    /// Key must be a string
    KeyMustBeAString,
    /// SerializeMap's serialize_key called twice!
    SerializeMapKeyCalledTwice,
    /// SerializeMap's serialize_value called before serialize_key!
    SerializeMapValueBeforeKey,
    /// String set contains non-string element
    StringSetExpectedType,
    /// Number set contains non-number element
    NumberSetExpectedType,
    /// Binary set contains non-binary element
    BinarySetExpectedType,
}

#[allow(clippy::from_over_into)]
impl Into<Error> for ErrorImpl {
    fn into(self) -> Error {
        Error(self)
    }
}

impl Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorImpl::Message(ref s) => f.write_str(s),
            ErrorImpl::NotMaplike => f.write_str("Not a map-like object"),
            ErrorImpl::NotSetlike => f.write_str("Not a set-like sequence"),
            ErrorImpl::ExpectedString => f.write_str("Expected string"),
            ErrorImpl::ExpectedMap => f.write_str("Expected map"),
            ErrorImpl::ExpectedSeq => f.write_str("Expected seq"),
            ErrorImpl::ExpectedNum => f.write_str("Expected num"),
            ErrorImpl::ExpectedBool => f.write_str("Expected bool"),
            ErrorImpl::ExpectedChar => f.write_str("Expected char"),
            ErrorImpl::ExpectedUnit => f.write_str("Expected unit"),
            ErrorImpl::ExpectedUnitStruct => f.write_str("Expected unit struct"),
            ErrorImpl::ExpectedEnum => f.write_str("Expected enum"),
            ErrorImpl::ExpectedBytes => f.write_str("Expected binary data"),
            ErrorImpl::ExpectedSingleKey => f.write_str("Expected an item with a single key"),
            ErrorImpl::FailedToParseInt(s, err) => {
                write!(f, "Failed to parse '{s}' as an integer: {err}")
            }
            ErrorImpl::FailedToParseFloat(s, err) => {
                write!(f, "Failed to parse '{s}' as a float: {err}")
            }
            ErrorImpl::KeyMustBeAString => f.write_str("Key must be a string"),
            ErrorImpl::SerializeMapKeyCalledTwice => {
                f.write_str("SerializeMap::serialize_key called twice")
            }
            ErrorImpl::SerializeMapValueBeforeKey => f.write_str(
                "SerializeMap::serialize_value called before SerializeMap::serialize_key",
            ),
            ErrorImpl::StringSetExpectedType => {
                f.write_str("String set element does not serialize to string")
            }
            ErrorImpl::NumberSetExpectedType => {
                f.write_str("Number set element does not serialize to number")
            }
            ErrorImpl::BinarySetExpectedType => {
                f.write_str("Binary set element does not serialize to binary")
            }
        }
    }
}

impl std::error::Error for ErrorImpl {}

impl ser::Error for ErrorImpl {
    fn custom<T: Display>(msg: T) -> Self {
        ErrorImpl::Message(msg.to_string())
    }
}

impl de::Error for ErrorImpl {
    fn custom<T: Display>(msg: T) -> Self {
        ErrorImpl::Message(msg.to_string())
    }
}

/// Alias for a `Result` with the error type `serde_dynamo::Error`
pub type Result<T, E = Error> = std::result::Result<T, E>;
