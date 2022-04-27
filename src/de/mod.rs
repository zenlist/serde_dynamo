use std::collections::HashMap;

use super::AttributeValue;
use crate::{error::ErrorImpl, Error, Item, Items, Result};
use serde::Deserialize;

mod deserializer;
mod deserializer_bytes;
mod deserializer_enum;
mod deserializer_map;
mod deserializer_number;
mod deserializer_seq;

#[cfg(test)]
mod tests;

pub use deserializer::Deserializer;

/// Interpret an [`AttributeValue`] as an instance of type `T`.
///
/// In most cases, you will want to be using [`from_item`] instead. This function is provided as a
/// dual of [`super::to_attribute_value`] and may be useful in very narrow circumstances.
pub fn from_attribute_value<'a, T>(attribute_value: AttributeValue) -> Result<T>
where
    T: Deserialize<'a>,
{
    let deserializer = Deserializer::from_attribute_value(attribute_value);
    T::deserialize(deserializer)
}

/// Interpret an [`Item`] as an instance of type `T`.
///
/// ```
/// # use __aws_sdk_dynamodb_0_10::client::Client;
/// # use serde_derive::{Serialize, Deserialize};
/// # use serde_dynamo::from_item;
/// #
/// # async fn scan(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
/// #[derive(Serialize, Deserialize)]
/// pub struct User {
///     id: String,
///     name: String,
///     age: u8,
/// };
///
/// // Get documents from DynamoDB
/// let result = client.scan().table_name("user").send().await?;
///
/// // And deserialize them as strongly-typed data structures
/// for item in result.items.unwrap() {
///     let user: User = from_item(item.into())?;
///     println!("{} is {}", user.name, user.age);
/// }
/// # Ok(())
/// # }
/// ```
pub fn from_item<'a, T>(item: Item) -> Result<T>
where
    T: Deserialize<'a>,
{
    let deserializer = Deserializer::from_attribute_value(AttributeValue::M(item.into()));
    T::deserialize(deserializer)
}

/// Interpret a [`Items`] as a `Vec<T>`.
///
/// ```
/// # use __aws_sdk_dynamodb_0_10::client::Client;
/// # use serde_derive::{Serialize, Deserialize};
/// # use serde_dynamo::from_items;
/// #
/// # async fn scan(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
/// #[derive(Serialize, Deserialize)]
/// pub struct User {
///     id: String,
///     name: String,
///     age: u8,
/// };
///
/// // Get documents from DynamoDB
/// let result = client.scan().table_name("user").send().await?;
///
/// // And deserialize them as strongly-typed data structures
/// if let Some(items) = result.items {
///     let users: Vec<User> = from_items(items.into())?;
///     println!("Got {} users", users.len());
/// }
/// # Ok(())
/// # }
/// ```
pub fn from_items<'a, T>(items: Items) -> Result<Vec<T>>
where
    T: Deserialize<'a>,
{
    let items = Vec::<HashMap<String, AttributeValue>>::from(items);
    let attribute_value = AttributeValue::L(items.into_iter().map(AttributeValue::M).collect());
    let deserializer = Deserializer::from_attribute_value(attribute_value);
    Vec::<T>::deserialize(deserializer)
}
