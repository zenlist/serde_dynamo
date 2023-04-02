macro_rules! aws_sdk_streams_macro {
    (
        feature = $feature:literal,
        crate_name = $crate_name:ident,
        mod_name = $mod_name:ident,
        attribute_value_path = $attribute_value_path:path,
        blob_path = $blob_path:path,
        aws_version = $version:literal,
    ) => {
        #[cfg(feature = $feature)]
        #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
        pub mod $mod_name {
            #![doc = concat!("Support for [aws-sdk-dynamodbstreams](https://docs.rs/aws-sdk-dynamodbstreams/", $version, ") version ", $version)]
            //!
            //! Because [aws-sdk-dynamodbstreams] has not yet reached version 1.0, a feature is required to
            //! enable support. Add the following to your dependencies.
            //!
            //! ```toml
            //! [dependencies]
            #![doc = concat!("aws-sdk-dynamodbstreams = ", stringify!($version))]
            #![doc = concat!("serde_dynamo = { version = \"4\", features = [", stringify!($feature), "] }")]
            //! ```
            //!
            //! [aws-sdk-dynamodbstreams]: https://docs.rs/aws-sdk-dynamodbstreams
            //! [aws_sdk_dynamodbstreams::model::AttributeValue]: https://docs.rs/rusoto_dynamodbstreams/0.47.0/rusoto_dynamodbstreams/struct.AttributeValue.html

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
            /// [`aws-sdk-dynamodbstreams::model::AttributeValue`](AttributeValue).
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
            /// [`aws-sdk-dynamodbstreams::model::AttributeValue`](AttributeValue).
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
            /// [`aws-sdk-dynamodbstreams::model::AttributeValue`](AttributeValue).
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
            /// [`aws-sdk-dynamodbstreams::model::AttributeValue`](AttributeValue).
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
            /// [`aws-sdk-dynamodbstreams::model::AttributeValue`](AttributeValue).
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

pub(crate) use aws_sdk_streams_macro;
