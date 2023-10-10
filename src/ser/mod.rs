use super::AttributeValue;
use crate::{error::ErrorImpl, Error, Item, Result};
use serde::Serialize;

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

/// Convert a `T` into an [`AttributeValue`].
///
/// In some circumstances, building [aws_sdk_dynamodb::model::AttributeValue]s directly is required.
///
/// For example, when generating a key to supply to [get_item].
///
/// ```no_run
/// use serde_dynamo::to_attribute_value;
/// # use __aws_sdk_dynamodb_0_33::client::Client;
/// # use std::collections::HashMap;
/// #
/// # async fn get(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
/// #
/// # struct User { id: String };
/// # let user = User { id: "fSsgVtal8TpP".to_string() };
///
/// // Create the unique key of the record in DynamoDB in a way rusoto understands
/// let key = HashMap::from([
///     (String::from("id"), to_attribute_value(&user.id)?),
/// ]);
///
/// // And get the record
/// client.get_item().table_name("users").set_key(Some(key)).send().await?;
/// # Ok(())
/// # }
/// ```
///
/// Or when generating attribute values in a [query] call.
///
/// ```no_run
/// use serde_dynamo::to_attribute_value;
/// # use __aws_sdk_dynamodb_0_33::client::Client;
/// # use std::collections::HashMap;
/// #
/// # async fn query(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
/// # let user_type = "user";
/// # let yesterday = "1985-04-21";
///
/// // Declare all of the expression inputs for a query call
/// let expression_attribute_values = HashMap::from([
///     (String::from(":user_type"), to_attribute_value(user_type)?),
///     (String::from(":last_login"), to_attribute_value(yesterday)?),
/// ]);
///
/// client.query()
///     .table_name("users")
///     .index_name("by_type_and_last_login")
///     .key_condition_expression("user_type = :user_type AND last_login > :last_login")
///     .set_expression_attribute_values(Some(expression_attribute_values))
///     .send()
///     .await?;
/// # Ok(())
/// # }
/// ```
///
/// [aws-sdk-dynamodb]: https://docs.rs/aws-sdk-dynamodb
/// [get_item]: https://docs.rs/aws-sdk-dynamodb/*/aws_sdk_dynamodb/client/struct.Client.html#method.get_item
/// [put_item]: https://docs.rs/aws-sdk-dynamodb/*/aws_sdk_dynamodb/client/struct.Client.html#method.put_item
/// [query]: https://docs.rs/aws-sdk-dynamodb/*/aws_sdk_dynamodb/client/struct.Client.html#method.query
/// [aws_sdk_dynamodb::model::AttributeValue]: https://docs.rs/aws-sdk-dynamodb/*/aws_sdk_dynamodb/types/enum.AttributeValue.html
pub fn to_attribute_value<T, AV>(value: T) -> Result<AV>
where
    T: Serialize,
    AV: From<AttributeValue>,
{
    let serializer = Serializer;
    let attribute_value = value.serialize(serializer)?;
    Ok(AV::from(attribute_value))
}

/// Convert a `T` into an [`Item`].
///
/// This is frequently used when serializing an entire data structure to be sent to DynamoDB.
///
/// ```no_run
/// # use __aws_sdk_dynamodb_0_33::client::Client;
/// # use serde_derive::{Serialize, Deserialize};
/// # use serde_dynamo::to_item;
/// #
/// # async fn put(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
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
/// client.put_item().table_name("users").set_item(Some(item)).send().await?;
/// # Ok(())
/// # }
/// ```
pub fn to_item<T, I>(value: T) -> Result<I>
where
    T: Serialize,
    I: From<Item>,
{
    let serializer = Serializer;
    let attribute_value = value.serialize(serializer)?;
    if let AttributeValue::M(item) = attribute_value {
        let item = Item::from(item);
        Ok(I::from(item))
    } else {
        Err(ErrorImpl::NotMaplike.into())
    }
}
