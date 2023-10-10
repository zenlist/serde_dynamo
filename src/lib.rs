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
//! # use serde_dynamo::AttributeValue;
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
//! # let item: serde_dynamo::Item = item;
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
//! serde_dynamo = { version = "4", features = ["aws-sdk-dynamodb+0_33"] }
//! ```
//!
//! See [`aws_sdk_dynamodb_0_33`] for examples and more information. See
//! [`aws_sdk_dynamodbstreams_0_33`] for DynamoDb streams support.
//!
//! ## aws_lambda_events support
//!
//! [aws_lambda_events], starting with version 0.8, uses **serde_dynamo** directly, so no feature
//! flags need to be enabled.
//!
//! ```toml
//! [dependencies]
//! serde_dynamo = { version = "4" }
//! ```
//!
//! ## rusoto support
//!
//! **serde_dynamo** works well with [rusoto_dynamodb].
//!
//! Add the following to your dependencies.
//!
//! ```toml
//! [dependencies]
//! serde_dynamo = { version = "4", features = ["rusoto_dynamodb+0.48"] }
//! ```
//!
//! See [`rusoto_dynamodb_0_48`] for examples and more information.
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
//! # use serde_dynamo::AttributeValue;
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
//! # let direct_item: serde_dynamo::Item = direct_item;
//!
//! // Serialize indirectly through JSON
//! let json = serde_json::to_value(user.clone())?;
//! let indirect_item = to_item(json)?;
//! # let indirect_item: serde_dynamo::Item = indirect_item;
//!
//! // The result should be the same!
//! assert_eq!(direct_item, indirect_item);
//! # Ok(())
//! # }
//! ```
//!
//! ## Features
//!
//! **serde_dynamo** is a stable library ready to use in production. Because of that, it's major
//! version is above 1.0.
//!
//! This creates problems when supporting dynamodb libraries that have version numbers less than
//! 1.0.
//!
//! To avoid doing a major version bump for every release of `aws-sdk-dynamodb` and
//! `aws_lambda_events`, **serde_dynamo** uses features to opt into the correct version of the
//! dynamodb library.
//!
//! See the [modules](#modules) section for all possible features. Feature names are largely
//! guessable: the library name, a plus, and the library version (with underscores instead of dots,
//! because crates.io doesn't support feature names with dots). For example, support for
//! `aws-sdk-dynamodb` version `0.13` is enabled with the feature `aws-sdk-dynamodb+0_13`.
//!
//! ## Converting to and from DynamoDB JSON
//!
//! In most cases, libraries already exist to handle the raw DynamoDB JSON and convert it into an
//! item. For example, [aws-sdk-dynamodb] deals with the raw JSON if you're making API calls, and
//! [aws_lambda_events] deals with the raw JSON if you're writing lambdas that react on DynamoDB
//! change streams.
//!
//! However, in very rare cases, you may need to convert the DynamoDB JSON yourself. In those cases,
//! both [Item] and [AttributeValue] implement [serde::Serialize] and [serde::Deserialize].
//!
//! ```
//! # use serde_dynamo::{AttributeValue, Item};
//! let input = r#"{
//!     "Id":{
//!         "N":"103"
//!     },
//!     "Title":{
//!         "S":"Book 103 Title"
//!     },
//!     "Authors":{
//!         "SS":[
//!             "Author1",
//!             "Author2"
//!         ]
//!     },
//!     "InPublication":{
//!         "BOOL":false
//!     }
//! }"#;
//!
//! let item: Item = serde_json::from_str(input)
//!     .expect("expected to deserialize DynamoDB JSON format");
//!
//! assert_eq!(
//!     item.get("Id").unwrap(),
//!     &AttributeValue::N(String::from("103")),
//! );
//! ```
//!
//! [DynamoDB]: https://aws.amazon.com/dynamodb/
//! [serde]: https://docs.rs/serde
//! [serde_json]: https://docs.rs/serde_json
//! [flattening]: https://serde.rs/attr-flatten.html
//! [adjacently tagged enums]: https://serde.rs/enum-representations.html#adjacently-tagged
//! [untagged enums]: https://serde.rs/enum-representations.html#untagged
//! [aws-sdk-dynamodb]: https://docs.rs/aws-sdk-dynamodb
//! [aws_lambda_events]: https://docs.rs/aws_lambda_events
//! [rusoto_dynamodb]: https://docs.rs/rusoto_dynamodb

