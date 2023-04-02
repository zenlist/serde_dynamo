macro_rules! aws_lambda_events_macro {
    (feature = $feature:literal, crate_name = $crate_name:ident, mod_name = $mod_name:ident, aws_lambda_events_version = $version:literal,) => {
        #[cfg(feature = $feature)]
        #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
        pub mod $mod_name {
            #![doc = concat!("Support for [aws_lambda_events](https://docs.rs/aws_lambda_events/", $version, ") version ", $version)]
            //!
            //! Because [aws_lambda_events] has not yet reached version 1.0, a feature is required to
            //! enable support. Add the following to your dependencies.
            //!
            //! ```toml
            //! [dependencies]
            #![doc = concat!("aws_lambda_events = ", stringify!($version))]
            #![doc = concat!("serde_dynamo = { version = \"4\", features = [", stringify!($feature), "] }")]
            //! ```
            //!
            //! [aws_lambda_events]: https://docs.rs/aws_sdk_dynamodb

            use crate::Result;
            use ::$crate_name::dynamodb::attributes::AttributeValue;

            impl From<crate::AttributeValue> for AttributeValue {
                fn from(attribute_value: crate::AttributeValue) -> AttributeValue {
                    match attribute_value {
                        crate::AttributeValue::N(n) => AttributeValue::Number(n.parse().unwrap()),
                        crate::AttributeValue::S(s) => AttributeValue::String(s),
                        crate::AttributeValue::Bool(b) => AttributeValue::Boolean(b),
                        crate::AttributeValue::B(v) => AttributeValue::Binary(v),
                        crate::AttributeValue::Null(_) => AttributeValue::Null,
                        crate::AttributeValue::M(m) => AttributeValue::AttributeMap(m.into_iter().map(|(key, attribute_value)| (key, AttributeValue::from(attribute_value))).collect()),
                        crate::AttributeValue::L(l) => AttributeValue::AttributeList(l.into_iter().map(AttributeValue::from).collect()),
                        crate::AttributeValue::Ss(ss) => AttributeValue::StringSet(ss),
                        crate::AttributeValue::Ns(ns) => AttributeValue::NumberSet(ns.into_iter().map(|n| n.parse().unwrap()).collect()),
                        crate::AttributeValue::Bs(bs) => AttributeValue::BinarySet(bs),
                    }
                }
            }

            impl From<AttributeValue> for crate::AttributeValue {
                fn from(attribute_value: AttributeValue) -> crate::AttributeValue {
                    match attribute_value {
                        AttributeValue::Number(n) => crate::AttributeValue::N(n.to_string()),
                        AttributeValue::String(s) => crate::AttributeValue::S(s),
                        AttributeValue::Boolean(b) => crate::AttributeValue::Bool(b),
                        AttributeValue::Binary(v) => crate::AttributeValue::B(v),
                        AttributeValue::Null => crate::AttributeValue::Null(true),
                        AttributeValue::AttributeMap(m) => crate::AttributeValue::M(m.into_iter().map(|(key, attribute_value)| (key, crate::AttributeValue::from(attribute_value))).collect()),
                        AttributeValue::AttributeList(l) => crate::AttributeValue::L(l.into_iter().map(crate::AttributeValue::from).collect()),
                        AttributeValue::StringSet(ss) => crate::AttributeValue::Ss(ss),
                        AttributeValue::NumberSet(ns) => crate::AttributeValue::Ns(ns.into_iter().map(|n| n.to_string()).collect()),
                        AttributeValue::BinarySet(bs) => crate::AttributeValue::Bs(bs),
                    }
                }
            }

            /// A version of [`crate::to_attribute_value`] that returns an
            /// [`aws_lambda_events`-specific AttributeValue](AttributeValue) instead of a
            /// [`serde_dynamo`-specific AttributeValue](crate::AttributeValue).
            pub fn to_attribute_value<T>(value: T) -> Result<AttributeValue>
            where
                T: serde::ser::Serialize,
            {
                crate::ser::to_attribute_value(value)
            }

            /// A version of [`crate::to_item`] that returns an
            /// `aws_lambda_events`-specific `HashMap<String, AttributeValue>` instead of a
            /// [`serde_dynamo`-specific Item](crate::Item).
            pub fn to_item<T>(value: T) -> Result<std::collections::HashMap<String, AttributeValue>>
            where
                T: serde::ser::Serialize,
            {
                crate::ser::to_item(value)
            }

            /// A version of [`crate::from_attribute_value`] that accept an
            /// [`aws_lambda_events`-specific AttributeValue](AttributeValue) instead of a
            /// [`serde_dynamo`-specific AttributeValue](crate::AttributeValue).
            pub fn from_attribute_value<'a, T>(attribute_value: AttributeValue) -> Result<T>
            where
                T: serde::de::Deserialize<'a>,
            {
                crate::de::from_attribute_value(attribute_value)
            }

            /// A version of [`crate::from_item`] that accepts an
            /// `aws_lambda_events`-specific `HashMap<String, AttributeValue>` instead of a
            /// [`serde_dynamo`-specific Item](crate::Item).
            pub fn from_item<'a, T>(
                item: std::collections::HashMap<String, AttributeValue>,
            ) -> Result<T>
            where
                T: serde::de::Deserialize<'a>,
            {
                crate::de::from_item(item)
            }

            /// A version of [`crate::from_items`] that accepts an
            /// `aws_lambda_events`-specific `Vec<HashMap<String, AttributeValue>>` instead of a
            /// [`serde_dynamo`-specific Items](crate::Items).
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
