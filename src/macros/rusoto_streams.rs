macro_rules! rusoto_streams_macro {
    (feature = $feature:literal, crate_name = $crate_name:ident, mod_name = $mod_name:ident, rusoto_version = $version:literal,) => {
        #[cfg(feature = $feature)]
        #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
        pub mod $mod_name {
            #![doc = concat!("Support for [rusoto_dynamodbstreams](https://docs.rs/rusoto_dynamodbstreams/", $version, ") version ", $version)]
            //!
            //! Because [rusoto_dynamodbstreams] has not yet reached version 1.0, a feature is
            //! required to enable support. Add the following to your dependencies.
            //!
            //! ```toml
            //! [dependencies]
            #![doc = concat!("rusoto_core = { version = ", stringify!($version), ", default-features = false, features = [\"rustls\"] }")]
            #![doc = concat!("rusoto_dynamodbstreams = { version = ", stringify!($version), ", default-features = false, features = [\"rustls\"] }")]
            #![doc = concat!("serde_dynamo = { version = \"4\", features = [", stringify!($feature), "] }")]
            //! ```
            //!
            //! [rusoto_dynamodbstreams]: https://docs.rs/rusoto_dynamodbstreams

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
            /// [`rusoto_dynamodbstreams::AttributeValue`](AttributeValue).
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
            /// [`rusoto_dynamodbstreams::AttributeValue`](AttributeValue).
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
            /// [`rusoto_dynamodbstreams::AttributeValue`](AttributeValue).
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
            /// [`rusoto_dynamodbstreams::AttributeValue`](AttributeValue).
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
            /// [`rusoto_dynamodbstreams::AttributeValue`](AttributeValue).
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

pub(crate) use rusoto_streams_macro;
