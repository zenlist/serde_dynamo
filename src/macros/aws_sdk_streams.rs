macro_rules! aws_sdk_streams_macro {
    (feature = $feature:literal, crate_name = $mod_name:ident, aws_version = $version:literal,) => {
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
            #![doc = concat!("serde_dynamo = { version = \"3\", features = [", stringify!($feature), "] }")]
            //! ```
            //!
            //! [aws-sdk-dynamodbstreams]: https://docs.rs/aws-sdk-dynamodbstreams
            //! [aws_sdk_dynamodbstreams::model::AttributeValue]: https://docs.rs/rusoto_dynamodbstreams/0.47.0/rusoto_dynamodbstreams/struct.AttributeValue.html

            use crate::Result;
            use ::$mod_name::model::AttributeValue;
            use std::collections::HashMap;

            impl crate::AttributeValue for AttributeValue {
                fn is_n(&self) -> bool {
                    matches!(self, AttributeValue::N(..))
                }

                fn is_s(&self) -> bool {
                    matches!(self, AttributeValue::S(..))
                }

                fn is_bool(&self) -> bool {
                    matches!(self, AttributeValue::Bool(..))
                }

                fn is_b(&self) -> bool {
                    matches!(self, AttributeValue::B(..))
                }

                fn is_null(&self) -> bool {
                    matches!(self, AttributeValue::Null(..))
                }

                fn is_m(&self) -> bool {
                    matches!(self, AttributeValue::M(..))
                }

                fn is_l(&self) -> bool {
                    matches!(self, AttributeValue::L(..))
                }

                fn is_ss(&self) -> bool {
                    matches!(self, AttributeValue::Ss(..))
                }

                fn is_ns(&self) -> bool {
                    matches!(self, AttributeValue::Ns(..))
                }

                fn is_bs(&self) -> bool {
                    matches!(self, AttributeValue::Bs(..))
                }

                fn as_n(&self) -> Option<&str> {
                    if let AttributeValue::N(ref v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn as_s(&self) -> Option<&str> {
                    if let AttributeValue::S(ref v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn as_bool(&self) -> Option<bool> {
                    if let AttributeValue::Bool(v) = self {
                        Some(*v)
                    } else {
                        None
                    }
                }

                fn as_b(&self) -> Option<&[u8]> {
                    if let AttributeValue::B(ref v) = self {
                        Some(v.as_ref())
                    } else {
                        None
                    }
                }

                fn as_null(&self) -> Option<bool> {
                    if let AttributeValue::Null(v) = self {
                        Some(*v)
                    } else {
                        None
                    }
                }

                fn as_m(&self) -> Option<&HashMap<String, Self>> {
                    if let AttributeValue::M(ref v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn as_l(&self) -> Option<&[Self]> {
                    if let AttributeValue::L(ref v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn as_ss(&self) -> Option<&[String]> {
                    if let AttributeValue::Ss(ref v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn as_ns(&self) -> Option<&[String]> {
                    if let AttributeValue::Ns(ref v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn into_n(self) -> Option<String> {
                    if let AttributeValue::N(v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn into_s(self) -> Option<String> {
                    if let AttributeValue::S(v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn into_bool(self) -> Option<bool> {
                    if let AttributeValue::Bool(v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn into_b(self) -> Option<Vec<u8>> {
                    if let AttributeValue::B(v) = self {
                        Some(v.into_inner())
                    } else {
                        None
                    }
                }

                fn into_null(self) -> Option<bool> {
                    if let AttributeValue::Null(v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn into_m(self) -> Option<HashMap<String, Self>> {
                    if let AttributeValue::M(v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn into_l(self) -> Option<Vec<Self>> {
                    if let AttributeValue::L(v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn into_ss(self) -> Option<Vec<String>> {
                    if let AttributeValue::Ss(v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn into_ns(self) -> Option<Vec<String>> {
                    if let AttributeValue::Ns(v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn into_bs(self) -> Option<Vec<Vec<u8>>> {
                    if let AttributeValue::Bs(v) = self {
                        Some(v.into_iter().map(|b| b.into_inner()).collect())
                    } else {
                        None
                    }
                }

                fn construct_n(input: String) -> Self {
                    AttributeValue::N(input)
                }

                fn construct_s(input: String) -> Self {
                    AttributeValue::S(input)
                }

                fn construct_bool(input: bool) -> Self {
                    AttributeValue::Bool(input)
                }

                fn construct_b(input: &[u8]) -> Self {
                    AttributeValue::B($mod_name::types::Blob::new(input))
                }

                fn construct_null(input: bool) -> Self {
                    AttributeValue::Null(input)
                }

                fn construct_m(input: HashMap<String, Self>) -> Self {
                    AttributeValue::M(input)
                }

                fn construct_l(input: Vec<Self>) -> Self {
                    AttributeValue::L(input)
                }

                fn construct_ss(input: Vec<String>) -> Self {
                    AttributeValue::Ss(input)
                }

                fn construct_ns(input: Vec<String>) -> Self {
                    AttributeValue::Ns(input)
                }

                fn construct_bs(input: Vec<Vec<u8>>) -> Self {
                    let input = input.into_iter().map($mod_name::types::Blob::new).collect();
                    AttributeValue::Bs(input)
                }
            }

            /// A version of [`crate::to_attribute_value`] where the `Tout` generic is tied to
            /// [`aws-sdk-dynamodbstreams::model::AttributeValue`](AttributeValue).
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
            /// [`aws-sdk-dynamodbstreams::model::AttributeValue`](AttributeValue).
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
            /// [`aws-sdk-dynamodbstreams::model::AttributeValue`](AttributeValue).
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
            /// [`aws-sdk-dynamodbstreams::model::AttributeValue`](AttributeValue).
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
            /// [`aws-sdk-dynamodbstreams::model::AttributeValue`](AttributeValue).
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

pub(crate) use aws_sdk_streams_macro;
