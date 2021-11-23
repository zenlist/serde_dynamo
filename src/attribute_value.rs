use std::collections::HashMap;

/// A type that can be used as a DynamoDb attribute value
///
/// This trait will not typically be used by user code. It exists to abstract over the
/// `AttributeValue` definitions in different libraries: `aws-sdk-dynamodb`, `rusoto_dynamodb`, and
/// `rusoto_dynamodbstreams`.
pub trait AttributeValue: Sized {
    /// Determine if the attribute value represents a number
    fn is_n(&self) -> bool;
    /// Determine if the attribute value represents a string
    fn is_s(&self) -> bool;
    /// Determine if the attribute value represents a boolean
    fn is_bool(&self) -> bool;
    /// Determine if the attribute value represents bytes
    fn is_b(&self) -> bool;
    /// Determine if the attribute value represents a null
    fn is_null(&self) -> bool;
    /// Determine if the attribute value represents a map of string/attribute value pairs
    fn is_m(&self) -> bool;
    /// Determine if the attribute value represents a list of attribute values
    fn is_l(&self) -> bool;
    /// Determine if the attribute value represents a list of strings
    fn is_ss(&self) -> bool;
    /// Determine if the attribute value represents a list of numbers
    fn is_ns(&self) -> bool;
    /// Determine if the attribute value represents a list of byte strings
    fn is_bs(&self) -> bool;

    /// Get the number without consuming the attribute value
    fn as_n(&self) -> Option<&str>;
    /// Get the string without consuming the attribute value
    fn as_s(&self) -> Option<&str>;
    /// Get the bool without consuming the attribute value
    fn as_bool(&self) -> Option<bool>;
    /// Get the bytes without consuming the attribute value
    fn as_b(&self) -> Option<&[u8]>;
    /// Get the bool without consuming the attribute value
    fn as_null(&self) -> Option<bool>;
    /// Get the map without consuming the attribute value
    fn as_m(&self) -> Option<&HashMap<String, Self>>;
    /// Get the list without consuming the attribute value
    fn as_l(&self) -> Option<&[Self]>;
    /// Get the string list without consuming the attribute value
    fn as_ss(&self) -> Option<&[String]>;
    /// Get the number list without consuming the attribute value
    fn as_ns(&self) -> Option<&[String]>;

    /// Consume the attribute value and return the number
    fn into_n(self) -> Option<String>;
    /// Consume the attribute value and return the string
    fn into_s(self) -> Option<String>;
    /// Consume the attribute value and return the boolan
    fn into_bool(self) -> Option<bool>;
    /// Consume the attribute value and return the bytes
    fn into_b(self) -> Option<Vec<u8>>;
    /// Consume the attribute value and return the null
    fn into_null(self) -> Option<bool>;
    /// Consume the attribute value and return the map
    fn into_m(self) -> Option<HashMap<String, Self>>;
    /// Consume the attribute value and return the list
    fn into_l(self) -> Option<Vec<Self>>;
    /// Consume the attribute value and return the string list
    fn into_ss(self) -> Option<Vec<String>>;
    /// Consume the attribute value and return the number list
    fn into_ns(self) -> Option<Vec<String>>;
    /// Consume the attribute value and return the byte string list
    fn into_bs(self) -> Option<Vec<Vec<u8>>>;

    /// Create a new attribute value from a number
    fn construct_n(input: String) -> Self;
    /// Create a new attribute value from a string
    fn construct_s(input: String) -> Self;
    /// Create a new attribute value from a bool
    fn construct_bool(input: bool) -> Self;
    /// Create a new attribute value from bytes
    fn construct_b(input: &[u8]) -> Self;
    /// Create a new attribute value from a null
    fn construct_null(input: bool) -> Self;
    /// Create a new attribute value from a map
    fn construct_m(input: HashMap<String, Self>) -> Self;
    /// Create a new attribute value from a list
    fn construct_l(input: Vec<Self>) -> Self;
    /// Create a new attribute value from a string list
    fn construct_ss(input: Vec<String>) -> Self;
    /// Create a new attribute value from a number list
    fn construct_ns(input: Vec<String>) -> Self;
    /// Create a new attribute value from a byte string list
    fn construct_bs(input: Vec<Vec<u8>>) -> Self;
}
