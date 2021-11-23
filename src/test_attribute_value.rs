use crate::AttributeValue;
use std::collections::HashMap;

/// An attribute value used for tests
///
/// This is publicly exposed because it's used in lots of the doc tests. However, it's
/// #[doc(hidden)] because this is of zero use to an end user.
#[derive(Debug, Clone, Eq, PartialEq)]
#[doc(hidden)]
pub enum TestAttributeValue {
    /// Number
    N(String),
    /// String
    S(String),
    /// Boolean
    Bool(bool),
    /// Bytes
    B(Vec<u8>),
    /// Null
    Null,
    /// Map
    M(HashMap<String, TestAttributeValue>),
    /// List
    L(Vec<TestAttributeValue>),
    /// String list
    SS(Vec<String>),
    /// Number list
    NS(Vec<String>),
    /// Byte string list
    BS(Vec<Vec<u8>>),
}

impl AttributeValue for TestAttributeValue {
    /// TODO
    fn is_n(&self) -> bool {
        matches!(self, TestAttributeValue::N(..))
    }
    /// TODO
    fn is_s(&self) -> bool {
        matches!(self, TestAttributeValue::S(..))
    }
    /// TODO
    fn is_bool(&self) -> bool {
        matches!(self, TestAttributeValue::Bool(..))
    }
    /// TODO
    fn is_b(&self) -> bool {
        matches!(self, TestAttributeValue::B(..))
    }
    /// TODO
    fn is_null(&self) -> bool {
        matches!(self, TestAttributeValue::Null)
    }
    /// TODO
    fn is_m(&self) -> bool {
        matches!(self, TestAttributeValue::M(..))
    }
    /// TODO
    fn is_l(&self) -> bool {
        matches!(self, TestAttributeValue::L(..))
    }
    /// TODO
    fn is_ss(&self) -> bool {
        matches!(self, TestAttributeValue::SS(..))
    }
    /// TODO
    fn is_ns(&self) -> bool {
        matches!(self, TestAttributeValue::NS(..))
    }
    /// TODO
    fn is_bs(&self) -> bool {
        matches!(self, TestAttributeValue::BS(..))
    }

    /// TODO
    fn as_n(&self) -> Option<&str> {
        match self {
            TestAttributeValue::N(ref n) => Some(n),
            _ => None,
        }
    }
    /// TODO
    fn as_s(&self) -> Option<&str> {
        match self {
            TestAttributeValue::S(ref s) => Some(s),
            _ => None,
        }
    }
    /// TODO
    fn as_bool(&self) -> Option<bool> {
        match self {
            TestAttributeValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    /// TODO
    fn as_b(&self) -> Option<&[u8]> {
        match self {
            TestAttributeValue::B(ref b) => Some(b),
            _ => None,
        }
    }
    /// TODO
    fn as_null(&self) -> Option<bool> {
        match self {
            TestAttributeValue::Null => Some(true),
            _ => None,
        }
    }
    /// TODO
    fn as_m(&self) -> Option<&HashMap<String, Self>> {
        match self {
            TestAttributeValue::M(ref m) => Some(m),
            _ => None,
        }
    }
    /// TODO
    fn as_l(&self) -> Option<&[Self]> {
        match self {
            TestAttributeValue::L(ref l) => Some(l),
            _ => None,
        }
    }
    /// TODO
    fn as_ss(&self) -> Option<&[String]> {
        match self {
            TestAttributeValue::SS(ref ss) => Some(ss),
            _ => None,
        }
    }
    /// TODO
    fn as_ns(&self) -> Option<&[String]> {
        match self {
            TestAttributeValue::NS(ref ns) => Some(ns),
            _ => None,
        }
    }

    /// TODO
    fn into_n(self) -> Option<String> {
        match self {
            TestAttributeValue::N(n) => Some(n),
            _ => None,
        }
    }
    /// TODO
    fn into_s(self) -> Option<String> {
        match self {
            TestAttributeValue::S(s) => Some(s),
            _ => None,
        }
    }
    /// TODO
    fn into_bool(self) -> Option<bool> {
        match self {
            TestAttributeValue::Bool(b) => Some(b),
            _ => None,
        }
    }
    /// TODO
    fn into_b(self) -> Option<Vec<u8>> {
        match self {
            TestAttributeValue::B(b) => Some(b),
            _ => None,
        }
    }
    /// TODO
    fn into_null(self) -> Option<bool> {
        match self {
            TestAttributeValue::Null => Some(true),
            _ => None,
        }
    }
    /// TODO
    fn into_m(self) -> Option<HashMap<String, Self>> {
        match self {
            TestAttributeValue::M(m) => Some(m),
            _ => None,
        }
    }
    /// TODO
    fn into_l(self) -> Option<Vec<Self>> {
        match self {
            TestAttributeValue::L(l) => Some(l),
            _ => None,
        }
    }
    /// TODO
    fn into_ss(self) -> Option<Vec<String>> {
        match self {
            TestAttributeValue::SS(ss) => Some(ss),
            _ => None,
        }
    }
    /// TODO
    fn into_ns(self) -> Option<Vec<String>> {
        match self {
            TestAttributeValue::NS(ns) => Some(ns),
            _ => None,
        }
    }
    /// TODO
    fn into_bs(self) -> Option<Vec<Vec<u8>>> {
        match self {
            TestAttributeValue::BS(bs) => Some(bs),
            _ => None,
        }
    }

    /// TODO
    fn construct_n(input: String) -> Self {
        TestAttributeValue::N(input)
    }
    /// TODO
    fn construct_s(input: String) -> Self {
        TestAttributeValue::S(input)
    }
    /// TODO
    fn construct_bool(input: bool) -> Self {
        TestAttributeValue::Bool(input)
    }
    /// TODO
    fn construct_b(input: &[u8]) -> Self {
        TestAttributeValue::B(Vec::from(input))
    }
    /// TODO
    fn construct_null(_input: bool) -> Self {
        TestAttributeValue::Null
    }
    /// TODO
    fn construct_m(input: HashMap<String, Self>) -> Self {
        TestAttributeValue::M(input)
    }
    /// TODO
    fn construct_l(input: Vec<Self>) -> Self {
        TestAttributeValue::L(input)
    }
    /// TODO
    fn construct_ss(input: Vec<String>) -> Self {
        TestAttributeValue::SS(input)
    }
    /// TODO
    fn construct_ns(input: Vec<String>) -> Self {
        TestAttributeValue::NS(input)
    }
    /// TODO
    fn construct_bs(input: Vec<Vec<u8>>) -> Self {
        TestAttributeValue::BS(input)
    }
}