mod attribute_value;
mod de;
mod error;
mod macros;
mod ser;

pub mod binary_set;
pub mod number_set;
pub mod string_set;

pub use attribute_value::{AttributeValue, Item, Items};
pub use de::{from_attribute_value, from_item, from_items, Deserializer};
pub use error::{Error, Result};
use macros::{
    aws_lambda_events_macro, aws_sdk_macro, aws_sdk_streams_macro, rusoto_macro,
    rusoto_streams_macro,
};
pub use ser::{to_attribute_value, to_item, Serializer};

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_7",
    crate_name = __aws_sdk_dynamodb_0_7,
    mod_name = aws_sdk_dynamodb_0_7,
    attribute_value_path = ::__aws_sdk_dynamodb_0_7::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_7::types::Blob,
    aws_version = "0.7",
    config_version = "0.7",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_8",
    crate_name = __aws_sdk_dynamodb_0_8,
    mod_name = aws_sdk_dynamodb_0_8,
    attribute_value_path = ::__aws_sdk_dynamodb_0_8::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_8::types::Blob,
    aws_version = "0.8",
    config_version = "0.8",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_9",
    crate_name = __aws_sdk_dynamodb_0_9,
    mod_name = aws_sdk_dynamodb_0_9,
    attribute_value_path = ::__aws_sdk_dynamodb_0_9::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_9::types::Blob,
    aws_version = "0.9",
    config_version = "0.9",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_10",
    crate_name = __aws_sdk_dynamodb_0_10,
    mod_name = aws_sdk_dynamodb_0_10,
    attribute_value_path = ::__aws_sdk_dynamodb_0_10::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_10::types::Blob,
    aws_version = "0.10",
    config_version = "0.40",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_11",
    crate_name = __aws_sdk_dynamodb_0_11,
    mod_name = aws_sdk_dynamodb_0_11,
    attribute_value_path = ::__aws_sdk_dynamodb_0_11::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_11::types::Blob,
    aws_version = "0.11",
    config_version = "0.41",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_12",
    crate_name = __aws_sdk_dynamodb_0_12,
    mod_name = aws_sdk_dynamodb_0_12,
    attribute_value_path = ::__aws_sdk_dynamodb_0_12::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_12::types::Blob,
    aws_version = "0.12",
    config_version = "0.42",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_13",
    crate_name = __aws_sdk_dynamodb_0_13,
    mod_name = aws_sdk_dynamodb_0_13,
    attribute_value_path = ::__aws_sdk_dynamodb_0_13::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_13::types::Blob,
    aws_version = "0.13",
    config_version = "0.43",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_14",
    crate_name = __aws_sdk_dynamodb_0_14,
    mod_name = aws_sdk_dynamodb_0_14,
    attribute_value_path = ::__aws_sdk_dynamodb_0_14::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_14::types::Blob,
    aws_version = "0.14",
    config_version = "0.44",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_15",
    crate_name = __aws_sdk_dynamodb_0_15,
    mod_name = aws_sdk_dynamodb_0_15,
    attribute_value_path = ::__aws_sdk_dynamodb_0_15::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_15::types::Blob,
    aws_version = "0.15",
    config_version = "0.45",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_16",
    crate_name = __aws_sdk_dynamodb_0_16,
    mod_name = aws_sdk_dynamodb_0_16,
    attribute_value_path = ::__aws_sdk_dynamodb_0_16::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_16::types::Blob,
    aws_version = "0.16",
    config_version = "0.46",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_17",
    crate_name = __aws_sdk_dynamodb_0_17,
    mod_name = aws_sdk_dynamodb_0_17,
    attribute_value_path = ::__aws_sdk_dynamodb_0_17::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_17::types::Blob,
    aws_version = "0.17",
    config_version = "0.47",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_18",
    crate_name = __aws_sdk_dynamodb_0_18,
    mod_name = aws_sdk_dynamodb_0_18,
    attribute_value_path = ::__aws_sdk_dynamodb_0_18::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_18::types::Blob,
    aws_version = "0.18",
    config_version = "0.48",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_19",
    crate_name = __aws_sdk_dynamodb_0_19,
    mod_name = aws_sdk_dynamodb_0_19,
    attribute_value_path = ::__aws_sdk_dynamodb_0_19::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_19::types::Blob,
    aws_version = "0.19",
    config_version = "0.49",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_21",
    crate_name = __aws_sdk_dynamodb_0_21,
    mod_name = aws_sdk_dynamodb_0_21,
    attribute_value_path = ::__aws_sdk_dynamodb_0_21::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_21::types::Blob,
    aws_version = "0.21",
    config_version = "0.51",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_22",
    crate_name = __aws_sdk_dynamodb_0_22,
    mod_name = aws_sdk_dynamodb_0_22,
    attribute_value_path = ::__aws_sdk_dynamodb_0_22::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_22::types::Blob,
    aws_version = "0.22",
    config_version = "0.52",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_23",
    crate_name = __aws_sdk_dynamodb_0_23,
    mod_name = aws_sdk_dynamodb_0_23,
    attribute_value_path = ::__aws_sdk_dynamodb_0_23::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_23::types::Blob,
    aws_version = "0.23",
    config_version = "0.53",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_24",
    crate_name = __aws_sdk_dynamodb_0_24,
    mod_name = aws_sdk_dynamodb_0_24,
    attribute_value_path = ::__aws_sdk_dynamodb_0_24::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_24::types::Blob,
    aws_version = "0.24",
    config_version = "0.54",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_25",
    crate_name = __aws_sdk_dynamodb_0_25,
    mod_name = aws_sdk_dynamodb_0_25,
    attribute_value_path = ::__aws_sdk_dynamodb_0_25::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_25::primitives::Blob,
    aws_version = "0.25",
    config_version = "0.55",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_26",
    crate_name = __aws_sdk_dynamodb_0_26,
    mod_name = aws_sdk_dynamodb_0_26,
    attribute_value_path = ::__aws_sdk_dynamodb_0_26::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_26::primitives::Blob,
    aws_version = "0.26",
    config_version = "0.55",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_27",
    crate_name = __aws_sdk_dynamodb_0_27,
    mod_name = aws_sdk_dynamodb_0_27,
    attribute_value_path = ::__aws_sdk_dynamodb_0_27::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_27::primitives::Blob,
    aws_version = "0.27",
    config_version = "0.55",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_28",
    crate_name = __aws_sdk_dynamodb_0_28,
    mod_name = aws_sdk_dynamodb_0_28,
    attribute_value_path = ::__aws_sdk_dynamodb_0_28::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_28::primitives::Blob,
    aws_version = "0.28",
    config_version = "0.55",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_29",
    crate_name = __aws_sdk_dynamodb_0_29,
    mod_name = aws_sdk_dynamodb_0_29,
    attribute_value_path = ::__aws_sdk_dynamodb_0_29::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_29::primitives::Blob,
    aws_version = "0.29",
    config_version = "0.56",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_30",
    crate_name = __aws_sdk_dynamodb_0_30,
    mod_name = aws_sdk_dynamodb_0_30,
    attribute_value_path = ::__aws_sdk_dynamodb_0_30::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_30::primitives::Blob,
    aws_version = "0.30",
    config_version = "0.56",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_31",
    crate_name = __aws_sdk_dynamodb_0_31,
    mod_name = aws_sdk_dynamodb_0_31,
    attribute_value_path = ::__aws_sdk_dynamodb_0_31::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_31::primitives::Blob,
    aws_version = "0.31",
    config_version = "0.56",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_32",
    crate_name = __aws_sdk_dynamodb_0_32,
    mod_name = aws_sdk_dynamodb_0_32,
    attribute_value_path = ::__aws_sdk_dynamodb_0_32::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_32::primitives::Blob,
    aws_version = "0.32",
    config_version = "0.56",
);

