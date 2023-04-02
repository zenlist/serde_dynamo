macro_rules! aws_sdk_macro {
    (
        feature = $feature:literal,
        crate_name = $crate_name:ident,
        mod_name = $mod_name:ident,
        attribute_value_path = $attribute_value_path:path,
        blob_path = $blob_path:path,
        aws_version = $version:literal,
        config_version = $config_version:literal,
    ) => {
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
            #![doc = concat!("aws-config = ", stringify!($config_version))]
            #![doc = concat!("aws-sdk-dynamodb = ", stringify!($version))]
            #![doc = concat!("serde_dynamo = { version = \"4\", features = [", stringify!($feature), "] }")]
            //! ```
            //!
            //!
            //! ## Parsing items as strongly-typed data structures.
            //!
            //! Items received from a [aws-sdk-dynamodb] call can be run through [`from_items`].
            //!
            //! ```
            #![doc = concat!("# use ", stringify!($crate_name), "::client::Client;")]
            //! # use serde_derive::{Serialize, Deserialize};
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
            //! if let Some(items) = result.items().map(|slice| slice.to_vec()) {
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
            #![doc = concat!("# use ", stringify!($crate_name), "::client::Client;")]
            //! # use serde_derive::{Serialize, Deserialize};
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
            //! for item in result.items().map(|slice| slice.to_vec()).unwrap() {
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
            #![doc = concat!("# use ", stringify!($crate_name), "::client::Client;")]
            //! # use serde_derive::{Serialize, Deserialize};
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
            //! // Turn it into an item that aws-sdk-dynamodb understands
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
            #![doc = concat!("# use ", stringify!($crate_name), "::client::Client;")]
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
            #![doc = concat!("# use ", stringify!($crate_name), "::client::Client;")]
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
            use $attribute_value_path;
            use $blob_path;

            impl From<crate::AttributeValue> for AttributeValue {
                fn from(attribute_value: crate::AttributeValue) -> AttributeValue {
                    match attribute_value {
                        crate::AttributeValue::N(n) => AttributeValue::N(n),
                        crate::AttributeValue::S(s) => AttributeValue::S(s),
                        crate::AttributeValue::Bool(b) => AttributeValue::Bool(b),
                        crate::AttributeValue::B(v) => AttributeValue::B(Blob::new(v)),
                        crate::AttributeValue::Null(null) => AttributeValue::Null(null),
                        crate::AttributeValue::M(m) => AttributeValue::M(m.into_iter().map(|(key, attribute_value)| (key, AttributeValue::from(attribute_value))).collect()),
                        crate::AttributeValue::L(l) => AttributeValue::L(l.into_iter().map(AttributeValue::from).collect()),
                        crate::AttributeValue::Ss(ss) => AttributeValue::Ss(ss),
                        crate::AttributeValue::Ns(ns) => AttributeValue::Ns(ns),
                        crate::AttributeValue::Bs(bs) => AttributeValue::Bs(bs.into_iter().map(Blob::new).collect()),
                    }
                }
            }

            impl From<AttributeValue> for crate::AttributeValue {
                fn from(attribute_value: AttributeValue) -> crate::AttributeValue {
                    match attribute_value {
                        AttributeValue::N(n) => crate::AttributeValue::N(n),
                        AttributeValue::S(s) => crate::AttributeValue::S(s),
                        AttributeValue::Bool(b) => crate::AttributeValue::Bool(b),
                        AttributeValue::B(v) => crate::AttributeValue::B(v.into_inner()),
                        AttributeValue::Null(null) => crate::AttributeValue::Null(null),
                        AttributeValue::M(m) => crate::AttributeValue::M(m.into_iter().map(|(key, attribute_value)| (key, crate::AttributeValue::from(attribute_value))).collect()),
                        AttributeValue::L(l) => crate::AttributeValue::L(l.into_iter().map(crate::AttributeValue::from).collect()),
                        AttributeValue::Ss(ss) => crate::AttributeValue::Ss(ss),
                        AttributeValue::Ns(ns) => crate::AttributeValue::Ns(ns),
                        AttributeValue::Bs(bs) => crate::AttributeValue::Bs(bs.into_iter().map(Blob::into_inner).collect()),
                        _ => panic!("Unexpectedly did not match any possible data types"),
                    }
                }
            }

            /// A version of [`crate::to_attribute_value`] where the `AV` generic is tied to
            /// [`aws-sdk-dynamodb::model::AttributeValue`](AttributeValue).
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
            /// [`aws-sdk-dynamodb::model::AttributeValue`](AttributeValue).
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
            /// [`aws-sdk-dynamodb::model::AttributeValue`](AttributeValue).
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
            /// [`aws-sdk-dynamodb::model::AttributeValue`](AttributeValue).
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
            /// [`aws-sdk-dynamodb::model::AttributeValue`](AttributeValue).
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
            use $attribute_value_path;

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

pub(crate) use aws_sdk_macro;
