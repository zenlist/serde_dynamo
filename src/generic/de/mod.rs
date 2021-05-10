use super::AttributeValue;
use crate::{error::ErrorImpl, Error, Result};
use serde::Deserialize;
use std::collections::HashMap;

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
pub fn from_attribute_value<'a, Tin, Tout>(attribute_value: Tin) -> Result<Tout>
where
    Tin: AttributeValue,
    Tout: Deserialize<'a>,
{
    let deserializer = Deserializer::from_attribute_value(attribute_value);
    let t = Tout::deserialize(deserializer)?;
    Ok(t)
}

/// Interpret an Item – a hashmap from [`String`] to [`AttributeValue`] – as an instance of type `T`.
///
/// TODO
/// ```no_check
/// # use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ScanInput};
/// # use serde::{Serialize, Deserialize};
/// # use serde_dynamo::from_item;
/// #
/// # async fn scan(client: &DynamoDbClient) -> Result<(), Box<dyn std::error::Error>> {
/// #[derive(Serialize, Deserialize)]
/// pub struct User {
///     id: String,
///     name: String,
///     age: u8,
/// };
///
/// // Get documents from DynamoDB
/// let input = ScanInput {
///     table_name: "users".to_string(),
///     ..ScanInput::default()
/// };
/// let result = client.scan(input).await?;
///
/// // And deserialize them as strongly-typed data structures
/// for item in result.items.unwrap() {
///     let user: User = from_item(item)?;
///     println!("{} is {}", user.name, user.age);
/// }
/// # Ok(())
/// # }
/// ```
pub fn from_item<'a, Tin, Tout>(item: HashMap<String, Tin>) -> Result<Tout>
where
    Tin: AttributeValue,
    Tout: Deserialize<'a>,
{
    let attribute_value = AttributeValue::construct_m(item);
    let deserializer = Deserializer::from_attribute_value(attribute_value);
    let t = Tout::deserialize(deserializer)?;
    Ok(t)
}

/// Interpret an `Vec<Item>` as `Vec<T>`.
///
/// TODO
/// ```no_check
/// # use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ScanInput};
/// # use serde::{Serialize, Deserialize};
/// # use serde_dynamo::from_items;
/// #
/// # async fn scan(client: &DynamoDbClient) -> Result<(), Box<dyn std::error::Error>> {
/// #[derive(Serialize, Deserialize)]
/// pub struct User {
///     id: String,
///     name: String,
///     age: u8,
/// };
///
/// // Get documents from DynamoDB
/// let input = ScanInput {
///     table_name: "users".to_string(),
///     ..ScanInput::default()
/// };
/// let result = client.scan(input).await?;
///
/// // And deserialize them as strongly-typed data structures
/// if let Some(items) = result.items {
///     let users: Vec<User> = from_items(items)?;
///     println!("Got {} users", users.len());
/// }
/// # Ok(())
/// # }
/// ```
pub fn from_items<'a, Tin, Tout>(items: Vec<HashMap<String, Tin>>) -> Result<Vec<Tout>>
where
    Tin: AttributeValue,
    Tout: Deserialize<'a>,
{
    let attribute_value = Tin::construct_l(items.into_iter().map(Tin::construct_m).collect());
    let deserializer = Deserializer::from_attribute_value(attribute_value);
    let t = Vec::<Tout>::deserialize(deserializer)?;
    Ok(t)
}
