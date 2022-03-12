macro_rules! aws_lambda_events_macro {
    (feature = $feature:literal, crate_name = $mod_name:ident, aws_lambda_events_version = $version:literal,) => {
        #[cfg(feature = $feature)]
        #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
        pub mod $mod_name {
            #![doc = concat!("Support for [aws-sdk-dynamodb](https://docs.rs/aws-sdk-dynamodb/", $version, ") version ", $version)]
            //!
            //! Because [aws-sdk-dynamodb] has not yet reached version 1.0, a feature is required to
            //! enable support. Add the following to your dependencies.
            //!
            //! ```toml
            //! [dependencies]
            #![doc = concat!("aws-config = ", stringify!($version))]
            #![doc = concat!("aws-sdk-dynamodb = ", stringify!($version))]
            #![doc = concat!("serde_dynamo = { version = \"3\", features = [", stringify!($feature), "] }")]
            //! ```
            //!
            //!
            //! ## Parsing items as strongly-typed data structures.
            //!
            //! Items received from a [aws-sdk-dynamodb] call can be run through [`from_items`].
            //!
            //! ```
            #![doc = concat!("# use ", stringify!($mod_name), "::client::Client;")]
            //! # use serde::{Serialize, Deserialize};
            //! # use serde_dynamo::from_items;
            //! #
            //! # async fn scan(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
            //! #[derive(Serialize, Deserialize)]
            //! pub struct User {
            //!     id: String,
            //!     name: String,
            //!     age: u8,
            //! };
            //!
            //! // Get documents from DynamoDB
            //! let result = client.scan().table_name("user").send().await?;
            //!
            //! // And deserialize them as strongly-typed data structures
            //! if let Some(items) = result.items {
            //!     let users: Vec<User> = from_items(items)?;
            //!     println!("Got {} users", users.len());
            //! }
            //! # Ok(())
            //! # }
            //! ```
            //!
            //! Alternatively, to deserialize one item at a time, [`from_item`] can be used.
            //!
            //! ```
            #![doc = concat!("# use ", stringify!($mod_name), "::client::Client;")]
            //! # use serde::{Serialize, Deserialize};
            //! # use serde_dynamo::from_item;
            //! #
            //! # async fn scan(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
            //! #[derive(Serialize, Deserialize)]
            //! pub struct User {
            //!     id: String,
            //!     name: String,
            //!     age: u8,
            //! };
            //!
            //! // Get documents from DynamoDB
            //! let result = client.scan().table_name("user").send().await?;
            //!
            //! // And deserialize them as strongly-typed data structures
            //! for item in result.items.unwrap() {
            //!     let user: User = from_item(item)?;
            //!     println!("{} is {}", user.name, user.age);
            //! }
            //! # Ok(())
            //! # }
            //! ```
            //!
            //!
            //! ## Creating items by serializing data structures
            //!
            //! Writing an entire data structure to DynamoDB typically involves using [`to_item`] to serialize
            //! it.
            //!
            //! ```
            #![doc = concat!("# use ", stringify!($mod_name), "::client::Client;")]
            //! # use serde::{Serialize, Deserialize};
            //! # use serde_dynamo::to_item;
            //! #
            //! # async fn put(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
            //! #[derive(Serialize, Deserialize)]
            //! pub struct User {
            //!     id: String,
            //!     name: String,
            //!     age: u8,
            //! };
            //!
            //! // Create a user
            //! let user = User {
            //!     id: "fSsgVtal8TpP".to_string(),
            //!     name: "Arthur Dent".to_string(),
            //!     age: 42,
            //! };
            //!
            //! // Turn it into an item that rusoto understands
            //! let item = to_item(user)?;
            //!
            //! // And write it!
            //! client.put_item().table_name("users").set_item(Some(item)).send().await?;
            //! # Ok(())
            //! # }
            //! ```
            //!
            //!
            //! ## Using to_attribute_value for more control
            //!
            //! In some circumstances, building [aws_sdk_dynamodb::model::AttributeValue]s directly is required.
            //!
            //! For example, when generating a key to supply to [get_item].
            //!
            //! ```
            //! use serde_dynamo::to_attribute_value;
            #![doc = concat!("# use ", stringify!($mod_name), "::client::Client;")]
            //! # use std::collections::HashMap;
            //! #
            //! # async fn get(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
            //! #
            //! # struct User { id: String };
            //! # let user = User { id: "fSsgVtal8TpP".to_string() };
            //!
            //! // Create the unique key of the record in DynamoDB in a way rusoto understands
            //! let key = HashMap::from([
            //!     (String::from("id"), to_attribute_value(&user.id)?),
            //! ]);
            //!
            //! // And get the record
            //! client.get_item().table_name("users").set_key(Some(key)).send().await?;
            //! # Ok(())
            //! # }
            //! ```
            //!
            //! Or when generating attribute values in a [query] call.
            //!
            //! ```
            //! use serde_dynamo::to_attribute_value;
            #![doc = concat!("# use ", stringify!($mod_name), "::client::Client;")]
            //! # use std::collections::HashMap;
            //! #
            //! # async fn query(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
            //! # let user_type = "user";
            //! # let yesterday = "1985-04-21";
            //!
            //! // Declare all of the expression inputs for a query call
            //! let expression_attribute_values = HashMap::from([
            //!     (String::from(":user_type"), to_attribute_value(user_type)?),
            //!     (String::from(":last_login"), to_attribute_value(yesterday)?),
            //! ]);
            //!
            //! client.query()
            //!     .table_name("users")
            //!     .index_name("by_type_and_last_login")
            //!     .key_condition_expression("user_type = :user_type AND last_login > :last_login")
            //!     .set_expression_attribute_values(Some(expression_attribute_values))
            //!     .send()
            //!     .await?;
            //! # Ok(())
            //! # }
            //! ```
            //! [aws-sdk-dynamodb]: https://docs.rs/aws-sdk-dynamodb
            //! [get_item]: https://docs.rs/aws-sdk-dynamodb/*/aws_sdk_dynamodb/client/struct.Client.html#method.get_item
            //! [put_item]: https://docs.rs/aws-sdk-dynamodb/*/aws_sdk_dynamodb/client/struct.Client.html#method.put_item
            //! [query]: https://docs.rs/aws-sdk-dynamodb/*/aws_sdk_dynamodb/client/struct.Client.html#method.query
            //! [aws_sdk_dynamodb::model::AttributeValue]: https://docs.rs/rusoto_dynamodb/0.47.0/rusoto_dynamodb/struct.AttributeValue.html

            use crate::Result;
            use ::$mod_name::dynamodb::attributes::AttributeValue;
            use std::collections::HashMap;

            impl crate::AttributeValue for AttributeValue {
                fn is_n(&self) -> bool {
                    matches!(self, AttributeValue::Number(..))
                }

                fn is_s(&self) -> bool {
                    matches!(self, AttributeValue::String(..))
                }

                fn is_bool(&self) -> bool {
                    matches!(self, AttributeValue::Boolean(..))
                }

                fn is_b(&self) -> bool {
                    matches!(self, AttributeValue::Binary(..))
                }

                fn is_null(&self) -> bool {
                    matches!(self, AttributeValue::Null)
                }

                fn is_m(&self) -> bool {
                    matches!(self, AttributeValue::AttributeMap(..))
                }

                fn is_l(&self) -> bool {
                    matches!(self, AttributeValue::AttributeList(..))
                }

                fn is_ss(&self) -> bool {
                    matches!(self, AttributeValue::StringSet(..))
                }

                fn is_ns(&self) -> bool {
                    matches!(self, AttributeValue::NumberSet(..))
                }

                fn is_bs(&self) -> bool {
                    matches!(self, AttributeValue::BinarySet(..))
                }

                fn as_n(&self) -> Option<&str> {
                    None
                    // if let AttributeValue::N(ref v) = self {
                    //     Some(v)
                    // } else {
                    //     None
                    // }
                }

                fn as_s(&self) -> Option<&str> {
                    None
                    // if let AttributeValue::S(ref v) = self {
                    //     Some(v)
                    // } else {
                    //     None
                    // }
                }

                fn as_bool(&self) -> Option<bool> {
                    None
                    // if let AttributeValue::Bool(v) = self {
                    //     Some(*v)
                    // } else {
                    //     None
                    // }
                }

                fn as_b(&self) -> Option<&[u8]> {
                    None
                    // if let AttributeValue::B(ref v) = self {
                    //     Some(v.as_ref())
                    // } else {
                    //     None
                    // }
                }

                fn as_null(&self) -> Option<bool> {
                    None
                    // if let AttributeValue::Null = self {
                    //     Some(())
                    // } else {
                    //     None
                    // }
                }

                fn as_m(&self) -> Option<&HashMap<String, Self>> {
                    None
                    // if let AttributeValue::M(ref v) = self {
                    //     Some(v)
                    // } else {
                    //     None
                    // }
                }

                fn as_l(&self) -> Option<&[Self]> {
                    None
                    // if let AttributeValue::L(ref v) = self {
                    //     Some(v)
                    // } else {
                    //     None
                    // }
                }

                fn as_ss(&self) -> Option<&[String]> {
                    None
                    // if let AttributeValue::Ss(ref v) = self {
                    //     Some(v)
                    // } else {
                    //     None
                    // }
                }

                fn as_ns(&self) -> Option<&[String]> {
                    None
                    // if let AttributeValue::Ns(ref v) = self {
                    //     Some(v)
                    // } else {
                    //     None
                    // }
                }

                fn into_n(self) -> Option<String> {
                    None
                    // if let AttributeValue::N(v) = self {
                    //     Some(v)
                    // } else {
                    //     None
                    // }
                }

                fn into_s(self) -> Option<String> {
                    None
                    // if let AttributeValue::S(v) = self {
                    //     Some(v)
                    // } else {
                    //     None
                    // }
                }

                fn into_bool(self) -> Option<bool> {
                    None
                    // if let AttributeValue::Bool(v) = self {
                    //     Some(v)
                    // } else {
                    //     None
                    // }
                }

                fn into_b(self) -> Option<Vec<u8>> {
                    None
                    // if let AttributeValue::B(v) = self {
                    //     Some(v.into_inner())
                    // } else {
                    //     None
                    // }
                }

                fn into_null(self) -> Option<bool> {
                    None
                    // if let AttributeValue::Null(v) = self {
                    //     Some(v)
                    // } else {
                    //     None
                    // }
                }

                fn into_m(self) -> Option<HashMap<String, Self>> {
                    None
                    // if let AttributeValue::M(v) = self {
                    //     Some(v)
                    // } else {
                    //     None
                    // }
                }

                fn into_l(self) -> Option<Vec<Self>> {
                    None
                    // if let AttributeValue::L(v) = self {
                    //     Some(v)
                    // } else {
                    //     None
                    // }
                }

                fn into_ss(self) -> Option<Vec<String>> {
                    None
                    // if let AttributeValue::Ss(v) = self {
                    //     Some(v)
                    // } else {
                    //     None
                    // }
                }

                fn into_ns(self) -> Option<Vec<String>> {
                    None
                    // if let AttributeValue::Ns(v) = self {
                    //     Some(v)
                    // } else {
                    //     None
                    // }
                }

                fn into_bs(self) -> Option<Vec<Vec<u8>>> {
                    None
                    // if let AttributeValue::Bs(v) = self {
                    //     Some(v.into_iter().map(|b| b.into_inner()).collect())
                    // } else {
                    //     None
                    // }
                }

                fn construct_n(input: String) -> Self {
                    AttributeValue::Number(input.parse().unwrap())
                }

                fn construct_s(input: String) -> Self {
                    AttributeValue::String(input)
                }

                fn construct_bool(input: bool) -> Self {
                    AttributeValue::Boolean(input)
                }

                fn construct_b(input: &[u8]) -> Self {
                    AttributeValue::Binary(input.to_vec())
                }

                fn construct_null(_input: bool) -> Self {
                    AttributeValue::Null
                }

                fn construct_m(input: HashMap<String, Self>) -> Self {
                    AttributeValue::AttributeMap(input)
                }

                fn construct_l(input: Vec<Self>) -> Self {
                    AttributeValue::AttributeList(input)
                }

                fn construct_ss(input: Vec<String>) -> Self {
                    AttributeValue::StringSet(input)
                }

                fn construct_ns(input: Vec<String>) -> Self {
                    let input = input.iter().map(|n|n.parse().expect("parsing value to number")).collect();
                    AttributeValue::NumberSet(input)
                }

                fn construct_bs(input: Vec<Vec<u8>>) -> Self {
                    AttributeValue::BinarySet(input)
                }
            }

            /// A version of [`crate::to_attribute_value`] where the `Tout` generic is tied to
            /// [`aws-sdk-dynamodb::model::AttributeValue`](AttributeValue).
            ///
            /// Useful in very generic code where the type checker can't determine the type of
            /// `Tout`.
            pub fn to_attribute_value<T>(value: T) -> Result<AttributeValue>
            where
                T: serde::ser::Serialize,
            {
                crate::ser::to_attribute_value(value)
            }

            /// A version of [`crate::to_item`] where the `Tout` generic is tied to
            /// [`aws-sdk-dynamodb::model::AttributeValue`](AttributeValue).
            ///
            /// Useful in very generic code where the type checker can't determine the type of
            /// `Tout`.
            pub fn to_item<T>(value: T) -> Result<std::collections::HashMap<String, AttributeValue>>
            where
                T: serde::ser::Serialize,
            {
                crate::ser::to_item(value)
            }

            /// A version of [`crate::from_attribute_value`] where the `Tin` generic is tied to
            /// [`aws-sdk-dynamodb::model::AttributeValue`](AttributeValue).
            ///
            /// Useful in very generic code where the type checker can't determine the type of
            /// `Tin`.
            pub fn from_attribute_value<'a, T>(attribute_value: AttributeValue) -> Result<T>
            where
                T: serde::de::Deserialize<'a>,
            {
                crate::de::from_attribute_value(attribute_value)
            }

            /// A version of [`crate::from_item`] where the `Tin` generic is tied to
            /// [`aws-sdk-dynamodb::model::AttributeValue`](AttributeValue).
            ///
            /// Useful in very generic code where the type checker can't determine the type of
            /// `Tin`.
            pub fn from_item<'a, T>(
                item: std::collections::HashMap<String, AttributeValue>,
            ) -> Result<T>
            where
                T: serde::de::Deserialize<'a>,
            {
                crate::de::from_item(item)
            }

            /// A version of [`crate::from_items`] where the `Tin` generic is tied to
            /// [`aws-sdk-dynamodb::model::AttributeValue`](AttributeValue).
            ///
            /// Useful in very generic code where the type checker can't determine the type of
            /// `Tin`.
            pub fn from_items<'a, T>(
                items: Vec<std::collections::HashMap<String, AttributeValue>>,
            ) -> Result<Vec<T>>
            where
                T: serde::de::Deserialize<'a>,
            {
                crate::de::from_items(items)
            }
        }
    };
}

pub(crate) use aws_lambda_events_macro;
