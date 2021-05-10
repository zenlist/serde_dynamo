//! TODO

use std::collections::HashMap;

mod de;
mod ser;

/// TODO
pub trait AttributeValue: Sized {
    /// TODO
    fn is_n(&self) -> bool;
    /// TODO
    fn is_s(&self) -> bool;
    /// TODO
    fn is_bool(&self) -> bool;
    /// TODO
    fn is_b(&self) -> bool;
    /// TODO
    fn is_null(&self) -> bool;
    /// TODO
    fn is_m(&self) -> bool;
    /// TODO
    fn is_l(&self) -> bool;
    /// TODO
    fn is_ss(&self) -> bool;
    /// TODO
    fn is_ns(&self) -> bool;
    /// TODO
    fn is_bs(&self) -> bool;

    /// TODO
    fn as_n(&self) -> Option<&str>;
    /// TODO
    fn as_s(&self) -> Option<&str>;
    /// TODO
    fn as_bool(&self) -> Option<bool>;
    /// TODO
    fn as_b(&self) -> Option<&[u8]>;
    /// TODO
    fn as_null(&self) -> Option<bool>;
    /// TODO
    fn as_m(&self) -> Option<&HashMap<String, Self>>;
    /// TODO
    fn as_l(&self) -> Option<&[Self]>;
    /// TODO
    fn as_ss(&self) -> Option<&[String]>;
    /// TODO
    fn as_ns(&self) -> Option<&[String]>;
    // /// TODO
    // fn as_bs(&self) -> Option<Vec<Vec<u8>>>;

    /// TODO
    fn into_n(self) -> Option<String>;
    /// TODO
    fn into_s(self) -> Option<String>;
    /// TODO
    fn into_bool(self) -> Option<bool>;
    /// TODO
    fn into_b(self) -> Option<Vec<u8>>;
    /// TODO
    fn into_null(self) -> Option<bool>;
    /// TODO
    fn into_m(self) -> Option<HashMap<String, Self>>;
    /// TODO
    fn into_l(self) -> Option<Vec<Self>>;
    /// TODO
    fn into_ss(self) -> Option<Vec<String>>;
    /// TODO
    fn into_ns(self) -> Option<Vec<String>>;
    /// TODO
    fn into_bs(self) -> Option<Vec<Vec<u8>>>;

    /// TODO
    fn construct_n(input: String) -> Self;
    /// TODO
    fn construct_s(input: String) -> Self;
    /// TODO
    fn construct_bool(input: bool) -> Self;
    /// TODO
    fn construct_b(input: &[u8]) -> Self;
    /// TODO
    fn construct_null(input: bool) -> Self;
    /// TODO
    fn construct_m(input: HashMap<String, Self>) -> Self;
    /// TODO
    fn construct_l(input: Vec<Self>) -> Self;
    // /// TODO
    // fn construct_ss(input: Vec<String>) -> Self;
    // /// TODO
    // fn construct_ns(input: Vec<String>) -> Self;
    // /// TODO
    // fn construct_bs(input: Vec<Vec<u8>>) -> Self;
}

pub use de::{from_attribute_value, from_item, from_items, Deserializer};
pub use ser::{to_attribute_value, to_item, Serializer};
