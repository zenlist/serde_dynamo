//! TODO

#[cfg(test)]
mod tests;

extern crate aws_sdk_dynamodb as ext_aws_sdk_dynamodb;

use crate::Result;
use ext_aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An "Item" used in [aws-sdk-dynamodb]'s [get_item], [write_item], [put_item], etc.
///
/// Nowhere in aws_sdk_dynamodb is this type named explicitly, so we name it here to be clear about
/// exactly what is being taken in and being returned.
///
/// [aws-sdk-dynamodb]: https://github.com/awslabs/aws-sdk-rust
pub type Item = HashMap<String, AttributeValue>;

impl crate::generic::AttributeValue for AttributeValue {
    fn is_n(&self) -> bool {
        matches!(self, AttributeValue::N(..))
    }

    fn is_s(&self) -> bool {
        matches!(self, AttributeValue::S(..))
    }

    fn is_bool(&self) -> bool {
        matches!(self, AttributeValue::Bool(..))
    }

    fn is_b(&self) -> bool {
        matches!(self, AttributeValue::B(..))
    }

    fn is_null(&self) -> bool {
        matches!(self, AttributeValue::Null(..))
    }

    fn is_m(&self) -> bool {
        matches!(self, AttributeValue::M(..))
    }

    fn is_l(&self) -> bool {
        matches!(self, AttributeValue::L(..))
    }

    fn is_ss(&self) -> bool {
        matches!(self, AttributeValue::Ss(..))
    }

    fn is_ns(&self) -> bool {
        matches!(self, AttributeValue::Ns(..))
    }

    fn is_bs(&self) -> bool {
        matches!(self, AttributeValue::Bs(..))
    }

    fn as_n(&self) -> Option<&str> {
        if let AttributeValue::N(ref v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn as_s(&self) -> Option<&str> {
        if let AttributeValue::S(ref v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn as_bool(&self) -> Option<bool> {
        if let AttributeValue::Bool(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    fn as_b(&self) -> Option<&[u8]> {
        if let AttributeValue::B(ref v) = self {
            Some(v.as_ref())
        } else {
            None
        }
    }

    fn as_null(&self) -> Option<bool> {
        if let AttributeValue::Null(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    fn as_m(&self) -> Option<&HashMap<String, Self>> {
        if let AttributeValue::M(ref v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn as_l(&self) -> Option<&[Self]> {
        if let AttributeValue::L(ref v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn as_ss(&self) -> Option<&[String]> {
        if let AttributeValue::Ss(ref v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn as_ns(&self) -> Option<&[String]> {
        if let AttributeValue::Ns(ref v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn into_n(self) -> Option<String> {
        if let AttributeValue::N(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn into_s(self) -> Option<String> {
        if let AttributeValue::S(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn into_bool(self) -> Option<bool> {
        if let AttributeValue::Bool(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn into_b(self) -> Option<Vec<u8>> {
        if let AttributeValue::B(v) = self {
            Some(v.into_inner())
        } else {
            None
        }
    }

    fn into_null(self) -> Option<bool> {
        if let AttributeValue::Null(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn into_m(self) -> Option<HashMap<String, Self>> {
        if let AttributeValue::M(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn into_l(self) -> Option<Vec<Self>> {
        if let AttributeValue::L(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn into_ss(self) -> Option<Vec<String>> {
        if let AttributeValue::Ss(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn into_ns(self) -> Option<Vec<String>> {
        if let AttributeValue::Ns(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn into_bs(self) -> Option<Vec<Vec<u8>>> {
        if let AttributeValue::Bs(v) = self {
            Some(v.into_iter().map(|b| b.into_inner()).collect())
        } else {
            None
        }
    }

    fn construct_n(input: String) -> Self {
        AttributeValue::N(input)
    }

    fn construct_s(input: String) -> Self {
        AttributeValue::S(input)
    }

    fn construct_bool(input: bool) -> Self {
        AttributeValue::Bool(input)
    }

    fn construct_b(input: &[u8]) -> Self {
        AttributeValue::B(aws_sdk_dynamodb::Blob::new(input))
    }

    fn construct_null(input: bool) -> Self {
        AttributeValue::Null(input)
    }

    fn construct_m(input: HashMap<String, Self>) -> Self {
        AttributeValue::M(input)
    }

    fn construct_l(input: Vec<Self>) -> Self {
        AttributeValue::L(input)
    }
}

/// Interpret a [aws_sdk_dynamodb::model::AttributeValue] as an instance of type `T`.
///
/// In most cases, you will want to be using [`from_item`] instead. This function is provided as a
/// dual of [`to_attribute_value`] and may be useful in very narrow circumstances.
///
pub fn from_attribute_value<'a, T>(attribute_value: AttributeValue) -> Result<T>
where
    T: Deserialize<'a>,
{
    crate::generic::from_attribute_value(attribute_value)
}

/// Interpret an [`Item`] as an instance of type `T`.
///
/// ```
/// # use aws_sdk_dynamodb::{Client, model::AttributeValue};
/// # use serde::{Deserialize};
/// # use serde_dynamo::aws_sdk_dynamodb::{from_item};
/// #
/// # async fn get_item(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
/// #[derive(Deserialize)]
/// pub struct User {
///     id: String,
///     name: String,
///     age: u8,
/// };
///
/// // Get a document from DynamoDB
/// let result = client
///     .get_item()
///     .table_name("users")
///     .key("name", AttributeValue::S("ferris".into()))
///     .send()
///     .await?;
///
/// // And deserialize it as a strongly-typed data structure
/// if let Some(item) = result.item {
///    let user: User = from_item(item)?;
///    println!("Deserialized User!");
/// }
/// # Ok(())
/// # }
/// ```
pub fn from_item<'a, T>(item: Item) -> Result<T>
where
    T: Deserialize<'a>,
{
    crate::generic::from_item(item)
}

/// Interpret an `Vec<Item>` as `Vec<T>`.
///
/// ```
/// # use aws_sdk_dynamodb::{Client};
/// # use serde::{Deserialize};
/// # use serde_dynamo::aws_sdk_dynamodb::{from_items};
/// #
/// # async fn scan(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
/// #[derive(Deserialize)]
/// pub struct User {
///     id: String,
///     name: String,
///     age: u8,
/// };
///
/// // Get documents from DynamoDB
/// let result = client.scan().table_name("users").limit(10).send().await?;
///
/// // And deserialize them as strongly-typed data structures
/// if let Some(items) = result.items {
///     let users: Vec<User> = from_items(items)?;
///     println!("Got {} users", users.len());
/// }
/// # Ok(())
/// # }
/// ```
pub fn from_items<'a, T>(items: Vec<Item>) -> Result<Vec<T>>
where
    T: Deserialize<'a>,
{
    crate::generic::from_items(items)
}

/// TODO!
pub fn to_attribute_value<T>(value: T) -> Result<AttributeValue>
where
    T: Serialize,
{
    crate::generic::to_attribute_value(value)
}

/// TODO!
pub fn to_item<T>(value: T) -> Result<Item>
where
    T: Serialize,
{
    crate::generic::to_item(value)
}
