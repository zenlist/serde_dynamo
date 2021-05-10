//! TODO

extern crate aws_sdk_dynamodb as ext_aws_sdk_dynamodb;

use crate::Result;
use ext_aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An "Item" used in [aws-sdk-dynamodb]'s [get_item], [write_item], [put_item], etc.
///
/// Nowhere in rusoto_dynamodb is this type named explicitely, so we name it here to be clear about
/// exactly what is being taken in and being returned.
///
/// [aws-sdk-dynamodb]: https://github.com/awslabs/aws-sdk-rust
/// [get_item]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/trait.DynamoDb.html#tymethod.get_item
/// [write_item]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/trait.DynamoDb.html#tymethod.write_item
/// [put_item]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/trait.DynamoDb.html#tymethod.put_item
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
            Some(v.as_ref().to_vec())
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
            Some(v.into_iter().map(|b| b.as_ref().to_vec()).collect())
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

/// Interpret a [rusoto_dynamodb::AttributeValue] as an instance of type `T`.
///
/// In most cases, you will want to be using [`from_item`] instead. This function is provided as a
/// dual of [`to_attribute_value`] and may be useful in very narrow circumstances.
///
/// [rusoto_dynamodb::AttributeValue]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/struct.AttributeValue.html
pub fn from_attribute_value<'a, T>(attribute_value: AttributeValue) -> Result<T>
where
    T: Deserialize<'a>,
{
    crate::generic::from_attribute_value(attribute_value)
}

/// Interpret an [`Item`] as an instance of type `T`.
///
/// ```
/// # use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ScanInput};
/// # use serde::{Serialize, Deserialize};
/// # use serde_dynamo::rusoto_dynamodb::from_item;
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
    crate::generic::from_item(item)
}

/// Interpret an `Vec<Item>` as `Vec<T>`.
///
/// ```
/// # use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ScanInput};
/// # use serde::{Serialize, Deserialize};
/// # use serde_dynamo::rusoto_dynamodb::from_items;
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
    crate::generic::from_items(items)
}

/// Convert a `T` into a [rusoto_dynamodb::AttributeValue] which is rusoto's representation of a
/// DynamoDb value.
///
/// In some circumstances, building [rusoto_dynamodb::AttributeValue]s directly is required.
///
/// For example, when generating a key to supply to [get_item].
///
/// ```
/// use maplit::hashmap;
/// use serde_dynamo::rusoto_dynamodb::to_attribute_value;
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
    crate::generic::to_attribute_value(value)
}

/// Convert a `T` into an [`Item`] which is [rusoto_dynamodb]'s representation of a DynamoDb item.
///
/// This is frequently used when serializing an entire data structure to be sent to DynamoDB.
///
/// ```
/// # use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput};
/// # use serde::{Serialize, Deserialize};
/// # use serde_dynamo::rusoto_dynamodb::to_item;
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
pub fn to_item<T>(value: T) -> Result<Item>
where
    T: Serialize,
{
    crate::generic::to_item(value)
}
