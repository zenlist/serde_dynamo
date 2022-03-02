macro_rules! rusoto_macro {
    (feature = $feature:literal, crate_name = $mod_name:ident, rusoto_version = $version:literal,) => {
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
            #![doc = concat!("serde_dynamo = { version = \"3\", features = [", stringify!($feature), "] }")]
            //! ```
            //!
            //! ## Parsing items as strongly-typed data structures.
            //!
            //! Items received from a [rusoto_dynamodb] call can be run through [`from_items`].
            //!
            //! ```
            #![doc = concat!("# use ", stringify!($mod_name), "::{DynamoDb, DynamoDbClient, ScanInput};")]
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
            #![doc = concat!("# use ", stringify!($mod_name), "::{DynamoDb, DynamoDbClient, ScanInput};")]
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
            #![doc = concat!("# use ", stringify!($mod_name), "::{DynamoDb, DynamoDbClient, PutItemInput};")]
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
            #![doc = concat!("# use ", stringify!($mod_name), "::{DynamoDb, DynamoDbClient, GetItemInput};")]
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
            #![doc = concat!("# use ", stringify!($mod_name), "::{DynamoDb, DynamoDbClient, QueryInput};")]
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
            use ::$mod_name::AttributeValue;

            impl crate::AttributeValue for AttributeValue {
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

                fn as_m(&self) -> Option<&std::collections::HashMap<String, Self>> {
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
                    self.b.map(|bytes| bytes.into_iter().collect())
                }

                fn into_null(self) -> Option<bool> {
                    self.null
                }

                fn into_m(self) -> Option<std::collections::HashMap<String, Self>> {
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
                    self.bs.map(|vec| {
                        vec.into_iter()
                            .map(|bytes| bytes.into_iter().collect())
                            .collect()
                    })
                }

                fn construct_n(input: String) -> Self {
                    Self {
                        n: Some(input),
                        ..Default::default()
                    }
                }

                fn construct_s(input: String) -> Self {
                    Self {
                        s: Some(input),
                        ..Default::default()
                    }
                }

                fn construct_bool(input: bool) -> Self {
                    Self {
                        bool: Some(input),
                        ..Default::default()
                    }
                }

                fn construct_b(input: &[u8]) -> Self {
                    Self {
                        b: Some(input.to_vec().into()),
                        ..Default::default()
                    }
                }

                fn construct_null(input: bool) -> Self {
                    Self {
                        null: Some(input),
                        ..Default::default()
                    }
                }

                fn construct_m(input: std::collections::HashMap<String, Self>) -> Self {
                    Self {
                        m: Some(input),
                        ..Default::default()
                    }
                }

                fn construct_l(input: Vec<Self>) -> Self {
                    Self {
                        l: Some(input),
                        ..Default::default()
                    }
                }

                fn construct_ss(input: Vec<String>) -> Self {
                    Self {
                        ss: Some(input),
                        ..Default::default()
                    }
                }

                fn construct_ns(input: Vec<String>) -> Self {
                    Self {
                        ns: Some(input),
                        ..Default::default()
                    }
                }

                fn construct_bs(input: Vec<Vec<u8>>) -> Self {
                    let input = input.into_iter().map(|vec| vec.into()).collect();
                    Self {
                        bs: Some(input),
                        ..Default::default()
                    }
                }
            }

            /// A version of [`crate::to_attribute_value`] where the `Tout` generic is tied to
            /// [`rusoto_dynamodb::AttributeValue`](AttributeValue).
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
            /// [`rusoto_dynamodb::AttributeValue`](AttributeValue).
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
            /// [`rusoto_dynamodb::AttributeValue`](AttributeValue).
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
            /// [`rusoto_dynamodb::AttributeValue`](AttributeValue).
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
            /// [`rusoto_dynamodb::AttributeValue`](AttributeValue).
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

pub(crate) use rusoto_macro;
