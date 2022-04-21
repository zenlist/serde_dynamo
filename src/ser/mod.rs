use crate::{error::ErrorImpl, Error, Result};
use aws_sdk_dynamodb::model::AttributeValue;
use serde::Serialize;
use std::collections::HashMap;

mod serializer;
mod serializer_map;
mod serializer_seq;
mod serializer_struct;
mod serializer_struct_variant;
mod serializer_tuple_variant;

#[cfg(test)]
mod tests;

pub use serializer::Serializer;
use serializer_map::SerializerMap;
use serializer_seq::SerializerSeq;
use serializer_struct::SerializerStruct;
use serializer_struct_variant::SerializerStructVariant;
use serializer_tuple_variant::SerializerTupleVariant;

/// Convert a `T` into an [`AttributeValue`] which is rusoto's representation of a
/// DynamoDb value.
///
/// In some circumstances, building [`AttributeValue`]s directly is required.
///
/// For example, when generating a key to supply to [get_item].
///
/// TODO
/// ```no_check
/// use maplit::hashmap;
/// use serde_dynamo::to_attribute_value;
/// # use rusoto_dynamodb::{DynamoDb, DynamoDbClient, GetItemInput};
/// #
/// # async fn get(client: &DynamoDbClient) -> Result<(), Box<dyn std::error::Error>> {
/// #
/// # struct User { id: String };
/// # let user = User { id: "fSsgVtal8TpP".to_string() };
///
/// // Create the unique key of the record in DynamoDB in a way rusoto understands
/// let key = hashmap! {
///     "id".into() => to_attribute_value(&user.id)?,
/// };
///
/// // And get the record
/// let input = GetItemInput {
///     table_name: "users".to_string(),
///     key: key,
///     ..GetItemInput::default()
/// };
/// client.get_item(input).await?;
/// # Ok(())
/// # }
/// ```
///
/// Or when generating attribute values in a [query] call.
///
/// TODO
/// ```no_check
/// use maplit::hashmap;
/// use serde_dynamo::to_attribute_value;
/// # use rusoto_dynamodb::{DynamoDb, DynamoDbClient, QueryInput};
/// #
/// # async fn query(client: &DynamoDbClient) -> Result<(), Box<dyn std::error::Error>> {
/// # let user_type = "user";
/// # let yesterday = "1985-04-21";
///
/// // Declare all of the expression inputs for a query call
/// let expression_attribute_values = hashmap! {
///     ":user_type".to_string() => to_attribute_value(user_type)?,
///     ":last_login".to_string() => to_attribute_value(yesterday)?,
/// };
///
/// // And execute the query
/// let input = QueryInput {
///     table_name: "users".to_string(),
///     index_name: Some("by_type_and_last_login".to_string()),
///     key_condition_expression: Some("user_type = :user_type AND last_login > :last_login".to_string()),
///     expression_attribute_values: Some(expression_attribute_values),
///     ..QueryInput::default()
/// };
/// client.query(input).await?;
/// # Ok(())
/// # }
/// ```
///
/// [rusoto_dynamodb::AttributeValue]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/struct.AttributeValue.html
/// [get_item]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/trait.DynamoDb.html#tymethod.get_item
/// [query]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/trait.DynamoDb.html#tymethod.get_item
pub fn to_attribute_value<T>(value: T) -> Result<AttributeValue>
where
    T: Serialize,
{
    let serializer = Serializer::default();
    let attribute_value = value.serialize(serializer)?;
    Ok(attribute_value)
}

/// Convert a `T` into an `Item`.
///
/// This is frequently used when serializing an entire data structure to be sent to DynamoDB.
///
/// TODO
/// ```no_check
/// # use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput};
/// # use serde::{Serialize, Deserialize};
/// # use serde_dynamo::to_item;
/// #
/// # async fn put(client: &DynamoDbClient) -> Result<(), Box<dyn std::error::Error>> {
/// #[derive(Serialize, Deserialize)]
/// pub struct User {
///     id: String,
///     name: String,
///     age: u8,
/// };
///
/// // Create a user
/// let user = User {
///     id: "fSsgVtal8TpP".to_string(),
///     name: "Arthur Dent".to_string(),
///     age: 42,
/// };
///
/// // Turn it into an item that rusoto understands
/// let item = to_item(user)?;
///
/// // And write it!
/// let input = PutItemInput {
///     table_name: "users".to_string(),
///     item: item,
///     ..PutItemInput::default()
/// };
/// client.put_item(input).await?;
/// # Ok(())
/// # }
/// ```
///
/// [rusoto_dynamodb]: https://docs.rs/rusoto_dynamodb
pub fn to_item<T>(value: T) -> Result<HashMap<String, AttributeValue>>
where
    T: Serialize,
{
    let attribute_value = to_attribute_value(value)?;
    if let AttributeValue::M(m) = attribute_value {
        Ok(m)
    } else {
        Err(ErrorImpl::NotMaplike.into())
    }
}
