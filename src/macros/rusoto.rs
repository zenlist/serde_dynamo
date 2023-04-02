macro_rules! rusoto_macro {
    (feature = $feature:literal, crate_name = $crate_name:ident, mod_name = $mod_name:ident, rusoto_version = $version:literal,) => {
        #[cfg(feature = $feature)]
        #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
        pub mod $mod_name {
            #![doc = concat!("Support for [rusoto_dynamodb](https://docs.rs/rusoto_dynamodb/", $version, ") version ", $version)]
            //!
            //! Because [rusoto_dynamodb] has not yet reached version 1.0, a feature is required to
            //! enable support. Add the following to your dependencies.
            //!
            //! ```toml
            //! [dependencies]
            #![doc = concat!("rusoto_core = { version = ", stringify!($version), ", default-features = false, features = [\"rustls\"] }")]
            #![doc = concat!("rusoto_dynamodb = { version = ", stringify!($version), ", default-features = false, features = [\"rustls\"] }")]
            #![doc = concat!("serde_dynamo = { version = \"4\", features = [", stringify!($feature), "] }")]
            //! ```
            //!
            //! ## Parsing items as strongly-typed data structures.
            //!
            //! Items received from a [rusoto_dynamodb] call can be run through [`from_items`].
            //!
            //! ```
            #![doc = concat!("# use ", stringify!($crate_name), "::{DynamoDb, DynamoDbClient, ScanInput};")]
            //! # use serde::{Serialize, Deserialize};
            //! # use serde_dynamo::from_items;
            //! #
            //! # async fn scan(client: &DynamoDbClient) -> Result<(), Box<dyn std::error::Error>> {
            //! #[derive(Serialize, Deserialize)]
            //! pub struct User {
            //!     id: String,
            //!     name: String,
            //!     age: u8,
            //! };
            //!
            //! // Get documents from DynamoDB
            //! let input = ScanInput {
            //!     table_name: "users".to_string(),
            //!     ..ScanInput::default()
            //! };
            //! let result = client.scan(input).await?;
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
            #![doc = concat!("# use ", stringify!($crate_name), "::{DynamoDb, DynamoDbClient, ScanInput};")]
            //! # use serde::{Serialize, Deserialize};
            //! # use serde_dynamo::from_item;
            //! #
            //! # async fn scan(client: &DynamoDbClient) -> Result<(), Box<dyn std::error::Error>> {
            //! #[derive(Serialize, Deserialize)]
            //! pub struct User {
            //!     id: String,
            //!     name: String,
            //!     age: u8,
            //! };
            //!
            //! // Get documents from DynamoDB
            //! let input = ScanInput {
            //!     table_name: "users".to_string(),
            //!     ..ScanInput::default()
            //! };
            //! let result = client.scan(input).await?;
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
            #![doc = concat!("# use ", stringify!($crate_name), "::{DynamoDb, DynamoDbClient, PutItemInput};")]
            //! # use serde::{Serialize, Deserialize};
            //! # use serde_dynamo::to_item;
            //! #
            //! # async fn put(client: &DynamoDbClient) -> Result<(), Box<dyn std::error::Error>> {
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
            //! let input = PutItemInput {
            //!     table_name: "users".to_string(),
            //!     item: item,
            //!     ..PutItemInput::default()
            //! };
            //! client.put_item(input).await?;
            //! # Ok(())
            //! # }
            //! ```
            //!
            //!
            //! ## Using to_attribute_value for more control
            //!
            //! In some circumstances, building [rusoto_dynamodb::AttributeValue]s directly is required.
            //!
            //! For example, when generating a key to supply to [get_item].
            //!
            //! ```
            //! use serde_dynamo::to_attribute_value;
            #![doc = concat!("# use ", stringify!($crate_name), "::{DynamoDb, DynamoDbClient, GetItemInput};")]
            //! # use std::collections::HashMap;
            //! #
            //! # async fn get(client: &DynamoDbClient) -> Result<(), Box<dyn std::error::Error>> {
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
            //! let input = GetItemInput {
            //!     table_name: "users".to_string(),
            //!     key: key,
            //!     ..GetItemInput::default()
            //! };
            //! client.get_item(input).await?;
            //! # Ok(())
            //! # }
            //! ```
            //!
            //! Or when generating attribute values in a [query] call.
            //!
            //! ```
            //! use serde_dynamo::to_attribute_value;
            #![doc = concat!("# use ", stringify!($crate_name), "::{DynamoDb, DynamoDbClient, QueryInput};")]
            //! # use std::collections::HashMap;
            //! #
            //! # async fn query(client: &DynamoDbClient) -> Result<(), Box<dyn std::error::Error>> {
            //! # let user_type = "user";
            //! # let yesterday = "1985-04-21";
            //!
            //! // Declare all of the expression inputs for a query call
            //! let expression_attribute_values = HashMap::from([
            //!     (String::from(":user_type"), to_attribute_value(user_type)?),
            //!     (String::from(":last_login"), to_attribute_value(yesterday)?),
            //! ]);
            //!
            //! // And execute the query
            //! let input = QueryInput {
            //!     table_name: "users".to_string(),
            //!     index_name: Some("by_type_and_last_login".to_string()),
            //!     key_condition_expression: Some("user_type = :user_type AND last_login > :last_login".to_string()),
            //!     expression_attribute_values: Some(expression_attribute_values),
            //!     ..QueryInput::default()
            //! };
            //! client.query(input).await?;
            //! # Ok(())
            //! # }
            //! ```
            //! [rusoto_dynamodb]: https://docs.rs/rusoto_dynamodb
            //! [get_item]: https://docs.rs/rusoto_dynamodb/0.47.0/rusoto_dynamodb/trait.DynamoDb.html#tymethod.get_item
            //! [put_item]: https://docs.rs/rusoto_dynamodb/0.47.0/rusoto_dynamodb/trait.DynamoDb.html#tymethod.put_item
            //! [query]: https://docs.rs/rusoto_dynamodb/0.47.0/rusoto_dynamodb/trait.DynamoDb.html#tymethod.query
            //! [rusoto_dynamodb::AttributeValue]: https://docs.rs/rusoto_dynamodb/0.47.0/rusoto_dynamodb/struct.AttributeValue.html

            use crate::Result;
            use ::$crate_name::AttributeValue;

            impl From<crate::AttributeValue> for AttributeValue {
                fn from(attribute_value: crate::AttributeValue) -> Self {
                    match attribute_value {
                        crate::AttributeValue::N(n) => AttributeValue{ n: Some(n), ..Default::default() },
                        crate::AttributeValue::S(s) => AttributeValue { s: Some(s), ..Default::default() },
                        crate::AttributeValue::Bool(b) => AttributeValue { bool: Some(b), ..Default::default() },
                        crate::AttributeValue::B(v) => AttributeValue { b: Some(v.into()), ..Default::default() },
                        crate::AttributeValue::Null(null) => AttributeValue { null: Some(null), ..Default::default() },
                        crate::AttributeValue::M(item) => AttributeValue { m: Some(item.into_iter().map(|(key, attribute_value)|
                            (key, AttributeValue::from(attribute_value))
                        ).collect()), ..Default::default() },
                        crate::AttributeValue::L(list) => AttributeValue { l: Some(list.into_iter().map(AttributeValue::from).collect()), ..Default::default() },
                        crate::AttributeValue::Ss(ss) => AttributeValue { ss: Some(ss), ..Default::default() },
                        crate::AttributeValue::Ns(ns) => AttributeValue { ns: Some(ns), ..Default::default() },
                        crate::AttributeValue::Bs(bs) => AttributeValue { bs: Some(bs.into_iter().map(Into::into).collect()), ..Default::default() },
                    }
                }
            }

            impl From<AttributeValue> for crate::AttributeValue {
                fn from(attribute_value: AttributeValue) -> crate::AttributeValue {
                    if let Some(n) = attribute_value.n { crate::AttributeValue::N(n) }
                    else if let Some(s) = attribute_value.s { crate::AttributeValue::S(s) }
                    else if let Some(b) = attribute_value.bool { crate::AttributeValue::Bool(b) }
                    else if let Some(v) = attribute_value.b { crate::AttributeValue::B(v.to_vec()) }
                    else if let Some(null) = attribute_value.null { crate::AttributeValue::Null(null) }
                    else if let Some(item) = attribute_value.m { crate::AttributeValue::M(item.into_iter().map(|(key, attribute_value)| (key, crate::AttributeValue::from(attribute_value))).collect()) }
                    else if let Some(list) = attribute_value.l { crate::AttributeValue::L(list.into_iter().map(crate::AttributeValue::from).collect()) }
                    else if let Some(ss)= attribute_value.ss { crate::AttributeValue::Ss(ss) }
                    else if let Some(ns)= attribute_value.ns { crate::AttributeValue::Ns(ns) }
                    else if let Some(bs)= attribute_value.bs { crate::AttributeValue::Bs(bs.into_iter().map(|b| b.to_vec()).collect()) }
                    else {
                        panic!("Unexpectedly did not match any possible data types")
                    }
                }
            }

            /// A version of [`crate::to_attribute_value`] where the `AV` generic is tied to
            /// [`rusoto_dynamodb::AttributeValue`](AttributeValue).
            ///
            /// Useful in very generic code where the type checker can't determine the type of
            /// `AV`.
            pub fn to_attribute_value<T>(value: T) -> Result<AttributeValue>
            where
                T: serde::ser::Serialize,
            {
                crate::ser::to_attribute_value(value)
            }

            /// A version of [`crate::to_item`] where the `AV` generic is tied to
            /// [`rusoto_dynamodb::AttributeValue`](AttributeValue).
            ///
            /// Useful in very generic code where the type checker can't determine the type of
            /// `AV`.
            pub fn to_item<T>(value: T) -> Result<std::collections::HashMap<String, AttributeValue>>
            where
                T: serde::ser::Serialize,
            {
                crate::ser::to_item(value)
            }

            /// A version of [`crate::from_attribute_value`] where the `AV` generic is tied to
            /// [`rusoto_dynamodb::AttributeValue`](AttributeValue).
            ///
            /// Useful in very generic code where the type checker can't determine the type of
            /// `AV`.
            pub fn from_attribute_value<'a, T>(attribute_value: AttributeValue) -> Result<T>
            where
                T: serde::de::Deserialize<'a>,
            {
                crate::de::from_attribute_value(attribute_value)
            }

            /// A version of [`crate::from_item`] where the `AV` generic is tied to
            /// [`rusoto_dynamodb::AttributeValue`](AttributeValue).
            ///
            /// Useful in very generic code where the type checker can't determine the type of
            /// `AV`.
            pub fn from_item<'a, T>(
                item: std::collections::HashMap<String, AttributeValue>,
            ) -> Result<T>
            where
                T: serde::de::Deserialize<'a>,
            {
                crate::de::from_item(item)
            }

            /// A version of [`crate::from_items`] where the `AV` generic is tied to
            /// [`rusoto_dynamodb::AttributeValue`](AttributeValue).
            ///
            /// Useful in very generic code where the type checker can't determine the type of
            /// `AV`.
            pub fn from_items<'a, T>(
                items: Vec<std::collections::HashMap<String, AttributeValue>>,
            ) -> Result<Vec<T>>
            where
                T: serde::de::Deserialize<'a>,
            {
                crate::de::from_items(items)
            }
        }

        #[cfg(feature = $feature)]
        #[doc(hidden)]
        #[deprecated(since = "4.0.0", note = "The double-underscore is no longer necessary")]
        pub mod $crate_name {
            use crate::Result;
            use ::$crate_name::AttributeValue;

            #[deprecated(since = "4.0.0", note = "The double-underscore on the mod name is no longer necessary")]
            pub fn to_attribute_value<T>(value: T) -> Result<AttributeValue>
            where
                T: serde::ser::Serialize,
            {
                crate::ser::to_attribute_value(value)
            }

            #[deprecated(since = "4.0.0", note = "The double-underscore on the mod name is no longer necessary")]
            pub fn to_item<T>(value: T) -> Result<std::collections::HashMap<String, AttributeValue>>
            where
                T: serde::ser::Serialize,
            {
                crate::ser::to_item(value)
            }

            #[deprecated(since = "4.0.0", note = "The double-underscore on the mod name is no longer necessary")]
            pub fn from_attribute_value<'a, T>(attribute_value: AttributeValue) -> Result<T>
            where
                T: serde::de::Deserialize<'a>,
            {
                crate::de::from_attribute_value(attribute_value)
            }

            #[deprecated(since = "4.0.0", note = "The double-underscore on the mod name is no longer necessary")]
            pub fn from_item<'a, T>(
                item: std::collections::HashMap<String, AttributeValue>,
            ) -> Result<T>
            where
                T: serde::de::Deserialize<'a>,
            {
                crate::de::from_item(item)
            }

            #[deprecated(since = "4.0.0", note = "The double-underscore on the mod name is no longer necessary")]
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

pub(crate) use rusoto_macro;
