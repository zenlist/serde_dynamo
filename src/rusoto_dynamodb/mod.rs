//! TODO

extern crate rusoto_dynamodb as ext_rusoto_dynamodb;

use crate::Result;
use ext_rusoto_dynamodb::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An "Item" used in [rusoto_dynamodb]'s [get_item], [write_item], [put_item], etc.
///
/// Nowhere in rusoto_dynamodb is this type named explicitely, so we name it here to be clear about
/// exactly what is being taken in and being returned.
///
/// [rusoto_dynamodb]: https://docs.rs/rusoto_dynamodb
/// [get_item]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/trait.DynamoDb.html#tymethod.get_item
/// [write_item]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/trait.DynamoDb.html#tymethod.write_item
/// [put_item]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/trait.DynamoDb.html#tymethod.put_item
pub type Item = HashMap<String, AttributeValue>;

impl crate::generic::AttributeValue for AttributeValue {
    fn is_n(&self) -> bool {
        self.n.is_some()
    }

    fn is_s(&self) -> bool {
        self.s.is_some()
    }

    fn is_bool(&self) -> bool {
        self.bool.is_some()
    }

    fn is_b(&self) -> bool {
        self.b.is_some()
    }

    fn is_null(&self) -> bool {
        self.null.is_some()
    }

    fn is_m(&self) -> bool {
        self.m.is_some()
    }

    fn is_l(&self) -> bool {
        self.l.is_some()
    }

    fn is_ss(&self) -> bool {
        self.ss.is_some()
    }

    fn is_ns(&self) -> bool {
        self.ns.is_some()
    }

    fn is_bs(&self) -> bool {
        self.bs.is_some()
    }

    fn as_n(&self) -> Option<&str> {
        self.n.as_deref()
    }

    fn as_s(&self) -> Option<&str> {
        self.s.as_deref()
    }

    fn as_bool(&self) -> Option<bool> {
        self.bool
    }

    fn as_b(&self) -> Option<&[u8]> {
        self.b.as_deref()
    }

    fn as_null(&self) -> Option<bool> {
        self.null
    }

    fn as_m(&self) -> Option<&HashMap<String, Self>> {
        self.m.as_ref()
    }

    fn as_l(&self) -> Option<&[Self]> {
        self.l.as_deref()
    }

    fn as_ss(&self) -> Option<&[String]> {
        self.ss.as_deref()
    }

    fn as_ns(&self) -> Option<&[String]> {
        self.ns.as_deref()
    }

    fn into_n(self) -> Option<String> {
        self.n
    }

    fn into_s(self) -> Option<String> {
        self.s
    }

    fn into_bool(self) -> Option<bool> {
        self.bool
    }

    fn into_b(self) -> Option<Vec<u8>> {
        self.b.map(|b| b.to_vec())
    }

    fn into_null(self) -> Option<bool> {
        self.null
    }

    fn into_m(self) -> Option<HashMap<String, Self>> {
        self.m
    }

    fn into_l(self) -> Option<Vec<Self>> {
        self.l
    }

    fn into_ss(self) -> Option<Vec<String>> {
        self.ss
    }

    fn into_ns(self) -> Option<Vec<String>> {
        self.ns
    }

    fn into_bs(self) -> Option<Vec<Vec<u8>>> {
        self.bs
            .map(|bs| bs.into_iter().map(|b| b.to_vec()).collect())
    }

    fn construct_n(input: String) -> Self {
        AttributeValue {
            n: Some(input),
            ..AttributeValue::default()
        }
    }

    fn construct_s(input: String) -> Self {
        AttributeValue {
            s: Some(input),
            ..AttributeValue::default()
        }
    }

    fn construct_bool(input: bool) -> Self {
        AttributeValue {
            bool: Some(input),
            ..AttributeValue::default()
        }
    }

    fn construct_b(input: &[u8]) -> Self {
        AttributeValue {
            b: Some(input.to_vec().into()),
            ..AttributeValue::default()
        }
    }

    fn construct_null(input: bool) -> Self {
        AttributeValue {
            null: Some(input),
            ..AttributeValue::default()
        }
    }

    fn construct_m(input: HashMap<String, Self>) -> Self {
        AttributeValue {
            m: Some(input),
            ..AttributeValue::default()
        }
    }

    fn construct_l(input: Vec<Self>) -> Self {
        AttributeValue {
            l: Some(input),
            ..AttributeValue::default()
        }
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
/// ```
/// use maplit::hashmap;
/// use serde_dynamo::rusoto_dynamodb::to_attribute_value;
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
