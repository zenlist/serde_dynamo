#![deny(warnings, missing_docs)]

//! TODO

// //! [DynamoDB] is an AWS database that stores key/value and document data.
// //!
// //! The most common way to access DynamoDB data from Rust is to use
// //! [rusoto_dynamodb]'s [get_item], [put_item], and related methods.
// //!
// //! **serde_dynamo** provides a way to serialize and deserialize between data stored in these
// //! [`Item`]s and strongly-typed Rust data structures.
// //!
// //!
// //! ## The full power of serde
// //!
// //! **serde_dynamo** supports the full power of [serde].
// //!
// //! Most uses of DynamoDB will involve simple structs mapping keys to values in type-safe ways.
// //!
// //! ```
// //! # use serde::{Serialize, Deserialize};
// //! #
// //! #[derive(Serialize, Deserialize)]
// //! pub struct User {
// //!     id: String,
// //!     name: String,
// //!     age: u8,
// //! }
// //! ```
// //!
// //! More advanced usage – including [flattening], [adjacently tagged enums], and [untagged enums] –
// //! is fully supported.
// //!
// //! ```
// //! # use chrono::{DateTime, Utc};
// //! # use serde::{Serialize, Deserialize};
// //! # use serde_dynamo::to_item;
// //! #
// //! #[derive(Serialize, Deserialize)]
// //! struct Message {
// //!     id: String,
// //!     #[serde(flatten)]
// //!     message_type: MessageType,
// //!     sent: DateTime<Utc>,
// //! }
// //!
// //! /// What type of message this is.
// //! ///
// //! /// By the power of Rust enums and serde serializating, we can guarantee that we _either_ have
// //! /// an email with all of its required fields, _or_ an SMS with all of its required fields.
// //! #[derive(Serialize, Deserialize)]
// //! #[serde(rename_all = "snake_case", tag = "message_type", content = "message_payload")]
// //! enum MessageType {
// //!     Email(Email),
// //!     Sms(Sms),
// //! }
// //!
// //! #[derive(Serialize, Deserialize)]
// //! struct Email {
// //!     email: String,
// //!     #[serde(skip_serializing_if = "Option::is_none")]
// //!     name: Option<String>,
// //!     subject: String,
// //!     body: String,
// //! }
// //!
// //! #[derive(Serialize, Deserialize)]
// //! #[serde(rename_all = "snake_case")]
// //! struct Sms {
// //!     phone_number: String,
// //!     body: String,
// //! }
// //!
// //! # fn test() -> Result<(), Box<dyn std::error::Error>> {
// //! let input = r#"
// //! {
// //!     "id": "HWCqBFBG2Gl4",
// //!     "message_type": "sms",
// //!     "message_payload": {
// //!         "phone_number": "5551234567",
// //!         "body": "Good morning!"
// //!     },
// //!     "sent": "1985-04-21T11:12:13Z"
// //! }
// //! "#;
// //! let message: Message = serde_json::from_str(input)?;
// //! let item = to_item(message)?;
// //! # Ok(())
// //! # }
// //! # test().unwrap()
// //! ```
// //!
// //!
// //! ## Parsing items as strongly-typed data structures.
// //!
// //! [`Item`]s received from a [rusoto_dynamodb] call can be run through [`from_items`].
// //!
// //! ```
// //! # use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ScanInput};
// //! # use serde::{Serialize, Deserialize};
// //! # use serde_dynamo::from_items;
// //! #
// //! # async fn scan(client: &DynamoDbClient) -> Result<(), Box<dyn std::error::Error>> {
// //! #[derive(Serialize, Deserialize)]
// //! pub struct User {
// //!     id: String,
// //!     name: String,
// //!     age: u8,
// //! };
// //!
// //! // Get documents from DynamoDB
// //! let input = ScanInput {
// //!     table_name: "users".to_string(),
// //!     ..ScanInput::default()
// //! };
// //! let result = client.scan(input).await?;
// //!
// //! // And deserialize them as strongly-typed data structures
// //! if let Some(items) = result.items {
// //!     let users: Vec<User> = from_items(items)?;
// //!     println!("Got {} users", users.len());
// //! }
// //! # Ok(())
// //! # }
// //! ```
// //!
// //! Alternatively, to deserialize one item at a time, [`from_item`] can be used.
// //!
// //! ```
// //! # use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ScanInput};
// //! # use serde::{Serialize, Deserialize};
// //! # use serde_dynamo::from_item;
// //! #
// //! # async fn scan(client: &DynamoDbClient) -> Result<(), Box<dyn std::error::Error>> {
// //! #[derive(Serialize, Deserialize)]
// //! pub struct User {
// //!     id: String,
// //!     name: String,
// //!     age: u8,
// //! };
// //!
// //! // Get documents from DynamoDB
// //! let input = ScanInput {
// //!     table_name: "users".to_string(),
// //!     ..ScanInput::default()
// //! };
// //! let result = client.scan(input).await?;
// //!
// //! // And deserialize them as strongly-typed data structures
// //! for item in result.items.unwrap() {
// //!     let user: User = from_item(item)?;
// //!     println!("{} is {}", user.name, user.age);
// //! }
// //! # Ok(())
// //! # }
// //! ```
// //!
// //!
// //! ## Creating items by serializing data structures
// //!
// //! Writing an entire data structure to DynamoDB typically involves using [`to_item`] to serialize
// //! it.
// //!
// //! ```
// //! # use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput};
// //! # use serde::{Serialize, Deserialize};
// //! # use serde_dynamo::to_item;
// //! #
// //! # async fn put(client: &DynamoDbClient) -> Result<(), Box<dyn std::error::Error>> {
// //! #[derive(Serialize, Deserialize)]
// //! pub struct User {
// //!     id: String,
// //!     name: String,
// //!     age: u8,
// //! };
// //!
// //! // Create a user
// //! let user = User {
// //!     id: "fSsgVtal8TpP".to_string(),
// //!     name: "Arthur Dent".to_string(),
// //!     age: 42,
// //! };
// //!
// //! // Turn it into an item that rusoto understands
// //! let item = to_item(user)?;
// //!
// //! // And write it!
// //! let input = PutItemInput {
// //!     table_name: "users".to_string(),
// //!     item: item,
// //!     ..PutItemInput::default()
// //! };
// //! client.put_item(input).await?;
// //! # Ok(())
// //! # }
// //! ```
// //!
// //!
// //! ## Using to_attribute_value for more control
// //!
// //! In some circumstances, building [rusoto_dynamodb::AttributeValue]s directly is required.
// //!
// //! For example, when generating a key to supply to [get_item].
// //!
// //! ```
// //! use maplit::hashmap;
// //! use serde_dynamo::to_attribute_value;
// //! # use rusoto_dynamodb::{DynamoDb, DynamoDbClient, GetItemInput};
// //! #
// //! # async fn get(client: &DynamoDbClient) -> Result<(), Box<dyn std::error::Error>> {
// //! #
// //! # struct User { id: String };
// //! # let user = User { id: "fSsgVtal8TpP".to_string() };
// //!
// //! // Create the unique key of the record in DynamoDB in a way rusoto understands
// //! let key = hashmap! {
// //!     "id".into() => to_attribute_value(&user.id)?,
// //! };
// //!
// //! // And get the record
// //! let input = GetItemInput {
// //!     table_name: "users".to_string(),
// //!     key: key,
// //!     ..GetItemInput::default()
// //! };
// //! client.get_item(input).await?;
// //! # Ok(())
// //! # }
// //! ```
// //!
// //! Or when generating attribute values in a [query] call.
// //!
// //! ```
// //! use maplit::hashmap;
// //! use serde_dynamo::to_attribute_value;
// //! # use rusoto_dynamodb::{DynamoDb, DynamoDbClient, QueryInput};
// //! #
// //! # async fn query(client: &DynamoDbClient) -> Result<(), Box<dyn std::error::Error>> {
// //! # let user_type = "user";
// //! # let yesterday = "1985-04-21";
// //!
// //! // Declare all of the expression inputs for a query call
// //! let expression_attribute_values = hashmap! {
// //!     ":user_type".to_string() => to_attribute_value(user_type)?,
// //!     ":last_login".to_string() => to_attribute_value(yesterday)?,
// //! };
// //!
// //! // And execute the query
// //! let input = QueryInput {
// //!     table_name: "users".to_string(),
// //!     index_name: Some("by_type_and_last_login".to_string()),
// //!     key_condition_expression: Some("user_type = :user_type AND last_login > :last_login".to_string()),
// //!     expression_attribute_values: Some(expression_attribute_values),
// //!     ..QueryInput::default()
// //! };
// //! client.query(input).await?;
// //! # Ok(())
// //! # }
// //! ```
// //!
// //! ## JSON
// //!
// //! DynamoDB's items share strong similarities with JSON, and it is very common to store JSON data
// //! in DynamoDB either directly or as a subfield.
// //!
// //! To support this, **serde_dynamo** supports serializing JSON just like any other Rust data
// //! structure.
// //!
// //! ```
// //! # use serde::{Serialize, Deserialize};
// //! #
// //! #[derive(Serialize, Deserialize)]
// //! struct IncludesJson {
// //!     id: String,
// //!     data: serde_json::Value,
// //! }
// //! ```
// //!
// //! In addition, **serde_dynamo** also maps strongly-typed data structures nearly identically as
// //! [serde_json]. This means that, in almost all cases, serializing to JSON first and then to an
// //! [`Item`] will result in the exact same representation as serializing directly to an `Item`.
// //! (The caveat here is for byte data, which loses fidelity because JSON doesn't support byte data
// //! natively, but DynamoDB does.)
// //!
// //! ```
// //! # use serde_dynamo::to_item;
// //! # use serde::{Serialize, Deserialize};
// //! #
// //! # #[derive(Clone, Serialize, Deserialize)]
// //! # struct User {
// //! #   name: String,
// //! #   age: u8,
// //! # }
// //! #
// //! # fn equivalence() -> Result<(), Box<dyn std::error::Error>> {
// //! let user = User {
// //!     name: "Arthur Dent".to_string(),
// //!     age: 42,
// //! };
// //!
// //! // Serialize directly from the data structure to an item
// //! let direct_item = to_item(user.clone())?;
// //!
// //! // Serialize indirectly through JSON
// //! let json = serde_json::to_value(user.clone())?;
// //! let indirect_item = to_item(json)?;
// //!
// //! // The result should be the same!
// //! assert_eq!(direct_item, indirect_item);
// //! # Ok(())
// //! # }
// //! ```
// //!
// //! [DynamoDB]: https://aws.amazon.com/dynamodb/
// //! [rusoto_dynamodb]: https://docs.rs/rusoto_dynamodb
// //! [get_item]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/trait.DynamoDb.html#tymethod.get_item
// //! [put_item]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/trait.DynamoDb.html#tymethod.put_item
// //! [query]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/trait.DynamoDb.html#tymethod.query
// //! [serde]: https://docs.rs/serde
// //! [serde_json]: https://docs.rs/serde_json
// //! [flattening]: https://serde.rs/attr-flatten.html
// //! [adjacently tagged enums]: https://serde.rs/enum-representations.html#adjacently-tagged
// //! [untagged enums]: https://serde.rs/enum-representations.html#untagged
// //! [rusoto_dynamodb::AttributeValue]: https://docs.rs/rusoto_dynamodb/0.45.0/rusoto_dynamodb/struct.AttributeValue.html

// use rusoto_dynamodb::AttributeValue;
// use std::collections::HashMap;

// mod de;
mod error;
// mod ser;

pub use error::{Error, Result};

pub mod aws_sdk_dynamodb;
pub mod generic;
pub mod rusoto_dynamodb;

// use error::ErrorImpl;

// #[cfg(test)]
// mod tests;