aws_sdk_macro!(
    feature = "aws-sdk-dynamodb+0_33",
    crate_name = __aws_sdk_dynamodb_0_33,
    mod_name = aws_sdk_dynamodb_0_33,
    attribute_value_path = ::__aws_sdk_dynamodb_0_33::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodb_0_33::primitives::Blob,
    aws_version = "0.33",
    config_version = "0.56",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_8",
    crate_name = __aws_sdk_dynamodbstreams_0_8,
    mod_name = aws_sdk_dynamodbstreams_0_8,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_8::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_8::types::Blob,
    aws_version = "0.8",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_9",
    crate_name = __aws_sdk_dynamodbstreams_0_9,
    mod_name = aws_sdk_dynamodbstreams_0_9,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_9::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_9::types::Blob,
    aws_version = "0.9",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_10",
    crate_name = __aws_sdk_dynamodbstreams_0_10,
    mod_name = aws_sdk_dynamodbstreams_0_10,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_10::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_10::types::Blob,
    aws_version = "0.10",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_11",
    crate_name = __aws_sdk_dynamodbstreams_0_11,
    mod_name = aws_sdk_dynamodbstreams_0_11,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_11::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_11::types::Blob,
    aws_version = "0.11",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_12",
    crate_name = __aws_sdk_dynamodbstreams_0_12,
    mod_name = aws_sdk_dynamodbstreams_0_12,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_12::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_12::types::Blob,
    aws_version = "0.12",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_13",
    crate_name = __aws_sdk_dynamodbstreams_0_13,
    mod_name = aws_sdk_dynamodbstreams_0_13,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_13::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_13::types::Blob,
    aws_version = "0.13",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_14",
    crate_name = __aws_sdk_dynamodbstreams_0_14,
    mod_name = aws_sdk_dynamodbstreams_0_14,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_14::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_14::types::Blob,
    aws_version = "0.14",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_15",
    crate_name = __aws_sdk_dynamodbstreams_0_15,
    mod_name = aws_sdk_dynamodbstreams_0_15,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_15::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_15::types::Blob,
    aws_version = "0.15",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_16",
    crate_name = __aws_sdk_dynamodbstreams_0_16,
    mod_name = aws_sdk_dynamodbstreams_0_16,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_16::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_16::types::Blob,
    aws_version = "0.16",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_17",
    crate_name = __aws_sdk_dynamodbstreams_0_17,
    mod_name = aws_sdk_dynamodbstreams_0_17,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_17::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_17::types::Blob,
    aws_version = "0.17",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_18",
    crate_name = __aws_sdk_dynamodbstreams_0_18,
    mod_name = aws_sdk_dynamodbstreams_0_18,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_18::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_18::types::Blob,
    aws_version = "0.18",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_19",
    crate_name = __aws_sdk_dynamodbstreams_0_19,
    mod_name = aws_sdk_dynamodbstreams_0_19,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_19::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_19::types::Blob,
    aws_version = "0.19",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_21",
    crate_name = __aws_sdk_dynamodbstreams_0_21,
    mod_name = aws_sdk_dynamodbstreams_0_21,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_21::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_21::types::Blob,
    aws_version = "0.21",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_22",
    crate_name = __aws_sdk_dynamodbstreams_0_22,
    mod_name = aws_sdk_dynamodbstreams_0_22,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_22::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_22::types::Blob,
    aws_version = "0.22",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_23",
    crate_name = __aws_sdk_dynamodbstreams_0_23,
    mod_name = aws_sdk_dynamodbstreams_0_23,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_23::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_23::types::Blob,
    aws_version = "0.23",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_24",
    crate_name = __aws_sdk_dynamodbstreams_0_24,
    mod_name = aws_sdk_dynamodbstreams_0_24,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_24::model::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_24::types::Blob,
    aws_version = "0.24",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_25",
    crate_name = __aws_sdk_dynamodbstreams_0_25,
    mod_name = aws_sdk_dynamodbstreams_0_25,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_25::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_25::primitives::Blob,
    aws_version = "0.25",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_26",
    crate_name = __aws_sdk_dynamodbstreams_0_26,
    mod_name = aws_sdk_dynamodbstreams_0_26,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_26::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_26::primitives::Blob,
    aws_version = "0.26",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_27",
    crate_name = __aws_sdk_dynamodbstreams_0_27,
    mod_name = aws_sdk_dynamodbstreams_0_27,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_27::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_27::primitives::Blob,
    aws_version = "0.27",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_28",
    crate_name = __aws_sdk_dynamodbstreams_0_28,
    mod_name = aws_sdk_dynamodbstreams_0_28,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_28::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_28::primitives::Blob,
    aws_version = "0.28",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_29",
    crate_name = __aws_sdk_dynamodbstreams_0_29,
    mod_name = aws_sdk_dynamodbstreams_0_29,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_29::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_29::primitives::Blob,
    aws_version = "0.29",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_30",
    crate_name = __aws_sdk_dynamodbstreams_0_30,
    mod_name = aws_sdk_dynamodbstreams_0_30,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_30::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_30::primitives::Blob,
    aws_version = "0.30",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_31",
    crate_name = __aws_sdk_dynamodbstreams_0_31,
    mod_name = aws_sdk_dynamodbstreams_0_31,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_31::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_31::primitives::Blob,
    aws_version = "0.31",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_32",
    crate_name = __aws_sdk_dynamodbstreams_0_32,
    mod_name = aws_sdk_dynamodbstreams_0_32,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_32::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_32::primitives::Blob,
    aws_version = "0.32",
);

