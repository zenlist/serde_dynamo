#![deny(warnings)]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! [DynamoDB] is an AWS database that stores key/value and document data.
//!
//! **serde_dynamo** provides a way to serialize and deserialize between data stored in these
//! items and strongly-typed Rust data structures.
//!
//!
//! ## The full power of serde
//!
//! **serde_dynamo** supports the full power of [serde].
//!
//! Most uses of DynamoDB will involve simple structs mapping keys to values in type-safe ways.
//!
//! ```
//! # use serde_derive::{Serialize, Deserialize};
//! #
//! #[derive(Serialize, Deserialize)]
//! #[serde(transparent)]
//! pub struct UserId(String);
//!
//! #[derive(Serialize, Deserialize)]
//! pub struct User {
//!     id: UserId,
//!     name: String,
//!     age: u8,
//! }
//! ```
//!
//! More advanced usage – including [flattening], [adjacently tagged enums], and [untagged enums] –
//! is fully supported.
//!
//! ```
//! # use chrono::{DateTime, Utc};
//! # use serde_derive::{Serialize, Deserialize};
//! # use serde_dynamo::to_item;
//! # use std::collections::HashMap;
//! #
//! #[derive(Serialize, Deserialize)]
//! struct Message {
//!     id: String,
//!     #[serde(flatten)]
//!     message_type: MessageType,
//!     sent: DateTime<Utc>,
//! }
//!
//! /// What type of message this is.
//! ///
//! /// By the power of Rust enums and serde serializating, we can guarantee that we _either_ have
//! /// an email with all of its required fields, _or_ an SMS with all of its required fields.
//! #[derive(Serialize, Deserialize)]
//! #[serde(rename_all = "snake_case", tag = "message_type", content = "message_payload")]
//! enum MessageType {
//!     Email(Email),
//!     Sms(Sms),
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! struct Email {
//!     email: String,
//!     #[serde(skip_serializing_if = "Option::is_none")]
//!     name: Option<String>,
//!     subject: String,
//!     body: String,
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! #[serde(rename_all = "snake_case")]
//! struct Sms {
//!     phone_number: String,
//!     body: String,
//! }
//!
//! # fn test() -> Result<(), Box<dyn std::error::Error>> {
//! let input = r#"
//! {
//!     "id": "HWCqBFBG2Gl4",
//!     "message_type": "sms",
//!     "message_payload": {
//!         "phone_number": "5551234567",
//!         "body": "Good morning!"
//!     },
//!     "sent": "1985-04-21T11:12:13Z"
//! }
//! "#;
//! let message: Message = serde_json::from_str(input)?;
//! let item = to_item(message)?;
//! # Ok(())
//! # }
//! # test().unwrap()
//! ```
//!
//! ## aws-sdk support
//!
//! **serde_dynamo** works well with [aws-sdk-dynamodb].
//!
//! Add the following to your dependencies.
//!
//! ```toml
//! [dependencies]
//! serde_dynamo = { version = "3", features = ["aws-sdk-dynamodb+0_10"] }
//! ```
//!
//! See [`__aws_sdk_dynamodb_0_10`] for examples and more information.
//!
//!
//! **serde_dynamo** works well with [aws-sdk-dynamodbstreams].
//!
//! Add the following to your dependencies.
//!
//! ```toml
//! [dependencies]
//! serde_dynamo = { version = "3", features = ["aws-sdk-dynamodbstreams+0_8"] }
//! ```
//!
//! See [`__aws_sdk_dynamodbstreams_0_8`] for examples and more information.
//!
//!
//! ## rusoto support
//!
//! **serde_dynamo** works well with [rusoto_dynamodb].
//!
//! Add the following to your dependencies.
//!
//! ```toml
//! [dependencies]
//! serde_dynamo = { version = "3", features = ["rusoto_dynamodb+0.47"] }
//! ```
//!
//! See [`__rusoto_dynamodb_0_47`] for examples and more information.
//!
//!
//! ## JSON
//!
//! DynamoDB's items share strong similarities with JSON, and it is very common to store JSON data
//! in DynamoDB either directly or as a subfield.
//!
//! To support this, **serde_dynamo** supports serializing JSON just like any other Rust data
//! structure.
//!
//! ```
//! # use serde_derive::{Serialize, Deserialize};
//! #
//! #[derive(Serialize, Deserialize)]
//! struct IncludesJson {
//!     id: String,
//!     data: serde_json::Value,
//! }
//! ```
//!
//! In addition, **serde_dynamo** also maps strongly-typed data structures nearly identically as
//! [serde_json]. This means that, in almost all cases, serializing to JSON first and then to an
//! DynamoDb item will result in the exact same representation as serializing directly to a DynamoDb
//! item. (The caveat here is for byte data, which loses fidelity because JSON doesn't support byte
//! data natively, but DynamoDB does.)
//!
//! ```
//! # use serde_dynamo::to_item;
//! # use serde_derive::{Serialize, Deserialize};
//! # use std::collections::HashMap;
//! #
//! # #[derive(Clone, Serialize, Deserialize)]
//! # struct User {
//! #   name: String,
//! #   age: u8,
//! # }
//! #
//! # fn equivalence() -> Result<(), Box<dyn std::error::Error>> {
//! let user = User {
//!     name: "Arthur Dent".to_string(),
//!     age: 42,
//! };
//!
//! // Serialize directly from the data structure to an item
//! let direct_item = to_item(user.clone())?;
//!
//! // Serialize indirectly through JSON
//! let json = serde_json::to_value(user.clone())?;
//! let indirect_item = to_item(json)?;
//!
//! // The result should be the same!
//! assert_eq!(direct_item, indirect_item);
//! # Ok(())
//! # }
//! ```
//!
//! [DynamoDB]: https://aws.amazon.com/dynamodb/
//! [serde]: https://docs.rs/serde
//! [serde_json]: https://docs.rs/serde_json
//! [flattening]: https://serde.rs/attr-flatten.html
//! [adjacently tagged enums]: https://serde.rs/enum-representations.html#adjacently-tagged
//! [untagged enums]: https://serde.rs/enum-representations.html#untagged
//! [aws-sdk-dynamodb]: https://docs.rs/aws-sdk-dynamodb
//! [aws-sdk-dynamodbstreams]: https://docs.rs/aws-sdk-dynamodbstreams
//! [rusoto_dynamodb]: https://docs.rs/rusoto_dynamodb

mod de;
mod error;
mod ser;

pub use de::{from_attribute_value, from_item, from_items, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_attribute_value, to_item, Serializer};
#[cfg(test)]
mod tests;
