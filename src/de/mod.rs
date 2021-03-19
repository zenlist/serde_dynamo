use super::{Error, ErrorImpl, Item, Result};
use rusoto_dynamodb::AttributeValue;
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

/// Interpret a [rusoto_dynamodb::AttributeValue] as an instance of type `T`.
///
/// In most cases, you will want to be using [`from_item`] instead. This function is provided as a
/// dual of [`to_attribute_value`](crate::to_attribute_value) and may be useful in very narrow circumstances.
///
/// [rusoto_dynamodb::AttributeValue]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/struct.AttributeValue.html
pub fn from_attribute_value<'a, T>(attribute_value: AttributeValue) -> Result<T>
where
    T: Deserialize<'a>,
{
    let deserializer = Deserializer::from_attribute_value(attribute_value);
    let t = T::deserialize(deserializer)?;
    Ok(t)
}

/// Interpret an [`Item`] as an instance of type `T`.
///
/// ```
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
pub fn from_item<'a, T>(item: Item) -> Result<T>
where
    T: Deserialize<'a>,
{
    let attribute_value = AttributeValue {
        m: Some(item),
        ..AttributeValue::default()
    };
    let deserializer = Deserializer::from_attribute_value(attribute_value);
    let t = T::deserialize(deserializer)?;
    Ok(t)
}

/// Interpret an `Vec<Item>` as `Vec<T>`.
///
/// ```
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
pub fn from_items<'a, T>(items: Vec<Item>) -> Result<Vec<T>>
where
    T: Deserialize<'a>,
{
    let attribute_value = AttributeValue {
        l: Some(
            items
                .into_iter()
                .map(|item| AttributeValue {
                    m: Some(item),
                    ..AttributeValue::default()
                })
                .collect(),
        ),
        ..AttributeValue::default()
    };
    let deserializer = Deserializer::from_attribute_value(attribute_value);
    let t = Vec::<T>::deserialize(deserializer)?;
    Ok(t)
}