aws_sdk_streams_macro!(
    feature = "aws-sdk-dynamodbstreams+0_33",
    crate_name = __aws_sdk_dynamodbstreams_0_33,
    mod_name = aws_sdk_dynamodbstreams_0_33,
    attribute_value_path = ::__aws_sdk_dynamodbstreams_0_33::types::AttributeValue,
    blob_path = ::__aws_sdk_dynamodbstreams_0_33::primitives::Blob,
    aws_version = "0.33",
);

rusoto_macro!(
    feature = "rusoto_dynamodb+0_46",
    crate_name = __rusoto_dynamodb_0_46,
    mod_name = rusoto_dynamodb_0_46,
    rusoto_version = "0.46",
);

rusoto_macro!(
    feature = "rusoto_dynamodb+0_47",
    crate_name = __rusoto_dynamodb_0_47,
    mod_name = rusoto_dynamodb_0_47,
    rusoto_version = "0.47",
);

rusoto_macro!(
    feature = "rusoto_dynamodb+0_48",
    crate_name = __rusoto_dynamodb_0_48,
    mod_name = rusoto_dynamodb_0_48,
    rusoto_version = "0.48",
);

rusoto_streams_macro!(
    feature = "rusoto_dynamodbstreams+0_46",
    crate_name = __rusoto_dynamodbstreams_0_46,
    mod_name = rusoto_dynamodbstreams_0_46,
    rusoto_version = "0.46",
);

rusoto_streams_macro!(
    feature = "rusoto_dynamodbstreams+0_47",
    crate_name = __rusoto_dynamodbstreams_0_47,
    mod_name = rusoto_dynamodbstreams_0_47,
    rusoto_version = "0.47",
);

rusoto_streams_macro!(
    feature = "rusoto_dynamodbstreams+0_48",
    crate_name = __rusoto_dynamodbstreams_0_48,
    mod_name = rusoto_dynamodbstreams_0_48,
    rusoto_version = "0.48",
);

aws_lambda_events_macro!(
    feature = "aws_lambda_events+0_6",
    crate_name = __aws_lambda_events_0_6,
    mod_name = aws_lambda_events_0_6,
    aws_lambda_events_version = "0.6",
);

aws_lambda_events_macro!(
    feature = "aws_lambda_events+0_7",
    crate_name = __aws_lambda_events_0_7,
    mod_name = aws_lambda_events_0_7,
    aws_lambda_events_version = "0.7",
);

#[cfg(test)]
mod tests;
